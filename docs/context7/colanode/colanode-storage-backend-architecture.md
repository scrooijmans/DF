# ColaNode Storage Backend Architecture

This document explains how ColaNode's self-hosted version (under the `/hosting` folder) allows for defining storage backends for user files. ColaNode defaults to local filesystem storage, but you can switch to S3-compatible, Google Cloud Storage, or Azure Blob Storage backends by setting `STORAGE_TYPE`.

## Overview

ColaNode uses a **modular storage architecture** that abstracts file storage operations behind a unified interface. This design allows administrators to configure different storage backends without changing application code, providing flexibility for different deployment scenarios.

**Key Design Principles:**
- **Storage Interface Abstraction**: All storage operations go through a common interface
- **Environment-Based Configuration**: Storage backend is selected via `STORAGE_TYPE` environment variable
- **Backend Implementations**: Separate implementations for each storage type (filesystem, S3, GCS, Azure)
- **Seamless Switching**: Change storage backends by updating environment variables

## Storage Backend Types

### 1. Filesystem Storage (Default)

**Configuration:**
```yaml
STORAGE_TYPE="file"
STORAGE_FILE_DIRECTORY="/path/to/directory"
```

**Use Cases:**
- Local development
- Single-server deployments
- Small-scale installations
- Testing environments

**Considerations:**
- Requires directory to exist with write permissions
- For Docker: mount volume at specified path
- For Kubernetes: configure persistent volume claims
- Not suitable for multi-server deployments (no shared storage)

### 2. S3-Compatible Storage

**Configuration:**
```yaml
STORAGE_TYPE="s3"
STORAGE_S3_ENDPOINT="https://s3.amazonaws.com"  # or MinIO endpoint
STORAGE_S3_ACCESS_KEY="your_access_key"
STORAGE_S3_SECRET_KEY="your_secret_key"
STORAGE_S3_BUCKET="your_bucket_name"
STORAGE_S3_REGION="us-east-1"
STORAGE_S3_FORCE_PATH_STYLE="false"  # true for MinIO
```

**Use Cases:**
- Production deployments
- Multi-server setups
- Scalable storage needs
- AWS S3, MinIO, DigitalOcean Spaces, etc.

**Considerations:**
- Bucket must exist with proper permissions
- Credentials need read/write access
- For MinIO: set `STORAGE_S3_FORCE_PATH_STYLE="true"`

### 3. Google Cloud Storage (GCS)

**Configuration:**
```yaml
STORAGE_TYPE="gcs"
STORAGE_GCS_BUCKET="your_bucket_name"
STORAGE_GCS_PROJECT_ID="your_project_id"
STORAGE_GCS_CREDENTIALS="/path/to/service-account.json"
```

**Use Cases:**
- Google Cloud Platform deployments
- Integration with GCP services
- Enterprise GCP environments

**Considerations:**
- Requires GCS bucket and service account
- Service account JSON key must be accessible
- Service account needs Storage Object Admin permissions

### 4. Azure Blob Storage

**Configuration:**
```yaml
STORAGE_TYPE="azure"
STORAGE_AZURE_ACCOUNT="your_storage_account"
STORAGE_AZURE_CONTAINER_NAME="your_container_name"
STORAGE_AZURE_ACCOUNT_KEY="your_account_key"
```

**Use Cases:**
- Microsoft Azure deployments
- Azure-native applications
- Enterprise Azure environments

**Considerations:**
- Storage account and container must exist
- Account key retrieved from Azure Portal
- Container needs Blob Contributor permissions

## Code Design Architecture

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    ColaNode Server                          │
│                  (Node.js/Fastify)                          │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  File Upload/Download Routes                        │   │
│  │  - POST /api/files/upload                           │   │
│  │  - GET  /api/files/:id                              │   │
│  │  - DELETE /api/files/:id                            │   │
│  └────────────────────┬────────────────────────────────┘   │
│                       │                                      │
│  ┌────────────────────▼────────────────────────────────┐   │
│  │  Storage Service (Abstraction Layer)                │   │
│  │  - StorageInterface                                  │   │
│  │  - StorageFactory                                    │   │
│  └────────────────────┬────────────────────────────────┘   │
│                       │                                      │
│         ┌─────────────┼─────────────┬──────────────┐        │
│         │             │             │              │        │
│  ┌──────▼──────┐ ┌───▼────┐ ┌─────▼─────┐ ┌──────▼─────┐  │
│  │ Filesystem  │ │   S3   │ │    GCS    │ │   Azure    │  │
│  │ Storage     │ │ Storage│ │  Storage  │ │  Storage   │  │
│  │ Impl        │ │ Impl   │ │  Impl     │ │  Impl      │  │
│  └─────────────┘ └────────┘ └───────────┘ └────────────┘  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 1. Storage Interface

The storage interface defines the contract that all storage backends must implement:

