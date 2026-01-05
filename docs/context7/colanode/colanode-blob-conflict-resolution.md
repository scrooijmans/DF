# ColaNode Blob Conflict Resolution Architecture

This document explains how ColaNode handles conflict resolution for blob (binary) data, which uses a different strategy than CRDT-based conflict resolution used for SQL/structured data.

## Overview

**Key Finding**: ColaNode does **NOT** use CRDT conflict resolution for blobs. Instead, it uses:

- **Content-addressed storage** (hash-based deduplication)
- **Metadata versioning** in PostgreSQL
- **Traditional conflict resolution** (last-write-wins, version checks)
- **S3-compatible object storage** for blob content

## Blob vs. SQL Data: Different Conflict Resolution Strategies

### SQL/Structured Data (CRDT-based)

- Uses **Yjs CRDTs** for conflict resolution
- Automatic merging of concurrent edits
- No explicit conflict detection needed
- Examples: Pages, database records, collaborative fields

### Blob Data (Non-CRDT)

- Uses **content-addressed storage** with hash-based deduplication
- **Metadata versioning** for conflict detection
- **Traditional sync patterns** (last-write-wins, server authority)
- Examples: File uploads, images, documents, media files

## Blob Storage Architecture

### Storage Components

```
┌─────────────────────────────────────────────────────────────┐
│                    Client (Local)                           │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   SQLite     │  │  Blob Store  │  │   Yjs Docs   │     │
│  │              │  │  (Content-   │  │   (CRDT)     │     │
│  │ - File       │  │   Addressed) │  │              │     │
│  │   Metadata   │  │              │  │ - Pages     │     │
│  │ - Versions   │  │ - Blob       │  │ - Records   │     │
│  │ - References │  │   Content    │  │              │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
                        │                    │
                        │ WebSocket          │ WebSocket
                        │ (Yjs Updates)      │ (Yjs Updates)
                        ▼                    ▼
┌─────────────────────────────────────────────────────────────┐
│                    Server                                    │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │  PostgreSQL  │  │  S3 Storage  │  │  Yjs Server  │     │
│  │              │  │              │  │              │     │
│  │ - File       │  │ - Blob      │  │ - Yjs        │     │
│  │   Metadata   │  │   Content   │  │   Snapshots  │     │
│  │ - Versions   │  │   (by hash) │  │ - Operations │     │
│  │ - References │  │              │  │              │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### Content-Addressed Storage

Blobs are stored using **content-addressed storage** where the blob's location is determined by its hash:

```typescript
// Blob storage path structure
blobs/
├── a3/
│   └── f2/
│       └── a3f2b8c9d1e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0.parquet
└── 7b/
    └── 1c/
        └── 7b1c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2.parquet
```

**Benefits:**

- **Automatic deduplication**: Same content = same hash = same storage location
- **Integrity verification**: Hash can verify blob hasn't been corrupted
- **Efficient sync**: Only transfer blobs that don't exist locally

## Blob Conflict Resolution Strategy

### 1. Content-Addressed Deduplication

The primary conflict resolution mechanism is **content-addressed storage**:

```typescript
// Pseudo-code for blob storage
class BlobStore {
	async store(data: Uint8Array): Promise<string> {
		// 1. Compute hash
		const hash = sha256(data);
		const path = this.hashToPath(hash);

		// 2. Check if already exists (deduplication)
		if (await this.exists(hash)) {
			return hash; // Already stored, no conflict
		}

		// 3. Store blob atomically
		await this.writeAtomic(path, data);

		return hash;
	}
}
```

**Conflict Resolution:**

- If two clients upload the same file content → Same hash → Only one copy stored
- No conflict because content is identical
- Both clients reference the same blob hash

### 2. Metadata Versioning

File metadata (name, path, permissions) is stored in PostgreSQL with versioning:

```sql
-- File metadata table
CREATE TABLE files (
  id UUID PRIMARY KEY,
  workspace_id UUID NOT NULL,
  name TEXT NOT NULL,
  blob_hash TEXT NOT NULL, -- Reference to content-addressed blob
  version INTEGER NOT NULL DEFAULT 1,
  updated_at TIMESTAMP NOT NULL,
  updated_by UUID, -- User who made the change
  -- ... other metadata
);

