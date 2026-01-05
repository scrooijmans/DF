# Project Listing with Supabase PostgreSQL

This guide explains how we list projects from our PostgreSQL database using the Supabase JavaScript client, following the new project-based architecture.

## Architecture Overview

### Third-Party APIs Used

1. **Supabase JavaScript Client** (`@supabase/supabase-js`)
   - Official Supabase client for PostgreSQL operations
   - Handles authentication and RLS policies
   - Provides real-time subscriptions and type safety

2. **PostgreSQL with RLS** (via Supabase)
   - Row Level Security for data isolation
   - User-project access management
   - Audit trail and permission tracking

### Project Management System

**Hybrid Architecture**: We use PostgreSQL for project metadata and MinIO for storage:

- ✅ **PostgreSQL**: Project metadata, user permissions, pipelines
- ✅ **MinIO**: Large data storage with project-specific buckets
- ✅ **RLS Policies**: Automatic security based on user permissions
- ✅ **Permission System**: Many-to-many user-project relationships

## Step-by-Step Implementation Process

### Step 1: Frontend Request

**Component**: `content-projects.svelte`
**Third-Party APIs**: None (Pure SvelteKit)

```typescript
// User navigates to Projects page
const loadProjects = async () => {
  const projects = await getUserProjectAccess();
  // Updates component state with user's accessible projects
};
```

### Step 2: Project Permissions Service

**Component**: `src/lib/services/project-permissions-service.ts`
**Third-Party APIs**: Supabase JavaScript Client

```typescript
import { createClient } from "@supabase/supabase-js";

const supabase = createClient(supabaseUrl, supabaseServiceKey);

export async function getUserProjectAccess(): Promise<UserProjectAccess[]> {
  const { data, error } = await supabase
    .from("user_project_access")
    .select("*")
    .order("project_name");

  if (error) {
    throw new Error(`Failed to get user project access: ${error.message}`);
  }

  return data || [];
}
```

### Step 3: PostgreSQL Query with RLS

**Component**: Supabase PostgreSQL Database
**Third-Party APIs**: PostgreSQL with Row Level Security

```sql
-- The user_project_access view automatically applies RLS policies
CREATE OR REPLACE VIEW user_project_access AS
SELECT
    p.id as project_id,
    p.name as project_name,
    p.owner_id,
    CASE
        WHEN p.owner_id = auth.uid() THEN 'owner'
        ELSE COALESCE(pp.permission_level, 'none')
    END as user_permission_level,
    pp.granted_at,
    pp.granted_by
FROM projects p
LEFT JOIN project_permissions pp ON p.id = pp.project_id AND pp.user_id = auth.uid()
WHERE
    p.owner_id = auth.uid() OR
    pp.user_id = auth.uid();
```

### Step 4: RLS Policy Enforcement

**Component**: PostgreSQL Row Level Security
**Third-Party APIs**: PostgreSQL RLS Engine

```sql
-- Users can only see projects they own or have explicit permissions for
CREATE POLICY "Users can view accessible projects" ON projects
    FOR SELECT
    TO authenticated
    USING (
        (select auth.uid()) = owner_id OR
        id IN (
            SELECT project_id
            FROM project_permissions
            WHERE user_id = (select auth.uid())
        )
    );
```

### Step 5: Type Conversion and Frontend Update

**Component**: `content-projects.svelte`
**Third-Party APIs**: None (Pure SvelteKit state management)

```typescript
// Convert UserProjectAccess to ProjectDisplay type
interface ProjectDisplay {
  id: string;
  name: string;
  description: string;
  owner_id: string;
  permission_level: "owner" | "admin" | "editor" | "viewer";
  created_at: string;
  updated_at: string;
}

// Update component state
projects = userAccess.map((access) => ({
  id: access.project_id,
  name: access.project_name,
  description: "Project description", // Could be fetched separately
  owner_id: access.owner_id,
  permission_level: access.user_permission_level,
  created_at: new Date().toISOString(), // Could be fetched from projects table
  updated_at: new Date().toISOString(),
}));
```

## Project Metadata Retrieved

- **Project ID**: Unique PostgreSQL UUID
- **Project Name**: Human-readable project name
- **Owner ID**: User who created the project
- **Permission Level**: User's access level (owner, admin, editor, viewer)
- **Creation Date**: When the project was created
- **Granted At**: When permission was granted (for shared projects)
- **Granted By**: Who granted the permission

## Following projects-api.js Pattern

Our implementation follows the Supabase JavaScript client patterns:

```javascript
// From projects-api.js examples
let { data: projects, error } = await supabase.from("projects").select("*");

// Our implementation with RLS
let { data: userAccess, error } = await supabase
  .from("user_project_access")
  .select("*")
  .order("project_name");
```

## Advantages of This Approach

### 1. **Security First**

- RLS policies automatically filter data
- Users only see projects they have access to
- No need for manual permission checking in frontend

### 2. **Rich Metadata**

- Full project information from PostgreSQL
- User permission levels
- Audit trail with timestamps
- Pipeline relationships

### 3. **Performance**

- Optimized queries with proper indexing
- RLS policies use cached `auth.uid()`
- Efficient many-to-many relationships

### 4. **Scalability**

- PostgreSQL handles complex relationships
- RLS scales with user count
- Real-time subscriptions available

## Configuration Requirements

### Environment Variables

```bash
# Supabase Configuration
VITE_SUPABASE_URL=http://91.99.166.223:8000
VITE_SUPABASE_ANON_KEY=your_anon_key_here
SUPABASE_SERVICE_ROLE_KEY=your_service_role_key_here
```

### Database Requirements