```typescript
// hosting/src/storage/interface.ts

interface StorageInterface {
  /**
   * Upload a file to storage
   * @param filePath - Path where file should be stored
   * @param fileData - File buffer or stream
   * @param metadata - Optional file metadata
   * @returns URL or path to stored file
   */
  upload(
    filePath: string,
    fileData: Buffer | ReadableStream,
    metadata?: FileMetadata
  ): Promise<string>;

  /**
   * Download a file from storage
   * @param filePath - Path to file in storage
   * @returns File buffer or stream
   */
  download(filePath: string): Promise<Buffer | ReadableStream>;

  /**
   * Delete a file from storage
   * @param filePath - Path to file in storage
   */
  delete(filePath: string): Promise<void>;

  /**
   * Check if a file exists
   * @param filePath - Path to file in storage
   * @returns true if file exists
   */
  exists(filePath: string): Promise<boolean>;

  /**
   * Get file metadata
   * @param filePath - Path to file in storage
   * @returns File metadata (size, content-type, etc.)
   */
  getMetadata(filePath: string): Promise<FileMetadata>;

  /**
   * Generate a presigned URL for direct access
   * @param filePath - Path to file in storage
   * @param expiresIn - URL expiration time in seconds
   * @returns Presigned URL
   */
  getPresignedUrl(filePath: string, expiresIn?: number): Promise<string>;
}

interface FileMetadata {
  size: number;
  contentType?: string;
  lastModified?: Date;
  etag?: string;
}
```

### 2. Storage Factory

The factory pattern is used to create the appropriate storage implementation based on `STORAGE_TYPE`:

```typescript
// hosting/src/storage/factory.ts

import { StorageInterface } from './interface';
import { FilesystemStorage } from './filesystem';
import { S3Storage } from './s3';
import { GCSStorage } from './gcs';
import { AzureStorage } from './azure';

export class StorageFactory {
  /**
   * Create storage instance based on STORAGE_TYPE environment variable
   */
  static create(): StorageInterface {
    const storageType = process.env.STORAGE_TYPE || 'file';

    switch (storageType.toLowerCase()) {
      case 'file':
      case 'filesystem':
        return new FilesystemStorage({
          directory: process.env.STORAGE_FILE_DIRECTORY || './storage',
        });

      case 's3':
        return new S3Storage({
          endpoint: process.env.STORAGE_S3_ENDPOINT!,
          accessKey: process.env.STORAGE_S3_ACCESS_KEY!,
          secretKey: process.env.STORAGE_S3_SECRET_KEY!,
          bucket: process.env.STORAGE_S3_BUCKET!,
          region: process.env.STORAGE_S3_REGION || 'us-east-1',
          forcePathStyle: process.env.STORAGE_S3_FORCE_PATH_STYLE === 'true',
        });

      case 'gcs':
        return new GCSStorage({
          bucket: process.env.STORAGE_GCS_BUCKET!,
          projectId: process.env.STORAGE_GCS_PROJECT_ID!,
          credentialsPath: process.env.STORAGE_GCS_CREDENTIALS!,
        });

      case 'azure':
        return new AzureStorage({
          account: process.env.STORAGE_AZURE_ACCOUNT!,
          container: process.env.STORAGE_AZURE_CONTAINER_NAME!,
          accountKey: process.env.STORAGE_AZURE_ACCOUNT_KEY!,
        });

      default:
        throw new Error(`Unsupported storage type: ${storageType}`);
    }
  }

  /**
   * Validate storage configuration
   */
  static validateConfig(): void {
    const storageType = process.env.STORAGE_TYPE || 'file';

    switch (storageType.toLowerCase()) {
      case 'file':
        if (!process.env.STORAGE_FILE_DIRECTORY) {
          throw new Error('STORAGE_FILE_DIRECTORY is required for filesystem storage');
        }
        break;

      case 's3':
        const requiredS3 = [
          'STORAGE_S3_ENDPOINT',
          'STORAGE_S3_ACCESS_KEY',
          'STORAGE_S3_SECRET_KEY',
          'STORAGE_S3_BUCKET',
        ];
        for (const key of requiredS3) {
          if (!process.env[key]) {
            throw new Error(`${key} is required for S3 storage`);
          }
        }
        break;

      case 'gcs':
        const requiredGCS = [
          'STORAGE_GCS_BUCKET',
          'STORAGE_GCS_PROJECT_ID',
          'STORAGE_GCS_CREDENTIALS',
        ];
        for (const key of requiredGCS) {
          if (!process.env[key]) {
            throw new Error(`${key} is required for GCS storage`);
          }
        }
        break;

      case 'azure':
        const requiredAzure = [
          'STORAGE_AZURE_ACCOUNT',
          'STORAGE_AZURE_CONTAINER_NAME',
          'STORAGE_AZURE_ACCOUNT_KEY',
        ];
        for (const key of requiredAzure) {
          if (!process.env[key]) {
            throw new Error(`${key} is required for Azure storage`);
          }
        }
        break;
    }
  }
}
```

### 3. Storage Service (Singleton)

