# Harbor Blob Upload with Presigned URLs: Architecture & Design (Context7 Summary)

This document dives deep into **Harbor's architecture and design for blob uploads with presigned URLs**, focusing on how Harbor (via Docker Distribution) handles direct client-to-storage uploads, particularly for S3-compatible backends. It covers the upload flow, presigned URL generation, and how this pattern enables efficient, scalable blob uploads.

It builds on:

- `harbor-object-storage-architecture.md`
- `harbor-blob-download-architecture.md`

and provides detailed call stacks and implementation patterns for presigned URL-based blob uploads.

---

## 1. High-Level Architecture: Direct Client Upload Pattern

### 1.1 The Problem

Traditional blob upload flow:

- Client → Harbor API → Storage Backend
- All data passes through Harbor servers
- Harbor becomes a bottleneck for large uploads
- High bandwidth costs and latency

**Solution**: Presigned URLs enable **direct client-to-storage uploads**:

- Client → S3/Object Storage (direct)
- Harbor only orchestrates (generates presigned URLs, tracks metadata)
- Reduces server load and improves upload performance

### 1.2 Two Upload Patterns

Harbor supports two upload patterns:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Harbor Blob Upload Patterns                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Pattern 1: Proxy Upload (Traditional)                      │  │
│  │  ┌──────────┐      ┌──────────┐      ┌──────────┐          │  │
│  │  │ Client   │─────▶│ Harbor   │─────▶│ Storage  │          │  │
│  │  │          │      │ API      │      │ Backend  │          │  │
│  │  └──────────┘      └──────────┘      └──────────┘          │  │
│  │  - All data flows through Harbor                            │  │
│  │  - Works with any storage backend                           │  │
│  │  - Higher server load                                       │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Pattern 2: Presigned URL Upload (S3/Azure/GCS)             │  │
│  │  ┌──────────┐      ┌──────────┐                             │  │
│  │  │ Client   │─────▶│ Storage  │  (Direct upload)            │  │
│  │  │          │      │ Backend  │                             │  │
│  │  └──────────┘      └──────────┘                             │  │
│  │         │                                                      │  │
│  │         │ 1. Request presigned URL                            │  │
│  │         ▼                                                      │  │
│  │  ┌──────────┐                                                 │  │
│  │  │ Harbor   │  (Orchestrates, tracks metadata)                │  │
│  │  │ API      │                                                 │  │
│  │  └──────────┘                                                 │  │
│  │  - Direct client-to-storage upload                            │  │
│  │  - Harbor only generates presigned URLs                       │  │
│  │  - Lower server load, better performance                      │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Docker Distribution Blob Upload API

### 2.1 Standard Upload Flow (Proxy Pattern)

Harbor extends Docker Distribution, which provides a standard blob upload API:

```
POST /v2/{name}/blobs/uploads/
  → 202 Accepted
  → Location: /v2/{name}/blobs/uploads/{uuid}
  → Upload-UUID: {uuid}

PATCH /v2/{name}/blobs/uploads/{uuid}
  → 204 No Content
  → Range: bytes=0-1048575
  → Upload-UUID: {uuid}

PUT /v2/{name}/blobs/uploads/{uuid}?digest=sha256:...
  → 201 Created
  → Location: /v2/{name}/blobs/{digest}
  → Content-Length: 0
```

### 2.2 Upload Session Management

Docker Distribution manages upload sessions:

```go
// Upload session (simplified)
type UploadSession struct {
    UUID      string    // Unique upload session ID
    Repository string   // Repository name
    StartedAt time.Time
    Size      int64     // Total expected size (if known)
    Chunks    []Chunk   // Uploaded chunks
}

type Chunk struct {
    Offset int64
    Length int64
    Data   []byte
}
```

**Storage Driver Interface**:

```go
type Driver interface {
    // Initiate upload session
    Writer(ctx context.Context, path string, append bool) (io.WriteCloser, error)

    // Resume upload (for chunked uploads)
    ResumeWriter(ctx context.Context, path string, offset int64) (io.WriteCloser, error)

    // Get upload status
    Stat(ctx context.Context, path string) (FileInfo, error)

    // Delete upload session
    Delete(ctx context.Context, path string) error
}
```

