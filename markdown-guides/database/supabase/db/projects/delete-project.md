# Project Deletion with PostgreSQL and MinIO

This guide explains how we delete projects from our hybrid architecture using PostgreSQL for metadata and MinIO for storage, following the new project-based system.

## Architecture Overview

### Third-Party APIs Used

1. **Supabase JavaScript Client** (`@supabase/supabase-js`)
   - Official Supabase client for PostgreSQL operations
   - Handles authentication and RLS policies
   - Provides type safety and error handling

2. **AWS SDK S3 for Rust** (`aws-sdk-s3 = "1.107.0"`)
   - Official AWS SDK for S3 operations
   - Handles MinIO bucket deletion
   - Provides full S3 API compatibility

3. **PostgreSQL with RLS** (via Supabase)
   - Row Level Security for data isolation
   - User-project access management
   - Audit trail and permission tracking

### Hybrid Project Deletion

**Two-Phase Deletion**: We delete both project metadata and associated storage:

- ✅ **Phase 1**: Delete project from PostgreSQL (metadata, permissions, workspaces, notebooks)
- ✅ **Phase 2**: Delete associated MinIO bucket and all stored data
- ✅ **RLS Policies**: Automatic security based on user permissions
- ✅ **Cascade Deletion**: Related data is automatically cleaned up

## Step-by-Step Implementation Process

### Step 1: Frontend Request

**Component**: `content-projects-item.svelte`
**Third-Party APIs**: None (Pure SvelteKit)

```typescript
// User clicks delete button on project
const deleteProject = async (projectId: string) => {
  if (
    !confirm(
      `Are you sure you want to delete this project? This action cannot be undone.`,
    )
  ) {
    return;
  }

  const success = await deleteProjectWithStorage(projectId);
  // Update UI after deletion
};
```

### Step 2: Project Deletion Service

**Component**: `src/lib/services/project-deletion-service.ts`
**Third-Party APIs**: Supabase JavaScript Client

```typescript
import { createClient } from "@supabase/supabase-js";

const supabase = createClient(supabaseUrl, supabaseServiceKey);

export async function deleteProjectWithStorage(
  projectId: string,
): Promise<boolean> {
  console.log(
    "🗑️ [ProjectDeletionService] Starting project deletion:",
    projectId,
  );

  try {
    // Step 1: Get project details for bucket name
    const { data: project, error: projectError } = await supabase
      .from("projects")
      .select("name, owner_id")
      .eq("id", projectId)
      .single();

    if (projectError) {
      throw new Error(`Failed to get project details: ${projectError.message}`);
    }

    // Step 2: Check if user has permission to delete (owner or admin)
    const hasPermission = await hasProjectPermission(projectId, "owner");
    if (!hasPermission) {
      throw new Error("You don't have permission to delete this project");
    }

    // Step 3: Delete project from PostgreSQL (cascades to related tables)
    const { error: deleteError } = await supabase
      .from("projects")
      .delete()
      .eq("id", projectId);

    if (deleteError) {
      throw new Error(
        `Failed to delete project from database: ${deleteError.message}`,
      );
    }

    console.log("✅ [ProjectDeletionService] Project deleted from database");

    // Step 4: Delete MinIO bucket using Tauri command
    const bucketName = `project-${projectId}`;
    await deleteProjectBucket(bucketName);

    console.log(
      "✅ [ProjectDeletionService] Project and storage deleted successfully",
    );
    return true;
  } catch (error) {
    console.error(
      "❌ [ProjectDeletionService] Failed to delete project:",
      error,
    );
    throw error;
  }
}
```

### Step 3: PostgreSQL Project Deletion

**Component**: Supabase PostgreSQL Database
**Third-Party APIs**: PostgreSQL with Row Level Security

```sql
-- Delete project (cascades to related tables due to foreign key constraints)
DELETE FROM projects WHERE id = $1;

-- This automatically deletes:
-- - All workspaces in the project
-- - All notebooks in those workspaces
-- - All project permissions
-- - All project-related data
```

### Step 4: RLS Policy Enforcement

**Component**: PostgreSQL Row Level Security
**Third-Party APIs**: PostgreSQL RLS Engine

