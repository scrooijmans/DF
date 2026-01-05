# Project Name Editing with PostgreSQL

This guide explains how we edit project names in our hybrid architecture using PostgreSQL for metadata and MinIO for storage, following the new project-based system.

## Architecture Overview

### Third-Party APIs Used

1. **Supabase JavaScript Client** (`@supabase/supabase-js`)
   - Official Supabase client for PostgreSQL operations
   - Handles authentication and RLS policies
   - Provides type safety and error handling

2. **PostgreSQL with RLS** (via Supabase)
   - Row Level Security for data isolation
   - User-project access management
   - Audit trail and permission tracking

### Project Name Editing

**Owner-Only Editing**: Only project owners can edit project names:

- ✅ **Permission Check**: Verify user is project owner
- ✅ **Input Validation**: Validate name format and length
- ✅ **Uniqueness Check**: Ensure name is not already taken
- ✅ **Database Update**: Update project name in PostgreSQL
- ✅ **UI Update**: Refresh the frontend display

## Step-by-Step Implementation Process

### Step 1: Frontend Edit Mode Toggle

**Component**: `content-projects-item.svelte`
**Third-Party APIs**: None (Pure SvelteKit)

```typescript
// User clicks edit button on project
let isEditing = $state(false);
let editName = $state(project.name);

function toggleEdit() {
  isEditing = !isEditing;
  if (isEditing) {
    editName = project.name;
  }
}
```

### Step 2: Project Editing Service

**Component**: `src/lib/services/project-editing-service.ts`
**Third-Party APIs**: Supabase JavaScript Client

```typescript
import { supabase } from "$lib/supabase";

export interface ProjectUpdateData {
  name?: string;
  description?: string;
}

export async function updateProject(
  projectId: string,
  updateData: ProjectUpdateData,
): Promise<boolean> {
  console.log(
    "✏️ [ProjectEditingService] Starting project update:",
    projectId,
    updateData,
  );

  try {
    // Step 1: Check if user has permission to edit (owner only)
    const hasPermission = await hasProjectPermission(projectId, "owner");
    if (!hasPermission) {
      throw new Error("Only the project owner can edit this project");
    }

    // Step 2: Validate input data
    if (updateData.name !== undefined) {
      if (!updateData.name.trim()) {
        throw new Error("Project name cannot be empty");
      }
      if (updateData.name.length > 100) {
        throw new Error("Project name must be 100 characters or less");
      }
      if (
        !/^[a-zA-Z0-9][a-zA-Z0-9\s\-_]*[a-zA-Z0-9]$/.test(
          updateData.name.trim(),
        )
      ) {
        throw new Error(
          "Project name must start and end with alphanumeric characters and can only contain letters, numbers, spaces, hyphens, and underscores",
        );
      }
    }

    // Step 3: Check if project name is already taken
    if (updateData.name) {
      const { data: existingProject, error: checkError } = await supabase
        .from("projects")
        .select("id")
        .eq("name", updateData.name.trim())
        .neq("id", projectId)
        .single();

      if (existingProject) {
        throw new Error("A project with this name already exists");
      }
    }

    // Step 4: Update project in PostgreSQL
    const updatePayload: any = {
      updated_at: new Date().toISOString(),
    };

    if (updateData.name !== undefined) {
      updatePayload.name = updateData.name.trim();
    }
    if (updateData.description !== undefined) {
      updatePayload.description = updateData.description.trim();
    }

    const { error: updateError } = await supabase
      .from("projects")
      .update(updatePayload)
      .eq("id", projectId);

    if (updateError) {
      throw new Error(`Failed to update project: ${updateError.message}`);
    }

    console.log("✅ [ProjectEditingService] Project updated successfully");
    return true;
  } catch (error) {
    console.error(
      "❌ [ProjectEditingService] Failed to update project:",
      error,
    );
    throw error;
  }
}
```

### Step 3: Permission Validation

**Component**: `project-editing-service.ts`
**Third-Party APIs**: Supabase JavaScript Client

```typescript
export async function hasProjectPermission(
  projectId: string,
  requiredLevel: PermissionLevel,
): Promise<boolean> {
  try {
    // Get current user
    const {
      data: { user },
      error: userError,
    } = await supabase.auth.getUser();
    if (userError || !user) {
      return false;
    }

    // Check if user owns the project
    const { data: project, error: projectError } = await supabase
      .from("projects")
      .select("owner_id")
      .eq("id", projectId)
      .single();

    if (projectError) {
      return false;
    }

    // If user owns the project, they have all permissions
    if (project.owner_id === user.id) {
      return true;
    }

    // Check if user has explicit permissions
    const { data: permission, error: permissionError } = await supabase
      .from("project_permissions")
      .select("role")
      .eq("project_id", projectId)
      .eq("user_id", user.id)
      .single();

    if (permissionError) {
      return false;
    }

    return checkPermissionLevel(permission.role, requiredLevel);
  } catch (error) {
    return false;
  }
}
```

