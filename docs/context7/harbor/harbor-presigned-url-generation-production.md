# Harbor Presigned URL Generation for Production Blobs: Architecture & Design (Context7 Summary)

This document dives deep into **Harbor's architecture and design for presigned URL generation for production blob downloads** (S3/MinIO). It focuses on how Harbor generates presigned URLs that allow direct client-to-storage downloads, bypassing Harbor servers for improved performance and reduced bandwidth costs.

It builds on:

- `harbor-object-storage-architecture.md`
- `harbor-blob-download-architecture.md`
- `harbor-blob-upload-presigned-url-architecture.md`

and provides detailed call stacks and implementation patterns specifically for presigned URL generation in production environments.

---

## 1. High-Level Architecture: Presigned URL Generation Pattern

### 1.1 The Problem

Traditional blob download flow:

- Client → Harbor API → Storage Backend → Harbor API → Client
- All data passes through Harbor servers
- Harbor becomes a bottleneck for large downloads
- High bandwidth costs and latency
- Reduced scalability

**Solution**: Presigned URLs enable **direct client-to-storage downloads**:

- Client → S3/Object Storage (direct)
- Harbor only orchestrates (generates presigned URLs, validates access)
- Reduces server load and improves download performance
- Enables CDN-like performance for blob downloads

### 1.2 Two Download Patterns

Harbor supports two download patterns:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Harbor Blob Download Patterns                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Pattern 1: Proxy Download (Traditional)                    │  │
│  │  ┌──────────┐      ┌──────────┐      ┌──────────┐          │  │
│  │  │ Client   │─────▶│ Harbor   │─────▶│ Storage  │          │  │
│  │  │          │◀─────│ API      │◀─────│ Backend  │          │  │
│  │  └──────────┘      └──────────┘      └──────────┘          │  │
│  │  - All data flows through Harbor                            │  │
│  │  - Works with any storage backend                           │  │
│  │  - Higher server load and bandwidth costs                   │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Pattern 2: Presigned URL Download (S3/MinIO)               │  │
│  │  ┌──────────┐      ┌──────────┐                             │  │
│  │  │ Client   │─────▶│ Storage  │  (Direct download)          │  │
│  │  │          │      │ Backend  │                             │  │
│  │  └──────────┘      └──────────┘                             │  │
│  │         │                                                      │  │
│  │         │ 1. Request presigned URL                            │  │
│  │         ▼                                                      │  │
│  │  ┌──────────┐                                                 │  │
│  │  │ Harbor   │  (Orchestrates, validates access)               │  │
│  │  │ API      │                                                 │  │
│  │  └──────────┘                                                 │  │
│  │  - Direct client-to-storage download                          │  │
│  │  - Harbor only generates presigned URLs                       │  │
│  │  - Lower server load, better performance                     │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Presigned URL Generation API

### 2.1 Harbor API Endpoint

Harbor provides an API endpoint for generating presigned URLs:

```
GET /api/v2.0/blobs/{digest}/url
  ?expires_in=3600
  &repository={repository_name}
```

**Request Parameters**:

- `digest` (path): Blob digest (e.g., `sha256:abc123...`)
- `expires_in` (query, optional): URL expiration time in seconds (default: 3600)
- `repository` (query, optional): Repository name for access control

**Response**:

```json
{
	"url": "https://s3.amazonaws.com/bucket/blobs/sha256/ab/c1/abc123...?X-Amz-Algorithm=...&X-Amz-Expires=3600&...",
	"expires_at": "2025-01-15T11:30:00Z"
}
```

### 2.2 Docker Distribution Integration

Harbor integrates with Docker Distribution's storage driver interface:

```go
// Storage driver interface (simplified)
type Driver interface {
    // Standard blob operations
    Reader(ctx context.Context, path string, offset int64) (io.ReadCloser, error)
    Writer(ctx context.Context, path string, append bool) (io.WriteCloser, error)

    // Presigned URL generation (S3/MinIO specific)
    PresignedURL(ctx context.Context, path string, expiresIn time.Duration) (string, error)

    // Blob metadata
    Stat(ctx context.Context, path string) (FileInfo, error)
    Delete(ctx context.Context, path string) error
}
```

---

## 3. Complete Call Stack: Presigned URL Generation