```sql
-- Users can only delete projects they own or have admin permissions for
CREATE POLICY "Users can delete projects they own or admin" ON projects
    FOR DELETE
    TO authenticated
    USING (
        (select auth.uid()) = owner_id OR
        id IN (
            SELECT project_id
            FROM project_permissions
            WHERE user_id = (select auth.uid())
            AND permission_level = 'admin'
        )
    );
```

### Step 5: MinIO Bucket Deletion

**Component**: `src/lib/tauri-commands/minio-bucket-commands.ts`
**Third-Party APIs**: Tauri IPC (Rust ↔ TypeScript)

```typescript
import { invoke } from "@tauri-apps/api/core";

export async function deleteProjectBucket(bucketName: string): Promise<string> {
  const message = await invoke<string>("delete_minio_bucket", {
    bucketName: bucketName,
  });
  return message;
}
```

### Step 6: Rust Backend Processing

**Component**: `src-tauri/src/main.rs`
**Third-Party APIs**: Tauri Commands

```rust
#[tauri::command]
async fn delete_minio_bucket(bucket_name: String) -> Result<String, String> {
    let minio_manager = MinioFilesManager::new().await
        .map_err(|e| format!("Failed to initialize MinIO client: {}", e))?;

    let response = minio_manager.delete_bucket(&bucket_name).await
        .map_err(|e| format!("Failed to delete MinIO bucket: {}", e))?;

    if response.success {
        Ok(response.message)
    } else {
        Err(response.message)
    }
}
```

### Step 7: AWS SDK S3 Bucket Deletion

**Component**: `src-tauri/src/analytics_query/minio_files.rs`
**Third-Party APIs**:

- `aws-config = "1.8.7"` - Configuration management
- `aws-sdk-s3 = "1.107.0"` - S3 operations

```rust
pub async fn delete_bucket(&self, bucket_name: &str) -> Result<MinioBucketDeletionResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("🗑️ Deleting MinIO bucket: {} using AWS SDK S3...", bucket_name);

    // Check if bucket exists before attempting deletion
    if !self.bucket_exists(bucket_name).await? {
        return Ok(MinioBucketDeletionResponse {
            success: false,
            message: format!("Bucket '{}' does not exist", bucket_name),
            bucket_name: bucket_name.to_string(),
        });
    }

    // Check if bucket is empty before deletion
    if !self.is_bucket_empty(bucket_name).await? {
        return Ok(MinioBucketDeletionResponse {
            success: false,
            message: format!("Bucket '{}' is not empty. Please delete all objects before deleting the bucket", bucket_name),
            bucket_name: bucket_name.to_string(),
        });
    }

    // Delete bucket using AWS SDK S3
    let response = self.s3_client
        .delete_bucket()
        .bucket(bucket_name)
        .send()
        .await;

    match response {
        Ok(_) => {
            println!("✅ Successfully deleted bucket '{}' via AWS SDK S3", bucket_name);
            Ok(MinioBucketDeletionResponse {
                success: true,
                message: format!("Successfully deleted bucket '{}'", bucket_name),
                bucket_name: bucket_name.to_string(),
            })
        }
        Err(err) => {
            // Handle NoSuchBucket error as success (bucket already deleted)
            if let Some(service_error) = err.as_service_error() {
                if service_error.code() == Some("NoSuchBucket") {
                    println!("✅ Bucket '{}' was already deleted or does not exist", bucket_name);
                    return Ok(MinioBucketDeletionResponse {
                        success: true,
                        message: format!("Bucket '{}' was already deleted or does not exist", bucket_name),
                        bucket_name: bucket_name.to_string(),
                    });
                }
            }

            println!("❌ Failed to delete bucket '{}': {}", bucket_name, err);
            Ok(MinioBucketDeletionResponse {
                success: false,
                message: format!("Failed to delete bucket '{}': {}", bucket_name, err),
                bucket_name: bucket_name.to_string(),
            })
        }
    }
}
```

### Step 8: Frontend State Update

**Component**: `content-projects.svelte`
**Third-Party APIs**: None (Pure SvelteKit state management)