### Step 4: Input Validation

**Component**: `project-editing-service.ts`
**Third-Party APIs**: None (Pure JavaScript validation)

```typescript
// Name validation rules
if (updateData.name !== undefined) {
  // 1. Not empty
  if (!updateData.name.trim()) {
    throw new Error("Project name cannot be empty");
  }

  // 2. Length limit
  if (updateData.name.length > 100) {
    throw new Error("Project name must be 100 characters or less");
  }

  // 3. Format validation (alphanumeric start/end, allows spaces, hyphens, underscores)
  if (
    !/^[a-zA-Z0-9][a-zA-Z0-9\s\-_]*[a-zA-Z0-9]$/.test(updateData.name.trim())
  ) {
    throw new Error(
      "Project name must start and end with alphanumeric characters and can only contain letters, numbers, spaces, hyphens, and underscores",
    );
  }
}

// Description validation
if (
  updateData.description !== undefined &&
  updateData.description.length > 500
) {
  throw new Error("Project description must be 500 characters or less");
}
```

### Step 5: Uniqueness Check

**Component**: `project-editing-service.ts`
**Third-Party APIs**: Supabase JavaScript Client

```typescript
// Check if project name is already taken (if name is being updated)
if (updateData.name) {
  const { data: existingProject, error: checkError } = await supabase
    .from("projects")
    .select("id")
    .eq("name", updateData.name.trim())
    .neq("id", projectId) // Exclude current project
    .single();

  if (checkError && checkError.code !== "PGRST116") {
    // PGRST116 = no rows found
    throw new Error("Failed to check if project name is available");
  }

  if (existingProject) {
    throw new Error("A project with this name already exists");
  }
}
```

### Step 6: PostgreSQL Update

**Component**: Supabase PostgreSQL Database
**Third-Party APIs**: PostgreSQL with Row Level Security

```sql
-- Update project name and description
UPDATE projects
SET
  name = $1,
  description = $2,
  updated_at = NOW()
WHERE id = $3;
```

### Step 7: RLS Policy Enforcement

**Component**: PostgreSQL Row Level Security
**Third-Party APIs**: PostgreSQL RLS Engine

```sql
-- Users can only update projects they own
CREATE POLICY "Users can update projects they own" ON projects
    FOR UPDATE
    TO authenticated
    USING (
        (select auth.uid()) = owner_id
    );
```

### Step 8: Frontend State Update

**Component**: `content-projects-item.svelte`
**Third-Party APIs**: None (Pure SvelteKit state management)

```typescript
// Update component state after successful edit
async function saveEdit() {
  if (editName.trim() && editName !== project.name) {
    try {
      const success = await updateProject(project.id, {
        name: editName.trim(),
      });

      if (success) {
        // Update the project name in the UI
        project.name = editName.trim();
        toast.success(`Project renamed to "${editName.trim()}"`);
      } else {
        throw new Error("Failed to rename project");
      }
    } catch (error) {
      toast.error(error.message);
      // Reset the edit name to the original
      editName = project.name;
    }
  }
  isEditing = false;
}
```

## Project Name Editing Safety Features

### 1. **Owner-Only Access**

- Only project owners can edit project names
- Permission validation before any update
- RLS policies enforce security at database level

### 2. **Input Validation**

- Name cannot be empty
- Maximum length of 100 characters
- Must start and end with alphanumeric characters
- Can contain letters, numbers, spaces, hyphens, and underscores
- Description limited to 500 characters

### 3. **Uniqueness Enforcement**

- Checks if name is already taken by another project
- Prevents duplicate project names
- Database-level uniqueness constraints

### 4. **Error Handling**

- Comprehensive error handling at each step
- User-friendly error messages
- Graceful fallback to original name on failure

## Following PostgreSQL Best Practices

Our implementation follows PostgreSQL update patterns:

```sql
-- Update with proper timestamp
UPDATE projects
SET
  name = $1,
  description = $2,
  updated_at = NOW()
WHERE id = $3;

-- RLS policy for owner-only updates
CREATE POLICY "Users can update projects they own" ON projects
    FOR UPDATE
    TO authenticated
    USING (
        (select auth.uid()) = owner_id
    );
```

## Advantages of This Approach

### 1. **Security**

- Owner-only access control
- RLS policies prevent unauthorized updates
- Permission validation at multiple levels

### 2. **Data Integrity**

- Input validation prevents invalid data
- Uniqueness checks prevent duplicates
- Database constraints ensure consistency

### 3. **User Experience**

- Real-time validation feedback
- Clear error messages
- Immediate UI updates on success

### 4. **Performance**

- Efficient database queries
- Minimal data transfer
- Optimized permission checks

## Configuration Requirements

### Environment Variables

```bash
# Supabase Configuration
VITE_SUPABASE_URL=http://91.99.166.223:8000
VITE_SUPABASE_ANON_KEY=your_anon_key_here
```

### Database Requirements

- RLS enabled on `projects` table
- Proper indexes for performance
- Update policies configured
- Unique constraints on project names