---

## 3. Presigned URL Upload Architecture

### 3.1 S3 Presigned URL Pattern

For S3-compatible storage, Harbor can generate **presigned URLs** that allow direct client uploads:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Presigned URL Upload Flow                                          │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: Client Requests Upload Session                      │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  POST /v2/{name}/blobs/uploads/                        │  │  │
│  │  │  Authorization: Bearer {token}                          │  │  │
│  │  │                                                          │  │  │
│  │  │  Harbor:                                                │  │  │
│  │  │  - Authenticate client                                  │  │  │
│  │  │  - Check permissions                                    │  │  │
│  │  │  - Check storage quota                                  │  │  │
│  │  │  - Generate upload UUID                                 │  │  │
│  │  │  - Create upload session in PostgreSQL                  │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Response: 202 Accepted                    │
│                         │ Location: /v2/{name}/blobs/uploads/{uuid} │
│                         │ Upload-UUID: {uuid}                       │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Client Requests Presigned URL                      │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  GET /v2/{name}/blobs/uploads/{uuid}?presigned=true    │  │  │
│  │  │  Authorization: Bearer {token}                          │  │  │
│  │  │                                                          │  │  │
│  │  │  Harbor:                                                │  │  │
│  │  │  - Validate upload session                              │  │  │
│  │  │  - Generate presigned PUT URL for S3:                   │  │  │
│  │  │    s3://bucket/blobs/sha256/temp/{uuid}                 │  │  │
│  │  │  - Set expiration (e.g., 1 hour)                        │  │  │
│  │  │  - Include required headers (Content-Type, etc.)        │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Response: 200 OK                          │
│                         │ {                                         │
│                         │   "presigned_url": "https://s3...",      │
│                         │   "expires_at": "2025-01-15T11:00:00Z",  │
│                         │   "headers": {                            │
│                         │     "Content-Type": "application/octet-stream" │
│                         │   }                                       │
│                         │ }                                         │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: Client Uploads Directly to S3                      │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  PUT {presigned_url}                                   │  │  │
│  │  │  Content-Type: application/octet-stream                │  │  │
│  │  │  Body: <blob data>                                     │  │  │
│  │  │                                                          │  │  │
│  │  │  S3:                                                    │  │  │
│  │  │  - Validates presigned URL signature                   │  │  │
│  │  │  - Checks expiration                                   │  │  │
│  │  │  - Stores blob at temp location                        │  │  │
│  │  │  - Returns 200 OK                                      │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Client notifies Harbor of completion      │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 4: Client Finalizes Upload                            │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  PUT /v2/{name}/blobs/uploads/{uuid}?digest=sha256:... │  │  │
│  │  │  Authorization: Bearer {token}                          │  │  │
│  │  │                                                          │  │  │
│  │  │  Harbor:                                                │  │  │
│  │  │  - Verify upload session exists                         │  │  │
│  │  │  - Request S3 to compute blob digest                    │  │  │
│  │  │  - Verify digest matches client-provided digest         │  │  │
│  │  │  - Move blob from temp to final location:               │  │  │
│  │  │    s3://bucket/blobs/sha256/a3/f2/{digest}             │  │  │
│  │  │  - Check for deduplication (blob already exists?)       │  │  │
│  │  │  - Update upload session status                         │  │  │
│  │  │  - Return 201 Created                                   │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 4. Presigned URL Generation (S3 Storage Driver)

### 4.1 S3 Presigned PUT URL Generation