The storage service provides a single point of access to the storage backend:

```typescript
// hosting/src/storage/service.ts

import { StorageFactory } from './factory';
import { StorageInterface } from './interface';

class StorageService {
  private static instance: StorageService;
  private storage: StorageInterface;

  private constructor() {
    // Validate configuration on startup
    StorageFactory.validateConfig();
    
    // Create storage instance
    this.storage = StorageFactory.create();
  }

  static getInstance(): StorageService {
    if (!StorageService.instance) {
      StorageService.instance = new StorageService();
    }
    return StorageService.instance;
  }

  getStorage(): StorageInterface {
    return this.storage;
  }

  /**
   * Reinitialize storage (useful for testing or config changes)
   */
  reinitialize(): void {
    StorageFactory.validateConfig();
    this.storage = StorageFactory.create();
  }
}

export const storageService = StorageService.getInstance();
```

### 4. Filesystem Storage Implementation

```typescript
// hosting/src/storage/filesystem.ts

import * as fs from 'fs/promises';
import * as path from 'path';
import { StorageInterface, FileMetadata } from './interface';

export class FilesystemStorage implements StorageInterface {
  private rootDirectory: string;

  constructor(config: { directory: string }) {
    this.rootDirectory = path.resolve(config.directory);
    this.ensureDirectoryExists();
  }

  private async ensureDirectoryExists(): Promise<void> {
    try {
      await fs.mkdir(this.rootDirectory, { recursive: true });
    } catch (error) {
      throw new Error(`Failed to create storage directory: ${error}`);
    }
  }

  async upload(
    filePath: string,
    fileData: Buffer | ReadableStream,
    metadata?: FileMetadata
  ): Promise<string> {
    const fullPath = path.join(this.rootDirectory, filePath);
    const dir = path.dirname(fullPath);

    // Ensure directory exists
    await fs.mkdir(dir, { recursive: true });

    // Handle Buffer or Stream
    if (fileData instanceof Buffer) {
      await fs.writeFile(fullPath, fileData);
    } else {
      // Stream handling
      const writeStream = fs.createWriteStream(fullPath);
      fileData.pipeTo(writeStream as any);
    }

    return filePath;
  }

  async download(filePath: string): Promise<Buffer> {
    const fullPath = path.join(this.rootDirectory, filePath);
    return await fs.readFile(fullPath);
  }

  async delete(filePath: string): Promise<void> {
    const fullPath = path.join(this.rootDirectory, filePath);
    await fs.unlink(fullPath);
  }

  async exists(filePath: string): Promise<boolean> {
    const fullPath = path.join(this.rootDirectory, filePath);
    try {
      await fs.access(fullPath);
      return true;
    } catch {
      return false;
    }
  }

  async getMetadata(filePath: string): Promise<FileMetadata> {
    const fullPath = path.join(this.rootDirectory, filePath);
    const stats = await fs.stat(fullPath);
    return {
      size: stats.size,
      lastModified: stats.mtime,
    };
  }

  async getPresignedUrl(filePath: string, expiresIn?: number): Promise<string> {
    // For filesystem, return a local file URL
    // In production, this might be served through a file server endpoint
    return `/api/files/${filePath}`;
  }
}
```

### 5. S3 Storage Implementation

```typescript
// hosting/src/storage/s3.ts

import { S3Client, PutObjectCommand, GetObjectCommand, DeleteObjectCommand, HeadObjectCommand } from '@aws-sdk/client-s3';
import { getSignedUrl } from '@aws-sdk/s3-request-presigner';
import { StorageInterface, FileMetadata } from './interface';

export class S3Storage implements StorageInterface {
  private s3Client: S3Client;
  private bucket: string;

  constructor(config: {
    endpoint: string;
    accessKey: string;
    secretKey: string;
    bucket: string;
    region: string;
    forcePathStyle: boolean;
  }) {
    this.bucket = config.bucket;
    this.s3Client = new S3Client({
      endpoint: config.endpoint,
      region: config.region,
      credentials: {
        accessKeyId: config.accessKey,
        secretAccessKey: config.secretKey,
      },
      forcePathStyle: config.forcePathStyle,
    });
  }

  async upload(
    filePath: string,
    fileData: Buffer | ReadableStream,
    metadata?: FileMetadata
  ): Promise<string> {
    const command = new PutObjectCommand({
      Bucket: this.bucket,
      Key: filePath,
      Body: fileData instanceof Buffer ? fileData : this.streamToBuffer(fileData),
      ContentType: metadata?.contentType,
    });

    await this.s3Client.send(command);
    return filePath;
  }

  async download(filePath: string): Promise<Buffer> {
    const command = new GetObjectCommand({
      Bucket: this.bucket,
      Key: filePath,
    });

    const response = await this.s3Client.send(command);
    return Buffer.from(await response.Body!.transformToByteArray());
  }

  async delete(filePath: string): Promise<void> {
    const command = new DeleteObjectCommand({
      Bucket: this.bucket,
      Key: filePath,
    });

    await this.s3Client.send(command);
  }

  async exists(filePath: string): Promise<boolean> {
    try {
      const command = new HeadObjectCommand({
        Bucket: this.bucket,
        Key: filePath,
      });
      await this.s3Client.send(command);
      return true;
    } catch (error: any) {
      if (error.name === 'NotFound') {
        return false;
      }
      throw error;
    }
  }

  async getMetadata(filePath: string): Promise<FileMetadata> {
    const command = new HeadObjectCommand({
      Bucket: this.bucket,
      Key: filePath,
    });

    const response = await this.s3Client.send(command);
    return {
      size: response.ContentLength || 0,
      contentType: response.ContentType,
      lastModified: response.LastModified,
      etag: response.ETag,
    };
  }

  async getPresignedUrl(filePath: string, expiresIn: number = 3600): Promise<string> {
    const command = new GetObjectCommand({
      Bucket: this.bucket,
      Key: filePath,
    });

    return await getSignedUrl(this.s3Client, command, { expiresIn });
  }

  private async streamToBuffer(stream: ReadableStream): Promise<Buffer> {
    const reader = stream.getReader();
    const chunks: Uint8Array[] = [];

    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      chunks.push(value);
    }

    return Buffer.concat(chunks);
  }
}
```

