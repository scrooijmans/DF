# Harbor Object Storage & Metadata Architecture (Context7 Summary)

This document dives deep into **Harbor's** architecture for storing objects in object storage (filesystem,
S3, Azure Blob, GCS) while maintaining metadata in PostgreSQL. Harbor is a textbook "metadata in DB,
blobs elsewhere" system with features like versioning, access control, replication, and immutability.

It is particularly relevant to DataForge's architecture where Parquet files (blobs) are stored in
object storage while metadata (wells, curves, versions) is stored in SQLite/PostgreSQL.

---

## 1. High-Level Architecture

### 1.1 Two-Tier Storage Model

Harbor separates **metadata** (relational, queryable) from **blobs** (immutable, content-addressed):

```
┌─────────────────────────────────────────────────────────────────┐
│                    Harbor Architecture                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Metadata Layer (PostgreSQL)                             │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │  │
│  │  │ Projects     │  │ Repositories │  │ Artifacts    │  │  │
│  │  │ Users        │  │ Tags         │  │ Blob Refs    │  │  │
│  │  │ Permissions  │  │ Manifests    │  │ Access Logs  │  │  │
│  │  │ Quotas       │  │ Replication  │  │ Scan Results │  │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘  │  │
│  └──────────────────────────────────────────────────────────┘  │
│                              │                                   │
│                              │ References (digest, path)         │
│                              ▼                                   │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Blob Storage Layer (Object Storage)                     │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │  │
│  │  │ Filesystem   │  │ S3/MinIO     │  │ Azure Blob   │  │  │
│  │  │ Local disk   │  │ S3-compat    │  │ GCS          │  │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘  │  │
│  │                                                           │  │
│  │  Content-Addressed Storage:                              │  │
│  │  - Blobs stored by SHA-256 digest                        │  │
│  │  - Immutable (never modified, only deleted)              │  │
│  │  - Deduplication (same digest = same blob)               │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 Key Components

- **Harbor Core API** (Go):
  - REST API for projects, repositories, artifacts, users, permissions.
  - Orchestrates blob uploads/downloads via Docker Distribution registry.
  - Manages metadata in PostgreSQL.
- **Docker Distribution Registry** (extended by Harbor):
  - Handles OCI/Docker image push/pull operations.
  - Manages blob storage via pluggable storage drivers.
  - Provides blob upload/download endpoints.
- **PostgreSQL**:
  - Stores all metadata: projects, repositories, artifacts, tags, manifests, blobs (references),
    users, permissions, quotas, replication policies, scan results.
- **Object Storage Backends**:
  - **Filesystem**: Local disk storage (default).
  - **S3/MinIO**: S3-compatible object storage.
  - **Azure Blob Storage**: Azure cloud storage.
  - **Google Cloud Storage (GCS)**: GCP cloud storage.

---

## 2. PostgreSQL Metadata Schema

### 2.1 Core Tables

Harbor's PostgreSQL schema tracks artifacts, blobs, and their relationships:

```sql
-- Projects (top-level namespace)
CREATE TABLE project (
    project_id SERIAL PRIMARY KEY,
    owner_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE,
    creation_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted BOOLEAN DEFAULT FALSE,
    owner_name VARCHAR(255),
    -- Metadata JSONB for flexible attributes
    metadata JSONB,
    -- Storage quota (bytes, -1 = unlimited)
    storage_limit BIGINT DEFAULT -1
);

-- Repositories (within projects)
CREATE TABLE repository (
    repository_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    project_id INTEGER NOT NULL REFERENCES project(project_id),
    description TEXT,
    pull_count INTEGER DEFAULT 0,
    star_count INTEGER DEFAULT 0,
    creation_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(project_id, name)
);

-- Artifacts (images, charts, OCI artifacts)
CREATE TABLE artifact (
    id SERIAL PRIMARY KEY,
    type VARCHAR(255) NOT NULL, -- 'IMAGE', 'CHART', 'CNAB', 'OPA', etc.
    media_type VARCHAR(255) NOT NULL,
    manifest_media_type VARCHAR(255),
    project_id INTEGER NOT NULL REFERENCES project(project_id),
    repository_id INTEGER NOT NULL REFERENCES repository(repository_id),
    repository_name VARCHAR(255) NOT NULL,
    digest VARCHAR(255) NOT NULL, -- SHA-256 digest of manifest
    size BIGINT NOT NULL, -- Total size (manifest + all blobs)
    push_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    pull_time TIMESTAMP,
    extra_attrs JSONB, -- Additional metadata
    annotations JSONB,
    UNIQUE(repository_id, digest)
);