```go
package storage

import (
    "context"
    "time"
    "github.com/aws/aws-sdk-go/aws"
    "github.com/aws/aws-sdk-go/aws/session"
    "github.com/aws/aws-sdk-go/service/s3"
)

type S3Driver struct {
    s3Client *s3.S3
    bucket   string
    region   string
}

// GeneratePresignedUploadURL generates a presigned URL for direct client upload
func (d *S3Driver) GeneratePresignedUploadURL(
    ctx context.Context,
    uploadUUID string,
    expiresIn time.Duration,
) (string, error) {
    // Temporary path for upload session
    key := fmt.Sprintf("blobs/sha256/temp/%s", uploadUUID)

    // Create PutObject request
    req, _ := d.s3Client.PutObjectRequest(&s3.PutObjectInput{
        Bucket:      aws.String(d.bucket),
        Key:         aws.String(key),
        ContentType: aws.String("application/octet-stream"),
        // Optional: Set ACL or other metadata
        // ACL: aws.String("private"),
    })

    // Presign the request (expires in specified duration)
    presignedURL, err := req.Presign(expiresIn)
    if err != nil {
        return "", fmt.Errorf("failed to presign upload URL: %w", err)
    }

    return presignedURL, nil
}

// FinalizeUpload moves blob from temp location to final digest-based path
func (d *S3Driver) FinalizeUpload(
    ctx context.Context,
    uploadUUID string,
    digest string,
) error {
    tempKey := fmt.Sprintf("blobs/sha256/temp/%s", uploadUUID)

    // Derive final path from digest
    finalKey := d.deriveBlobPath(digest)

    // Check if blob already exists (deduplication)
    exists, err := d.blobExists(ctx, finalKey)
    if err != nil {
        return err
    }
    if exists {
        // Blob already exists, delete temp upload
        d.s3Client.DeleteObject(&s3.DeleteObjectInput{
            Bucket: aws.String(d.bucket),
            Key:    aws.String(tempKey),
        })
        return nil // Success (deduplication)
    }

    // Copy blob from temp to final location
    copySource := fmt.Sprintf("%s/%s", d.bucket, tempKey)
    _, err = d.s3Client.CopyObject(&s3.CopyObjectInput{
        Bucket:     aws.String(d.bucket),
        CopySource: aws.String(copySource),
        Key:        aws.String(finalKey),
    })
    if err != nil {
        return fmt.Errorf("failed to finalize upload: %w", err)
    }

    // Delete temp upload
    d.s3Client.DeleteObject(&s3.DeleteObjectInput{
        Bucket: aws.String(d.bucket),
        Key:    aws.String(tempKey),
    })

    return nil
}

// VerifyUploadDigest verifies that uploaded blob matches expected digest
func (d *S3Driver) VerifyUploadDigest(
    ctx context.Context,
    uploadUUID string,
    expectedDigest string,
) error {
    tempKey := fmt.Sprintf("blobs/sha256/temp/%s", uploadUUID)

    // Get object metadata
    headOutput, err := d.s3Client.HeadObject(&s3.HeadObjectInput{
        Bucket: aws.String(d.bucket),
        Key:    aws.String(tempKey),
    })
    if err != nil {
        return fmt.Errorf("failed to get upload metadata: %w", err)
    }

    // S3 stores ETag (MD5) by default, but we need SHA-256
    // Option 1: Download and compute SHA-256 (for verification)
    // Option 2: Use S3 Object Lambda to compute SHA-256 server-side
    // Option 3: Trust client-provided digest (less secure)

    // For now, we'll download and verify (can be optimized)
    getOutput, err := d.s3Client.GetObject(&s3.GetObjectInput{
        Bucket: aws.String(d.bucket),
        Key:    aws.String(tempKey),
    })
    if err != nil {
        return fmt.Errorf("failed to download blob for verification: %w", err)
    }
    defer getOutput.Body.Close()

    // Compute SHA-256 digest
    hasher := sha256.New()
    if _, err := io.Copy(hasher, getOutput.Body); err != nil {
        return fmt.Errorf("failed to compute digest: %w", err)
    }

    computedDigest := fmt.Sprintf("sha256:%x", hasher.Sum(nil))

    if computedDigest != expectedDigest {
        return fmt.Errorf("digest mismatch: expected %s, got %s", expectedDigest, computedDigest)
    }

    return nil
}

// Derive blob path from digest
func (d *S3Driver) deriveBlobPath(digest string) string {
    // Extract hash from "sha256:abc123..."
    hash := strings.TrimPrefix(digest, "sha256:")

    // Path structure: blobs/sha256/{first2}/{next2}/{full_hash}
    return fmt.Sprintf("blobs/sha256/%s/%s/%s", hash[0:2], hash[2:4], hash)
}
```