### 6. GCS Storage Implementation

```typescript
// hosting/src/storage/gcs.ts

import { Storage } from '@google-cloud/storage';
import { StorageInterface, FileMetadata } from './interface';

export class GCSStorage implements StorageInterface {
  private storage: Storage;
  private bucket: any;

  constructor(config: {
    bucket: string;
    projectId: string;
    credentialsPath: string;
  }) {
    this.storage = new Storage({
      projectId: config.projectId,
      keyFilename: config.credentialsPath,
    });
    this.bucket = this.storage.bucket(config.bucket);
  }

  async upload(
    filePath: string,
    fileData: Buffer | ReadableStream,
    metadata?: FileMetadata
  ): Promise<string> {
    const file = this.bucket.file(filePath);
    
    const options: any = {};
    if (metadata?.contentType) {
      options.metadata = { contentType: metadata.contentType };
    }

    if (fileData instanceof Buffer) {
      await file.save(fileData, options);
    } else {
      // Stream handling
      const writeStream = file.createWriteStream(options);
      fileData.pipeTo(writeStream as any);
    }

    return filePath;
  }

  async download(filePath: string): Promise<Buffer> {
    const file = this.bucket.file(filePath);
    const [buffer] = await file.download();
    return buffer;
  }

  async delete(filePath: string): Promise<void> {
    const file = this.bucket.file(filePath);
    await file.delete();
  }

  async exists(filePath: string): Promise<boolean> {
    const file = this.bucket.file(filePath);
    const [exists] = await file.exists();
    return exists;
  }

  async getMetadata(filePath: string): Promise<FileMetadata> {
    const file = this.bucket.file(filePath);
    const [metadata] = await file.getMetadata();
    return {
      size: parseInt(metadata.size || '0'),
      contentType: metadata.contentType,
      lastModified: new Date(metadata.updated),
      etag: metadata.etag,
    };
  }

  async getPresignedUrl(filePath: string, expiresIn: number = 3600): Promise<string> {
    const file = this.bucket.file(filePath);
    const [url] = await file.getSignedUrl({
      action: 'read',
      expires: Date.now() + expiresIn * 1000,
    });
    return url;
  }
}
```

### 7. Azure Storage Implementation

