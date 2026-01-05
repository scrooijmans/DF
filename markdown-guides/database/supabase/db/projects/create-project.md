# Project Creation with Supabase PostgreSQL and MinIO Storage

This guide explains the current end-to-end process for creating projects in MudRock, using Supabase PostgreSQL for metadata and MinIO for storage. The form uses **Superforms** and **Zod** for type-safe validation with real-time feedback.

## Current Implementation: JavaScript-Based Approach

**Architecture**: We use the JavaScript Supabase client directly from the frontend for database operations, bypassing the Rust backend for PostgreSQL operations. MinIO bucket creation still uses Tauri commands.

**Why**: This approach simplifies the implementation and avoids JWT signature issues with the Rust PostgREST client. The JavaScript Supabase client handles authentication and RLS policies automatically.

## Architecture Overview

### Form Validation Architecture

**Validation Stack**: We use a modern form validation approach:

- ✅ **Superforms** (`sveltekit-superforms`): Type-safe form handling with client-side validation
- ✅ **Zod** (`zod`): Schema validation library for runtime type checking
- ✅ **FormSnap** (`formsnap`): Accessible form components with automatic ARIA attributes
- ✅ **SPA Mode**: Client-side only form handling without server-side actions

### Database + Storage Architecture

**Hybrid Approach**: We use both Supabase PostgreSQL and MinIO storage:

- ✅ **Supabase PostgreSQL**: Project metadata, user permissions, collaboration data, pipelines
- ✅ **MinIO Storage**: Large data files, datasets, pipeline execution artifacts
- ✅ **PostgREST API**: Auto-generated REST API from PostgreSQL schema (accessed via JS client)
- ✅ **Row Level Security**: Built-in access control and data isolation

### Project Creation Flow

```
User fills form → Zod/Superforms validation → Uniqueness check → Create project in DB → Create bucket (optional) → Realtime update → Navigate to projects list
```

## Step-by-Step Implementation Process

### Step 1: Frontend Form Submission with Zod/Superforms Validation

**Component**: `src/routes/projects/new/+page.svelte`

The user fills out a GitHub-style project creation form with real-time validation using **Superforms** and **Zod**. The form includes:
- Project name (required, validated with Zod schema)
- Description (optional, max 350 characters)
- Collaborators (optional, via team members)

#### Form Schema Definition

**File**: `src/lib/schemas/project-schema.ts`

```typescript
import { z } from "zod";

export const projectSchema = z.object({
  name: z
    .string()
    .min(1, "Project name is required")
    .max(100, "Project name must be no more than 100 characters")
    .regex(
      /^[a-zA-Z0-9-_]+$/,
      "Project name can only contain letters, numbers, hyphens, and underscores",
    ),
  description: z
    .string()
    .max(350, "Description must be no more than 350 characters")
    .optional()
    .nullable(),
  permissions: z.array(z.string().uuid()).default([]),
});

// Async validation function to check project name uniqueness
export async function checkProjectNameUnique(
  name: string,
  projects: Array<{ name: string }>,
): Promise<boolean> {
  const normalizedName = name.trim().toLowerCase();
  return !projects.some(
    (p) => p.name.trim().toLowerCase() === normalizedName,
  );
}
```

#### Form Initialization with Superforms

```typescript
import { superForm, defaults } from "sveltekit-superforms";
import { zodClient, zod } from "sveltekit-superforms/adapters";
import { projectSchema, checkProjectNameUnique } from "$lib/schemas/project-schema";
import * as Form from "$lib/components/ui/form/index.js";

const projectsState = getPostgresProjectsState();

// Initialize form with SPA mode for client-side handling
const form = superForm(
  defaults(
    {
      name: "",
      description: null,
      permissions: [],
    },
    zod(projectSchema),
  ),
  {
    SPA: true, // Client-side only form handling
    validators: zodClient(projectSchema), // Client-side Zod validation
    onUpdate: async ({ form }) => {
      // Additional validation: check project name uniqueness
      const name = form.data.name as string;
      if (name && name.trim()) {
        // Validate whitespace
        if (name.trim().length === 0) {
          form.errors.name = ["Project name cannot be only whitespace"];
          form.valid = false;
          return;
        }
        
        // Check uniqueness against existing projects
        const isUnique = await checkProjectNameUnique(
          name,
          projectsState.projects,
        );
        if (!isUnique) {
          form.errors.name = ["A project with this name already exists"];
          form.valid = false;
        }
      }
    },
  },
);

const { form: formData, enhance, errors, validateForm } = form;
```

