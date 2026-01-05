# Harbor Blob Download Architecture: Pulling Changes & Downloading Referenced Blobs (Context7 Summary)

This document dives deep into **Harbor's architecture and call stack for downloading blobs** that are referenced by pulled changes (artifacts/manifests). This is particularly relevant to DataForge's sync architecture where Parquet files (blobs) need to be downloaded when syncing changes from the server.

It builds on:

- `harbor-object-storage-architecture.md`

and provides detailed call stacks and implementation patterns for the critical "pull changes → identify missing blobs → download blobs" flow.

---

## 1. High-Level Architecture Pattern

### 1.1 The Problem

When pulling changes from Harbor (or any container registry), the client receives:

1. **Manifest** (metadata about the artifact).
2. **Blob references** (digests of blob layers that need to be downloaded).
3. **Missing blob list** (blobs the client doesn't have locally).

The client must:

- **Parse** the manifest to extract blob digests.
- **Check** which blobs are already present locally (deduplication).
- **Download** missing blobs from object storage.
- **Verify** blob integrity (SHA-256 digest).
- **Store** blobs in local storage.

### 1.2 The Solution: Manifest → Blob References → Download

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Pull Changes from Server                          │
│  GET /api/sync/pull?workspace_id=X&from_version=Y                   │
│  Response: {                                                         │
│    changes: [                                                        │
│      {entity_type: "artifact", action: "create", data: {...}},      │
│    ],                                                                │
│    missing_blobs: ["sha256:abc...", "sha256:def..."]                │
│  }                                                                   │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ Parse manifest, extract blob digests
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Identify Missing Blobs                                              │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  FOR each blob digest in manifest:                           │  │
│  │    - Check if blob exists locally (by digest)                │  │
│  │    - IF not exists: Add to missing_blobs list                │  │
│  └──────────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ Download missing blobs
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Download Blobs from Object Storage                                  │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  FOR each missing blob:                                      │  │
│  │    - GET /v2/{name}/blobs/{digest}                          │  │
│  │    - Stream blob data                                        │  │
│  │    - Verify SHA-256 digest                                   │  │
│  │    - Store in local blob storage                             │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Harbor's Blob Download Architecture

### 2.1 Docker Distribution API

Harbor uses the **Docker Distribution API** (OCI specification) for blob downloads:

```
GET /v2/{name}/blobs/{digest}
```

Where:

- `{name}` = repository name (e.g., `project/app`).
- `{digest}` = SHA-256 digest of the blob (e.g., `sha256:a3f2b8c9...`).

### 2.2 Storage Driver Abstraction

Harbor's storage drivers handle blob retrieval transparently:

- **Filesystem Driver**: Reads from local disk.
- **S3 Driver**: Downloads from S3-compatible storage.
- **Azure Blob Driver**: Downloads from Azure Blob Storage.
- **GCS Driver**: Downloads from Google Cloud Storage.

All drivers implement the same interface:

```go
type Driver interface {
    // Read blob from storage
    Reader(ctx context.Context, path string, offset int64) (io.ReadCloser, error)

    // Stat blob (get metadata: size, digest)
    Stat(ctx context.Context, path string) (FileInfo, error)
}
```

---

## 3. Complete Call Stack: Pull Changes & Download Blobs

### 3.1 High-Level Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  1. Pull Changes from Server                                        │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  GET /api/sync/pull?workspace_id=X&from_version=Y            │  │
│  │  Response: {                                                  │  │
│  │    changes: [                                                 │  │
│  │      {                                                        │  │
│  │        entity_type: "artifact",                               │  │
│  │        entity_id: "...",                                      │  │
│  │        action: "create",                                      │  │
│  │        data: {                                                │  │
│  │          digest: "sha256:manifest-digest",                    │  │
│  │          manifest: {...}  // Full manifest JSON               │  │
│  │        }                                                      │  │
│  │      }                                                        │  │
│  │    ],                                                         │  │
│  │    missing_blobs: ["sha256:abc...", "sha256:def..."]         │  │
│  │  }                                                            │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Parse manifest, extract blob digests
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Parse Manifest & Extract Blob References                        │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  const manifest = JSON.parse(change.data.manifest);          │  │
│  │  const blobDigests = [];                                     │  │
│  │                                                               │  │
│  │  // Extract config blob digest                               │  │
│  │  if (manifest.config) {                                      │  │
│  │    blobDigests.push(manifest.config.digest);                 │  │
│  │  }                                                            │  │
│  │                                                               │  │
│  │  // Extract layer blob digests                               │  │
│  │  if (manifest.layers) {                                      │  │
│  │    manifest.layers.forEach(layer => {                        │  │
│  │      blobDigests.push(layer.digest);                         │  │
│  │    });                                                        │  │
│  │  }                                                            │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Check which blobs are missing locally
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Check Local Blob Storage                                        │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  const missingBlobs = [];                                    │  │
│  │                                                               │  │
│  │  FOR each blobDigest in blobDigests:                         │  │
│  │    - Check if blob exists locally:                           │  │
│  │      SELECT 1 FROM blob_registry                             │  │
│  │      WHERE digest = ?                                        │  │
│  │                                                               │  │
│  │    - IF not exists:                                          │  │
│  │        missingBlobs.push(blobDigest);                        │  │
│  │    - ELSE:                                                   │  │
│  │        Skip (blob already downloaded)                        │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Download missing blobs
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  4. Download Missing Blobs                                          │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  FOR each missingBlob in missingBlobs:                       │  │
│  │    a. Request blob download                                  │  │
│  │    b. Stream blob data                                       │  │
│  │    c. Verify digest                                          │  │
│  │    d. Store in local storage                                 │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Update metadata
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  5. Update Local Metadata                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  FOR each downloaded blob:                                   │  │
│  │    - INSERT INTO blob_registry (digest, size, path, ...)    │  │
│  │    - Link blob to artifact:                                  │  │
│  │      INSERT INTO artifact_blob_ref (artifact_id, blob_digest)│  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 4. Detailed Call Stack: Downloading a Single Blob

### 4.1 Blob Download Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  Client: Request Blob Download                                      │
│  GET /v2/{name}/blobs/{digest}                                      │
│  Headers:                                                            │
│    Authorization: Bearer {token}                                    │
│    Range: bytes=0-1048575  (optional, for chunked download)         │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ Harbor Core API: Authentication
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
                          │ Proxy to Docker Distribution Registry
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Docker Distribution: Resolve Blob Path                          │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Parse digest: sha256:a3f2b8c9...                          │  │
│  │  - Derive blob path:                                         │  │
│  │    blobs/sha256/a3/f2/a3f2b8c9...                            │  │
│  │  - Check if blob exists in storage                           │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Storage Driver: Open Blob Reader
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Storage Driver: Open Blob Reader                                │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Filesystem Driver:                                          │  │
│  │    - Open file: /var/lib/registry/blobs/sha256/a3/f2/...    │  │
│  │    - Return file reader (with seek support)                 │  │
│  │                                                               │  │
│  │  S3 Driver:                                                  │  │
│  │    - Generate presigned URL (if configured)                 │  │
│  │    - Or: s3.GetObject(bucket, key)                          │  │
│  │    - Return streaming reader                                │  │
│  │                                                               │  │
│  │  Azure Blob Driver:                                          │  │
│  │    - azure.GetBlob(container, blobName)                     │  │
│  │    - Return streaming reader                                │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Stream blob data to client
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  4. Stream Blob Data to Client                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Read blob data in chunks (e.g., 64KB)                    │  │
│  │  - Stream to HTTP response                                   │  │
│  │  - Support Range requests (resume downloads)                 │  │
│  │  - Set Content-Type, Content-Length headers                  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Client: Receive & Verify Blob
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  5. Client: Receive & Verify Blob                                   │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Stream blob data to temporary file                        │  │
│  │  - Compute SHA-256 digest while downloading                  │  │
│  │  - Verify digest matches expected digest                     │  │
│  │  - Move to final location:                                   │  │
│  │    blobs/sha256/a3/f2/a3f2b8c9...                            │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 5. Manifest Parsing & Blob Reference Extraction

### 5.1 Docker/OCI Manifest Structure

```json
{
	"schemaVersion": 2,
	"mediaType": "application/vnd.docker.distribution.manifest.v2+json",
	"config": {
		"mediaType": "application/vnd.docker.container.image.v1+json",
		"size": 1500,
		"digest": "sha256:244718b7c845e40677786874f5e889f2c289292a67d34828a745782f21c9891a"
	},
	"layers": [
		{
			"mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip",
			"size": 32000000,
			"digest": "sha256:4c4458b57c2187a4b03281b6524e65015799765819c578140e665c136a563c30"
		},
		{
			"mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip",
			"size": 15000000,
			"digest": "sha256:7b1c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c"
		}
	]
}
```

### 5.2 Extract Blob Digests from Manifest

```typescript
interface Manifest {
	schemaVersion: number;
	mediaType: string;
	config?: {
		mediaType: string;
		size: number;
		digest: string;
	};
	layers?: Array<{
		mediaType: string;
		size: number;
		digest: string;
	}>;
}

function extractBlobDigests(manifest: Manifest): string[] {
	const digests: string[] = [];

	// Extract config blob digest
	if (manifest.config?.digest) {
		digests.push(manifest.config.digest);
	}

	// Extract layer blob digests
	if (manifest.layers) {
		manifest.layers.forEach((layer) => {
			if (layer.digest) {
				digests.push(layer.digest);
			}
		});
	}

	return digests;
}

// Example usage
const manifest: Manifest = JSON.parse(change.data.manifest);
const blobDigests = extractBlobDigests(manifest);
// Result: [
//   "sha256:244718b7c845e40677786874f5e889f2c289292a67d34828a745782f21c9891a",
//   "sha256:4c4458b57c2187a4b03281b6524e65015799765819c578140e665c136a563c30",
//   "sha256:7b1c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c"
// ]
```

---

## 6. Blob Download Implementation

### 6.1 Check Local Blob Storage

```typescript
async function checkLocalBlobs(db: Database, blobDigests: string[]): Promise<string[]> {
	const missingBlobs: string[] = [];

	for (const digest of blobDigests) {
		// Check if blob exists locally
		const exists = await db.get('SELECT 1 FROM blob_registry WHERE digest = ?', [digest]);

		if (!exists) {
			missingBlobs.push(digest);
		}
	}

	return missingBlobs;
}
```

### 6.2 Download Blob with Verification

```typescript
import crypto from 'crypto';
import fs from 'fs';
import path from 'path';

async function downloadBlob(
	client: HttpClient,
	repositoryName: string,
	digest: string,
	localBlobPath: string
): Promise<void> {
	// Derive blob path from digest
	// sha256:a3f2b8c9... -> blobs/sha256/a3/f2/a3f2b8c9...
	const digestHash = digest.replace('sha256:', '');
	const blobPath = `blobs/sha256/${digestHash.slice(0, 2)}/${digestHash.slice(2, 4)}/${digestHash}`;

	// Ensure directory exists
	const blobDir = path.dirname(localBlobPath);
	await fs.promises.mkdir(blobDir, { recursive: true });

	// Request blob download
	const response = await client.get(`/v2/${repositoryName}/blobs/${digest}`, {
		responseType: 'stream' // Stream response for large files
	});

	// Create temporary file for download
	const tempPath = `${localBlobPath}.tmp`;
	const writeStream = fs.createWriteStream(tempPath);

	// Create SHA-256 hash for verification
	const hash = crypto.createHash('sha256');

	// Stream blob data to file and compute hash
	return new Promise((resolve, reject) => {
		response.data.on('data', (chunk: Buffer) => {
			writeStream.write(chunk);
			hash.update(chunk);
		});

		response.data.on('end', async () => {
			writeStream.end();

			// Verify digest
			const computedDigest = `sha256:${hash.digest('hex')}`;
			if (computedDigest !== digest) {
				// Delete corrupted blob
				await fs.promises.unlink(tempPath);
				reject(new Error(`Digest mismatch: expected ${digest}, got ${computedDigest}`));
				return;
			}

			// Move temporary file to final location (atomic)
			await fs.promises.rename(tempPath, localBlobPath);

			resolve();
		});

		response.data.on('error', (error: Error) => {
			writeStream.destroy();
			fs.promises.unlink(tempPath).catch(() => {});
			reject(error);
		});
	});
}
```

### 6.3 Batch Blob Download with Progress

```typescript
interface BlobDownloadProgress {
	digest: string;
	status: 'pending' | 'downloading' | 'completed' | 'failed';
	bytesDownloaded: number;
	totalBytes: number;
	error?: string;
}

async function downloadBlobs(
	client: HttpClient,
	repositoryName: string,
	missingBlobs: string[],
	localBlobStorage: string,
	onProgress?: (progress: BlobDownloadProgress) => void
): Promise<void> {
	const progress: Map<string, BlobDownloadProgress> = new Map();

	// Initialize progress tracking
	missingBlobs.forEach((digest) => {
		progress.set(digest, {
			digest,
			status: 'pending',
			bytesDownloaded: 0,
			totalBytes: 0
		});
	});

	// Download blobs in parallel (with concurrency limit)
	const concurrency = 3; // Download 3 blobs at a time
	const semaphore = new Semaphore(concurrency);

	const downloadPromises = missingBlobs.map(async (digest) => {
		await semaphore.acquire();

		try {
			// Update status
			progress.set(digest, {
				...progress.get(digest)!,
				status: 'downloading'
			});
			onProgress?.(progress.get(digest)!);

			// Derive local blob path
			const digestHash = digest.replace('sha256:', '');
			const localBlobPath = path.join(
				localBlobStorage,
				'blobs',
				'sha256',
				digestHash.slice(0, 2),
				digestHash.slice(2, 4),
				digestHash
			);

			// Download blob
			await downloadBlob(client, repositoryName, digest, localBlobPath);

			// Update status
			progress.set(digest, {
				...progress.get(digest)!,
				status: 'completed',
				bytesDownloaded: (await fs.promises.stat(localBlobPath)).size,
				totalBytes: (await fs.promises.stat(localBlobPath)).size
			});
			onProgress?.(progress.get(digest)!);
		} catch (error) {
			// Update status
			progress.set(digest, {
				...progress.get(digest)!,
				status: 'failed',
				error: error.message
			});
			onProgress?.(progress.get(digest)!);
			throw error;
		} finally {
			semaphore.release();
		}
	});

	await Promise.all(downloadPromises);
}
```

---

## 7. Storage Driver Implementations

### 7.1 Filesystem Driver

```go
// Filesystem driver implementation (simplified)
type filesystemDriver struct {
    rootDirectory string
}

func (d *filesystemDriver) Reader(ctx context.Context, path string, offset int64) (io.ReadCloser, error) {
    fullPath := filepath.Join(d.rootDirectory, path)

    file, err := os.Open(fullPath)
    if err != nil {
        return nil, err
    }

    // Seek to offset (for Range requests)
    if offset > 0 {
        _, err = file.Seek(offset, io.SeekStart)
        if err != nil {
            file.Close()
            return nil, err
        }
    }

    return file, nil
}

func (d *filesystemDriver) Stat(ctx context.Context, path string) (FileInfo, error) {
    fullPath := filepath.Join(d.rootDirectory, path)

    stat, err := os.Stat(fullPath)
    if err != nil {
        return nil, err
    }

    return &fileInfo{
        size: stat.Size(),
        modTime: stat.ModTime(),
    }, nil
}
```

### 7.2 S3 Driver

```go
// S3 driver implementation (simplified)
type s3Driver struct {
    bucket string
    s3Client *s3.S3
}

func (d *s3Driver) Reader(ctx context.Context, path string, offset int64) (io.ReadCloser, error) {
    // S3 key is the blob path
    key := path

    // Get object with Range request (if offset > 0)
    input := &s3.GetObjectInput{
        Bucket: aws.String(d.bucket),
        Key: aws.String(key),
    }

    if offset > 0 {
        // Range: bytes=offset-
        rangeHeader := fmt.Sprintf("bytes=%d-", offset)
        input.Range = aws.String(rangeHeader)
    }

    result, err := d.s3Client.GetObjectWithContext(ctx, input)
    if err != nil {
        return nil, err
    }

    return result.Body, nil
}

func (d *s3Driver) Stat(ctx context.Context, path string) (FileInfo, error) {
    key := path

    headInput := &s3.HeadObjectInput{
        Bucket: aws.String(d.bucket),
        Key: aws.String(key),
    }

    result, err := d.s3Client.HeadObjectWithContext(ctx, headInput)
    if err != nil {
        return nil, err
    }

    return &fileInfo{
        size: *result.ContentLength,
        modTime: *result.LastModified,
    }, nil
}
```

### 7.3 Presigned URL Pattern (S3)

For S3, Harbor can generate **presigned URLs** for direct client downloads:

```go
// Generate presigned URL for blob download
func (d *s3Driver) PresignedURL(ctx context.Context, path string, expiresIn time.Duration) (string, error) {
    key := path

    req, _ := d.s3Client.GetObjectRequest(&s3.GetObjectInput{
        Bucket: aws.String(d.bucket),
        Key: aws.String(key),
    })

    url, err := req.Presign(expiresIn)
    if err != nil {
        return "", err
    }

    return url, nil
}
```

**Client-side usage**:

```typescript
// Get presigned URL from Harbor API
const presignedUrl = await client.get(`/api/v2.0/blobs/url?digest=${digest}`);

// Download directly from S3 (bypasses Harbor)
const response = await fetch(presignedUrl);
const blob = await response.blob();
```

---

## 8. Complete Integration: Pull Changes → Download Blobs

### 8.1 Complete Flow

```typescript
async function pullChangesAndDownloadBlobs(
	client: SyncClient,
	workspaceId: string,
	fromVersion: number,
	localBlobStorage: string,
	db: Database
): Promise<void> {
	// Step 1: Pull changes from server
	const response = await client.pull({
		workspace_id: workspaceId,
		from_version: fromVersion
	});

	// Step 2: Parse manifests and extract blob digests
	const allBlobDigests: Set<string> = new Set();

	for (const change of response.changes) {
		if (change.action === 'create' || change.action === 'update') {
			const manifest = JSON.parse(change.data.manifest || '{}');
			const blobDigests = extractBlobDigests(manifest);
			blobDigests.forEach((digest) => allBlobDigests.add(digest));
		}
	}

	// Step 3: Check which blobs are missing locally
	const missingBlobs = await checkLocalBlobs(db, Array.from(allBlobDigests));

	// Also check server-provided missing_blobs list
	if (response.missing_blobs) {
		response.missing_blobs.forEach((digest) => {
			if (!allBlobDigests.has(digest)) {
				missingBlobs.push(digest);
			}
		});
	}

	// Step 4: Download missing blobs
	if (missingBlobs.length > 0) {
		console.log(`Downloading ${missingBlobs.length} missing blobs...`);

		await downloadBlobs(
			client,
			workspaceId, // repository name
			missingBlobs,
			localBlobStorage,
			(progress) => {
				console.log(
					`Blob ${progress.digest}: ${progress.status} (${progress.bytesDownloaded}/${progress.totalBytes} bytes)`
				);
			}
		);

		// Step 5: Update blob registry
		for (const digest of missingBlobs) {
			const digestHash = digest.replace('sha256:', '');
			const localBlobPath = path.join(
				localBlobStorage,
				'blobs',
				'sha256',
				digestHash.slice(0, 2),
				digestHash.slice(2, 4),
				digestHash
			);

			const stats = await fs.promises.stat(localBlobPath);

			await db.run(
				`INSERT INTO blob_registry (digest, size, path, created_at)
         VALUES (?, ?, ?, ?)`,
				[digest, stats.size, localBlobPath, new Date().toISOString()]
			);
		}
	}

	// Step 6: Apply changes to local SQLite (link blobs to artifacts)
	await applyServerChanges(db, response.changes, response.server_version, workspaceId);
}
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

### 9.2 Missing Blob Detection

**Harbor Pattern**:

- Server returns `missing_blobs` list in pull response.
- Client also checks local storage before downloading.

**DataForge Application**:

- Server returns `missing_blobs` list in sync pull response.
- Client checks local `blob_registry` table before downloading.
- Deduplication: skip download if blob already exists locally.

### 9.3 Blob Download with Verification

**Harbor Pattern**:

- Compute SHA-256 digest while downloading.
- Verify digest matches expected digest before storing.
- Atomic write (write to temp, then rename).

**DataForge Application**:

- Compute SHA-256 hash while downloading Parquet file.
- Verify hash matches expected hash before storing.
- Atomic write to prevent corruption.

### 9.4 Presigned URLs for Direct Downloads

**Harbor Pattern**:

- For S3, generate presigned URLs for direct client downloads.
- Bypasses Harbor server (reduces load, faster downloads).

**DataForge Application**:

- For S3/MinIO, generate presigned URLs for Parquet file downloads.
- Client downloads directly from S3 (bypasses sync server).
- Sync server only provides presigned URLs, not blob data.

---

## 10. Summary

### ✅ Key Patterns

1. **Manifest → Blob References**: Parse manifest to extract blob digests.
2. **Missing Blob Detection**: Check local storage before downloading.
3. **Content-Addressed Storage**: Store blobs by digest (deduplication).
4. **Blob Verification**: Compute and verify SHA-256 digest during download.
5. **Atomic Writes**: Write to temp file, then rename (prevents corruption).
6. **Presigned URLs**: Direct S3 downloads (bypasses server).

### 📋 Best Practices

1. **Check local storage first** (deduplication).
2. **Verify blob integrity** (SHA-256 digest).
3. **Use atomic writes** (temp file → rename).
4. **Support Range requests** (resume downloads).
5. **Download in parallel** (with concurrency limit).
6. **Track download progress** (for UI feedback).

This architecture provides a robust foundation for downloading blobs referenced by pulled changes while maintaining data integrity and supporting efficient deduplication.