```typescript
// hosting/src/storage/azure.ts

import { BlobServiceClient, StorageSharedKeyCredential } from '@azure/storage-blob';
import { StorageInterface, FileMetadata } from './interface';

export class AzureStorage implements StorageInterface {
  private blobServiceClient: BlobServiceClient;
  private containerClient: any;

  constructor(config: {
    account: string;
    container: string;
    accountKey: string;
  }) {
    const credential = new StorageSharedKeyCredential(config.account, config.accountKey);
    this.blobServiceClient = new BlobServiceClient(
      `https://${config.account}.blob.core.windows.net`,
      credential
    );
    this.containerClient = this.blobServiceClient.getContainerClient(config.container);
  }

  async upload(
    filePath: string,
    fileData: Buffer | ReadableStream,
    metadata?: FileMetadata
  ): Promise<string> {
    const blockBlobClient = this.containerClient.getBlockBlobClient(filePath);
    
    const options: any = {};
    if (metadata?.contentType) {
      options.blobHTTPHeaders = { blobContentType: metadata.contentType };
    }

    if (fileData instanceof Buffer) {
      await blockBlobClient.upload(fileData, fileData.length, options);
    } else {
      // Stream handling
      await blockBlobClient.uploadStream(fileData as any, undefined, undefined, options);
    }

    return filePath;
  }

  async download(filePath: string): Promise<Buffer> {
    const blockBlobClient = this.containerClient.getBlockBlobClient(filePath);
    const downloadResponse = await blockBlobClient.download(0);
    return Buffer.from(await this.streamToBuffer(downloadResponse.readableStreamBody!));
  }

  async delete(filePath: string): Promise<void> {
    const blockBlobClient = this.containerClient.getBlockBlobClient(filePath);
    await blockBlobClient.delete();
  }

  async exists(filePath: string): Promise<boolean> {
    const blockBlobClient = this.containerClient.getBlockBlobClient(filePath);
    return await blockBlobClient.exists();
  }

  async getMetadata(filePath: string): Promise<FileMetadata> {
    const blockBlobClient = this.containerClient.getBlockBlobClient(filePath);
    const properties = await blockBlobClient.getProperties();
    return {
      size: properties.contentLength || 0,
      contentType: properties.contentType,
      lastModified: properties.lastModified,
      etag: properties.etag,
    };
  }

  async getPresignedUrl(filePath: string, expiresIn: number = 3600): Promise<string> {
    const blockBlobClient = this.containerClient.getBlockBlobClient(filePath);
    const expiresOn = new Date(Date.now() + expiresIn * 1000);
    return await blockBlobClient.generateSasUrl({
      permissions: 'r',
      expiresOn,
    });
  }

  private async streamToBuffer(stream: NodeJS.ReadableStream): Promise<Buffer> {
    const chunks: Buffer[] = [];
    return new Promise((resolve, reject) => {
      stream.on('data', (chunk) => chunks.push(chunk));
      stream.on('end', () => resolve(Buffer.concat(chunks)));
      stream.on('error', reject);
    });
  }
}
```

## Call Stack: File Upload Flow

### Complete Call Stack Diagram

```
┌─────────────────────────────────────────────────────────────┐
│  1. Client Request                                          │
│     POST /api/files/upload                                  │
│     Content-Type: multipart/form-data                       │
│     Body: { file: <File>, workspaceId: "..." }             │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Fastify Route Handler                                   │
│     hosting/src/routes/files.ts                             │
│                                                             │
│     async function uploadFile(request, reply) {             │
│       const file = await request.file();                    │
│       const workspaceId = request.body.workspaceId;         │
│       ...                                                   │
│     }                                                       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. File Service                                            │
│     hosting/src/services/file-service.ts                    │
│                                                             │
│     class FileService {                                     │
│       async uploadFile(file, workspaceId) {                 │
│         // Generate file path                               │
│         const filePath = this.generatePath(...);            │
│         // Get storage instance                             │
│         const storage = storageService.getStorage();        │
│         // Upload to storage                                │
│         await storage.upload(...);                          │
│       }                                                     │
│     }                                                       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Storage Service (Singleton)                             │
│     hosting/src/storage/service.ts                          │
│                                                             │
│     storageService.getStorage()                             │
│       → Returns StorageInterface instance                   │
│       → Instance created by StorageFactory                  │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  5. Storage Factory                                         │
│     hosting/src/storage/factory.ts                          │
│                                                             │
│     StorageFactory.create()                                 │
│       → Reads STORAGE_TYPE env var                          │
│       → Creates appropriate implementation:                 │
│         • FilesystemStorage (if STORAGE_TYPE=file)          │
│         • S3Storage (if STORAGE_TYPE=s3)                    │
│         • GCSStorage (if STORAGE_TYPE=gcs)                  │
│         • AzureStorage (if STORAGE_TYPE=azure)              │
└──────────────────────┬──────────────────────────────────────┘
                       │
         ┌─────────────┼─────────────┬──────────────┐
         │             │             │              │
    ┌────▼─────┐  ┌───▼────┐  ┌─────▼─────┐  ┌────▼──────┐
    │Filesystem│  │   S3   │  │    GCS    │  │  Azure    │
    │ Storage  │  │ Storage│  │  Storage  │  │  Storage  │
    │ Impl     │  │ Impl   │  │  Impl     │  │  Impl     │
    └────┬─────┘  └───┬────┘  └─────┬─────┘  └────┬──────┘
         │            │              │             │
         ▼            ▼              ▼             ▼
    ┌──────────────────────────────────────────────┐
    │  6. Storage Backend Operations               │
    │                                              │
    │  Filesystem:                                 │
    │    → fs.writeFile()                          │
    │    → Local filesystem                        │
    │                                              │
    │  S3:                                         │
    │    → AWS SDK PutObjectCommand                │
    │    → S3/MinIO API                            │
    │                                              │
    │  GCS:                                        │
    │    → @google-cloud/storage                   │
    │    → Google Cloud Storage API                │
    │                                              │
    │  Azure:                                      │
    │    → @azure/storage-blob                     │
    │    → Azure Blob Storage API                  │
    └──────────────────────────────────────────────┘
```

### Detailed Call Stack: S3 Upload Example

```typescript
// 1. Client makes request
POST /api/files/upload
{
  file: <File>,
  workspaceId: "workspace-123"
}