#### Form Component Structure with FormSnap

```svelte
<form use:enhance onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  <!-- Project Name Field -->
  <Form.Field {form} name="name">
    <Form.Control>
      {#snippet children({ props })}
        <Form.Label>Project name <span class="text-red-500">*</span></Form.Label>
        <input
          {...props}
          bind:value={$formData.name}
          placeholder="project-name"
          class="flex-1 {$errors.name ? 'border-red-500' : ''}"
          oninput={async () => {
            await validateForm(); // Real-time validation
          }}
        />
      {/snippet}
    </Form.Control>
    <Form.Description>
      Project names must be 1-100 characters, letters, numbers, hyphens, and underscores only.
    </Form.Description>
    <Form.FieldErrors />
  </Form.Field>

  <!-- Description Field -->
  <Form.Field {form} name="description">
    <Form.Control>
      {#snippet children({ props })}
        <Form.Label>Description</Form.Label>
        <textarea
          {...props}
          bind:value={$formData.description}
          maxlength={350}
        />
      {/snippet}
    </Form.Control>
    <Form.Description>
      Optional description for your project (max 350 characters).
    </Form.Description>
    <Form.FieldErrors />
  </Form.Field>
</form>
```

#### Form Submission Handler

```typescript
async function handleSubmit() {
  if (!currentUser) {
    toast.error("You must be logged in to create a project");
    return;
  }

  try {
    isCreating = true;

    // Validate form first
    const validationResult = await validateForm();
    
    const name = $formData.name as string;
    const description = $formData.description as string | null | undefined;
    const permissions = $formData.permissions as string[];
    
    // Check uniqueness (double-check before submission)
    const isUnique = await checkProjectNameUnique(
      name,
      projectsState.projects,
    );
    
    if (!isUnique) {
      toast.error("A project with this name already exists");
      return;
    }

    if (!validationResult.valid) {
      toast.error("Please fix the validation errors before submitting");
      return;
    }

    // Create project with storage
    const createdProject = await createProjectWithStorageJS({
      name: name.trim(),
      description: description?.trim() || undefined,
      permissions: permissions,
      settings: {
        storage_quota: 1000000000,
        auto_save: true,
        default_workspace_type: "analysis",
      },
    });

    // Optional: Create bucket
    try {
      await createProjectBucket(createdProject.bucket_name);
    } catch (bucketError) {
      console.warn("Bucket creation failed (non-critical):", bucketError);
    }

    toast.success(`Project "${createdProject.project.name}" created successfully!`);
    goto("/projects");
  } catch (error) {
    toast.error(error instanceof Error ? error.message : "Failed to create project");
  } finally {
    isCreating = false;
  }
}
```

### Step 1.5: Form Validation Features

**Key Validation Features:**

1. **Real-time Validation**: Zod schema validates as user types (`oninput` event triggers `validateForm()`)
2. **Character Restrictions**: Project name must match `/^[a-zA-Z0-9-_]+$/` regex pattern
3. **Length Validation**: Name 1-100 chars, description max 350 chars
4. **Uniqueness Check**: Async validation checks against existing projects in `onUpdate` handler
5. **Whitespace Validation**: Prevents names that are only whitespace
6. **Visual Feedback**: FormSnap components provide automatic error display and ARIA attributes

**Validation Rules:**

- ✅ Project name: Required, 1-100 characters, alphanumeric + hyphens/underscores only
- ✅ Description: Optional, max 350 characters
- ✅ Permissions: Array of valid UUIDs (for collaborators)
- ✅ Uniqueness: Case-insensitive check against existing project names

### Step 2: Project Creation Service

**Component**: `src/lib/services/project-creation-service.ts`
**Third-Party APIs**: Supabase JavaScript Client (`@supabase/supabase-js`)