---

## 5. Harbor API Integration

### 5.1 Upload Session Management

Harbor tracks upload sessions in PostgreSQL:

```sql
-- Upload session table
CREATE TABLE blob_upload_session (
    id SERIAL PRIMARY KEY,
    upload_uuid VARCHAR(255) NOT NULL UNIQUE,
    repository_name VARCHAR(255) NOT NULL,
    user_id INTEGER NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- 'pending', 'uploading', 'completed', 'failed'
    temp_path TEXT, -- Temporary storage path
    final_digest VARCHAR(255), -- Final blob digest (after finalization)
    size BIGINT, -- Expected/actual blob size
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP, -- Session expiration
    metadata JSONB -- Additional metadata
);

CREATE INDEX idx_blob_upload_session_uuid ON blob_upload_session(upload_uuid);
CREATE INDEX idx_blob_upload_session_status ON blob_upload_session(status);
```

### 5.2 Harbor API Endpoints

```go
// Harbor API handler for presigned upload URL
func (h *Handler) GetPresignedUploadURL(c *gin.Context) {
    uploadUUID := c.Param("uuid")
    repositoryName := c.Param("name")

    // Authenticate and authorize
    user, err := h.authenticate(c)
    if err != nil {
        c.JSON(401, gin.H{"error": "unauthorized"})
        return
    }

    // Check permissions
    if !h.hasPermission(user, repositoryName, "push") {
        c.JSON(403, gin.H{"error": "forbidden"})
        return
    }

    // Get upload session
    session, err := h.db.GetUploadSession(uploadUUID)
    if err != nil {
        c.JSON(404, gin.H{"error": "upload session not found"})
        return
    }

    // Check if session expired
    if time.Now().After(session.ExpiresAt) {
        c.JSON(410, gin.H{"error": "upload session expired"})
        return
    }

    // Get storage driver
    driver := h.getStorageDriver(repositoryName)

    // Check if driver supports presigned URLs
    s3Driver, ok := driver.(*S3Driver)
    if !ok {
        c.JSON(400, gin.H{"error": "presigned URLs not supported for this storage backend"})
        return
    }

    // Generate presigned URL (expires in 1 hour)
    presignedURL, err := s3Driver.GeneratePresignedUploadURL(
        c.Request.Context(),
        uploadUUID,
        time.Hour,
    )
    if err != nil {
        c.JSON(500, gin.H{"error": "failed to generate presigned URL"})
        return
    }

    // Update session status
    session.Status = "uploading"
    h.db.UpdateUploadSession(session)

    // Return presigned URL
    c.JSON(200, gin.H{
        "presigned_url": presignedURL,
        "expires_at":    time.Now().Add(time.Hour).Format(time.RFC3339),
        "headers": map[string]string{
            "Content-Type": "application/octet-stream",
        },
    })
}

// Finalize upload
func (h *Handler) FinalizeUpload(c *gin.Context) {
    uploadUUID := c.Param("uuid")
    digest := c.Query("digest")
    repositoryName := c.Param("name")

    // Authenticate and authorize
    user, err := h.authenticate(c)
    if err != nil {
        c.JSON(401, gin.H{"error": "unauthorized"})
        return
    }

    // Get upload session
    session, err := h.db.GetUploadSession(uploadUUID)
    if err != nil {
        c.JSON(404, gin.H{"error": "upload session not found"})
        return
    }

    // Verify digest
    driver := h.getStorageDriver(repositoryName)
    s3Driver := driver.(*S3Driver)

    if err := s3Driver.VerifyUploadDigest(c.Request.Context(), uploadUUID, digest); err != nil {
        c.JSON(400, gin.H{"error": "digest verification failed"})
        return
    }

    // Finalize upload (move to final location)
    if err := s3Driver.FinalizeUpload(c.Request.Context(), uploadUUID, digest); err != nil {
        c.JSON(500, gin.H{"error": "failed to finalize upload"})
        return
    }

    // Update session
    session.Status = "completed"
    session.FinalDigest = digest
    h.db.UpdateUploadSession(session)

    // Update metadata in PostgreSQL
    h.updateBlobMetadata(repositoryName, digest, session.Size)

    // Return success
    c.Header("Location", fmt.Sprintf("/v2/%s/blobs/%s", repositoryName, digest))
    c.Header("Content-Length", "0")
    c.Status(201)
}
```