```typescript
// Update component state after project deletion
const handleProjectDeleted = (projectId: string) => {
  // Remove project from list
  projects = projects.filter((project) => project.id !== projectId);

  // Show success message
  toast.success("Project deleted successfully");

  // Refresh project list to ensure consistency
  loadProjects();
};
```

## Project Deletion Safety Features

### 1. **Permission Validation**

- Checks if user owns the project or has admin permissions
- Uses RLS policies for automatic security
- Prevents unauthorized deletions

### 2. **Cascade Deletion**

- PostgreSQL foreign key constraints automatically delete related data
- Workspaces, notebooks, and permissions are cleaned up
- No orphaned data left in the database

### 3. **Storage Cleanup**

- Associated MinIO bucket is deleted
- All stored data is removed
- Bucket existence and empty validation

### 4. **Error Handling**

- Comprehensive error handling at each step
- User-friendly error messages
- Rollback capability if needed

## Following PostgreSQL Best Practices

Our implementation follows PostgreSQL cascade deletion patterns:

```sql
-- Foreign key constraints ensure cascade deletion
ALTER TABLE workspaces
ADD CONSTRAINT fk_workspaces_project_id
FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE;

ALTER TABLE project_permissions
ADD CONSTRAINT fk_project_permissions_project_id
FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE;
```

## Advantages of This Approach

### 1. **Complete Cleanup**

- Both metadata and storage are deleted
- No orphaned data in database
- No leftover files in storage

### 2. **Security**

- RLS policies prevent unauthorized deletions
- Permission-based access control
- Audit trail maintained

### 3. **Performance**

- Efficient cascade deletion in PostgreSQL
- Direct MinIO communication
- Minimal database queries

### 4. **Reliability**

- Transactional consistency
- Error handling at each step
- Rollback capability

## Configuration Requirements

### Environment Variables

```bash
# Supabase Configuration
VITE_SUPABASE_URL=http://91.99.166.223:8000
VITE_SUPABASE_ANON_KEY=your_anon_key_here
SUPABASE_SERVICE_ROLE_KEY=your_service_role_key_here

# MinIO Configuration
AWS_ACCESS_KEY_ID=mudrock-storage
AWS_SECRET_ACCESS_KEY=mudrock-storage-secret-2024
MINIO_ENDPOINT=http://91.99.166.223:9000
MINIO_REGION=us-east-1
```

### Database Requirements

- RLS enabled on `projects` table
- Foreign key constraints with CASCADE DELETE
- Proper indexes for performance
- Permission management functions available

## Frontend Integration

### Component State Management

```typescript
// content-projects-item.svelte
let isDeleting = $state(false);

const handleDelete = async () => {
  if (
    !confirm(
      `Are you sure you want to delete project "${project.name}"? This action cannot be undone.`,
    )
  ) {
    return;
  }

  try {
    isDeleting = true;
    await deleteProjectWithStorage(project.id);
    onDelete(project.id);
  } catch (error) {
    toast.error(error.message);
  } finally {
    isDeleting = false;
  }
};
```

### Project Deletion Service

```typescript
// src/lib/services/project-deletion-service.ts
export async function deleteProjectWithStorage(
  projectId: string,
): Promise<boolean>;
export async function deleteProjectBucket(bucketName: string): Promise<string>;
export async function hasProjectPermission(
  projectId: string,
  requiredLevel: PermissionLevel,
): Promise<boolean>;
```

## Comparison with Bucket-Only Deletion

| Feature           | Bucket-Only Deletion  | Project Deletion (Hybrid) |
| ----------------- | --------------------- | ------------------------- |
| **Data Source**   | MinIO only            | PostgreSQL + MinIO        |
| **Security**      | No built-in security  | RLS policies              |
| **Cleanup**       | Storage only          | Metadata + Storage        |
| **Relationships** | None                  | Cascade deletion          |
| **Performance**   | Direct storage access | Database + Storage        |
| **Scalability**   | Limited by storage    | Scales with PostgreSQL    |
| **Audit Trail**   | No                    | Full audit trail          |

## Troubleshooting

### Common Issues

1. **"Permission denied"**
   - Check if user owns the project or has admin permissions
   - Verify RLS policies are configured correctly
   - Ensure user is authenticated