-- Version history
CREATE TABLE file_versions (
  id UUID PRIMARY KEY,
  file_id UUID REFERENCES files(id),
  version INTEGER NOT NULL,
  blob_hash TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  created_by UUID
);
```

**Conflict Detection:**

- Each file has a `version` field
- Clients send `expected_version` when updating
- Server checks: `if (local.version != expected_version) → conflict`

### 3. Last-Write-Wins for Metadata

When metadata conflicts occur (e.g., two clients rename the same file):

```typescript
// Server-side conflict resolution
async function updateFileMetadata(
	fileId: string,
	updates: FileMetadataUpdate,
	expectedVersion: number
): Promise<FileMetadata> {
	const file = await db.getFile(fileId);

	// Check version conflict
	if (file.version !== expectedVersion) {
		// Conflict detected - use last-write-wins
		// Option 1: Server timestamp wins
		if (updates.updated_at > file.updated_at) {
			await db.updateFile(fileId, updates, file.version + 1);
			return await db.getFile(fileId);
		}

		// Option 2: Reject and return current state
		throw new ConflictError('File was modified by another client', file);
	}

	// No conflict - apply update
	await db.updateFile(fileId, updates, file.version + 1);
	return await db.getFile(fileId);
}
```

## Complete Call Stack: Blob Upload and Conflict Resolution

### Scenario: Two Clients Upload Different Versions of Same File

```
Client A                          Server                          Client B
   │                                 │                               │
   │ 1. Upload blob (v1)             │                               │
   ├────────────────────────────────>│                               │
   │                                 │ 2. Compute hash              │
   │                                 │    hash = sha256(data)       │
   │                                 │                               │
   │                                 │ 3. Store blob (S3)           │
   │                                 │    path = hashToPath(hash)   │
   │                                 │                               │
   │                                 │ 4. Create file metadata       │
   │                                 │    INSERT INTO files          │
   │                                 │    (id, blob_hash, version=1) │
   │                                 │                               │
   │ 5. Response: file_id, version=1│                               │
   │<────────────────────────────────┤                               │
   │                                 │                               │
   │                                 │                               │ 6. Upload blob (v2)
   │                                 │<──────────────────────────────┤
   │                                 │                               │
   │                                 │ 7. Compute hash              │
   │                                 │    hash2 = sha256(data2)     │
   │                                 │    (different from hash)     │
   │                                 │                               │
   │                                 │ 8. Store blob (S3)            │
   │                                 │    path2 = hashToPath(hash2) │
   │                                 │                               │
   │                                 │ 9. Update file metadata      │
   │                                 │    UPDATE files SET           │
   │                                 │      blob_hash = hash2,       │
   │                                 │      version = 2              │
   │                                 │    WHERE id = file_id        │
   │                                 │                               │
   │                                 │10. Response: version=2       │
   │                                 ├──────────────────────────────>│
   │                                 │                               │
   │11. Sync: Get file updates      │                               │
   ├────────────────────────────────>│                               │
   │                                 │12. Return file metadata      │
   │                                 │    (version=2, blob_hash2)    │
   │13. Response: version=2          │                               │
   │<────────────────────────────────┤                               │
   │                                 │                               │
   │14. Download new blob            │                               │
   │    (if hash2 not in local)     │                               │
   ├────────────────────────────────>│                               │
   │                                 │15. Get blob from S3          │
   │                                 │    by hash2                   │
   │16. Response: blob data          │                               │
   │<────────────────────────────────┤                               │
```

### Detailed Call Stack: Blob Upload

```typescript
// 1. Client: Upload blob
async function uploadBlob(file: File): Promise<BlobUploadResult> {
	// Read file data
	const data = await file.arrayBuffer();

	// Compute hash (client-side)
	const hash = await computeSHA256(data);

	// Check if blob already exists locally
	if (await localBlobStore.exists(hash)) {
		// Blob already stored - just update metadata
		return await updateFileMetadata(file.id, { blob_hash: hash });
	}

	// Upload blob to server
	const uploadResult = await server.uploadBlob({
		hash: hash,
		data: data,
		content_type: file.type,
		size: file.size
	});

	// Store locally (content-addressed)
	await localBlobStore.store(hash, data);

	// Update file metadata
	return await server.updateFileMetadata(file.id, {
		blob_hash: hash,
		version: uploadResult.version
	});
}

// 2. Server: Handle blob upload
async function handleBlobUpload(request: BlobUploadRequest): Promise<BlobUploadResponse> {
	// Verify hash matches data
	const computedHash = await computeSHA256(request.data);
	if (computedHash !== request.hash) {
		throw new Error('Hash mismatch');
	}

	// Check if blob already exists in S3
	const s3Key = hashToS3Key(request.hash);
	if (await s3.exists(s3Key)) {
		// Blob already stored - deduplication
		return {
			hash: request.hash,
			stored: false, // Not newly stored
			url: s3.getUrl(s3Key)
		};
	}

	// Store blob in S3
	await s3.put(s3Key, request.data, {
		ContentType: request.content_type,
		Metadata: {
			hash: request.hash,
			size: request.size.toString()
		}
	});

	return {
		hash: request.hash,
		stored: true, // Newly stored
		url: s3.getUrl(s3Key)
	};
}