---

## 6. Client-Side Upload Implementation

### 6.1 Client Upload Flow

```typescript
class HarborBlobUploadClient {
	constructor(
		private harborAPI: string,
		private authToken: string
	) {}

	/**
	 * Upload blob using presigned URL
	 */
	async uploadBlob(
		repositoryName: string,
		blobData: Blob | ArrayBuffer,
		expectedDigest?: string
	): Promise<string> {
		// Step 1: Initiate upload session
		const uploadSession = await this.initiateUpload(repositoryName);

		// Step 2: Get presigned URL
		const presignedURL = await this.getPresignedURL(repositoryName, uploadSession.uuid);

		// Step 3: Upload directly to S3
		await this.uploadToS3(presignedURL, blobData);

		// Step 4: Compute digest (if not provided)
		const digest = expectedDigest || (await this.computeDigest(blobData));

		// Step 5: Finalize upload
		await this.finalizeUpload(repositoryName, uploadSession.uuid, digest);

		return digest;
	}

	/**
	 * Step 1: Initiate upload session
	 */
	private async initiateUpload(repositoryName: string): Promise<UploadSession> {
		const response = await fetch(`${this.harborAPI}/v2/${repositoryName}/blobs/uploads/`, {
			method: 'POST',
			headers: {
				Authorization: `Bearer ${this.authToken}`
			}
		});

		if (!response.ok) {
			throw new Error(`Failed to initiate upload: ${response.statusText}`);
		}

		const uploadUUID = response.headers.get('Upload-UUID');
		const location = response.headers.get('Location');

		return {
			uuid: uploadUUID!,
			location: location!
		};
	}

	/**
	 * Step 2: Get presigned URL
	 */
	private async getPresignedURL(
		repositoryName: string,
		uploadUUID: string
	): Promise<PresignedURLResponse> {
		const response = await fetch(
			`${this.harborAPI}/v2/${repositoryName}/blobs/uploads/${uploadUUID}?presigned=true`,
			{
				method: 'GET',
				headers: {
					Authorization: `Bearer ${this.authToken}`
				}
			}
		);

		if (!response.ok) {
			throw new Error(`Failed to get presigned URL: ${response.statusText}`);
		}

		return await response.json();
	}

	/**
	 * Step 3: Upload directly to S3
	 */
	private async uploadToS3(presignedURL: string, blobData: Blob | ArrayBuffer): Promise<void> {
		const response = await fetch(presignedURL, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/octet-stream'
			},
			body: blobData
		});

		if (!response.ok) {
			throw new Error(`Failed to upload to S3: ${response.statusText}`);
		}
	}

	/**
	 * Step 4: Compute SHA-256 digest
	 */
	private async computeDigest(blobData: Blob | ArrayBuffer): Promise<string> {
		const data = blobData instanceof Blob ? await blobData.arrayBuffer() : blobData;

		const hashBuffer = await crypto.subtle.digest('SHA-256', data);
		const hashArray = Array.from(new Uint8Array(hashBuffer));
		const hashHex = hashArray.map((b) => b.toString(16).padStart(2, '0')).join('');

		return `sha256:${hashHex}`;
	}

	/**
	 * Step 5: Finalize upload
	 */
	private async finalizeUpload(
		repositoryName: string,
		uploadUUID: string,
		digest: string
	): Promise<void> {
		const response = await fetch(
			`${this.harborAPI}/v2/${repositoryName}/blobs/uploads/${uploadUUID}?digest=${digest}`,
			{
				method: 'PUT',
				headers: {
					Authorization: `Bearer ${this.authToken}`
				}
			}
		);

		if (!response.ok) {
			throw new Error(`Failed to finalize upload: ${response.statusText}`);
		}
	}
}
```