### 3.1 High-Level Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  Presigned URL Generation: Complete Call Stack                      │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: Client Requests Presigned URL                      │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  GET /api/v2.0/blobs/{digest}/url?expires_in=3600     │  │  │
│  │  │  Authorization: Bearer {token}                         │  │  │
│  │  └──────────────────────┬─────────────────────────────────┘  │  │
│  └─────────────────────────┼─────────────────────────────────────┘  │
│                            │                                         │
│                            ▼                                         │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Harbor Core API: Authentication & Authorization    │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Validate JWT token / session                       │  │  │
│  │  │  - Extract user context                               │  │  │
│  │  │  - Check blob access permissions                       │  │  │
│  │  │  - Verify repository access (if repository specified)  │  │  │
│  │  └──────────────────────┬─────────────────────────────────┘  │  │
│  └─────────────────────────┼─────────────────────────────────────┘  │
│                            │                                         │
│                            ▼                                         │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: Harbor Core API: Resolve Blob Metadata            │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Query PostgreSQL for blob metadata                 │  │  │
│  │  │  - Verify blob exists                                 │  │  │
│  │  │  - Check blob size, digest                            │  │  │
│  │  │  - Derive storage path from digest                    │  │  │
│  │  └──────────────────────┬─────────────────────────────────┘  │  │
│  └─────────────────────────┼─────────────────────────────────────┘  │
│                            │                                         │
│                            ▼                                         │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 4: Storage Driver: Generate Presigned URL            │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Get S3 client from driver                           │  │  │
│  │  │  - Create GetObjectRequest                             │  │  │
│  │  │  - Presign request with expiration                     │  │  │
│  │  │  - Return presigned URL                                │  │  │
│  │  └──────────────────────┬─────────────────────────────────┘  │  │
│  └─────────────────────────┼─────────────────────────────────────┘  │
│                            │                                         │
│                            ▼                                         │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 5: Harbor Core API: Return Presigned URL             │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Return presigned URL to client                      │  │  │
│  │  │  - Include expiration timestamp                        │  │  │
│  │  │  - Log access for audit                                │  │  │
│  │  └──────────────────────┬─────────────────────────────────┘  │  │
│  └─────────────────────────┼─────────────────────────────────────┘  │
│                            │                                         │
│                            ▼                                         │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 6: Client Downloads Directly from S3                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Client uses presigned URL                           │  │  │
│  │  │  - Direct GET request to S3                            │  │  │
│  │  │  - S3 validates presigned URL signature                │  │  │
│  │  │  - S3 streams blob data to client                      │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 4. Detailed Implementation: Harbor Core API Handler

### 4.1 API Handler Implementation

```go
// Harbor Core API handler for presigned URL generation
package handler

import (
    "context"
    "net/http"
    "time"

    "github.com/goharbor/harbor/src/controller/blob"
    "github.com/goharbor/harbor/src/lib/errors"
    "github.com/goharbor/harbor/src/pkg/permission/types"
)

// GetBlobPresignedURL handles GET /api/v2.0/blobs/{digest}/url
func (h *Handler) GetBlobPresignedURL(w http.ResponseWriter, r *http.Request) {
    // Step 1: Extract request parameters
    digest := mux.Vars(r)["digest"]
    expiresIn := parseExpiresIn(r.URL.Query().Get("expires_in"))
    repository := r.URL.Query().Get("repository")

    // Step 2: Get user context from authentication middleware
    ctx := r.Context()
    user, ok := ctx.Value("user").(*models.User)
    if !ok {
        respondError(w, errors.UnauthorizedError(nil).WithMessage("Unauthorized"))
        return
    }

    // Step 3: Validate blob access permissions
    if err := h.validateBlobAccess(ctx, user, digest, repository); err != nil {
        respondError(w, err)
        return
    }

    // Step 4: Resolve blob metadata
    blobInfo, err := h.blobController.Get(ctx, digest)
    if err != nil {
        respondError(w, errors.NotFoundError(nil).WithMessage("Blob not found"))
        return
    }

    // Step 5: Get storage driver
    driver, err := h.getStorageDriver(ctx)
    if err != nil {
        respondError(w, errors.InternalError(nil).WithMessage("Failed to get storage driver"))
        return
    }

    // Step 6: Generate presigned URL
    presignedURL, err := driver.PresignedURL(ctx, blobInfo.Path, expiresIn)
    if err != nil {
        respondError(w, errors.InternalError(err).WithMessage("Failed to generate presigned URL"))
        return
    }

    // Step 7: Log access for audit
    h.logBlobAccess(ctx, user, digest, "presigned_url_generated")

    // Step 8: Return response
    response := &PresignedURLResponse{
        URL:       presignedURL,
        ExpiresAt: time.Now().Add(expiresIn).Format(time.RFC3339),
    }

    respondJSON(w, http.StatusOK, response)
}

// validateBlobAccess checks if user has permission to access blob
func (h *Handler) validateBlobAccess(
    ctx context.Context,
    user *models.User,
    digest string,
    repository string,
) error {
    // If repository is specified, check repository access
    if repository != "" {
        // Parse repository name (format: project/repo)
        parts := strings.Split(repository, "/")
        if len(parts) != 2 {
            return errors.BadRequestError(nil).WithMessage("Invalid repository name")
        }

        projectName := parts[0]
        repoName := parts[1]

        // Check project access
        project, err := h.projectController.Get(ctx, projectName)
        if err != nil {
            return errors.NotFoundError(nil).WithMessage("Project not found")
        }

        // Check user permissions
        if !h.hasPermission(ctx, user, project.ProjectID, types.ResourceRepository, types.ActionPull) {
            return errors.ForbiddenError(nil).WithMessage("Access denied")
        }

        // Verify blob belongs to repository
        if err := h.verifyBlobInRepository(ctx, digest, project.ProjectID, repoName); err != nil {
            return errors.ForbiddenError(nil).WithMessage("Blob not found in repository")
        }
    } else {
        // No repository specified - check if user has admin access
        if !h.isAdmin(ctx, user) {
            return errors.ForbiddenError(nil).WithMessage("Repository required for non-admin users")
        }
    }

    return nil
}
```