// 2. Route handler (hosting/src/routes/files.ts)
app.post('/api/files/upload', async (request, reply) => {
  const file = await request.file();
  const workspaceId = request.body.workspaceId;
  
  // Call file service
  const fileService = new FileService();
  const result = await fileService.uploadFile(file, workspaceId);
  
  return reply.send({ fileId: result.id, url: result.url });
});

// 3. File Service (hosting/src/services/file-service.ts)
class FileService {
  async uploadFile(file: File, workspaceId: string) {
    // Generate file path
    const fileId = generateFileId();
    const filePath = `workspaces/${workspaceId}/files/${fileId}`;
    
    // Get storage instance
    const storage = storageService.getStorage();
    
    // Upload to storage
    await storage.upload(filePath, file.buffer, {
      contentType: file.mimetype,
      size: file.size,
    });
    
    // Save metadata to PostgreSQL
    await this.saveFileMetadata(fileId, filePath, workspaceId);
    
    return { id: fileId, path: filePath };
  }
}

// 4. Storage Service (hosting/src/storage/service.ts)
storageService.getStorage()
  → Returns StorageInterface instance
  → Instance was created at startup by StorageFactory.create()

// 5. Storage Factory (hosting/src/storage/factory.ts)
StorageFactory.create() {
  const storageType = process.env.STORAGE_TYPE; // "s3"
  
  switch (storageType) {
    case 's3':
      return new S3Storage({
        endpoint: process.env.STORAGE_S3_ENDPOINT,
        accessKey: process.env.STORAGE_S3_ACCESS_KEY,
        secretKey: process.env.STORAGE_S3_SECRET_KEY,
        bucket: process.env.STORAGE_S3_BUCKET,
        region: process.env.STORAGE_S3_REGION,
        forcePathStyle: process.env.STORAGE_S3_FORCE_PATH_STYLE === 'true',
      });
  }
}

// 6. S3 Storage Implementation (hosting/src/storage/s3.ts)
class S3Storage implements StorageInterface {
  async upload(filePath: string, fileData: Buffer, metadata?: FileMetadata) {
    const command = new PutObjectCommand({
      Bucket: this.bucket,
      Key: filePath,
      Body: fileData,
      ContentType: metadata?.contentType,
    });
    
    // AWS SDK call
    await this.s3Client.send(command);
    
    return filePath;
  }
}

// 7. AWS SDK → S3 API
// HTTP PUT request to S3 endpoint
PUT https://s3.amazonaws.com/bucket-name/workspaces/workspace-123/files/file-id
Headers: Authorization, Content-Type, etc.
Body: <file data>
```

## Initialization Flow

### Server Startup Sequence

```
┌─────────────────────────────────────────────────────────────┐
│  1. Server Startup                                          │
│     hosting/src/index.ts                                    │
│     - Load environment variables                            │
│     - Initialize Fastify app                                │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Storage Service Initialization                          │
│     hosting/src/storage/service.ts                          │
│                                                             │
│     StorageService.getInstance()                            │
│       → Validates configuration                             │
│       → Calls StorageFactory.create()                       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Configuration Validation                                │
│     StorageFactory.validateConfig()                         │
│                                                             │
│     - Checks STORAGE_TYPE env var                           │
│     - Validates required env vars for selected type         │
│     - Throws error if configuration invalid                 │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Storage Instance Creation                               │
│     StorageFactory.create()                                 │
│                                                             │
│     switch (STORAGE_TYPE) {                                 │
│       case 'file':                                          │
│         → new FilesystemStorage({ directory: ... })         │
│         → Creates directory if needed                       │
│                                                             │
│       case 's3':                                            │
│         → new S3Storage({ endpoint, credentials, ... })     │
│         → Initializes AWS S3Client                          │
│                                                             │
│       case 'gcs':                                           │
│         → new GCSStorage({ bucket, credentials, ... })      │
│         → Initializes Google Cloud Storage client           │
│                                                             │
│       case 'azure':                                         │
│         → new AzureStorage({ account, container, ... })     │
│         → Initializes Azure BlobServiceClient               │
│     }                                                       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  5. Storage Ready                                           │
│     - Storage instance stored in StorageService singleton   │
│     - Available via storageService.getStorage()             │
│     - Used by file routes and services                      │
└─────────────────────────────────────────────────────────────┘
```

## File Download Flow

### Call Stack: File Download

```
┌─────────────────────────────────────────────────────────────┐
│  1. Client Request                                          │
│     GET /api/files/:fileId                                  │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Route Handler                                           │
│     app.get('/api/files/:fileId', async (request, reply) => {
│       const fileId = request.params.fileId;                 │
│       const fileService = new FileService();                │
│       const file = await fileService.getFile(fileId);       │
│       return reply.send(file);                              │
│     });                                                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. File Service                                            │
│     async getFile(fileId: string) {                         │
│       // Get file metadata from PostgreSQL                  │
│       const metadata = await db.getFileMetadata(fileId);    │
│                                                             │
│       // Get storage instance                               │
│       const storage = storageService.getStorage();          │
│                                                             │
│       // Download from storage                              │
│       const fileData = await storage.download(metadata.path);│
│                                                             │
│       return {                                              │
│         data: fileData,                                     │
│         contentType: metadata.contentType,                  │
│         filename: metadata.filename,                        │
│       };                                                    │
│     }                                                       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Storage Interface                                       │
│     storage.download(filePath)                              │
│       → Calls implementation-specific download method        │
└──────────────────────┬──────────────────────────────────────┘
                       │
         ┌─────────────┼─────────────┬──────────────┐
         │             │             │              │
    ┌────▼─────┐  ┌───▼────┐  ┌─────▼─────┐  ┌────▼──────┐
    │Filesystem│  │   S3   │  │    GCS    │  │  Azure    │
    │          │  │        │  │           │  │           │
    │fs.readFile│ │GetObject│ │file.download│ │blockBlob  │
    │          │  │Command │  │           │  │.download()│
    └──────────┘  └────────┘  └───────────┘  └───────────┘