-- Tags (human-readable references to artifacts)
CREATE TABLE tag (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repository(repository_id),
    artifact_id INTEGER NOT NULL REFERENCES artifact(id),
    name VARCHAR(255) NOT NULL,
    push_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    pull_time TIMESTAMP,
    UNIQUE(repository_id, name)
);

-- Blob References (links artifacts to their blob layers)
CREATE TABLE artifact_blob (
    id SERIAL PRIMARY KEY,
    digest VARCHAR(255) NOT NULL, -- SHA-256 digest of blob
    size BIGINT NOT NULL,
    content_type VARCHAR(255),
    creation_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Artifact-Blob relationships (many-to-many)
CREATE TABLE artifact_blob_ref (
    id SERIAL PRIMARY KEY,
    digest VARCHAR(255) NOT NULL, -- Artifact digest
    blob_digest VARCHAR(255) NOT NULL, -- Blob digest
    UNIQUE(digest, blob_digest)
);

-- Access logs (audit trail)
CREATE TABLE access_log (
    log_id SERIAL PRIMARY KEY,
    username VARCHAR(255),
    project_id INTEGER,
    repo_name VARCHAR(256),
    repo_tag VARCHAR(128),
    operation VARCHAR(20), -- 'push', 'pull', 'delete'
    op_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 2.2 Key Design Patterns

**Content-Addressed Storage**:

- Blobs are identified by **SHA-256 digest** (e.g., `sha256:a3f2b8c9...`).
- Same content = same digest = same blob (deduplication).
- Blobs are **immutable** (never modified, only deleted via garbage collection).

**Metadata Tracks References**:

- `artifact` table stores manifest digest and size.
- `artifact_blob_ref` links artifacts to their blob layers.
- `blob` table (or `artifact_blob`) tracks blob metadata (digest, size, content_type).
- **Blob storage path** is derived from digest (e.g., `blobs/sha256/a3/f2/a3f2b8c9...`).

**Versioning via Tags**:

- `tag` table provides human-readable names (`latest`, `v1.0.0`) pointing to artifact digests.
- Multiple tags can point to the same artifact (same digest).
- Artifact history is preserved via immutable digests.

---

## 3. Storage Driver Architecture

### 3.1 Pluggable Storage Drivers

Harbor (via Docker Distribution) supports multiple storage backends through **storage drivers**:

```go
// Storage driver interface (simplified)
type Driver interface {
    // Write blob to storage
    Writer(ctx context.Context, path string, append bool) (io.WriteCloser, error)

    // Read blob from storage
    Reader(ctx context.Context, path string, offset int64) (io.ReadCloser, error)

    // Delete blob
    Delete(ctx context.Context, path string) error

    // Stat blob (get metadata)
    Stat(ctx context.Context, path string) (FileInfo, error)

    // List blobs (for GC)
    Walk(ctx context.Context, path string, walkFn WalkFn) error
}
```

### 3.2 Storage Driver Configuration

**Filesystem Driver** (default):

```yaml
# registry/config.yml
storage:
  filesystem:
    rootdirectory: /var/lib/registry
```

**S3 Driver**:

```yaml
storage:
  s3:
    accesskey: YOUR_ACCESS_KEY
    secretkey: YOUR_SECRET_KEY
    region: us-east-1
    bucket: harbor-registry
    encrypt: true
    secure: true
    v4auth: true
```

**Azure Blob Driver**:

```yaml
storage:
  azure:
    accountname: repodocker
    accountkey: YOUR_ACCOUNT_KEY
    container: images
    realm: core.windows.net # or core.chinacloudapi.cn for China
```

**GCS Driver**:

```yaml
storage:
  gcs:
    bucket: harbor-registry
    keyfile: /path/to/service-account-key.json
    rootdirectory: /registry
```

### 3.3 Blob Storage Path Structure

Blobs are stored using a **content-addressable path** derived from their digest:

```
Filesystem:
/var/lib/registry/
└── docker/
    └── registry/
        └── v2/
            └── blobs/
                └── sha256/
                    ├── a3/
                    │   └── f2/
                    │       └── a3f2b8c9d1e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b
                    └── 7b/
                        └── 1c/
                            └── 7b1c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c

S3:
s3://harbor-registry/
└── docker/
    └── registry/
        └── v2/
            └── blobs/
                └── sha256/
                    ├── a3/f2/a3f2b8c9...
                    └── 7b/1c/7b1c3d4e...
```

**Path derivation**:

- Digest: `sha256:a3f2b8c9d1e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b`
- Path: `blobs/sha256/a3/f2/a3f2b8c9d1e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b`
- First 2 chars (`a3`) = directory level 1.
- Next 2 chars (`f2`) = directory level 2.
- Full digest = filename.

This structure:

- Prevents too many files in a single directory.
- Enables efficient lookup (O(1) path derivation from digest).
- Supports deduplication (same digest = same path).

---

## 4. Call Stack: Pushing Objects to Storage

### 4.1 Complete Push Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  Client: docker push harbor.example.com/project/app:latest         │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ HTTP POST /v2/project/app/blobs/uploads/
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Harbor Core API: Authentication & Authorization                │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Validate JWT token / session                              │  │
│  │  - Check project permissions (push access)                   │  │
│  │  - Verify storage quota (if enabled)                         │  │
│  │  - Create/update repository record in PostgreSQL             │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Proxy to Docker Distribution Registry
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Docker Distribution: Initiate Blob Upload                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  POST /v2/{name}/blobs/uploads/                              │  │
│  │  Response: 202 Accepted                                      │  │
│  │  Location: /v2/{name}/blobs/uploads/{uuid}                   │  │
│  │  Upload-UUID: {uuid}                                         │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Client uploads blob chunks
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Docker Distribution: Upload Blob Chunks                        │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  PATCH /v2/{name}/blobs/uploads/{uuid}                       │  │
│  │  Content-Range: bytes 0-1048575/52428800                     │  │
│  │  Body: <blob chunk data>                                     │  │
│  │                                                               │  │
│  │  Storage Driver:                                             │  │
│  │  - driver.Writer(ctx, "blobs/sha256/a3/f2/...", false)      │  │
│  │  - Write chunk to temporary location                         │  │
│  │  - Append to blob file                                       │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Finalize upload with digest
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  4. Docker Distribution: Finalize Blob Upload                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  PUT /v2/{name}/blobs/uploads/{uuid}                         │  │
│  │  Digest: sha256:a3f2b8c9...                                  │  │
│  │                                                               │  │
│  │  Storage Driver:                                             │  │
│  │  - Compute SHA-256 digest of uploaded blob                   │  │
│  │  - Verify digest matches client-provided digest              │  │
│  │  - Move blob from temp location to final path:               │  │
│  │    blobs/sha256/a3/f2/a3f2b8c9...                            │  │
│  │  - If blob already exists (same digest), skip write          │  │
│  │    (deduplication)                                           │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Upload manifest
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  5. Docker Distribution: Upload Manifest                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  PUT /v2/{name}/manifests/{tag}                              │  │
│  │  Content-Type: application/vnd.docker.distribution.manifest  │  │
│  │  Body: {                                                      │  │
│  │    "schemaVersion": 2,                                        │  │
│  │    "mediaType": "application/vnd.docker.distribution...",    │  │
│  │    "config": {                                                │  │
│  │      "mediaType": "application/vnd.docker.container...",     │  │
│  │      "size": 1234,                                            │  │
│  │      "digest": "sha256:config-digest"                        │  │
│  │    },                                                         │  │
│  │    "layers": [                                                │  │
│  │      {                                                        │  │
│  │        "mediaType": "application/vnd.docker.image.rootfs...",│  │
│  │        "size": 52428800,                                      │  │
│  │        "digest": "sha256:a3f2b8c9..."                        │  │
│  │      }                                                        │  │
│  │    ]                                                          │  │
│  │  }                                                            │  │
│  │                                                               │  │
│  │  Storage Driver:                                             │  │
│  │  - Store manifest as blob:                                   │  │
│  │    blobs/sha256/{manifest-digest}                            │  │
│  │  - Also store tag reference:                                 │  │
│  │    repositories/{project}/{repo}/_manifests/tags/{tag}/      │  │
│  │      current/link -> ../../revisions/sha256/{digest}/link    │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Webhook notification
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  6. Harbor Core API: Update Metadata in PostgreSQL                 │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Parse manifest JSON                                        │  │
│  │  - Extract artifact digest, size, media type                 │  │
│  │  - Extract blob digests from manifest layers                 │  │
│  │  - Begin transaction:                                        │  │
│  │    INSERT INTO artifact (...)                                │  │
│  │    INSERT INTO tag (repository_id, artifact_id, name)        │  │
│  │    INSERT INTO artifact_blob_ref (digest, blob_digest)      │  │
│  │    UPDATE repository SET update_time = NOW()                 │  │
│  │    UPDATE project SET storage_used = storage_used + size     │  │
│  │  - Commit transaction                                        │  │
│  │  - Trigger webhook: artifact.pushed                          │  │
│  │  - Queue replication job (if policy matches)                 │  │
│  │  - Queue vulnerability scan job (if auto-scan enabled)       │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 4.2 Key Implementation Details

**Blob Upload (Monolithic vs Chunked)**:

- Harbor/Distribution supports **monolithic** (single PUT) and **chunked** (PATCH) uploads.
- Chunked uploads use a temporary UUID location, then finalize to the digest-based path.
- Storage driver handles both patterns transparently.

**Deduplication**:

- Before writing a blob, check if a blob with the same digest already exists.
- If exists, skip write (return success immediately).
- Multiple artifacts can reference the same blob (e.g., shared base layers).

**Atomicity**:

- Blob writes are atomic (write to temp, then rename/move to final location).
- Manifest writes are atomic (write manifest blob, then update tag link).
- Metadata updates in PostgreSQL use transactions to ensure consistency.

**Quota Enforcement**:

- Before accepting blob upload, check project storage quota.
- Update `project.storage_used` atomically with artifact creation.
- Reject uploads that would exceed quota.

---

## 5. Call Stack: Retrieving Objects from Storage

### 5.1 Complete Pull Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  Client: docker pull harbor.example.com/project/app:latest         │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ HTTP GET /v2/project/app/manifests/latest
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Harbor Core API: Authentication & Authorization                │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Validate JWT token / session                              │  │
│  │  - Check project permissions (pull access)                   │  │
│  │  - Check if project/repository is public                     │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Query PostgreSQL for tag → artifact mapping
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Harbor Core API: Resolve Tag to Artifact                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  SELECT a.digest, a.manifest_media_type, a.size              │  │
│  │  FROM artifact a                                             │  │
│  │  JOIN tag t ON t.artifact_id = a.id                          │  │
│  │  WHERE t.repository_id = ? AND t.name = ?                    │  │
│  │                                                               │  │
│  │  If tag not found, return 404                                │  │
│  │  If found, return artifact digest                            │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Proxy to Docker Distribution Registry
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Docker Distribution: Retrieve Manifest                         │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  GET /v2/{name}/manifests/{tag}                              │  │
│  │                                                               │  │
│  │  Storage Driver:                                             │  │
│  │  - Resolve tag to manifest digest:                           │  │
│  │    repositories/{project}/{repo}/_manifests/tags/{tag}/      │  │
│  │      current/link -> ../../revisions/sha256/{digest}/link    │  │
│  │  - Read manifest blob:                                       │  │
│  │    blobs/sha256/{manifest-digest}                            │  │
│  │  - Return manifest JSON                                      │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Client parses manifest, requests blob layers
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  4. Docker Distribution: Retrieve Blob Layers                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  GET /v2/{name}/blobs/{digest}                               │  │
│  │  Range: bytes=0-1048575  (optional, for chunked download)    │  │
│  │                                                               │  │
│  │  Storage Driver:                                             │  │
│  │  - Derive blob path from digest:                             │  │
│  │    blobs/sha256/a3/f2/a3f2b8c9...                            │  │
│  │  - driver.Reader(ctx, path, offset)                          │  │
│  │  - Stream blob data to client                                │  │
│  │  - Support Range requests for partial content                │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Update access logs
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  5. Harbor Core API: Update Access Logs                            │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - INSERT INTO access_log (                                  │  │
│  │      username, project_id, repo_name, repo_tag,              │  │
│  │      operation, op_time                                      │  │
│  │    ) VALUES (                                                │  │
│  │      'user@example.com', 5, 'project/app', 'latest',         │  │
│  │      'pull', NOW()                                           │  │
│  │    )                                                         │  │
│  │  - UPDATE artifact SET pull_time = NOW()                    │  │
│  │  - UPDATE repository SET pull_count = pull_count + 1         │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 5.2 Key Implementation Details

**Manifest Resolution**:

- Tag → digest resolution can happen in PostgreSQL (Harbor metadata) or in storage (Distribution tag links).
- Harbor prefers PostgreSQL for consistency and auditability.
- Distribution falls back to storage-based tag resolution if needed.

**Blob Streaming**:

- Blobs are streamed directly from object storage to client (no buffering in Harbor).
- Supports HTTP Range requests for partial content (resume downloads).
- Storage driver handles streaming efficiently (S3 multipart, filesystem seek).

**Caching**:

- Harbor can cache frequently accessed manifests in Redis.
- Blob data is not cached (too large), but blob metadata (digest, size) can be cached.

**Access Control**:

- Permission checks happen in Harbor Core API before proxying to Distribution.
- Distribution itself is typically configured to trust Harbor's auth decisions.

---

## 6. Versioning & Immutability

### 6.1 Content-Addressed Versioning

Harbor uses **content-addressing** for versioning:

- **Digest = Version**: Each artifact/blob is identified by its SHA-256 digest.
- **Immutable**: Once stored, a blob with a given digest never changes.
- **Tags are Mutable**: Tags (`latest`, `v1.0.0`) can be moved to point to different digests.

```sql
-- Example: Tag "latest" points to different artifacts over time
-- Artifact 1 (digest: sha256:abc...)
INSERT INTO tag (repository_id, artifact_id, name) VALUES (1, 1, 'latest');

-- Later: Tag "latest" moved to Artifact 2 (digest: sha256:def...)
UPDATE tag SET artifact_id = 2 WHERE repository_id = 1 AND name = 'latest';

-- Artifact 1 still exists (immutable), just not tagged as "latest"
```

### 6.2 Tag Retention Policies

Harbor supports **tag retention policies** to automatically clean up old artifacts:

```sql
-- Retention policy table (simplified)
CREATE TABLE retention_policy (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES project(project_id),
    name VARCHAR(255) NOT NULL,
    rules JSONB NOT NULL, -- Retention rules (keep latest N, keep by pattern, etc.)
    trigger JSONB, -- Schedule (cron) or event-based
    enabled BOOLEAN DEFAULT TRUE
);
```

**Retention Rules** (examples):

- Keep latest 10 tags.
- Keep tags matching pattern `v[0-9]*`.
- Delete untagged artifacts older than 30 days.
- Keep artifacts with label `production=true`.

**Garbage Collection**:

- Retention policies trigger **garbage collection** jobs.
- GC marks artifacts/tags for deletion based on rules.
- GC deletes blobs only if no artifacts reference them (reference counting).
- GC updates PostgreSQL metadata (soft delete: `deleted = TRUE`).

---

## 7. Access Control & Security

### 7.1 Role-Based Access Control (RBAC)

```sql
-- User roles in projects
CREATE TABLE project_member (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES project(project_id),
    entity_id INTEGER NOT NULL, -- User ID or group ID
    entity_type VARCHAR(255) NOT NULL, -- 'u' (user) or 'g' (group)
    role_id INTEGER NOT NULL, -- 1=admin, 2=developer, 3=guest, etc.
    creation_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Permissions matrix (simplified)
-- role_id 1 (admin): push, pull, delete, manage members
-- role_id 2 (developer): push, pull
-- role_id 3 (guest): pull only
```

**Access Control Flow**:

1. Client authenticates (JWT token or session).
2. Harbor Core API resolves user identity.
3. Check `project_member` table for user's role in project.
4. Check role permissions for requested operation (push/pull/delete).
5. Allow or deny request.

### 7.2 Content Trust (Image Signing)

Harbor supports **Notary** (or **Cosign**) for image signing:

```sql
-- Signature metadata (simplified)
CREATE TABLE artifact_accessory (
    id SERIAL PRIMARY KEY,
    artifact_id INTEGER NOT NULL REFERENCES artifact(id),
    subject_artifact_id INTEGER NOT NULL REFERENCES artifact(id),
    digest VARCHAR(255) NOT NULL,
    subject_artifact_digest VARCHAR(255) NOT NULL,
    type VARCHAR(255) NOT NULL, -- 'signature.cosign', 'signature.notary', etc.
    size BIGINT,
    creation_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**Signing Flow**:

1. User signs artifact using `notation` or `cosign` CLI.
2. Signature is stored as an **accessory artifact** (OCI artifact type).
3. Signature references the signed artifact via `subject_artifact_digest`.
4. Harbor validates signatures on pull (if content trust enabled).

---

## 8. Replication

### 8.1 Replication Architecture

Harbor supports **cross-registry replication** (push/pull artifacts between registries):

```sql
-- Replication policy
CREATE TABLE replication_policy (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    src_registry_id INTEGER, -- Source registry (NULL = local)
    dest_registry_id INTEGER NOT NULL, -- Destination registry
    dest_namespace VARCHAR(255),
    trigger JSONB, -- 'manual', 'event_based', 'scheduled'
    filters JSONB, -- Which artifacts to replicate (by project, tag pattern, etc.)
    enabled BOOLEAN DEFAULT TRUE
);

-- Replication execution
CREATE TABLE replication_execution (
    id SERIAL PRIMARY KEY,
    policy_id INTEGER NOT NULL REFERENCES replication_policy(id),
    status VARCHAR(255) NOT NULL, -- 'running', 'succeed', 'failed'
    trigger VARCHAR(255) NOT NULL,
    start_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    end_time TIMESTAMP
);

-- Replication task (per artifact)
CREATE TABLE replication_task (
    id SERIAL PRIMARY KEY,
    execution_id INTEGER NOT NULL REFERENCES replication_execution(id),
    resource_type VARCHAR(255) NOT NULL, -- 'artifact', 'chart'
    src_resource VARCHAR(255) NOT NULL, -- Source artifact digest
    dst_resource VARCHAR(255), -- Destination artifact digest
    operation VARCHAR(255) NOT NULL, -- 'copy', 'delete'
    status VARCHAR(255) NOT NULL,
    start_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    end_time TIMESTAMP
);
```

### 8.2 Replication Call Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│  Trigger: Artifact Pushed (Event-Based Replication)                │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ Webhook: artifact.pushed
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Harbor Core API: Check Replication Policies                    │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  SELECT * FROM replication_policy                            │  │
│  │  WHERE enabled = TRUE                                        │  │
│  │    AND trigger->>'type' = 'event_based'                      │  │
│  │    AND filters->>'project_id' = ?                            │  │
│  │    AND filters->>'tag_pattern' MATCHES ?                     │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Create replication execution
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Harbor Job Service: Queue Replication Job                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  INSERT INTO replication_execution (...)                     │  │
│  │  INSERT INTO replication_task (execution_id, src_resource)   │  │
│  │  Queue job: replication.CopyArtifact                         │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Worker picks up job
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Replication Worker: Pull from Source                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - GET /v2/{name}/manifests/{tag} (from source registry)    │  │
│  │  - Parse manifest, extract blob digests                      │  │
│  │  - For each blob:                                            │  │
│  │    GET /v2/{name}/blobs/{digest} (stream to temp)           │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Push to destination
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  4. Replication Worker: Push to Destination                        │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - POST /v2/{name}/blobs/uploads/ (to dest registry)        │  │
│  │  - PATCH /v2/{name}/blobs/uploads/{uuid} (upload blob)      │  │
│  │  - PUT /v2/{name}/blobs/uploads/{uuid}?digest=... (finalize)│  │
│  │  - PUT /v2/{name}/manifests/{tag} (upload manifest)         │  │
│  │  - Update replication_task status = 'succeed'                │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 9. Design Patterns for DataForge

### 9.1 Content-Addressed Blob Storage

**Harbor Pattern**:

- Blobs stored by SHA-256 digest: `blobs/sha256/{first2}/{next2}/{full_digest}`.
- Immutable: same digest = same content (deduplication).
- Metadata in PostgreSQL tracks blob references.

**DataForge Application**:

- Store Parquet files by SHA-256 hash: `blobs/{hash[0:2]}/{hash[2:4]}/{hash}.parquet`.
- Multiple curves can reference the same Parquet file (if data is identical).
- SQLite/PostgreSQL tracks which curves reference which blob hashes.

### 9.2 Metadata Tracks References

**Harbor Pattern**:

- `artifact` table stores manifest digest and size.
- `artifact_blob_ref` links artifacts to blob layers.
- Blob storage path derived from digest (no path stored in DB).

**DataForge Application**:

- `curves` table stores curve metadata and `blob_hash` reference.
- `blob_registry` table tracks blob metadata (hash, size, created_at).
- Blob path derived from hash (no path stored in DB).

### 9.3 Versioning via Immutable Digests

**Harbor Pattern**:

- Artifacts identified by digest (immutable version).
- Tags provide mutable, human-readable references.
- History preserved via immutable digests.

**DataForge Application**:

- Curve versions identified by blob hash (immutable).
- Curve names/tags provide mutable references.
- History preserved via immutable blob hashes.

### 9.4 Quota & Access Control

**Harbor Pattern**:

- Project-level storage quotas tracked in PostgreSQL.
- RBAC for push/pull/delete operations.
- Access logs for audit trail.

**DataForge Application**:

- Workspace-level storage quotas (if needed).
- RBAC for workspace access (already implemented).
- Access logs for audit trail (optional).

### 9.5 Garbage Collection

**Harbor Pattern**:

- GC marks artifacts/tags for deletion based on retention policies.
- GC deletes blobs only if no artifacts reference them (reference counting).
- GC updates PostgreSQL metadata.

**DataForge Application**:

- GC can delete Parquet blobs only if no curves reference them.
- GC can delete old curve versions based on retention policies.
- GC updates SQLite/PostgreSQL metadata.

---

## 10. Architecture & Design for Change

### 10.1 Storage Driver Abstraction

**Harbor's Design**:

- Storage drivers implement a common interface (`Driver`).
- Can swap filesystem → S3 → Azure without changing application code.
- Configuration-driven (YAML config selects driver).

**Benefits for DataForge**:

- Start with filesystem storage, migrate to S3 later.
- Support multiple backends (local disk, S3, MinIO) via abstraction.
- Test with filesystem, deploy with S3.

### 10.2 Metadata-First Design

**Harbor's Design**:

- All queries go through PostgreSQL (fast, indexed).
- Blob storage is "dumb" (just content-addressed files).
- Metadata layer handles versioning, access control, quotas.

**Benefits for DataForge**:

- Fast queries (list curves, search by well, filter by date) via SQLite/PostgreSQL.
- Blob storage is simple (just Parquet files by hash).
- Metadata layer handles versioning, access control, quotas.

### 10.3 Separation of Concerns

**Harbor's Layers**:

1. **Harbor Core API**: Business logic, metadata, auth, quotas.
2. **Docker Distribution**: Blob storage, manifest handling, OCI spec compliance.
3. **Storage Driver**: Filesystem/S3/Azure/GCS abstraction.

**DataForge's Layers** (similar):

1. **DataForge Core API**: Business logic, metadata, auth, quotas.
2. **Blob Storage Service**: Parquet file storage, content-addressing.
3. **Storage Driver**: Filesystem/S3/MinIO abstraction.

### 10.4 Immutability & Deduplication

**Harbor's Design**:

- Blobs are immutable (never modified, only deleted).
- Same content = same digest = same blob (automatic deduplication).
- Multiple artifacts can share the same blob.

**DataForge Application**:

- Parquet files are immutable (never modified, only deleted).
- Same data = same hash = same Parquet file (automatic deduplication).
- Multiple curves can share the same Parquet file (if data is identical).

---

## 11. Summary

Harbor's architecture demonstrates a **textbook "metadata in DB, blobs elsewhere"** pattern:

### ✅ Key Strengths

1. **Clear Separation**: Metadata (PostgreSQL) vs. blobs (object storage).
2. **Content-Addressed Storage**: Blobs identified by SHA-256 digest (immutable, deduplicated).
3. **Pluggable Storage Drivers**: Support filesystem, S3, Azure, GCS via abstraction.
4. **Metadata-First Queries**: All queries go through PostgreSQL (fast, indexed).
5. **Versioning via Immutability**: Digests provide immutable versions; tags provide mutable references.
6. **Access Control**: RBAC in metadata layer; blob storage is "dumb".
7. **Replication**: Cross-registry replication via metadata-driven jobs.

### 📋 Patterns for DataForge

1. **Content-Addressed Parquet Storage**: Store Parquet files by SHA-256 hash.
2. **Metadata Tracks References**: SQLite/PostgreSQL tracks which curves reference which blob hashes.
3. **Storage Driver Abstraction**: Support filesystem, S3, MinIO via pluggable drivers.
4. **Immutability**: Parquet files are immutable (never modified, only deleted).
5. **Deduplication**: Same data = same hash = same Parquet file (automatic).
6. **Garbage Collection**: Delete Parquet blobs only if no curves reference them (reference counting).

This architecture provides a solid foundation for DataForge's blob storage needs while maintaining
fast metadata queries and supporting multiple storage backends.