// 3. Server: Update file metadata
async function updateFileMetadata(
	fileId: string,
	metadata: FileMetadataUpdate
): Promise<FileMetadata> {
	const db = await getDatabase();

	// Start transaction
	await db.beginTransaction();

	try {
		// Get current file state
		const currentFile = await db.query('SELECT * FROM files WHERE id = ?', [fileId]);

		if (!currentFile) {
			throw new Error('File not found');
		}

		// Check version conflict (if expected_version provided)
		if (metadata.expected_version !== undefined) {
			if (currentFile.version !== metadata.expected_version) {
				// Conflict: file was modified by another client
				await db.rollback();
				throw new ConflictError('File version mismatch', {
					expected: metadata.expected_version,
					actual: currentFile.version,
					current_file: currentFile
				});
			}
		}

		// Update file metadata
		const newVersion = currentFile.version + 1;
		await db.query(
			`UPDATE files 
       SET blob_hash = ?, 
           version = ?,
           updated_at = NOW(),
           updated_by = ?
       WHERE id = ?`,
			[metadata.blob_hash, newVersion, metadata.updated_by, fileId]
		);

		// Create version history entry
		await db.query(
			`INSERT INTO file_versions 
       (file_id, version, blob_hash, created_at, created_by)
       VALUES (?, ?, ?, NOW(), ?)`,
			[fileId, newVersion, metadata.blob_hash, metadata.updated_by]
		);

		// Commit transaction
		await db.commit();

		// Return updated file
		return await db.query('SELECT * FROM files WHERE id = ?', [fileId]);
	} catch (error) {
		await db.rollback();
		throw error;
	}
}
```

## Conflict Resolution Patterns

### Pattern 1: Concurrent Upload of Same Content

**Scenario**: Two clients upload identical file content simultaneously.

```
Client A: Upload file → hash = "abc123"
Client B: Upload file → hash = "abc123" (same content)

Resolution:
1. Client A uploads → Server stores blob at hash "abc123"
2. Client B uploads → Server detects hash exists → Returns existing blob
3. Both clients reference same blob hash
4. No conflict - content-addressed deduplication handles it
```

### Pattern 2: Concurrent Upload of Different Content

**Scenario**: Two clients upload different versions of the same file.

```
Client A: Upload file_v1 → hash = "abc123"
Client B: Upload file_v2 → hash = "def456" (different content)

Resolution:
1. Client A uploads → Server stores blob "abc123", creates file metadata (version=1)
2. Client B uploads → Server stores blob "def456", updates file metadata (version=2)
3. Client A syncs → Receives update: version=2, blob_hash="def456"
4. Client A downloads new blob "def456" if not cached
5. Last-write-wins: Client B's version is the current version
```

### Pattern 3: Concurrent Metadata Updates

**Scenario**: Two clients rename the same file simultaneously.

```
Client A: Rename file "doc.txt" → "document.txt" (version=1)
Client B: Rename file "doc.txt" → "file.txt" (version=1)

Resolution:
1. Client A sends: UPDATE files SET name='document.txt' WHERE id=X AND version=1
2. Client B sends: UPDATE files SET name='file.txt' WHERE id=X AND version=1
3. Server processes A first → Updates to version=2, name='document.txt'
4. Server processes B → Detects version mismatch (expected=1, actual=2)
5. Server returns ConflictError with current state (version=2, name='document.txt')
6. Client B receives conflict → Can retry with new version or show conflict UI
```

## Blob Sync Flow

### Client-Side Blob Sync

```typescript
class BlobSyncService {
	// Sync blobs from server
	async syncBlobs(missingHashes: string[]): Promise<void> {
		// 1. Request blob URLs from server
		const urls = await server.getBlobUrls(missingHashes);

		// 2. Download each blob
		for (const blobUrl of urls) {
			// Download blob data
			const data = await fetch(blobUrl.url).then((r) => r.arrayBuffer());

			// Verify integrity
			const actualHash = await computeSHA256(data);
			if (actualHash !== blobUrl.hash) {
				throw new Error(`Hash mismatch: expected ${blobUrl.hash}, got ${actualHash}`);
			}

			// Store locally (content-addressed)
			await localBlobStore.store(blobUrl.hash, data);

			// Register in blob registry
			await db.execute(
				`INSERT OR IGNORE INTO blob_registry 
         (hash, size_bytes, synced_to_server, synced_at)
         VALUES (?, ?, 1, datetime('now'))`,
				[blobUrl.hash, data.byteLength]
			);
		}
	}

