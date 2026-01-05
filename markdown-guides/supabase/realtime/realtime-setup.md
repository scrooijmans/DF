# Supabase Realtime Setup Guide

Complete configuration guide for setting up Supabase Realtime in MudRock for real-time database change notifications.

## Overview

This guide covers all configurations needed to enable Realtime functionality for the `wells` table. Realtime enables real-time synchronization of database changes to the frontend without requiring manual refresh.

## Architecture

```
PostgreSQL → Realtime Service → Kong Gateway → Frontend (WebSocket)
```

1. **PostgreSQL**: Publishes changes via logical replication
2. **Realtime Service**: Listens to PostgreSQL WAL and manages WebSocket connections
3. **Kong Gateway**: Routes WebSocket connections from frontend to Realtime
4. **Frontend**: Subscribes to changes via Supabase JS Client

---

## 1. Environment Variables (`supabase.env`)

**Required JWT Configuration:**

```env
# JWT Secret (used for all Supabase services)
GOTRUE_JWT_SECRET=cAdtagpNtA1Wy9a7pbRS+QLb0LkxtBncXWkc//hPdPg=

# Supabase URL (self-hosted)
PUBLIC_SUPABASE_URL=http://91.99.166.223:8000
PUBLIC_SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Critical:** The `GOTRUE_JWT_SECRET` must be the same across:

- Realtime service (`API_JWT_SECRET`, `JWT_SECRET`)
- Tenant `jwt_secret` in database (for tenant '91' and 'supabase-realtime')
- GoTrue Auth service

---

## 2. Docker Compose Configuration (`docker-compose-supabase.yml`)

**Realtime Service Configuration:**

```yaml
supabase-realtime:
  image: supabase/realtime:v2.34.47
  restart: unless-stopped
  container_name: mudrock-supabase-realtime
  depends_on:
    supabase-db:
      condition: service_healthy
  environment:
    # Port configuration
    PORT: 4000
    POSTGRES_PORT: 5432

    # Database connection
    DB_HOST: supabase-db
    DB_PORT: 5432
    DB_USER: supabase_admin
    DB_PASSWORD: ${POSTGRES_PASSWORD}
    DB_NAME: ${POSTGRES_DB}
    DB_AFTER_CONNECT_QUERY: SET search_path TO _realtime # CRITICAL: Must be _realtime

    # Encryption key (for encrypting tenant jwt_secret)
    DB_ENC_KEY: supabaserealtime # Fixed value used by Realtime

    # JWT secrets (MUST match GOTRUE_JWT_SECRET from supabase.env)
    API_JWT_SECRET: ${GOTRUE_JWT_SECRET} # For API authentication
    JWT_SECRET: ${GOTRUE_JWT_SECRET} # For tenant JWT verification

    # Publication and security
    PUBLICATIONS: '["supabase_realtime"]' # PostgreSQL publication name
    SECURE_CHANNELS: "true" # Enable secure channels

    # Fly.io compatibility (required even for self-hosted)
    FLY_ALLOC_ID: fly123
    FLY_APP_NAME: realtime
    FLY_REGION: "local"

    # Self-hosted configuration
    SEED_SELF_HOST: "true" # Enables automatic tenant seeding
    SECRET_KEY_BASE: UpNVntn3cDxHJpq99YMc1T1AQgQpc8kfYTuRgBiYa15BLrx8etQoXz3gZv1/u2oq

    # Erlang/Elixir configuration
    ERL_AFLAGS: -proto_dist inet_tcp
    ENABLE_TAILSCALE: "false"
    DNS_NODES: "''" # Quoted empty string
    RLIMIT_NOFILE: "10000" # File descriptor limit

  ports:
    - "4001:4000" # External:Internal
  env_file:
    - .env # Loads variables from supabase.env
