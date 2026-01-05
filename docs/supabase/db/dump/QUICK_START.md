# Quick Start: Recreating Database from Dump

## 🎯 Direct Answer to Your Question

**Can you execute `mudrock-dump.sql` in a blank SQL editor project to recreate the database?**

### ✅ **YES, but use the right version:**

1. **For SQL Editors** (DBeaver, pgAdmin, DataGrip, VS Code):
   - Use: **`mudrock-dump-no-owner.sql`** ✅
   - This version removes user ownership restrictions
   - Works in any SQL editor without user setup

2. **For Supabase Restore**:
   - Use: **`mudrock-dump.sql`** ✅
   - Original version with proper ownership

3. **For Fresh PostgreSQL**:
   - Use: **`mudrock-dump-portable.sql`** ✅
   - Uses CURRENT_USER instead of specific users

## 🚀 Quick Steps

### Option 1: SQL Editor (Easiest)

```sql
-- 1. Connect to your PostgreSQL database
-- 2. Install extensions first:
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pgjwt";
CREATE EXTENSION IF NOT EXISTS "postgis";
CREATE EXTENSION IF NOT EXISTS "pg_stat_statements";

-- 3. Execute sql/schemas/mudrock-dump-no-owner.sql
--    (Load file and run entire script)
```

### Option 2: Command Line

```bash
# For Supabase instance
psql -U supabase_admin -d postgres -f sql/schemas/mudrock-dump.sql

# For any PostgreSQL
psql -U your_user -d your_database -f sql/schemas/mudrock-dump-no-owner.sql
```

### Option 3: Automated Script

```bash
# Restore to Supabase instance (defaults to sql/schemas/mudrock-dump.sql)
./scripts/database/restore-supabase-db.sh

# Or specify custom file
./scripts/database/restore-supabase-db.sh sql/schemas/mudrock-dump-no-owner.sql
```

## 📋 What You'll Get

After executing the dump, you'll have:

- ✅ **6 schemas**: `_realtime`, `graphql`, `realtime`, `vault`, `pgbouncer`, `public`
- ✅ **51 tables**: All your tables (wells, curves, nodes, pipelines, etc.)
- ✅ **All your data**: Complete data from all tables
- ✅ **Functions & Triggers**: All custom functions and triggers
- ✅ **Indexes**: All indexes for performance

## ⚠️ Important Notes

1. **Extensions Required**: Some extensions (`pg_graphql`, `supabase_vault`) may need special installation
2. **Excluded Schemas**: `auth` and `storage` schemas are excluded (Supabase-managed)
3. **User Permissions**: Ensure your database user has CREATE privileges

## ✅ Verification

After restore, run:

```sql
SELECT COUNT(*) FROM public.wells;
SELECT COUNT(*) FROM public.curves;
SELECT COUNT(*) FROM public.nodes;
```

If these return counts matching your original database, the restore was successful!

## 📁 File Locations

All dump files are located in `sql/schemas/`:

- `sql/schemas/mudrock-dump.sql` - Original (for Supabase)
- `sql/schemas/mudrock-dump-portable.sql` - Uses CURRENT_USER
- `sql/schemas/mudrock-dump-no-owner.sql` - **Best for SQL editors** ⭐

## 🎉 Summary

**Yes, you can recreate your database!** Use `sql/schemas/mudrock-dump-no-owner.sql` for SQL editors, or the original `sql/schemas/mudrock-dump.sql` for Supabase instances. The dump is complete and production-ready.