---

## 5. Detailed Implementation: S3 Storage Driver

### 5.1 S3 Driver Presigned URL Generation

```go
// S3 storage driver implementation
package s3

import (
    "context"
    "fmt"
    "time"

    "github.com/aws/aws-sdk-go/aws"
    "github.com/aws/aws-sdk-go/aws/credentials"
    "github.com/aws/aws-sdk-go/aws/session"
    "github.com/aws/aws-sdk-go/service/s3"
)

// S3Driver implements storage.Driver interface
type S3Driver struct {
    s3Client *s3.S3
    bucket   string
    region   string
    endpoint string // For MinIO compatibility
}

// NewS3Driver creates a new S3 storage driver
func NewS3Driver(config S3Config) (*S3Driver, error) {
    // Create AWS session
    sess, err := session.NewSession(&aws.Config{
        Region:      aws.String(config.Region),
        Endpoint:    aws.String(config.Endpoint), // MinIO endpoint
        Credentials: credentials.NewStaticCredentials(
            config.AccessKey,
            config.SecretKey,
            "",
        ),
        S3ForcePathStyle: aws.Bool(true), // Required for MinIO
    })
    if err != nil {
        return nil, fmt.Errorf("failed to create S3 session: %w", err)
    }

    return &S3Driver{
        s3Client: s3.New(sess),
        bucket:   config.Bucket,
        region:   config.Region,
        endpoint: config.Endpoint,
    }, nil
}

// PresignedURL generates a presigned URL for blob download
func (d *S3Driver) PresignedURL(
    ctx context.Context,
    path string,
    expiresIn time.Duration,
) (string, error) {
    // Step 1: Create GetObject request
    req, _ := d.s3Client.GetObjectRequest(&s3.GetObjectInput{
        Bucket: aws.String(d.bucket),
        Key:    aws.String(path),
    })

    // Step 2: Presign the request
    // The Presign method adds AWS signature to the URL
    // The URL will be valid for expiresIn duration
    url, err := req.Presign(expiresIn)
    if err != nil {
        return "", fmt.Errorf("failed to presign request: %w", err)
    }

    // Step 3: Return presigned URL
    // Example URL format:
    // https://s3.amazonaws.com/bucket/path?X-Amz-Algorithm=...&X-Amz-Credential=...&X-Amz-Date=...&X-Amz-Expires=3600&X-Amz-SignedHeaders=host&X-Amz-Signature=...
    return url, nil
}

// PresignedURLWithHeaders generates a presigned URL with custom headers
func (d *S3Driver) PresignedURLWithHeaders(
    ctx context.Context,
    path string,
    expiresIn time.Duration,
    headers map[string]string,
) (string, error) {
    // Create GetObject request with headers
    input := &s3.GetObjectInput{
        Bucket: aws.String(d.bucket),
        Key:    aws.String(path),
    }

    // Add custom headers (e.g., Content-Disposition for download filename)
    for key, value := range headers {
        switch key {
        case "Content-Disposition":
            input.ContentDisposition = aws.String(value)
        case "Content-Type":
            input.ContentType = aws.String(value)
        case "ResponseCacheControl":
            input.ResponseCacheControl = aws.String(value)
        }
    }

    req, _ := d.s3Client.GetObjectRequest(input)

    // Presign with headers included in signature
    url, err := req.Presign(expiresIn)
    if err != nil {
        return "", fmt.Errorf("failed to presign request: %w", err)
    }

    return url, nil
}
```