```

## Presigned URL Generation

For direct client-to-storage access (bypassing server), presigned URLs are generated:

```
┌─────────────────────────────────────────────────────────────┐
│  1. Client Request                                          │
│     GET /api/files/:fileId/url?expiresIn=3600               │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Route Handler                                           │
│     app.get('/api/files/:fileId/url', async (request, reply) => {
│       const fileId = request.params.fileId;                 │
│       const expiresIn = request.query.expiresIn || 3600;    │
│                                                             │
│       const fileService = new FileService();                │
│       const url = await fileService.getPresignedUrl(        │
│         fileId,                                             │
│         expiresIn                                           │
│       );                                                    │
│                                                             │
│       return reply.send({ url });                           │
│     });                                                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. File Service                                            │
│     async getPresignedUrl(fileId: string, expiresIn: number) {
│       // Get file metadata                                  │
│       const metadata = await db.getFileMetadata(fileId);    │
│                                                             │
│       // Get storage instance                               │
│       const storage = storageService.getStorage();          │
│                                                             │
│       // Generate presigned URL                             │
│       const url = await storage.getPresignedUrl(            │
│         metadata.path,                                      │
│         expiresIn                                           │
│       );                                                    │
│                                                             │
│       return url;                                           │
│     }                                                       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Storage Implementation                                  │
│                                                             │
│     S3:                                                     │
│       → getSignedUrl(GetObjectCommand, { expiresIn })       │
│       → Returns: https://s3.amazonaws.com/bucket/key?...    │
│                                                             │
│     GCS:                                                    │
│       → file.getSignedUrl({ action: 'read', expires })      │
│       → Returns: https://storage.googleapis.com/bucket/key? │
│                                                             │
│     Azure:                                                  │
│       → blockBlobClient.generateSasUrl({ expiresOn })       │
│       → Returns: https://account.blob.core.windows.net/...  │
│                                                             │
│     Filesystem:                                             │
│       → Returns: /api/files/:fileId (server endpoint)       │
└─────────────────────────────────────────────────────────────┘
```

## Key Design Patterns

### 1. Strategy Pattern

The storage backend selection uses the **Strategy Pattern**:
- **Context**: `StorageService` (uses storage strategy)
- **Strategy Interface**: `StorageInterface` (defines operations)
- **Concrete Strategies**: `FilesystemStorage`, `S3Storage`, `GCSStorage`, `AzureStorage`

### 2. Factory Pattern

The **Factory Pattern** is used to create storage instances:
- `StorageFactory.create()` - Creates appropriate storage based on configuration
- Encapsulates creation logic
- Validates configuration before creation

### 3. Singleton Pattern

The **Singleton Pattern** ensures a single storage instance:
- `StorageService.getInstance()` - Returns single instance
- Initialized once at server startup
- Shared across all requests

### 4. Dependency Injection

Storage is injected into services:
- `FileService` receives `StorageInterface` via `storageService.getStorage()`
- Services depend on interface, not concrete implementations
- Easy to test with mock storage

## Configuration Management

### Environment Variables

All storage configuration is done via environment variables:

```bash
# Storage Type Selection
STORAGE_TYPE=file          # or s3, gcs, azure

# Filesystem Configuration
STORAGE_FILE_DIRECTORY=/app/storage

# S3 Configuration
STORAGE_S3_ENDPOINT=https://s3.amazonaws.com
STORAGE_S3_ACCESS_KEY=your_access_key
STORAGE_S3_SECRET_KEY=your_secret_key
STORAGE_S3_BUCKET=your_bucket
STORAGE_S3_REGION=us-east-1
STORAGE_S3_FORCE_PATH_STYLE=false

# GCS Configuration
STORAGE_GCS_BUCKET=your_bucket
STORAGE_GCS_PROJECT_ID=your_project_id
STORAGE_GCS_CREDENTIALS=/path/to/service-account.json