```

**Key Points:**

- `DB_AFTER_CONNECT_QUERY` must be `SET search_path TO _realtime` (not `realtime`)
- `API_JWT_SECRET` and `JWT_SECRET` must match `GOTRUE_JWT_SECRET`
- `PUBLICATIONS` must include `"supabase_realtime"` (matches PostgreSQL publication)
- `SEED_SELF_HOST: "true"` enables automatic tenant creation on startup

---

## 3. Kong API Gateway Configuration (`kong-simple.yml`)

**Realtime Service Route:**

```yaml
services:
  - name: supabase-realtime
    url: http://mudrock-supabase-realtime:4000/socket # No trailing slash
    routes:
      - name: supabase-realtime-all
        strip_path: true # Strips /realtime/v1/ from path
        paths:
          - /realtime/v1/ # Client connects to /realtime/v1/websocket
        protocols:
          - http
          - https # Kong handles WebSocket upgrade automatically
        preserve_host: true # Preserves Host header for tenant detection
    plugins:
      - name: cors
        config:
          origins:
            - "*"
          methods:
            - GET
            - POST
            - PUT
            - DELETE
            - OPTIONS
          headers:
            - Accept
            - Accept-Language
            - Content-Language
            - Content-Type
            - Authorization
            - apikey
            # WebSocket headers (required for WebSocket upgrade)
            - Upgrade
            - Connection
            - Sec-WebSocket-Key
            - Sec-WebSocket-Version
            - Sec-WebSocket-Protocol
            - Sec-WebSocket-Extensions
          exposed_headers:
            - Authorization
          credentials: true
          max_age: 3600
      - name: key-auth
        config:
          hide_credentials: false
          anonymous: anon # CRITICAL: Allows requests with anon key
```

**Key Points:**

- `strip_path: true` means `/realtime/v1/websocket` → `/socket/websocket` after stripping
- Service URL: `http://mudrock-supabase-realtime:4000/socket` (no trailing slash)
- `anonymous: anon` allows unauthenticated requests with the anon API key
- WebSocket headers must be included in CORS for WebSocket upgrade

**Routing Flow:**

```
Client: ws://91.99.166.223:8000/realtime/v1/websocket?apikey=...
    ↓
Kong strips /realtime/v1/ → /websocket
    ↓
Routes to: http://mudrock-supabase-realtime:4000/socket/websocket
    ↓
Realtime receives WebSocket connection
```

---

## 4. Database Configuration (SQL)

### 4.1 Create Realtime Schema

The `_realtime` schema is automatically created by Realtime service on startup if `SEED_SELF_HOST: "true"`.

### 4.2 Create Tenant for IP-Based Connections

**For IP-based connections** (e.g., `91.99.166.223`), Realtime extracts tenant ID from the first octet of the IP (`91`).

**File:** `sql/create_tenant_91_properly.sql`

```sql
-- Delete existing tenant '91' if it exists
DELETE FROM _realtime.extensions WHERE tenant_external_id = '91';
DELETE FROM _realtime.tenants WHERE external_id = '91';

-- Create tenant '91' by copying encrypted jwt_secret from working tenant
-- IMPORTANT: Copy jwt_secret (not plaintext) to preserve encryption format
INSERT INTO _realtime.tenants (
    id, name, external_id, jwt_secret, max_concurrent_users,
    inserted_at, updated_at, max_events_per_second, max_bytes_per_second,
    max_channels_per_client, max_joins_per_second, private_only
)
SELECT
    gen_random_uuid(),
    '91',
    '91',
    jwt_secret,  -- Copy encrypted jwt_secret from supabase-realtime tenant
    200,
    NOW(),
    NOW(),
    100,
    100000,
    100,
    500,
    false
FROM _realtime.tenants
WHERE external_id = 'supabase-realtime'  -- Must exist first
LIMIT 1;

-- Copy extension configuration from working tenant
INSERT INTO _realtime.extensions (
    id, type, settings, tenant_external_id, inserted_at, updated_at
)
SELECT
    gen_random_uuid(),
    type,
    settings,
    '91',
    NOW(),
    NOW()
FROM _realtime.extensions
WHERE tenant_external_id = 'supabase-realtime';

-- Verify creation
SELECT
    '✅ Tenant 91 Created' as status,
    external_id,
    name,
    LENGTH(jwt_secret::text) as jwt_secret_length,
    (SELECT COUNT(*) FROM _realtime.extensions WHERE tenant_external_id = '91') as extension_count
FROM _realtime.tenants
WHERE external_id = '91';
```