### 6.2 Chunked Upload with Presigned URLs

For large files, clients can upload in chunks:

```typescript
/**
 * Chunked upload with presigned URLs
 */
async uploadBlobChunked(
  repositoryName: string,
  blobData: Blob,
  chunkSize: number = 5 * 1024 * 1024 // 5MB chunks
): Promise<string> {
  // Step 1: Initiate upload session
  const uploadSession = await this.initiateUpload(repositoryName);

  // Step 2: Upload chunks
  const totalChunks = Math.ceil(blobData.size / chunkSize);
  const chunkDigests: string[] = [];

  for (let i = 0; i < totalChunks; i++) {
    const start = i * chunkSize;
    const end = Math.min(start + chunkSize, blobData.size);
    const chunk = blobData.slice(start, end);

    // Get presigned URL for this chunk
    const presignedURL = await this.getPresignedURL(
      repositoryName,
      `${uploadSession.uuid}-chunk-${i}`
    );

    // Upload chunk
    await this.uploadToS3(presignedURL, chunk);

    // Compute chunk digest
    const chunkDigest = await this.computeDigest(chunk);
    chunkDigests.push(chunkDigest);
  }

  // Step 3: Combine chunks and compute final digest
  // (In practice, S3 Multipart Upload would be used)
  const finalDigest = await this.computeDigest(blobData);

  // Step 4: Finalize upload
  await this.finalizeUpload(repositoryName, uploadSession.uuid, finalDigest);

  return finalDigest;
}
```

---

## 7. S3 Multipart Upload Pattern

For very large files, S3 supports **multipart uploads**:

```go
// S3 Multipart Upload with presigned URLs
func (d *S3Driver) InitiateMultipartUpload(
    ctx context.Context,
    uploadUUID string,
) (string, error) {
    key := fmt.Sprintf("blobs/sha256/temp/%s", uploadUUID)

    output, err := d.s3Client.CreateMultipartUpload(&s3.CreateMultipartUploadInput{
        Bucket:      aws.String(d.bucket),
        Key:         aws.String(key),
        ContentType: aws.String("application/octet-stream"),
    })
    if err != nil {
        return "", err
    }

    return *output.UploadId, nil
}

// Generate presigned URL for each part
func (d *S3Driver) GeneratePresignedPartURL(
    ctx context.Context,
    uploadUUID string,
    uploadID string,
    partNumber int64,
    expiresIn time.Duration,
) (string, error) {
    key := fmt.Sprintf("blobs/sha256/temp/%s", uploadUUID)

    req, _ := d.s3Client.UploadPartRequest(&s3.UploadPartInput{
        Bucket:     aws.String(d.bucket),
        Key:        aws.String(key),
        UploadId:   aws.String(uploadID),
        PartNumber: aws.Int64(partNumber),
    })

    presignedURL, err := req.Presign(expiresIn)
    if err != nil {
        return "", err
    }

    return presignedURL, nil
}

// Complete multipart upload
func (d *S3Driver) CompleteMultipartUpload(
    ctx context.Context,
    uploadUUID string,
    uploadID string,
    parts []s3.CompletedPart,
    digest string,
) error {
    tempKey := fmt.Sprintf("blobs/sha256/temp/%s", uploadUUID)
    finalKey := d.deriveBlobPath(digest)

    // Complete multipart upload
    _, err := d.s3Client.CompleteMultipartUpload(&s3.CompleteMultipartUploadInput{
        Bucket:   aws.String(d.bucket),
        Key:      aws.String(tempKey),
        UploadId: aws.String(uploadID),
        MultipartUpload: &s3.CompletedMultipartUpload{
            Parts: parts,
        },
    })
    if err != nil {
        return err
    }

    // Copy to final location
    copySource := fmt.Sprintf("%s/%s", d.bucket, tempKey)
    _, err = d.s3Client.CopyObject(&s3.CopyObjectInput{
        Bucket:     aws.String(d.bucket),
        CopySource: aws.String(copySource),
        Key:        aws.String(finalKey),
    })
    if err != nil {
        return err
    }

    // Delete temp upload
    d.s3Client.DeleteObject(&s3.DeleteObjectInput{
        Bucket: aws.String(d.bucket),
        Key:    aws.String(tempKey),
    })

    return nil
}
```