## Frontend Integration

### Component State Management

```typescript
// content-projects-item.svelte
let isEditing = $state(false);
let editName = $state(project.name);

const toggleEdit = () => {
  isEditing = !isEditing;
  if (isEditing) {
    editName = project.name;
  }
};

const saveEdit = async () => {
  if (editName.trim() && editName !== project.name) {
    try {
      await updateProject(project.id, { name: editName.trim() });
      project.name = editName.trim();
      toast.success(`Project renamed to "${editName.trim()}"`);
    } catch (error) {
      toast.error(error.message);
      editName = project.name;
    }
  }
  isEditing = false;
};
```

### Project Editing Service

```typescript
// src/lib/services/project-editing-service.ts
export async function updateProject(
  projectId: string,
  updateData: ProjectUpdateData,
): Promise<boolean>;
export async function hasProjectPermission(
  projectId: string,
  requiredLevel: PermissionLevel,
): Promise<boolean>;
export async function getProjectDetails(
  projectId: string,
): Promise<ProjectDetails | null>;
```

## Comparison with Bucket Renaming

| Feature           | Bucket Renaming (MinIO) | Project Name Editing (PostgreSQL) |
| ----------------- | ----------------------- | --------------------------------- |
| **Data Source**   | MinIO only              | PostgreSQL metadata               |
| **Security**      | No built-in security    | RLS policies                      |
| **Validation**    | Basic S3 rules          | Custom validation rules           |
| **Uniqueness**    | S3 bucket uniqueness    | Database uniqueness               |
| **Performance**   | Direct storage access   | Database query                    |
| **Audit Trail**   | No                      | Full audit trail                  |
| **Relationships** | None                    | Maintains project relationships   |

## Troubleshooting

### Common Issues

1. **"Permission denied"**
   - Check if user owns the project
   - Verify RLS policies are configured correctly
   - Ensure user is authenticated

2. **"Project name already exists"**
   - Check if another project has the same name
   - Verify uniqueness check is working
   - Check for case sensitivity issues

3. **"Invalid project name format"**
   - Check name validation rules
   - Ensure name starts and ends with alphanumeric
   - Verify no invalid characters

4. **"Update failed"**
   - Check database connection
   - Verify RLS policies
   - Check for constraint violations

### Debugging

```typescript
// Check user permissions
const hasPermission = await hasProjectPermission(projectId, "owner");
console.log("Has permission:", hasPermission);

// Check project details
const project = await getProjectDetails(projectId);
console.log("Project details:", project);

// Validate name format
const isValid = /^[a-zA-Z0-9][a-zA-Z0-9\s\-_]*[a-zA-Z0-9]$/.test(name);
console.log("Name is valid:", isValid);
```

## Third-Party APIs Summary

### Complete API Stack Used

| Step  | Component               | Third-Party APIs           | Purpose                              |
| ----- | ----------------------- | -------------------------- | ------------------------------------ |
| **1** | Frontend UI             | None (Pure SvelteKit)      | User interface and edit mode         |
| **2** | Project Editing Service | `@supabase/supabase-js`    | PostgreSQL client and authentication |
| **3** | Supabase Backend        | Supabase JavaScript Client | API routing and RLS enforcement      |
| **4** | PostgreSQL Database     | PostgreSQL with RLS        | Data storage and update operations   |
| **5** | Frontend Update         | None (Pure SvelteKit)      | UI state updates and rendering       |

### Key Third-Party Dependencies

1. **Supabase JavaScript Client** (`@supabase/supabase-js`)
   - **Purpose**: PostgreSQL client with authentication and RLS
   - **Used For**: Project updates, permission checking, data queries
   - **Why**: Official client, RLS integration, type safety

2. **PostgreSQL with RLS**
   - **Purpose**: Database backend with security policies
   - **Used For**: Data storage, permission management, audit trail
   - **Why**: ACID compliance, RLS security, data integrity

### Data Flow Architecture

```
Frontend (SvelteKit)
    ↓ Project Editing Service
Supabase JavaScript Client
    ↓ HTTP/WebSocket
Supabase Backend (API Gateway)
    ↓ SQL UPDATE with RLS
PostgreSQL Database (with RLS)
    ↓ Success Response
Project Editing Service
    ↓ Success Response
Frontend (UI Update)
```

## Conclusion

This implementation provides a secure, user-friendly way to edit project names in our hybrid architecture. By leveraging PostgreSQL's RLS policies and comprehensive validation, we ensure data integrity while maintaining a smooth user experience.

**Key Benefits:**

- **Security**: Owner-only access with RLS enforcement
- **Data Integrity**: Comprehensive validation and uniqueness checks
- **User Experience**: Real-time feedback and immediate updates
- **Performance**: Efficient database operations
- **Reliability**: Comprehensive error handling and fallback

**Next Steps:**

1. Test the complete project editing flow
2. Add description editing functionality
3. Implement bulk editing capabilities
4. Add audit logging for name changes