**Key Points:**

- **CRITICAL**: Copy `jwt_secret` from an existing tenant (like `supabase-realtime`) - do NOT insert plaintext
- The `jwt_secret` must be encrypted using `DB_ENC_KEY` (Realtime handles this automatically for API-created tenants)
- Tenant `supabase-realtime` must exist first (created via seeds or SQL)

### 4.3 Create Supabase-Realtime Tenant

**File:** `sql/fix_realtime_complete.sql`

```sql
-- Create tenant with external_id = 'supabase-realtime'
-- This is the default tenant used when connecting through Kong
INSERT INTO _realtime.tenants (
    id, name, external_id, jwt_secret, max_concurrent_users,
    inserted_at, updated_at, max_events_per_second, max_bytes_per_second,
    max_channels_per_client, max_joins_per_second, private_only
)
SELECT
    gen_random_uuid(),
    'supabase-realtime',
    'supabase-realtime',
    'cAdtagpNtA1Wy9a7pbRS+QLb0LkxtBncXWkc//hPdPg='::text,  -- GOTRUE_JWT_SECRET
    200,
    NOW(),
    NOW(),
    100,
    100000,
    100,
    500,
    false
FROM _realtime.tenants
WHERE external_id = 'realtime-dev'
LIMIT 1
ON CONFLICT (external_id) DO NOTHING;

-- Copy extension configuration
INSERT INTO _realtime.extensions (
    id, type, settings, tenant_external_id, inserted_at, updated_at
)
SELECT
    gen_random_uuid(),
    type,
    settings,
    'supabase-realtime',
    NOW(),
    NOW()
FROM _realtime.extensions
WHERE tenant_external_id = 'realtime-dev'
ON CONFLICT DO NOTHING;
```

**Note:** The `supabase-realtime` tenant is typically auto-created by Realtime seeds when `SEED_SELF_HOST: "true"`. Only run this SQL if the tenant doesn't exist.

### 4.4 Add Table to Publication

**Option 1: Enable Realtime for wells table only**

```sql
-- Add wells table to supabase_realtime publication
-- This enables logical replication for the wells table
-- Required for Realtime to detect changes
ALTER PUBLICATION supabase_realtime ADD TABLE public.wells;

-- Verify table is in publication
SELECT
    schemaname,
    tablename,
    pubname
FROM pg_publication_tables
WHERE pubname = 'supabase_realtime' AND tablename = 'wells';
-- Should return one row with tablename = 'wells'
```

**Option 2: Enable Realtime for all public tables**

**File:** `sql/enable_realtime_all_public_tables.sql`

This script automatically adds all public tables to the `supabase_realtime` publication and sets up RLS policies for `notebooks` and `pipelines` tables.

**Recommendation:** Use Option 1 if you only need Realtime for `wells`. Use Option 2 if you want Realtime for multiple tables.

### 4.5 Enable RLS and Create Policies

**File:** `sql/fix_realtime_complete.sql` (continued)

```sql
-- Enable RLS on wells table
ALTER TABLE public.wells ENABLE ROW LEVEL SECURITY;

-- SELECT policies (required for Realtime to verify permissions)
CREATE POLICY "Allow anon access to wells for Realtime"
    ON public.wells FOR SELECT TO anon USING (true);

CREATE POLICY "Allow authenticated access to wells"
    ON public.wells FOR SELECT TO authenticated USING (true);

-- INSERT policies (for well creation during data ingestion)
CREATE POLICY "Allow anon insert to wells"
    ON public.wells FOR INSERT TO anon WITH CHECK (true);

CREATE POLICY "Allow authenticated insert to wells"
    ON public.wells FOR INSERT TO authenticated WITH CHECK (true);
```