```typescript
export async function createProjectWithStorageJS(
  projectData: ProjectFormData,
): Promise<ProjectWithStorage> {
  // Get current authenticated user
  const { data: { user } } = await supabase.auth.getUser();
  if (!user) {
    throw new Error("User not authenticated");
  }

  // Step 2a: Create project in PostgreSQL
  const { data: project, error: projectError } = await supabase
    .from("projects")
    .insert({
      name: projectData.name,
      description: projectData.description || null,
      owner_id: user.id, // Set owner from authenticated user
      settings: projectData.settings || {
        storage_quota: 1000000000,
        auto_save: true,
        default_workspace_type: "analysis",
      },
      // Note: No 'permissions' column - permissions are in project_permissions table
    })
    .select()
    .single();

  if (projectError) {
    throw new Error(`Failed to create project in database: ${projectError.message}`);
  }

  // Step 2b: Create project permissions for collaborators
  // Note: Owner doesn't need a project_permissions entry because
  // ownership is determined by the owner_id column in the projects table.
  // The project_permissions table is only for additional collaborators.
  
  if (projectData.permissions && projectData.permissions.length > 0) {
    const collaboratorPermissions = projectData.permissions.map(
      (collaboratorUserId) => ({
        project_id: project.id,
        user_id: collaboratorUserId,
        role: "viewer", // Default role for collaborators added during creation
        granted_by: user.id,
      }),
    );

    const { error: collaboratorPermissionError } = await supabase
      .from("project_permissions")
      .insert(collaboratorPermissions);

    if (collaboratorPermissionError) {
      console.error("Failed to create collaborator permissions:", collaboratorPermissionError);
      // Don't fail the whole operation, but log the error
    }
  }

  return { project, bucket_name: `project-${project.id}` };
}
```

### Step 3: MinIO Bucket Creation (Optional)

**Component**: `src/lib/services/project-creation-service.ts`
**Third-Party APIs**: Tauri IPC for MinIO operations

```typescript
export async function createProjectBucket(bucketName: string): Promise<void> {
  const { invoke } = await import("@tauri-apps/api/core");
  
  // Call the existing Rust command for bucket creation
  await invoke("create_minio_bucket", { bucketName });
}
```

**Note**: This step is optional and can be skipped if bucket creation fails. The project will still be created in the database. Bucket creation is typically called separately after project creation:

```typescript
try {
  await createProjectBucket(createdProject.bucket_name);
} catch (bucketError) {
  console.warn("Bucket creation failed, but project was created:", bucketError);
  toast.warning("Project created, but failed to create storage bucket.");
}
```

### Step 4: Frontend State Update

**Component**: `src/routes/projects/new/+page.svelte`

After successful project creation:

1. **Show success message**: `toast.success("Project created successfully!")`
2. **Navigate to projects list**: `goto('/projects')`
3. **Realtime updates**: The project automatically appears in the projects list via Supabase Realtime subscriptions (handled by `RealtimeProjectsService`)

### Step 5: Automatic Project List Update

**Component**: `src/lib/components/pages/projects/content-projects/content-projects.svelte`

The projects list automatically updates via:

1. **Realtime Service**: `RealtimeProjectsService` subscribes to `projects` table changes
2. **State Management**: `PostgresProjectsState` receives realtime updates and updates the `projects` array
3. **UI Reactivity**: Svelte's reactivity automatically re-renders the list when `projectsState.projects` changes

No manual refresh needed - the new project appears automatically!

## Data Structures

### Project Form Data

```typescript
export interface ProjectFormData {
  name: string;
  description?: string;
  settings?: {
    storage_quota?: number;
    auto_save?: boolean;
    default_workspace_type?: string;
  };
  permissions?: string[]; // Array of user IDs from auth.users (collaborators)
}
```

### Project with Storage Info

```typescript
export interface ProjectWithStorage {
  project: Project;
  bucket_name: string;
}

export interface Project {
  id: string;
  name: string;
  description: string | null;
  owner_id: string; // UUID of project owner
  created_at: string;
  updated_at: string;
  settings: Record<string, any> | null;
}
```

## Database Schema

### Projects Table

```sql
CREATE TABLE public.projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    owner_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    settings JSONB DEFAULT '{}'::jsonb
);
```

**Key Points**:
- No `permissions` column - permissions are stored in `project_permissions` table
- `owner_id` determines project ownership
- Owner has implicit full access via `owner_id`

### Project Permissions Table

```sql
CREATE TABLE public.project_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('owner', 'editor', 'viewer')),
    granted_by UUID NOT NULL REFERENCES auth.users(id) ON DELETE SET NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT project_permissions_unique_user_project UNIQUE (project_id, user_id),
    CONSTRAINT project_permissions_no_self_grant CHECK (user_id <> granted_by)
);
```

**Key Points**:
- Only stores permissions for **collaborators**, not the owner
- Owner permissions are implicit via `projects.owner_id`
- Constraint prevents self-granting (`user_id <> granted_by`)
- Roles: `owner`, `editor`, `viewer`