	// Upload local blobs to server
	async uploadBlobs(hashes: string[]): Promise<void> {
		for (const hash of hashes) {
			// Check if already synced
			const synced = await db.query('SELECT synced_to_server FROM blob_registry WHERE hash = ?', [
				hash
			]);

			if (synced?.synced_to_server) {
				continue; // Already synced
			}

			// Read blob from local store
			const data = await localBlobStore.get(hash);

			// Upload to server
			await server.uploadBlob({
				hash: hash,
				data: data,
				size: data.byteLength
			});

			// Mark as synced
			await db.execute(
				`UPDATE blob_registry 
         SET synced_to_server = 1, synced_at = datetime('now')
         WHERE hash = ?`,
				[hash]
			);
		}
	}
}
```

### Server-Side Blob Management

```typescript
class ServerBlobService {
	// Get blob URLs for download
	async getBlobUrls(hashes: string[]): Promise<BlobUrl[]> {
		const urls: BlobUrl[] = [];

		for (const hash of hashes) {
			const s3Key = this.hashToS3Key(hash);

			// Check if blob exists
			if (await this.s3.exists(s3Key)) {
				// Generate presigned URL for download
				const url = await this.s3.getPresignedUrl(s3Key, {
					expiresIn: 3600 // 1 hour
				});

				urls.push({
					hash: hash,
					url: url,
					size: await this.s3.getSize(s3Key)
				});
			} else {
				// Blob not found
				throw new Error(`Blob not found: ${hash}`);
			}
		}

		return urls;
	}

	// Upload blob
	async uploadBlob(request: BlobUploadRequest): Promise<BlobUploadResponse> {
		// Verify hash
		const computedHash = await computeSHA256(request.data);
		if (computedHash !== request.hash) {
			throw new Error('Hash mismatch');
		}

		const s3Key = this.hashToS3Key(request.hash);

		// Check if already exists (deduplication)
		if (await this.s3.exists(s3Key)) {
			return {
				hash: request.hash,
				stored: false,
				url: await this.s3.getPresignedUrl(s3Key)
			};
		}

		// Store in S3
		await this.s3.put(s3Key, request.data, {
			ContentType: request.content_type,
			Metadata: {
				hash: request.hash,
				size: request.size.toString(),
				uploaded_at: new Date().toISOString()
			}
		});

		return {
			hash: request.hash,
			stored: true,
			url: await this.s3.getPresignedUrl(s3Key)
		};
	}
}
```

## Key Differences: Blob vs. SQL Conflict Resolution

| Aspect                  | SQL/Structured Data (CRDT)               | Blob Data (Non-CRDT)                            |
| ----------------------- | ---------------------------------------- | ----------------------------------------------- |
| **Conflict Resolution** | Automatic (Yjs CRDT)                     | Manual (version checks, last-write-wins)        |
| **Merge Strategy**      | Automatic merging                        | No merging - whole blob replacement             |
| **Conflict Detection**  | Not needed (CRDT guarantees convergence) | Version comparison, hash verification           |
| **Storage**             | PostgreSQL (metadata) + Yjs snapshots    | S3-compatible (content) + PostgreSQL (metadata) |
| **Sync Protocol**       | WebSocket (Yjs deltas)                   | REST/HTTP (blob upload/download)                |
| **Deduplication**       | N/A                                      | Content-addressed (hash-based)                  |
| **Offline Support**     | Queue Yjs operations                     | Queue blob uploads, download on demand          |

## Summary

**ColaNode's blob conflict resolution:**

1. **Does NOT use CRDTs** - Blobs use traditional conflict resolution
2. **Content-addressed storage** - Hash-based deduplication prevents conflicts for identical content
3. **Metadata versioning** - File metadata uses version numbers for conflict detection
4. **Last-write-wins** - When metadata conflicts occur, server timestamp determines winner
5. **Server authority** - Server is authoritative for file metadata; clients sync to server state
6. **No automatic merging** - Unlike CRDTs, blob conflicts result in whole-file replacement, not merging

**Architecture:**

- **Client**: Local blob store (content-addressed) + SQLite metadata
- **Server**: S3-compatible storage (blobs) + PostgreSQL (metadata)
- **Sync**: REST/HTTP for blobs, WebSocket for metadata updates
- **Conflict Resolution**: Version checks + last-write-wins for metadata, content-addressing for blob content

This design is appropriate because:

- Blobs are typically large and immutable (whole-file replacement is expected)
- Content-addressed storage provides automatic deduplication
- Metadata conflicts are rare and can be resolved with simple versioning
- CRDTs are overkill for binary data that doesn't need fine-grained merging
