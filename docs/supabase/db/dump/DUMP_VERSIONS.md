# MudRock Database Dump Versions

## Available Dump Files

### 1. `sql/schemas/mudrock-dump.sql` (Original)
- **Purpose**: Restore to Supabase instance
- **Users**: References `supabase_admin`, `postgres`, `pgbouncer`
- **Use Case**: Restoring to self-hosted Supabase or Supabase Cloud
- **Size**: ~597KB

### 2. `sql/schemas/mudrock-dump-portable.sql`
- **Purpose**: More portable, uses `CURRENT_USER`
- **Users**: Replaces specific users with `CURRENT_USER`
- **Use Case**: Restoring to any PostgreSQL database where you're connected as a user
- **Size**: ~597KB

### 3. `sql/schemas/mudrock-dump-no-owner.sql`
- **Purpose**: Maximum portability, no OWNER clauses
- **Users**: Removes all OWNER TO clauses
- **Use Case**: Executing in SQL editors (DBeaver, pgAdmin, etc.) without user restrictions
- **Size**: ~590KB

## Which Version Should You Use?

### ✅ Use `mudrock-dump.sql` if:
- Restoring to Supabase (self-hosted or cloud)
- You have `supabase_admin` user available
- You want exact ownership preserved

### ✅ Use `mudrock-dump-portable.sql` if:
- Restoring to fresh PostgreSQL database
- You're connected as a specific user
- You want ownership assigned to your current user

### ✅ Use `mudrock-dump-no-owner.sql` if:
- Executing in SQL editor (DBeaver, pgAdmin, DataGrip, etc.)
- You don't care about specific ownership
- You want maximum compatibility

## Creating Portable Versions

### Make Portable (CURRENT_USER)
```bash
sed 's/OWNER TO supabase_admin/OWNER TO CURRENT_USER/g' sql/schemas/mudrock-dump.sql \
  | sed 's/OWNER TO postgres/OWNER TO CURRENT_USER/g' \
  | sed 's/OWNER TO pgbouncer/OWNER TO CURRENT_USER/g' \
  > sql/schemas/mudrock-dump-portable.sql
```

### Remove OWNER Clauses
```bash
grep -v "^ALTER.*OWNER TO" sql/schemas/mudrock-dump.sql > sql/schemas/mudrock-dump-no-owner.sql
```

## Executing in SQL Editors

### DBeaver / pgAdmin / DataGrip

1. **Connect** to your PostgreSQL database
2. **Open** SQL editor
3. **Load** `sql/schemas/mudrock-dump-no-owner.sql` or `sql/schemas/mudrock-dump-portable.sql`
4. **Execute** the script

**Note**: Make sure required extensions are installed first:
```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pgjwt";
CREATE EXTENSION IF NOT EXISTS "postgis";
CREATE EXTENSION IF NOT EXISTS "pg_stat_statements";
```

### VS Code / Other Editors

Same process - just ensure you're connected to PostgreSQL and execute the SQL file.

## Verification

After executing any dump file, verify:

```sql
-- Check schemas
SELECT schema_name FROM information_schema.schemata 
WHERE schema_name NOT IN ('pg_catalog', 'information_schema', 'pg_toast')
ORDER BY schema_name;

-- Check tables in public schema
SELECT table_name FROM information_schema.tables 
WHERE table_schema = 'public'
ORDER BY table_name;

-- Check data counts
SELECT 
  'wells' as table_name, COUNT(*) as row_count FROM public.wells
UNION ALL
SELECT 'curves', COUNT(*) FROM public.curves
UNION ALL
SELECT 'nodes', COUNT(*) FROM public.nodes
UNION ALL
SELECT 'pipelines', COUNT(*) FROM public.pipelines
UNION ALL
SELECT 'projects', COUNT(*) FROM public.projects;
```

## All Files Include

- ✅ Complete schema structure (6 schemas, 51 tables)
- ✅ All table data (52 COPY statements)
- ✅ Functions, triggers, indexes
- ✅ Extensions (may need installation)
- ❌ Excludes: `auth`, `storage`, `extensions` schemas