## Permission Model

### Owner Permissions

- **Determined by**: `projects.owner_id` column
- **Access Level**: Full access (owner role)
- **No Entry Needed**: Owner does NOT have an entry in `project_permissions` table
- **Why**: Ownership is implicit and tracked via `owner_id`

### Collaborator Permissions

- **Stored in**: `project_permissions` table
- **Roles Available**: `owner`, `editor`, `viewer`
- **Default Role**: `viewer` (for collaborators added during project creation)
- **Granted By**: The project owner (or another user with permission management rights)

### Permission Checking

When checking if a user has access to a project:

```sql
-- Check if user is owner
SELECT * FROM projects WHERE id = $1 AND owner_id = auth.uid();

-- Check if user has collaborator permission
SELECT * FROM project_permissions 
WHERE project_id = $1 AND user_id = auth.uid();
```

## Storage Structure Created

When a MinIO bucket is created (optional step):

```
project-{project-id}/
├── shared/                    # Project-wide shared data
├── data/                      # Project datasets
├── pipelines/                 # Pipeline execution artifacts and outputs
└── exports/                  # Project-level exports
```

**Note**: Bucket creation is optional - projects can exist without buckets.

## Advantages of Current Approach

### 1. **Simplified Architecture**

- Direct JavaScript Supabase client usage
- No Rust backend complexity for database operations
- Automatic authentication and RLS handling

### 2. **Real-time Updates**

- Supabase Realtime subscriptions automatically update UI
- No manual refresh needed
- Instant project list updates

### 3. **Database-Driven Architecture**

- Rich metadata storage in PostgreSQL
- Relational data integrity
- Complex queries and relationships
- Built-in Row Level Security

### 4. **Hybrid Storage Strategy**

- PostgreSQL for metadata and collaboration
- MinIO for large data files
- Best of both worlds
- Scalable and cost-effective

### 5. **Collaboration Ready**

- User permissions and roles
- Project sharing capabilities
- Real-time updates via Supabase
- Multi-user workspace support

## Configuration Requirements

### Environment Variables

```bash
# Supabase Configuration (set in Supabase client initialization)
SUPABASE_URL=http://91.99.166.223:8000
SUPABASE_ANON_KEY=your-anon-key

# MinIO Configuration (for Tauri commands)
AWS_ACCESS_KEY_ID=mudrock-storage
AWS_SECRET_ACCESS_KEY=mudrock-storage-secret-2024
MINIO_ENDPOINT=http://91.99.166.223:9000
MINIO_REGION=us-east-1
```

### Database Requirements

- Supabase PostgreSQL with our schema
- Row Level Security enabled
- PostgREST API enabled
- Proper CORS configuration
- Realtime enabled for `projects` table

## Frontend Integration

### Component State Management with Superforms

The form uses Superforms for type-safe state management:

```typescript
// src/routes/projects/new/+page.svelte
import { superForm, defaults } from "sveltekit-superforms";
import { zodClient, zod } from "sveltekit-superforms/adapters";
import { projectSchema, checkProjectNameUnique } from "$lib/schemas/project-schema";
import * as Form from "$lib/components/ui/form/index.js";

// Initialize form with SPA mode
const form = superForm(
  defaults(
    {
      name: "",
      description: null,
      permissions: [],
    },
    zod(projectSchema),
  ),
  {
    SPA: true,
    validators: zodClient(projectSchema),
    onUpdate: async ({ form }) => {
      // Real-time uniqueness check
      const name = form.data.name as string;
      if (name && name.trim()) {
        const isUnique = await checkProjectNameUnique(
          name,
          projectsState.projects,
        );
        if (!isUnique) {
          form.errors.name = ["A project with this name already exists"];
          form.valid = false;
        }
      }
    },
  },
);

const { form: formData, enhance, errors, validateForm } = form;
let isCreating = $state(false);

async function handleSubmit() {
  isCreating = true;
  try {
    // Validate form first
    const validationResult = await validateForm();
    
    const name = $formData.name as string;
    const description = $formData.description as string | null | undefined;
    const permissions = $formData.permissions as string[];
    
    // Double-check uniqueness
    const isUnique = await checkProjectNameUnique(name, projectsState.projects);
    if (!isUnique) {
      toast.error("A project with this name already exists");
      return;
    }

    if (!validationResult.valid) {
      toast.error("Please fix the validation errors before submitting");
      return;
    }
    
    const createdProject = await createProjectWithStorageJS({
      name: name.trim(),
      description: description?.trim() || undefined,
      permissions: permissions,
      settings: {
        storage_quota: 1000000000,
        auto_save: true,
        default_workspace_type: "analysis",
      },
    });
    
    // Optional: Create bucket
    try {
      await createProjectBucket(createdProject.bucket_name);
    } catch (bucketError) {
      console.warn("Bucket creation failed (non-critical):", bucketError);
    }
    
    toast.success(`Project "${createdProject.project.name}" created!`);
    goto('/projects');
  } catch (error) {
    toast.error(error instanceof Error ? error.message : "Failed to create project");
  } finally {
    isCreating = false;
  }
}
```

