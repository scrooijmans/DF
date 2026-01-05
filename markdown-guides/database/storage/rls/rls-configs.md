# Supabase Storage RLS (Row Level Security) Configuration Guide

## Overview

This guide explains how to configure Row Level Security (RLS) policies for Supabase Storage to enable file uploads, downloads, and management operations. RLS policies control access to the `storage.objects` table, which stores metadata about uploaded files.

## Why RLS is Required

Supabase Storage uses PostgreSQL's Row Level Security to control access to file metadata. Without proper RLS policies:

- File uploads fail with "new row violates row-level security policy" errors
- File downloads and management operations are blocked
- The storage system becomes unusable for authenticated users

## Table Structure

The `storage.objects` table contains:

- `bucket_id`: The bucket name (e.g., 'pets', 'documents')
- `name`: The file path/name within the bucket
- `owner`: User ID who uploaded the file
- `created_at`, `updated_at`: Timestamps
- `metadata`: File metadata (size, content type, etc.)

## RLS Policy Types

### 1. Permissive Policies (Recommended for Development)

Allow operations for all authenticated users across all buckets.

### 2. Restrictive Policies (Recommended for Production)

Limit operations to specific buckets or users.

### 3. Role-Based Policies

Different permissions for different user roles (`anon`, `authenticated`, `service_role`).

## Configuration Scripts

### Script 1: Minimal RLS Policies (`sql/rls/fix-storage-rls-minimal.sql`)

**Purpose**: Quick setup for development with maximum permissions.

**Features**:

- Single policy per operation type
- All roles grouped together
- Permissive access to all buckets
- Easy to understand and maintain

**Usage**:

```sql
-- Enable RLS
ALTER TABLE storage.objects ENABLE ROW LEVEL SECURITY;

-- Drop existing policies
DROP POLICY IF EXISTS "Allow uploads" ON storage.objects;
DROP POLICY IF EXISTS "Allow reads" ON storage.objects;
DROP POLICY IF EXISTS "Allow updates" ON storage.objects;
DROP POLICY IF EXISTS "Allow deletes" ON storage.objects;

-- Create comprehensive policies
CREATE POLICY "Allow uploads"
ON storage.objects FOR INSERT
TO authenticated, service_role, anon
WITH CHECK (true);

CREATE POLICY "Allow reads"
ON storage.objects FOR SELECT
TO authenticated, service_role, anon
USING (true);

CREATE POLICY "Allow updates"
ON storage.objects FOR UPDATE
TO authenticated, service_role, anon
USING (true) WITH CHECK (true);

CREATE POLICY "Allow deletes"
ON storage.objects FOR DELETE
TO authenticated, service_role, anon
USING (true);
```

### Script 2: Detailed RLS Policies (`sql/rls/fix-storage-rls-policies.sql`)

**Purpose**: Granular control with separate policies per role and operation.

**Features**:

- Individual policies for each role (`authenticated`, `service_role`, `anon`)
- Separate policies for each operation (`INSERT`, `SELECT`, `UPDATE`, `DELETE`)
- Clear naming convention
- Easy to modify specific permissions

**Usage**:

```sql
-- Enable RLS
ALTER TABLE storage.objects ENABLE ROW LEVEL SECURITY;

-- Drop existing policies (example for INSERT)
DROP POLICY IF EXISTS "Allow authenticated uploads to any bucket" ON storage.objects;
DROP POLICY IF EXISTS "Allow service role uploads to any bucket" ON storage.objects;
DROP POLICY IF EXISTS "Allow anon uploads to any bucket" ON storage.objects;

-- Create role-specific policies
CREATE POLICY "Allow authenticated uploads to any bucket"
ON storage.objects FOR INSERT TO authenticated
WITH CHECK (true);

CREATE POLICY "Allow service role uploads to any bucket"
ON storage.objects FOR INSERT TO service_role
WITH CHECK (true);

CREATE POLICY "Allow anon uploads to any bucket"
ON storage.objects FOR INSERT TO anon
WITH CHECK (true);
```

### Script 3: Restrictive RLS Policies (`sql/rls/fix-storage-rls-policies-restrictive.sql`)

**Purpose**: Production-ready policies that restrict access to specific buckets.

**Features**:

- Bucket-specific access control
- Uses `bucket_id = 'pets'` condition
- More secure for production environments
- Prevents cross-bucket access

**Usage**:

```sql
-- Enable RLS
ALTER TABLE storage.objects ENABLE ROW LEVEL SECURITY;

-- Create bucket-restricted policies
CREATE POLICY "Allow authenticated uploads to pets bucket"
ON storage.objects FOR INSERT TO authenticated
WITH CHECK (bucket_id = 'pets');

CREATE POLICY "Allow service role uploads to pets bucket"
ON storage.objects FOR INSERT TO service_role
WITH CHECK (bucket_id = 'pets');

CREATE POLICY "Allow anon uploads to pets bucket"
ON storage.objects FOR INSERT TO anon
WITH CHECK (bucket_id = 'pets');
```

## Role Explanations

### `authenticated`

- **Purpose**: Logged-in users
- **Use Case**: Regular application users uploading/downloading files
- **Permissions**: Typically full access to their own files

### `service_role`

- **Purpose**: Backend services and server-side operations
- **Use Case**: Automated processes, admin operations, API endpoints
- **Permissions**: Usually full access to all files and buckets

### `anon`