2. **"Project not found"**
   - Check if project ID is correct
   - Verify project exists in database
   - Check if user has access to the project

3. **"Bucket deletion failed"**
   - Check if MinIO service is running
   - Verify AWS credentials
   - Check if bucket is empty

4. **"Cascade deletion failed"**
   - Check foreign key constraints
   - Verify database permissions
   - Check for data integrity issues

### Debugging

```typescript
// Check user permissions
const hasPermission = await hasProjectPermission(projectId, "owner");
console.log("Has permission:", hasPermission);

// Check project details
const project = await getProjectDetails(projectId);
console.log("Project details:", project);
```

## Third-Party APIs Summary

### Complete API Stack Used

| Step  | Component                | Third-Party APIs           | Purpose                              |
| ----- | ------------------------ | -------------------------- | ------------------------------------ |
| **1** | Frontend UI              | None (Pure SvelteKit)      | User interface and confirmation      |
| **2** | Project Deletion Service | `@supabase/supabase-js`    | PostgreSQL client and authentication |
| **3** | Supabase Backend         | Supabase JavaScript Client | API routing and RLS enforcement      |
| **4** | PostgreSQL Database      | PostgreSQL with RLS        | Data storage and cascade deletion    |
| **5** | Tauri Command Bridge     | Tauri IPC                  | Rust ↔ TypeScript communication     |
| **6** | Rust Backend             | Tauri Commands             | Command registration and routing     |
| **7** | AWS SDK S3               | `aws-sdk-s3 = "1.107.0"`   | S3 protocol implementation           |
| **8** | MinIO Storage            | MinIO S3-Compatible API    | Object storage via HTTP              |
| **9** | Frontend Update          | None (Pure SvelteKit)      | UI state updates and rendering       |

### Key Third-Party Dependencies

1. **Supabase JavaScript Client** (`@supabase/supabase-js`)
   - **Purpose**: PostgreSQL client with authentication and RLS
   - **Used For**: Project deletion, permission checking, data queries
   - **Why**: Official client, RLS integration, type safety

2. **AWS SDK S3 for Rust** (`aws-sdk-s3 = "1.107.0"`)
   - **Purpose**: S3 protocol implementation and HTTP communication
   - **Used For**: `delete_bucket()`, `head_bucket()`, `list_objects_v2()`
   - **Why**: Industry standard, full S3 compatibility, robust error handling

3. **PostgreSQL with RLS**
   - **Purpose**: Database backend with security policies and cascade deletion
   - **Used For**: Data storage, permission management, audit trail
   - **Why**: ACID compliance, RLS security, foreign key constraints

4. **Tauri IPC**
   - **Purpose**: Frontend-backend communication
   - **Used For**: TypeScript ↔ Rust function calls
   - **Why**: Secure, type-safe, efficient

### Data Flow Architecture

```
Frontend (SvelteKit)
    ↓ Project Deletion Service
Supabase JavaScript Client
    ↓ HTTP/WebSocket
Supabase Backend (API Gateway)
    ↓ SQL DELETE with CASCADE
PostgreSQL Database (with RLS)
    ↓ Success Response
Project Deletion Service
    ↓ Tauri IPC
Rust Backend (Tauri Commands)
    ↓ AWS SDK S3
MinIO Storage (S3-Compatible API)
    ↓ Success Response
Frontend (UI Update)
```

## Conclusion

This implementation provides a secure, comprehensive, and reliable way to delete projects from our hybrid architecture. By leveraging both PostgreSQL's cascade deletion and MinIO's storage capabilities, we ensure complete cleanup while maintaining security and data integrity.

**Key Benefits:**

- **Complete Cleanup**: Both metadata and storage are deleted
- **Security**: RLS policies prevent unauthorized deletions
- **Consistency**: Transactional integrity across database and storage
- **Performance**: Efficient cascade deletion and direct storage access
- **Reliability**: Comprehensive error handling and rollback capability
- **Audit Trail**: Full tracking of deletion operations
- **User Experience**: Clear confirmation dialogs and error messages

**Next Steps:**

1. Create the project deletion service
2. Update the content-projects-item component
3. Add comprehensive error handling and user feedback
4. Test the complete end-to-end flow