**Key Features:**
- **Type Safety**: Form data is typed via Zod schema inference
- **Real-time Validation**: `validateForm()` called on input changes
- **Error Display**: FormSnap components automatically show validation errors
- **Uniqueness Check**: Async validation prevents duplicate project names
- **Accessibility**: FormSnap provides ARIA attributes automatically

### Realtime Updates

Projects automatically appear in the list via `RealtimeProjectsService`:

```typescript
// src/lib/services/realtime-projects-service.ts
// Automatically subscribes to projects table changes
// Updates PostgresProjectsState.projects array reactively
// UI automatically re-renders when projects array changes
```

## Troubleshooting

### Common Issues

1. **"Could not find the 'permissions' column"**
   - ✅ **Fixed**: Removed `permissions` field from project insert
   - The `projects` table doesn't have a `permissions` column
   - Permissions are stored in `project_permissions` table

2. **"Project name already exists"**
   - Check database for existing project names
   - Use a different project name
   - Validation happens client-side before submission

4. **"User not authenticated"**
   - Verify user is logged in
   - Check Supabase auth session
   - Ensure `supabase.auth.getUser()` returns a user

5. **"Bucket creation failed"**
   - Verify MinIO credentials
   - Check MinIO service status
   - This is optional - project still created successfully

6. **"RLS policy violation"**
   - Check user permissions
   - Verify authentication
   - Ensure RLS policies allow INSERT for authenticated users

### Debugging

```typescript
// Enable debug logging in project-creation-service.ts
console.log("🔧 Creating project:", projectData.name);
console.log("✅ Project created:", project.id);
console.log("📦 Bucket name:", bucketName);

// Form validation debugging (in +page.svelte)
console.log("🔍 Form validation result:", validationResult);
console.log("🔍 Form errors:", $errors);
console.log("🔍 Form data:", $formData);
console.log("🔍 Uniqueness check:", isUnique);
```

## Implementation Status

### ✅ Completed

1. **Frontend Implementation**
   - ✅ Project creation form (`src/routes/projects/new/+page.svelte`)
   - ✅ Project creation service (`src/lib/services/project-creation-service.ts`)
   - ✅ Form validation and error handling
   - ✅ Success/error toast notifications

2. **Database Operations**
   - ✅ Project creation in PostgreSQL
   - ✅ Collaborator permissions creation
   - ✅ Owner permissions (via `owner_id`)

3. **Form Validation**
   - ✅ Zod schema validation with Superforms
   - ✅ Real-time validation feedback
   - ✅ Project name uniqueness checking
   - ✅ Character restrictions and length validation
   - ✅ FormSnap components for accessibility

4. **Realtime Updates**
   - ✅ `RealtimeProjectsService` for automatic UI updates
   - ✅ Project list auto-refresh on creation
   - ✅ No manual refresh needed

5. **Storage Integration**
   - ✅ Optional MinIO bucket creation
   - ✅ Tauri command integration
   - ✅ Error handling for bucket creation failures

### 📋 Future Enhancements

1. **Team Integration**
   - 📋 Add team members as collaborators during creation
   - 📋 Bulk permission assignment from teams
   - 📋 Team-based project discovery

2. **Advanced Permissions**
   - 📋 Custom permission roles
   - 📋 Permission inheritance
   - 📋 Permission templates

3. **Project Templates**
   - 📋 Pre-configured project settings
   - 📋 Workspace templates
   - 📋 Default collaborator lists

## Data Flow Architecture