### 5.2 MinIO Compatibility

MinIO is S3-compatible, so the same S3 driver works with MinIO:

```go
// MinIO configuration (same as S3, just different endpoint)
type MinIOConfig struct {
    Endpoint  string // e.g., "http://minio.example.com:9000"
    AccessKey string
    SecretKey string
    Bucket    string
    Region    string // MinIO uses "us-east-1" by default
    UseSSL    bool
}

// Create MinIO driver (uses S3 driver with MinIO endpoint)
func NewMinIODriver(config MinIOConfig) (*S3Driver, error) {
    endpoint := config.Endpoint
    if config.UseSSL {
        endpoint = "https://" + strings.TrimPrefix(endpoint, "http://")
    }

    return NewS3Driver(S3Config{
        Endpoint:  endpoint,
        AccessKey: config.AccessKey,
        SecretKey: config.SecretKey,
        Bucket:    config.Bucket,
        Region:    config.Region,
    })
}
```

---

## 6. Presigned URL Security & Access Control

### 6.1 Security Considerations

**Expiration**:

- Presigned URLs should expire after a reasonable time (default: 1 hour)
- Prevents unauthorized access if URL is leaked
- Configurable per request via `expires_in` parameter

**Scope**:

- Presigned URLs are scoped to specific operations (GET only for downloads)
- Include required headers in presigned request
- Cannot be used for other operations (PUT, DELETE)

**Access Control**:

- Harbor validates user permissions before generating presigned URL
- Presigned URLs respect Harbor's access control policies
- Only authenticated, authorized users can request presigned URLs

**Signature Verification**:

- S3/MinIO validates presigned URL signature server-side
- Signature includes request method, path, expiration, and credentials
- Tampered URLs are rejected by S3/MinIO

### 6.2 Access Control Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  Access Control for Presigned URL Generation                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: User Authentication                                │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Validate JWT token / session                       │  │  │
│  │  │  - Extract user context                               │  │  │
│  │  │  - Verify token expiration                            │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Repository Access Check                            │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Check if repository is public                       │  │  │
│  │  │  - Check user project membership                       │  │  │
│  │  │  - Check user permissions (pull access)                │  │  │
│  │  │  - Verify blob belongs to repository                   │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: Generate Presigned URL                             │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Only if access check passes                         │  │  │
│  │  │  - Include expiration in URL                           │  │  │
│  │  │  - Sign URL with S3 credentials                        │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 4: Log Access for Audit                               │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Log presigned URL generation                        │  │  │
│  │  │  - Record user, blob, timestamp                        │  │  │
│  │  │  - Track for security monitoring                       │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 7. Production Considerations

### 7.1 Performance Optimization

**Caching**:

- Cache presigned URLs for short duration (e.g., 5 minutes)
- Reduces S3 API calls for frequently accessed blobs
- Invalidate cache on blob deletion/update

**Batch Generation**:

- Support batch presigned URL generation for multiple blobs
- Reduces API round-trips
- Useful for downloading multiple layers

**CDN Integration**:

- Use CloudFront/Cloudflare in front of S3
- Presigned URLs work with CDN
- Improves global download performance

### 7.2 Monitoring & Logging

**Access Logging**:

```go
// Log presigned URL generation
func (h *Handler) logBlobAccess(
    ctx context.Context,
    user *models.User,
    digest string,
    operation string,
) {
    log.Info(ctx, "Blob access",
        "user", user.Username,
        "digest", digest,
        "operation", operation,
        "timestamp", time.Now(),
    )

    // Store in access_log table
    h.accessLogDAO.Create(ctx, &models.AccessLog{
        Username:  user.Username,
        BlobDigest: digest,
        Operation: operation,
        OpTime:    time.Now(),
    })
}
```

**Metrics**:

- Track presigned URL generation rate
- Monitor presigned URL usage (downloads via presigned URLs)
- Alert on unusual access patterns

### 7.3 Error Handling

```go
// Error handling for presigned URL generation
func (d *S3Driver) PresignedURL(
    ctx context.Context,
    path string,
    expiresIn time.Duration,
) (string, error) {
    // Validate expiration duration
    if expiresIn <= 0 || expiresIn > 7*24*time.Hour {
        return "", errors.BadRequestError(nil).
            WithMessage("expires_in must be between 1 second and 7 days")
    }

    // Check if blob exists (optional, for better error messages)
    _, err := d.Stat(ctx, path)
    if err != nil {
        return "", errors.NotFoundError(err).
            WithMessage("Blob not found in storage")
    }

    // Generate presigned URL
    req, _ := d.s3Client.GetObjectRequest(&s3.GetObjectInput{
        Bucket: aws.String(d.bucket),
        Key:    aws.String(path),
    })

    url, err := req.Presign(expiresIn)
    if err != nil {
        // Log error for debugging
        log.Error(ctx, "Failed to presign S3 request",
            "path", path,
            "bucket", d.bucket,
            "error", err,
        )
        return "", errors.InternalError(err).
            WithMessage("Failed to generate presigned URL")
    }

    return url, nil
}
```