---

## 8. Security Considerations

### 8.1 Presigned URL Security

**Expiration**:

- Presigned URLs should expire after a reasonable time (e.g., 1 hour).
- Prevents unauthorized access if URL is leaked.

**Scope**:

- Presigned URLs should be scoped to specific operations (PUT only).
- Include required headers (Content-Type) in presigned request.

**Verification**:

- Always verify blob digest after upload.
- Never trust client-provided digest without verification.

**Access Control**:

- Presigned URLs should respect Harbor's access control policies.
- Only generate presigned URLs for authenticated, authorized users.

### 8.2 Digest Verification

```go
// Always verify digest after upload
func (d *S3Driver) VerifyUploadDigest(
    ctx context.Context,
    uploadUUID string,
    expectedDigest string,
) error {
    // Option 1: Download and verify (secure but slow)
    // Option 2: Use S3 Object Lambda to compute SHA-256 server-side
    // Option 3: Use S3 Checksum feature (if available)

    // For production, use S3 Object Lambda or Checksum
    // For now, download and verify
    tempKey := fmt.Sprintf("blobs/sha256/temp/%s", uploadUUID)

    getOutput, err := d.s3Client.GetObject(&s3.GetObjectInput{
        Bucket: aws.String(d.bucket),
        Key:    aws.String(tempKey),
    })
    if err != nil {
        return err
    }
    defer getOutput.Body.Close()

    hasher := sha256.New()
    if _, err := io.Copy(hasher, getOutput.Body); err != nil {
        return err
    }

    computedDigest := fmt.Sprintf("sha256:%x", hasher.Sum(nil))
    if computedDigest != expectedDigest {
        return fmt.Errorf("digest mismatch")
    }

    return nil
}
```

---

## 9. Design Patterns for DataForge

### 9.1 Presigned URL Upload Pattern

**Harbor Pattern**:

- Generate presigned URLs for direct client-to-storage uploads.
- Harbor orchestrates (tracks sessions, verifies digests).
- Storage backend handles actual upload.

**DataForge Application**:

- Use presigned URLs for Parquet file uploads.
- Client uploads directly to S3/MinIO.
- DataForge sync server tracks metadata in PostgreSQL.

### 9.2 Upload Session Management

**Harbor Pattern**:

- Track upload sessions in PostgreSQL.
- Sessions expire after a timeout.
- Clean up failed/expired sessions.

**DataForge Application**:

- Track upload sessions in SQLite (local) or PostgreSQL (server).
- Support resumable uploads for large files.
- Clean up temp uploads on failure.

### 9.3 Digest Verification

**Harbor Pattern**:

- Always verify blob digest after upload.
- Use SHA-256 for content-addressed storage.
- Support deduplication (same digest = same blob).

**DataForge Application**:

- Verify Parquet file digests after upload.
- Use SHA-256 for content-addressed storage.
- Deduplicate identical files across workspaces.

---

## 10. Summary

### ✅ Key Patterns

1. **Presigned URLs**: Enable direct client-to-storage uploads, reducing server load.
2. **Upload Sessions**: Track upload state in database (PostgreSQL/SQLite).
3. **Digest Verification**: Always verify blob digest after upload for security.
4. **Deduplication**: Check if blob already exists before storing (content-addressed).
5. **Multipart Uploads**: Support chunked uploads for large files.

### 📋 Best Practices

1. **Generate presigned URLs** with reasonable expiration (1 hour).
2. **Scope presigned URLs** to specific operations (PUT only).
3. **Verify digests** after upload (never trust client-provided digests).
4. **Track upload sessions** in database for audit and cleanup.
5. **Support resumable uploads** for large files (multipart).
6. **Clean up temp uploads** on failure or expiration.
7. **Respect access control** (only generate presigned URLs for authorized users).

This architecture provides a scalable, efficient pattern for blob uploads while maintaining security and data integrity, directly applicable to DataForge's Parquet file upload workflow.