```
User fills form (new-project-page.svelte)
    ↓
Zod schema validation (real-time via Superforms)
    ↓
Uniqueness check (async validation in onUpdate)
    ↓
Form submission (handleSubmit)
    ↓
createProjectWithStorageJS() (project-creation-service.ts)
    ↓
Supabase JS Client
    ├─→ INSERT INTO projects (owner_id = user.id)
    └─→ INSERT INTO project_permissions (for collaborators)
    ↓
createProjectBucket() (optional, via Tauri)
    ↓
MinIO Bucket Creation
    ↓
Success Response
    ↓
Navigate to /projects
    ↓
RealtimeProjectsService detects INSERT
    ↓
PostgresProjectsState.projects updated
    ↓
UI automatically re-renders with new project
```

## Third-Party APIs Summary

### Complete API Stack Used

| Step  | Component                    | Third-Party APIs                    | Purpose                              |
| ----- | ---------------------------- | ----------------------------------- | ------------------------------------ |
| **1** | Frontend UI                  | None (Pure SvelteKit)               | User interface and form handling     |
| **1.5** | Form Validation            | Superforms + Zod                    | Real-time validation and error handling |
| **2** | Project Creation Service     | Supabase JS Client                  | Database operations via REST         |
| **3** | Supabase PostgreSQL          | PostgREST API (via JS client)       | Project metadata storage             |
| **4** | Realtime Service             | Supabase Realtime                   | Automatic UI updates                 |
| **5** | MinIO Bucket Creation        | Tauri IPC → Rust → AWS SDK S3       | Object storage bucket creation       |
| **6** | Frontend State Update        | None (Pure SvelteKit reactivity)    | UI state updates and rendering       |

### Key Third-Party Dependencies

1. **SvelteKit Superforms** (`sveltekit-superforms`)
   - **Purpose**: Type-safe form handling with client and server validation
   - **Used For**: Form state management, validation, error handling
   - **Why**: Winner of Svelte Hack 2023, provides progressive enhancement and type safety

2. **Zod** (`zod`)
   - **Purpose**: TypeScript-first schema validation library
   - **Used For**: Runtime type checking and validation rules
   - **Why**: Generates TypeScript types from schemas, excellent error messages

3. **FormSnap** (`formsnap`)
   - **Purpose**: Wrapper around Superforms for accessible forms
   - **Used For**: Form components with automatic ARIA attributes
   - **Why**: Makes forms accessible by default, provides Shadcn-Svelte components

4. **Supabase JavaScript Client** (`@supabase/supabase-js`)
   - **Purpose**: PostgreSQL client for Supabase
   - **Used For**: Database operations, authentication, RLS
   - **Why**: Automatic auth handling, RLS support, realtime subscriptions

5. **Supabase PostgreSQL**
   - **Purpose**: Database backend
   - **Used For**: Project metadata, user permissions, collaboration
   - **Why**: Managed PostgreSQL, built-in RLS, real-time features

6. **Supabase Realtime**
   - **Purpose**: Real-time database subscriptions
   - **Used For**: Automatic project list updates
   - **Why**: No manual refresh needed, instant UI updates

7. **AWS SDK S3** (via Tauri)
   - **Purpose**: S3 protocol implementation for MinIO
   - **Used For**: Bucket creation, object operations
   - **Why**: Industry standard, full S3 compatibility

8. **MinIO S3-Compatible API**
   - **Purpose**: Object storage backend
   - **Used For**: Large data files, datasets, artifacts
   - **Why**: S3-compatible, self-hosted, cost-effective

## Conclusion

The current implementation provides a robust, scalable, and collaborative way to create projects using a hybrid database + storage architecture. By using the JavaScript Supabase client directly, we simplify the implementation while maintaining all the benefits of Supabase's managed PostgreSQL and real-time features.

**Key Benefits:**

- **Simplicity**: Direct JS client usage, no Rust backend complexity
- **Real-time**: Automatic UI updates via Supabase Realtime
- **Collaboration**: Full user permissions and project sharing
- **Scalability**: Database for metadata, storage for large files
- **Reliability**: Managed PostgreSQL with built-in RLS
- **Performance**: Direct MinIO access for large data
- **Flexibility**: Rich metadata and complex relationships
- **Security**: Row Level Security and proper access control

**Current Status:**

✅ Fully functional project creation flow
✅ Real-time project list updates
✅ Collaborator permissions support
✅ Optional MinIO bucket creation
✅ Complete error handling and validation
✅ Zod/Superforms form validation with real-time feedback
✅ Project name uniqueness checking
✅ Character restrictions and length validation
✅ FormSnap components for accessibility