---

## 8. Client-Side Usage

### 8.1 Request Presigned URL

```typescript
// Client-side: Request presigned URL from Harbor
async function getBlobPresignedURL(
	client: HarborClient,
	digest: string,
	expiresIn: number = 3600
): Promise<string> {
	const response = await client.get(`/api/v2.0/blobs/${digest}/url`, {
		params: {
			expires_in: expiresIn
		},
		headers: {
			Authorization: `Bearer ${token}`
		}
	});

	return response.data.url;
}
```

### 8.2 Download Using Presigned URL

```typescript
// Client-side: Download blob using presigned URL
async function downloadBlob(
	presignedUrl: string,
	onProgress?: (progress: number) => void
): Promise<Blob> {
	const response = await fetch(presignedUrl, {
		method: 'GET'
	});

	if (!response.ok) {
		throw new Error(`Failed to download blob: ${response.statusText}`);
	}

	// Get content length for progress tracking
	const contentLength = parseInt(response.headers.get('Content-Length') || '0', 10);

	// Stream download with progress
	const reader = response.body?.getReader();
	const chunks: Uint8Array[] = [];
	let receivedLength = 0;

	if (reader) {
		while (true) {
			const { done, value } = await reader.read();

			if (done) break;

			chunks.push(value);
			receivedLength += value.length;

			if (onProgress && contentLength > 0) {
				onProgress((receivedLength / contentLength) * 100);
			}
		}
	}

	// Combine chunks into blob
	const blob = new Blob(chunks);
	return blob;
}
```

### 8.3 Complete Download Flow

```typescript
// Complete flow: Get presigned URL and download
async function downloadBlobWithPresignedURL(client: HarborClient, digest: string): Promise<Blob> {
	// Step 1: Get presigned URL from Harbor
	const presignedUrl = await getBlobPresignedURL(client, digest);

	// Step 2: Download directly from S3 using presigned URL
	const blob = await downloadBlob(presignedUrl, (progress) => {
		console.log(`Download progress: ${progress.toFixed(2)}%`);
	});

	return blob;
}
```

---

## 9. Design Patterns for DataForge

### 9.1 Presigned URL Download Pattern

**Harbor Pattern**:

- Generate presigned URLs for direct client-to-storage downloads
- Harbor orchestrates (validates access, generates URLs)
- Storage backend handles actual download

**DataForge Application**:

- Use presigned URLs for Parquet file downloads
- Client downloads directly from S3/MinIO
- DataForge sync server tracks metadata in PostgreSQL
- Reduces server bandwidth and improves download performance

### 9.2 Access Control Integration

**Harbor Pattern**:

- Validate user permissions before generating presigned URL
- Check repository access and blob ownership
- Log access for audit

**DataForge Application**:

- Validate workspace membership before generating presigned URL
- Check user permissions for workspace data
- Log access for audit and compliance

### 9.3 Production Deployment

**Harbor Pattern**:

- Use S3/MinIO for blob storage
- Generate presigned URLs with appropriate expiration
- Monitor access patterns and performance

**DataForge Application**:

- Use MinIO for Parquet file storage
- Generate presigned URLs for large file downloads
- Implement caching and monitoring

---

## 10. Summary

### ✅ Key Patterns

1. **Direct Client-to-Storage**: Presigned URLs enable direct downloads, bypassing Harbor servers
2. **Access Control First**: Validate permissions before generating presigned URLs
3. **Secure Signatures**: S3/MinIO validates presigned URL signatures server-side
4. **Configurable Expiration**: URLs expire after a set duration (default: 1 hour)
5. **Audit Logging**: Log all presigned URL generation for security monitoring

### 📋 Best Practices

1. **Validate access** before generating presigned URLs
2. **Set appropriate expiration** (1 hour default, max 7 days)
3. **Log all access** for audit and security
4. **Monitor usage** patterns and performance
5. **Use CDN** in front of S3 for global performance
6. **Cache presigned URLs** for frequently accessed blobs
7. **Handle errors gracefully** with clear error messages

This architecture provides a robust foundation for presigned URL generation in production environments, enabling efficient, secure, and scalable blob downloads while maintaining access control and auditability.