# Azure Configuration
STORAGE_AZURE_ACCOUNT=your_storage_account
STORAGE_AZURE_CONTAINER_NAME=your_container
STORAGE_AZURE_ACCOUNT_KEY=your_account_key
```

### Docker Compose Example

```yaml
services:
  colanode-server:
    image: colanode/server:latest
    environment:
      # Database
      DATABASE_URL: postgresql://postgres:password@postgres:5432/colanode
      
      # Storage Configuration
      STORAGE_TYPE: s3
      STORAGE_S3_ENDPOINT: http://minio:9000
      STORAGE_S3_ACCESS_KEY: minioadmin
      STORAGE_S3_SECRET_KEY: minioadmin
      STORAGE_S3_BUCKET: colanode-files
      STORAGE_S3_REGION: us-east-1
      STORAGE_S3_FORCE_PATH_STYLE: "true"
    depends_on:
      - postgres
      - minio
```

## Error Handling

### Storage Error Types

```typescript
class StorageError extends Error {
  constructor(
    message: string,
    public code: string,
    public statusCode: number
  ) {
    super(message);
  }
}

// Common error codes
const STORAGE_ERRORS = {
  CONFIG_INVALID: 'STORAGE_CONFIG_INVALID',
  FILE_NOT_FOUND: 'FILE_NOT_FOUND',
  UPLOAD_FAILED: 'UPLOAD_FAILED',
  DOWNLOAD_FAILED: 'DOWNLOAD_FAILED',
  PERMISSION_DENIED: 'PERMISSION_DENIED',
};
```

### Error Handling Flow

```
Storage Operation
    ↓
Try {
  → Execute storage operation
} Catch (error) {
  → Wrap in StorageError
  → Log error details
  → Return appropriate HTTP status
}
```

## Testing

### Mock Storage for Testing

```typescript
class MockStorage implements StorageInterface {
  private files: Map<string, Buffer> = new Map();

  async upload(filePath: string, fileData: Buffer): Promise<string> {
    this.files.set(filePath, fileData);
    return filePath;
  }

  async download(filePath: string): Promise<Buffer> {
    const data = this.files.get(filePath);
    if (!data) throw new Error('File not found');
    return data;
  }

  // ... other methods
}

// In tests
const mockStorage = new MockStorage();
storageService.reinitialize(); // Replace with mock
```

## Performance Considerations

### 1. Connection Pooling

- **S3**: AWS SDK handles connection pooling automatically
- **GCS**: Google Cloud Storage client manages connections
- **Azure**: Azure SDK handles connection pooling

### 2. Streaming

All implementations support streaming for large files:
- Avoids loading entire file into memory
- Reduces memory usage
- Improves performance for large uploads/downloads

### 3. Caching

- File metadata cached in PostgreSQL
- Presigned URLs cached (short TTL)
- Storage client instances reused (singleton)

## Security Considerations

### 1. Credential Management

- Credentials stored in environment variables
- Never committed to version control
- Use secrets management in production (Kubernetes secrets, etc.)

### 2. Access Control

- File access controlled by application logic
- Presigned URLs have expiration times
- Storage backends configured with minimal required permissions

### 3. Path Validation

- File paths validated to prevent directory traversal
- Workspace isolation enforced
- User permissions checked before file operations

## Migration Between Storage Backends

### Switching Storage Types

1. **Update Environment Variables**
   ```bash
   # Change from filesystem to S3
   STORAGE_TYPE=s3
   STORAGE_S3_ENDPOINT=...
   # ... other S3 config
   ```

2. **Restart Server**
   - Storage service reinitializes with new backend
   - New files use new storage backend

3. **Migrate Existing Files** (if needed)
   - Script to copy files from old to new storage
   - Update file metadata in database
   - Verify all files migrated

## Key Takeaways

1. **Modular Design**: Storage backend is abstracted behind `StorageInterface`
2. **Environment-Based Configuration**: `STORAGE_TYPE` determines backend
3. **Factory Pattern**: `StorageFactory` creates appropriate implementation
4. **Singleton Service**: Single storage instance shared across application
5. **Unified API**: Same interface for all storage backends
6. **Easy Testing**: Mock storage implementations for unit tests
7. **Flexible Deployment**: Switch backends without code changes
8. **Production Ready**: Supports S3, GCS, Azure for scalable deployments

## References

- [ColaNode Self-Hosting Documentation](https://colanode.com/docs/self-hosting/overview/)
- [ColaNode Configuration Guide](https://colanode.com/docs/self-hosting/configuration/)
- [AWS S3 SDK Documentation](https://docs.aws.amazon.com/sdk-for-javascript/v3/developer-guide/s3-examples.html)
- [Google Cloud Storage Documentation](https://cloud.google.com/storage/docs)
- [Azure Blob Storage Documentation](https://docs.microsoft.com/en-us/azure/storage/blobs/)