- **Purpose**: Anonymous/unauthenticated users
- **Use Case**: Public file access, guest uploads
- **Permissions**: Limited access, often read-only or specific buckets only

## Policy Components Explained

### `USING` Clause

- **Purpose**: Controls which existing rows can be accessed
- **Example**: `USING (bucket_id = 'pets')` - only access files in 'pets' bucket
- **Use Case**: `SELECT`, `UPDATE`, `DELETE` operations

### `WITH CHECK` Clause

- **Purpose**: Controls which new/updated rows are allowed
- **Example**: `WITH CHECK (bucket_id = 'pets')` - only allow uploads to 'pets' bucket
- **Use Case**: `INSERT`, `UPDATE` operations

### `FOR` Clause

- **Purpose**: Specifies which operations the policy applies to
- **Options**: `INSERT`, `SELECT`, `UPDATE`, `DELETE`, `ALL`
- **Use Case**: Granular control over different operations

## Common Patterns

### Pattern 1: Public Read, Authenticated Write

```sql
-- Anyone can read
CREATE POLICY "Public read access"
ON storage.objects FOR SELECT TO anon, authenticated, service_role
USING (true);

-- Only authenticated users can write
CREATE POLICY "Authenticated write access"
ON storage.objects FOR INSERT TO authenticated, service_role
WITH CHECK (true);
```

### Pattern 2: User-Specific Access

```sql
-- Users can only access their own files
CREATE POLICY "User-specific access"
ON storage.objects FOR ALL TO authenticated
USING (owner = auth.uid())
WITH CHECK (owner = auth.uid());
```

### Pattern 3: Bucket-Based Access

```sql
-- Different permissions per bucket
CREATE POLICY "Public bucket access"
ON storage.objects FOR SELECT TO anon, authenticated, service_role
USING (bucket_id = 'public');

CREATE POLICY "Private bucket access"
ON storage.objects FOR ALL TO authenticated, service_role
USING (bucket_id = 'private');
```

## Troubleshooting Common Issues

### Issue 1: "new row violates row-level security policy"

**Cause**: No `INSERT` policy or `WITH CHECK` condition too restrictive
**Solution**: Add appropriate `INSERT` policy with `WITH CHECK (true)` or specific condition

### Issue 2: "permission denied for table storage.objects"

**Cause**: RLS enabled but no policies exist
**Solution**: Create at least one policy for the operation you're trying to perform

### Issue 3: "syntax error at or near 'NOT'"

**Cause**: Using `CREATE POLICY IF NOT EXISTS` (not supported in PostgreSQL)
**Solution**: Use `DROP POLICY IF EXISTS` followed by `CREATE POLICY`

### Issue 4: Files not visible after upload

**Cause**: No `SELECT` policy or `USING` condition too restrictive
**Solution**: Add appropriate `SELECT` policy with `USING (true)` or specific condition

## Verification Queries

### Check if RLS is enabled:

```sql
SELECT schemaname, tablename, rowsecurity
FROM pg_tables
WHERE tablename = 'objects' AND schemaname = 'storage';
```

### List all policies:

```sql
SELECT schemaname, tablename, policyname, permissive, roles, cmd, qual, with_check
FROM pg_policies
WHERE tablename = 'objects' AND schemaname = 'storage'
ORDER BY policyname;
```

### Test policy effectiveness:

```sql
-- Test as different roles
SET ROLE authenticated;
SELECT * FROM storage.objects LIMIT 1;

SET ROLE service_role;
SELECT * FROM storage.objects LIMIT 1;

SET ROLE anon;
SELECT * FROM storage.objects LIMIT 1;
```

## Replication Steps

### For New Supabase Instance:

1. **Access Supabase Studio**:
   - Go to your Supabase project dashboard
   - Navigate to SQL Editor

2. **Choose Configuration Type**:
   - **Development**: Use `fix-storage-rls-minimal.sql`
   - **Production**: Use `fix-storage-rls-policies-restrictive.sql`
   - **Custom**: Use `fix-storage-rls-policies.sql` as template

3. **Execute Script**:
   - Copy the chosen SQL script
   - Paste into SQL Editor
   - Click "Run" to execute

4. **Verify Setup**:
   - Run verification queries
   - Test file upload/download
   - Check policy list

### For Existing Instance with Issues:

1. **Identify Problem**:
   - Check error messages
   - Run verification queries
   - Identify missing policies

2. **Fix Policies**:
   - Drop problematic policies
   - Create correct policies
   - Test operations

3. **Validate Fix**:
   - Test all operations
   - Verify security requirements
   - Document changes

## Security Considerations

### Development Environment:

- Use permissive policies for easy testing
- Allow all roles full access
- Focus on functionality over security

### Production Environment:

- Use restrictive policies
- Limit access to specific buckets
- Implement user-specific access where needed
- Regular security audits

### Best Practices:

- Document all policy changes
- Test policies thoroughly
- Use least-privilege principle
- Monitor access logs
- Regular policy reviews

## Maintenance

### Regular Tasks:

- Review policy effectiveness
- Update policies for new requirements
- Monitor for security issues
- Clean up unused policies

### Monitoring:

- Check access logs
- Monitor failed operations
- Review user feedback
- Track policy performance

## Conclusion

Proper RLS configuration is essential for Supabase Storage functionality. Choose the appropriate script based on your environment (development vs production) and security requirements. Always test policies thoroughly and document changes for future reference.

For questions or issues, refer to the Supabase documentation or check the troubleshooting section above.