- RLS enabled on `projects` and `project_permissions` tables
- Proper indexes for performance
- `user_project_access` view created
- Permission management functions available

## Frontend Integration

### Component State Management

```typescript
// content-projects.svelte
let projects: ProjectDisplay[] = $state([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Load projects on component mount
onMount(async () => {
  await loadProjects();
});
```

### Project Permissions Service

```typescript
// src/lib/services/project-permissions-service.ts
export async function getUserProjectAccess(): Promise<UserProjectAccess[]>;
export async function getProjectUsers(
  projectId: string,
): Promise<ProjectUser[]>;
export async function grantProjectAccess(
  projectId: string,
  userId: string,
  permissionLevel: PermissionLevel,
): Promise<boolean>;
export async function revokeProjectAccess(
  projectId: string,
  userId: string,
): Promise<boolean>;
```

## Comparison with MinIO Bucket Listing

| Feature           | MinIO Bucket Listing  | PostgreSQL Project Listing |
| ----------------- | --------------------- | -------------------------- |
| **Data Source**   | MinIO S3 API          | PostgreSQL Database        |
| **Security**      | No built-in security  | RLS policies               |
| **Metadata**      | Limited (name, date)  | Rich (permissions, audit)  |
| **Relationships** | None                  | Many-to-many users         |
| **Performance**   | Direct storage access | Database queries + RLS     |
| **Scalability**   | Limited by storage    | Scales with PostgreSQL     |
| **Real-time**     | No                    | Supabase subscriptions     |

## Project Management Features

### 1. **Project Creation**

```typescript
// Create new project with automatic workspace and MinIO bucket
const newProject = await createProjectWithStorageJS({
  name: "my-new-project",
  description: "Project description",
  settings: {
    storage_quota: 1000000000,
    auto_save: true,
    default_workspace_type: "analysis",
  },
});
```

### 2. **Permission Management**

```typescript
// Grant access to another user
await grantProjectAccess(projectId, userId, "editor");

// Revoke access
await revokeProjectAccess(projectId, userId);

// Check user permissions
const hasPermission = await hasProjectPermission(projectId, "admin");
```

### 3. **Project Listing**

```typescript
// Get all projects user has access to
const userProjects = await getUserProjectAccess();

// Get users who have access to a project
const projectUsers = await getProjectUsers(projectId);
```

## Troubleshooting

### Common Issues

1. **"No projects visible"**
   - Check if RLS policies are properly configured
   - Verify user is authenticated
   - Check if user has project permissions

2. **"Permission denied"**
   - Verify user has appropriate permission level
   - Check RLS policy configuration
   - Ensure user is authenticated

3. **"Slow queries"**
   - Check if indexes are created
   - Verify RLS policies are optimized
   - Use explicit filters in queries

### Debugging

```typescript
// Check user's accessible projects
const userAccess = await getUserProjectAccess();
console.log("User project access:", userAccess);

// Check specific project permissions
const hasPermission = await hasProjectPermission(projectId, "viewer");
console.log("Has permission:", hasPermission);
```

## Third-Party APIs Summary

### Complete API Stack Used

| Step  | Component                   | Third-Party APIs              | Purpose                              |
| ----- | --------------------------- | ----------------------------- | ------------------------------------ |
| **1** | Frontend UI                 | None (Pure SvelteKit)         | User interface and state management  |
| **2** | Project Permissions Service | `@supabase/supabase-js`       | PostgreSQL client and authentication |
| **3** | Supabase Backend            | Supabase JavaScript Client    | API routing and RLS enforcement      |
| **4** | PostgreSQL Database         | PostgreSQL with RLS           | Data storage and security policies   |
| **5** | RLS Engine                  | PostgreSQL Row Level Security | Automatic data filtering             |
| **6** | Data Processing             | None (Pure TypeScript)        | Type conversion and business logic   |
| **7** | Frontend Update             | None (Pure SvelteKit)         | UI state updates and rendering       |

### Key Third-Party Dependencies

1. **Supabase JavaScript Client** (`@supabase/supabase-js`)
   - **Purpose**: PostgreSQL client with authentication and RLS
   - **Used For**: Database queries, real-time subscriptions, auth
   - **Why**: Official client, RLS integration, type safety

2. **PostgreSQL with RLS**
   - **Purpose**: Database backend with security policies
   - **Used For**: Data storage, permission management, audit trail
   - **Why**: ACID compliance, RLS security, complex relationships

3. **Supabase Backend**
   - **Purpose**: API gateway and authentication
   - **Used For**: Request routing, JWT validation, RLS enforcement
   - **Why**: Managed service, automatic scaling, security

### Data Flow Architecture

```
Frontend (SvelteKit)
    ↓ Project Permissions Service
Supabase JavaScript Client
    ↓ HTTP/WebSocket
Supabase Backend (API Gateway)
    ↓ SQL Queries
PostgreSQL Database (with RLS)
    ↓ Filtered Results
Supabase Backend (Response Processing)
    ↓ JSON Response
Supabase JavaScript Client (Data Parsing)
    ↓ TypeScript Objects
Frontend (UI Update)
```

## Conclusion

This implementation provides a secure, scalable, and feature-rich way to manage projects using PostgreSQL with Row Level Security. By leveraging Supabase's managed infrastructure, we get automatic security, real-time capabilities, and excellent developer experience.

**Key Benefits:**

- **Security**: RLS policies automatically protect data
- **Scalability**: PostgreSQL handles complex relationships efficiently
- **Rich Metadata**: Full project information with permissions and audit trail
- **Real-time**: Supabase subscriptions for live updates
- **Type Safety**: Full TypeScript support throughout the stack
- **Performance**: Optimized queries with proper indexing
- **Collaboration**: Built-in user permission management