**Key Points:**

- RLS **MUST** be enabled for Realtime to verify permissions
- SELECT policies are required for Realtime to read data
- INSERT policies are required for well creation (if needed)

---

## 5. Frontend Configuration

### 5.1 Supabase Client (`src/lib/supabase/client.ts`)

```typescript
import { createClient } from "@supabase/supabase-js";

const supabaseUrl = "http://91.99.166.223:8000";
const supabaseAnonKey = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."; // Your anon key

// Singleton pattern to prevent multiple instances
let isInitialized = false;
let supabaseInstance: ReturnType<typeof createClient> | null = null;

export function getSupabaseClient() {
  if (!isInitialized) {
    supabaseInstance = createClient(supabaseUrl, supabaseAnonKey, {
      realtime: {
        params: {
          eventsPerSecond: 10,
          // WebSocket URL conversion is automatic (http:// → ws://)
        },
      },
      auth: {
        persistSession: true,
        autoRefreshToken: true,
        detectSessionInUrl: false,
      },
    });
    isInitialized = true;
  }
  return supabaseInstance!;
}
```

### 5.2 Realtime Service (`src/lib/services/realtime-wells-service.ts`)

```typescript
import { getSupabaseClient } from "$lib/supabase/client";
import { invalidateTableCache } from "$lib/tauri-commands/table-data-fetching";

export class RealtimeWellsService {
  private supabase = getSupabaseClient();
  private channel: any = null;
  private isConnected = false;
  private wellsState: any;

  async connect(wellsState: any) {
    if (this.isConnected) return;

    this.wellsState = wellsState;

    this.channel = this.supabase
      .channel("wells-changes", {
        config: {
          // Don't use private: true for postgres_changes
          // postgres_changes already respects RLS policies
        },
      })
      .on(
        "postgres_changes",
        {
          event: "*", // INSERT, UPDATE, DELETE
          schema: "public",
          table: "wells",
        },
        async (payload: any) => {
          await this.handleWellsChange(payload);
        },
      )
      .subscribe((status: any) => {
        if (status === "SUBSCRIBED") {
          this.isConnected = true;
          console.log("✅ Connected to wells changes");
        } else if (status === "CHANNEL_ERROR") {
          console.error("❌ Channel error - check server logs");
          this.isConnected = false;
        }
      });
  }

  private async handleWellsChange(payload: any) {
    const { eventType, new: newRecord, old: oldRecord } = payload;

    switch (eventType) {
      case "INSERT":
        await this.handleWellInsert(newRecord);
        break;
      case "UPDATE":
        await this.handleWellUpdate(newRecord, oldRecord);
        break;
      case "DELETE":
        await this.handleWellDelete(oldRecord);
        break;
    }
  }

  private async handleWellInsert(newWell: any) {
    // Add to global state
    this.wellsState.wells = [...this.wellsState.wells, well];

    // Invalidate PostgREST cache
    await invalidateTableCache("wells");
  }

  // ... handleWellUpdate, handleWellDelete similar
}
```

### 5.3 Integration with Global State

**File:** `src/lib/state/postgres/postgres-wells-state.svelte.ts`

```typescript
import { realtimeWellsService } from "$lib/services/realtime-wells-service";

export class PostgresWellsState {
  wells = $state<Well[]>([]);

  constructor() {
    // Initialize Realtime connection after DOM is ready
    requestAnimationFrame(() => {
      setTimeout(() => {
        realtimeWellsService.connect(this);
      }, 100);
    });
  }
}
```

---

## 6. Verification Checklist

### ✅ Service Status

```bash
# Check Realtime service is running
docker ps | grep realtime

# Check logs for startup errors
docker logs mudrock-supabase-realtime --tail 50

# Should see:
# - "Starting Realtime"
# - "Running RealtimeWeb.Endpoint with cowboy"
# - No "TenantNotFound" or "Crypto" errors
```

### ✅ Database Configuration

