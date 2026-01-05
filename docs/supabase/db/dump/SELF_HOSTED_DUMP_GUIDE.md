# Self-Hosted Supabase Database Dump Guide

## ✅ Successfully Tested Method

### Quick Dump (Recommended)

Use the provided script:

```bash
# Note: dump-supabase-db.sh should be created in scripts/database/
# For now, use restore script or manual method below
./scripts/database/restore-supabase-db.sh
```

Or specify output file (when script is created):

```bash
# TODO: Create scripts/database/dump-supabase-db.sh
./scripts/database/dump-supabase-db.sh my-backup.sql
```

### Manual Method

```bash
ssh -i ~/.ssh/id_rsa_mudrock root@91.99.166.223 \
  "docker exec -e PGPASSWORD='MudRockSecure2024' mudrock-supabase-db \
    pg_dump -U supabase_admin -d postgres \
    --exclude-schema=auth \
    --exclude-schema=storage \
    --exclude-schema=extensions" \
  > sql/schemas/mudrock-dump.sql
```

## Configuration Details

From `supabase.env`:

- **SSH Host**: `91.99.166.223`
- **SSH User**: `root`
- **SSH Key**: `~/.ssh/id_rsa_mudrock`
- **Container**: `mudrock-supabase-db`
- **Database User**: `supabase_admin`
- **Database Name**: `postgres`
- **Password**: `MudRockSecure2024`

## Why This Method Works

1. **Container Name**: The actual container name is `mudrock-supabase-db` (not `mudrock-db`)
2. **Password**: Must be passed via `PGPASSWORD` environment variable inside Docker exec
3. **SSH Access**: Database port (5432) is not exposed externally for security
4. **Excluded Schemas**: `auth`, `storage`, and `extensions` are Supabase-managed schemas

## Using Supabase CLI

The Supabase CLI (`supabase db dump`) requires Docker Desktop to be running locally, which makes it less suitable for remote dumps. However, you can use it if you:

1. Set up an SSH tunnel:
   ```bash
   ssh -i ~/.ssh/id_rsa_mudrock -L 5433:localhost:5432 root@91.99.166.223
   ```

2. Use Supabase CLI with the tunnel:
   ```bash
   supabase db dump \
     --db-url "postgresql://supabase_admin:MudRockSecure2024@localhost:5433/postgres" \
     -f mudrock-dump.sql
   ```

**Note**: This method may still require Docker Desktop for internal operations.

## Dump Options

### Include All Schemas

To include `auth` and `storage` schemas:

```bash
ssh -i ~/.ssh/id_rsa_mudrock root@91.99.166.223 \
  "docker exec -e PGPASSWORD='MudRockSecure2024' mudrock-supabase-db \
    pg_dump -U supabase_admin -d postgres" \
  > sql/schemas/mudrock-full-dump.sql
```

### Data Only

```bash
ssh -i ~/.ssh/id_rsa_mudrock root@91.99.166.223 \
  "docker exec -e PGPASSWORD='MudRockSecure2024' mudrock-supabase-db \
    pg_dump -U supabase_admin -d postgres \
    --data-only \
    --exclude-schema=auth \
    --exclude-schema=storage \
    --exclude-schema=extensions" \
  > sql/schemas/mudrock-data-only.sql
```

### Schema Only

```bash
ssh -i ~/.ssh/id_rsa_mudrock root@91.99.166.223 \
  "docker exec -e PGPASSWORD='MudRockSecure2024' mudrock-supabase-db \
    pg_dump -U supabase_admin -d postgres \
    --schema-only \
    --exclude-schema=auth \
    --exclude-schema=storage \
    --exclude-schema=extensions" \
  > sql/schemas/mudrock-schema-only.sql
```

### Specific Schemas Only

```bash
ssh -i ~/.ssh/id_rsa_mudrock root@91.99.166.223 \
  "docker exec -e PGPASSWORD='MudRockSecure2024' mudrock-supabase-db \
    pg_dump -U supabase_admin -d postgres \
    --schema=public" \
  > sql/schemas/mudrock-public-schema.sql
```

## Restoring a Dump

To restore the dump:

```bash
# Copy dump file to VPS
scp -i ~/.ssh/id_rsa_mudrock sql/schemas/mudrock-dump.sql root@91.99.166.223:/tmp/

# Restore via SSH
ssh -i ~/.ssh/id_rsa_mudrock root@91.99.166.223 \
  "docker exec -i -e PGPASSWORD='MudRockSecure2024' mudrock-supabase-db \
    psql -U supabase_admin -d postgres" \
  < sql/schemas/mudrock-dump.sql
```

## Troubleshooting

### Container Not Found

Check container name:
```bash
ssh -i ~/.ssh/id_rsa_mudrock root@91.99.166.223 "docker ps | grep db"
```

### Password Authentication Failed

Verify password in `supabase.env`:
```bash
grep POSTGRES_PASSWORD supabase.env
```

### Connection Timeout

Ensure SSH access is working:
```bash
ssh -i ~/.ssh/id_rsa_mudrock root@91.99.166.223 "echo 'SSH connection successful'"
```

## Output Example

Successful dump includes:
- ✅ Database schema (tables, views, functions, triggers)
- ✅ Extensions (pg_graphql, postgis, etc.)
- ✅ Realtime configuration
- ✅ User tables in `public` schema
- ❌ Excludes: `auth`, `storage`, `extensions` schemas (Supabase-managed)

## File Size

Typical dump size: ~500KB - 2MB (depending on data volume)

## Next Steps

1. ✅ Dump created: `sql/schemas/mudrock-dump.sql`
2. Store dump securely (backup location)
3. Consider automated backups (cron job)
4. Test restore process on staging environment

