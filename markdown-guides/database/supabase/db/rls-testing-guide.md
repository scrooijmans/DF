# RLS Testing Guide

## Overview

This guide explains how to test the Row Level Security (RLS) setup and user-project access management system.

## Prerequisites

1. Execute the SQL script from `rls-setup-guide.md` in Supabase Studio
2. Have at least 2-3 users in your `auth.users` table
3. Have some test projects created

## Testing Steps

### 1. Verify RLS is Enabled

Run this query in Supabase Studio SQL Editor:

```sql
-- Check RLS status
SELECT schemaname, tablename, rowsecurity
FROM pg_tables
WHERE tablename IN ('projects', 'project_permissions');
```

**Expected Result**: Both tables should show `rowsecurity: true`

### 2. Check Policies are Created

```sql
-- Check policies
SELECT schemaname, tablename, policyname, permissive, roles, cmd, qual
FROM pg_policies
WHERE tablename IN ('projects', 'project_permissions')
ORDER BY tablename, policyname;
```

**Expected Result**: Should see policies for both tables with `TO authenticated` roles

### 3. Test User Access (Run as Different Users)

#### Test 1: Check User Project Access

```sql
-- This should only show projects the current user owns or has permissions for
SELECT * FROM user_project_access;
```

**Expected Result**: Only projects accessible to the current user

#### Test 2: Test Project Creation

```sql
-- Create a test project (should work for authenticated users)
INSERT INTO projects (name, description, owner_id, settings)
VALUES ('test-rls-project', 'Testing RLS', auth.uid(), '{"auto_save": true}');
```

**Expected Result**: Project created successfully

#### Test 3: Test Permission Granting

```sql
-- Grant access to another user (replace with actual user ID)
SELECT grant_project_access(
    'your-project-id-here',
    'other-user-id-here',
    'viewer'
);
```

**Expected Result**: Returns `true`

#### Test 4: Test Permission Revocation

```sql
-- Revoke access
SELECT revoke_project_access(
    'your-project-id-here',
    'other-user-id-here'
);
```

**Expected Result**: Returns `true`

### 4. Test Security Boundaries

#### Test 1: Unauthenticated Access

```sql
-- This should return empty (no data visible to unauthenticated users)
SELECT * FROM projects;
```

**Expected Result**: Empty result set

#### Test 2: Cross-User Access Prevention

```sql
-- Try to access another user's project directly
SELECT * FROM projects WHERE owner_id != auth.uid();
```

**Expected Result**: Only projects you have explicit permissions for

#### Test 3: Permission Escalation Prevention

```sql
-- Try to grant yourself admin access to someone else's project
SELECT grant_project_access(
    'someone-elses-project-id',
    auth.uid(),
    'admin'
);
```

**Expected Result**: Should fail with "Only project owners can grant access"

### 5. Performance Testing

#### Test 1: Check Index Usage

```sql
-- Check if indexes are being used
EXPLAIN (ANALYZE, BUFFERS)
SELECT * FROM user_project_access;
```

**Expected Result**: Should show index usage, not sequential scans

#### Test 2: Large Dataset Performance

```sql
-- Test with many projects and permissions
-- (Create test data first)
EXPLAIN (ANALYZE, BUFFERS)
SELECT p.*, pp.permission_level
FROM projects p
LEFT JOIN project_permissions pp ON p.id = pp.project_id AND pp.user_id = auth.uid()
WHERE p.owner_id = auth.uid() OR pp.user_id = auth.uid();
```

**Expected Result**: Should use indexes efficiently

### 6. Frontend Integration Testing

#### Test 1: JavaScript Service

```javascript
// Test the project permissions service
import {
  getUserProjectAccess,
  grantProjectAccess,
} from "$lib/services/project-permissions-service";

// Get user's accessible projects
const userProjects = await getUserProjectAccess();
console.log("User projects:", userProjects);

// Grant access to another user
const success = await grantProjectAccess("project-id", "user-id", "viewer");
console.log("Access granted:", success);
```

#### Test 2: RLS with Different Users

1. Login as User A
2. Create a project
3. Login as User B
4. Verify User B cannot see User A's project
5. Grant User B access to User A's project
6. Verify User B can now see the project

### 7. Error Testing

#### Test 1: Invalid Permission Levels

```sql
-- Try to grant invalid permission level
SELECT grant_project_access(
    'project-id',
    'user-id',
    'invalid-level'
);
```

**Expected Result**: Should fail with constraint violation

#### Test 2: Non-existent Project

```sql
-- Try to grant access to non-existent project
SELECT grant_project_access(
    '00000000-0000-0000-0000-000000000000',
    'user-id',
    'viewer'
);
```

**Expected Result**: Should fail gracefully

## Expected Performance Benchmarks

Based on Supabase RLS best practices:

| Test                 | Without Indexes | With Indexes | Improvement |
| -------------------- | --------------- | ------------ | ----------- |
| Simple RLS Query     | ~171ms          | <0.1ms       | 99.94%      |
| Function Wrapping    | ~179ms          | ~9ms         | 94.97%      |
| Role-Specific Policy | ~170ms          | <0.1ms       | 99.78%      |

## Troubleshooting

### Common Issues

1. **RLS Not Working**: Check if RLS is enabled and policies exist
2. **Performance Issues**: Verify indexes are created and being used
3. **Permission Errors**: Check if user is authenticated and has proper permissions
4. **Empty Results**: Verify user has access to projects (owner or explicit permission)

### Debug Queries

```sql
-- Check current user
SELECT auth.uid();

-- Check user's permissions
SELECT * FROM project_permissions WHERE user_id = auth.uid();

-- Check user's owned projects
SELECT * FROM projects WHERE owner_id = auth.uid();

-- Check all accessible projects
SELECT * FROM user_project_access;
```

## Success Criteria

✅ **Security**: Users can only access their own projects or projects they have explicit permissions for
✅ **Performance**: Queries complete in <10ms for typical datasets
✅ **Functionality**: All CRUD operations work correctly with proper permissions
✅ **Audit Trail**: All permission changes are tracked with timestamps and grantor information
✅ **Error Handling**: Invalid operations fail gracefully with appropriate error messages