```sql
-- Verify tenants exist
SELECT external_id, name FROM _realtime.tenants
WHERE external_id IN ('91', 'supabase-realtime');

-- Verify extensions
SELECT tenant_external_id, type FROM _realtime.extensions
WHERE tenant_external_id IN ('91', 'supabase-realtime');

-- Verify publication
SELECT tablename FROM pg_publication_tables
WHERE pubname = 'supabase_realtime' AND tablename = 'wells';

-- Verify RLS
SELECT tablename, rowsecurity FROM pg_tables
WHERE schemaname = 'public' AND tablename = 'wells';
-- rowsecurity should be true
```

### ✅ Frontend Connection

Open browser console and look for:

```
✅ [PostgresWellsState] Realtime service connected
🔄 [RealtimeWellsService] Subscription status: "SUBSCRIBED"
✅ [RealtimeWellsService] Connected to wells changes
```

### ✅ Test Real-time Updates

1. Create a new well (via LAS upload or SQL)
2. Check browser console for:
   ```
   🔄 [RealtimeWellsService] Received change: {eventType: "INSERT", ...}
   ✅ [RealtimeWellsService] Added new well to global state
   ```
3. Verify the well appears in the UI immediately (no refresh needed)

---

## 7. Troubleshooting

### Issue: "TenantNotFound: Tenant not found: 91"

**Cause:** Tenant '91' doesn't exist or was deleted.

**Solution:** Run `sql/create_tenant_91_properly.sql` in Supabase Studio.

### Issue: "ArgumentError: errors were found at the given arguments: binary.part(...) :binary.part(<<...>>, 0, -19)"

**Cause:** Tenant `jwt_secret` is stored as plaintext instead of encrypted.

**Solution:**

1. Delete tenant '91'
2. Recreate it by copying `jwt_secret` from `supabase-realtime` (which has proper encryption)
3. Run `sql/create_tenant_91_properly.sql`

### Issue: "WebSocket connection failed: There was a bad response from the server"

**Causes:**

- Kong routing misconfigured
- Realtime service not running
- Tenant encryption issue (see above)

**Solution:**

1. Check Kong logs: `docker logs mudrock-supabase-kong --tail 50`
2. Check Realtime logs: `docker logs mudrock-supabase-realtime --tail 50`
3. Verify Kong service URL matches Docker container name: `mudrock-supabase-realtime`

### Issue: "CHANNEL_ERROR" status

**Cause:** Server-side error (tenant missing, JWT mismatch, RLS policy issue).

**Solution:**

1. Check Realtime logs for specific error
2. Verify tenant exists and has correct `jwt_secret`
3. Verify RLS policies allow SELECT for `anon` and `authenticated` roles

---

## 8. Important Notes

### JWT Secret Management

- **CRITICAL**: Never insert plaintext `jwt_secret` directly into `_realtime.tenants`
- Always copy `jwt_secret` from an existing working tenant (e.g., `supabase-realtime`)
- The `jwt_secret` is encrypted by Realtime using `DB_ENC_KEY`
- All `jwt_secret` values must match `GOTRUE_JWT_SECRET` from `supabase.env`

### Tenant ID Detection

- Realtime extracts tenant ID from the **first octet** of the IP address
- For IP `91.99.166.223`, Realtime looks for tenant `external_id = '91'`
- For hostname connections (via Kong), Realtime uses tenant `external_id = 'supabase-realtime'`

### Encryption Format

- Encrypted `jwt_secret` typically has length 64 bytes (vs 44 for plaintext)
- Always verify: `SELECT LENGTH(jwt_secret::text) FROM _realtime.tenants WHERE external_id = '91';`
- Should return `64` (encrypted) not `44` (plaintext)

---

## Summary

**Required Components:**

1. ✅ Environment variables (`GOTRUE_JWT_SECRET`, `PUBLIC_SUPABASE_URL`)
2. ✅ Docker Compose configuration (`docker-compose-supabase.yml`)
3. ✅ Kong Gateway routing (`kong-simple.yml`)
4. ✅ Database tenants (`sql/create_tenant_91_properly.sql`)
5. ✅ Database publication (`sql/enable_realtime_wells.sql`)
6. ✅ RLS policies (`sql/fix_realtime_complete.sql`)
7. ✅ Frontend Supabase client (`src/lib/supabase/client.ts`)
8. ✅ Frontend Realtime service (`src/lib/services/realtime-wells-service.ts`)
9. ✅ Global state integration (`src/lib/state/postgres/postgres-wells-state.svelte.ts`)

**All components must be configured correctly for Realtime to work!**

---

## Quick Reference

### SQL Scripts Execution Order

1. **First**: Run `sql/fix_realtime_complete.sql` to create `supabase-realtime` tenant
2. **Then**: Run `sql/create_tenant_91_properly.sql` to create tenant '91' (for IP connections)
3. **Verify**: Both tenants exist with encrypted `jwt_secret` (length 64, not 44)

### Environment Variables Summary

```env
# Required in supabase.env
GOTRUE_JWT_SECRET=cAdtagpNtA1Wy9a7pbRS+QLb0LkxtBncXWkc//hPdPg=
PUBLIC_SUPABASE_URL=http://91.99.166.223:8000
PUBLIC_SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### Key Configuration Points

- **JWT Secret**: Must be identical across `GOTRUE_JWT_SECRET`, `API_JWT_SECRET`, `JWT_SECRET`, and tenant `jwt_secret`
- **Tenant Encryption**: Always copy `jwt_secret` from existing tenant, never insert plaintext
- **Tenant ID**: For IP `91.99.166.223`, Realtime looks for tenant `external_id = '91'`
- **Kong Service URL**: Must match Docker container name `mudrock-supabase-realtime`
- **WebSocket Routing**: Kong strips `/realtime/v1/` and routes to `/socket/websocket`

### Verification Commands

```bash
# Check Realtime service
docker ps | grep realtime
docker logs mudrock-supabase-realtime --tail 20

# Check tenants in database
PGPASSWORD=MudRockSecure2024 docker exec -e PGPASSWORD=MudRockSecure2024 mudrock-supabase-db \
  psql -U supabase_admin -d postgres -c \
  "SELECT external_id, LENGTH(jwt_secret::text) as jwt_len FROM _realtime.tenants WHERE external_id IN ('91', 'supabase-realtime');"

# Should return:
#  external_id    | jwt_len
# ----------------+---------
#  91             |      64
#  supabase-realtime |      64
```

If `jwt_len` is `44` instead of `64`, the tenant has plaintext `jwt_secret` and needs to be recreated.

### Troubleshooting Quick Reference

| Error                                                           | Cause                                  | Solution                                                              |
| --------------------------------------------------------------- | -------------------------------------- | --------------------------------------------------------------------- |
| `TenantNotFound: Tenant not found: 91`                          | Tenant '91' doesn't exist              | Run `sql/create_tenant_91_properly.sql`                               |
| `ArgumentError: binary.part(...) :binary.part(<<...>>, 0, -19)` | Tenant `jwt_secret` is plaintext       | Delete tenant, recreate by copying encrypted `jwt_secret`             |
| `WebSocket connection failed: bad response`                     | Kong routing or Realtime service issue | Check Kong logs and Realtime service status                           |
| `CHANNEL_ERROR` status                                          | Server-side error (tenant/JWT/RLS)     | Check Realtime logs, verify tenant exists with encrypted `jwt_secret` |

### Next Steps

After completing setup:

1. **Test Connection**: Refresh frontend app and check browser console for:

   ```
   ✅ [RealtimeWellsService] Connected to wells changes
   ```

2. **Test Real-time Updates**: Create a new well (via SQL or upload) and verify it appears in UI immediately

3. **Monitor Logs**: Watch Realtime service logs for any errors:
   ```bash
   docker logs -f mudrock-supabase-realtime
   ```

For detailed implementation flow and architecture, see: [`realtime-wells-reactive-to-postgres-update.md`](./realtime-wells-reactive-to-postgres-update.md)
