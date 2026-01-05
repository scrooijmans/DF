# Supabase Realtime Integration Issues & Solutions

This document catalogs all issues encountered during the Supabase Realtime integration for the MudRock application and how they were resolved.

## Overview

The goal was to integrate Supabase Realtime to automatically sync Postgres database changes to the frontend global state, specifically for the `wells` table. This allows the UI to update in real-time when new wells are created during LAS, trajectory, or checkshot uploads.

## Issue 1: Realtime Service Restart Loop

### Problem

The Realtime service was constantly restarting and couldn't stay up, preventing any WebSocket connections.

### Error Messages

```
/app/run.sh: line 6: RLIMIT_NOFILE: unbound variable
ERROR 3F000 (invalid_schema_name) no schema has been selected to create in
```

### Root Cause

1. Missing environment variable `RLIMIT_NOFILE` required for Realtime initialization
2. Missing `_realtime` schema in the database
3. Missing `SEED_SELF_HOST` flag for self-hosted deployments

### Solution

**Step 1: Add Required Environment Variables**

Updated `docker-compose-supabase.yml`:

```yaml
supabase-realtime:
  environment:
    RLIMIT_NOFILE: "10000" # Required for Realtime startup
    SEED_SELF_HOST: "true" # Required for self-hosted deployments
    DNS_NODES: "''" # Use quoted empty string format
    DB_AFTER_CONNECT_QUERY: SET search_path TO _realtime
```

**Step 2: Create \_realtime Schema**

Executed SQL in database:

```sql
CREATE SCHEMA IF NOT EXISTS _realtime;
GRANT ALL ON SCHEMA _realtime TO supabase_admin;
```

### Result

✅ Realtime service now starts successfully and remains running

---

## Issue 2: TenantNotFound Error

### Problem

Realtime was looking for a tenant with `external_id = 'supabase-realtime'` but only `'realtime-dev'` existed in the database.

### Error Messages

```
project=supabase-realtime external_id=supabase-realtime error_code=TenantNotFound
[error] TenantNotFound: Tenant not found: supabase-realtime
```

### Root Cause

- The Supabase client library expects the tenant `external_id` to match the service name when connecting through Kong
- The default tenant created by Realtime has `external_id = 'realtime-dev'`
- We needed `external_id = 'supabase-realtime'` to match what the client expects

### Solution

**Created SQL Script: `sql/fix_realtime_complete.sql`**

This script:

1. Creates a new tenant with `external_id = 'supabase-realtime'`
2. Copies the extension configuration from `realtime-dev` to `supabase-realtime`
3. Enables RLS on the `wells` table
4. Creates RLS policies for `anon` and `authenticated` roles

**Key SQL Commands:**

```sql
-- Create tenant
INSERT INTO _realtime.tenants (id, name, external_id, jwt_secret, ...)
SELECT gen_random_uuid(), 'supabase-realtime', 'supabase-realtime', jwt_secret, ...
FROM _realtime.tenants WHERE external_id = 'realtime-dev';

-- Copy extension
INSERT INTO _realtime.extensions (id, type, settings, tenant_external_id, ...)
SELECT gen_random_uuid(), type, settings, 'supabase-realtime', ...
FROM _realtime.extensions WHERE tenant_external_id = 'realtime-dev';
```

### Result

✅ Tenant `supabase-realtime` now exists and Realtime can find it

---

## Issue 3: Wrong Database Schema Path

### Problem

Realtime was still showing "TenantNotFound" errors even after creating the tenant.

### Error Messages

```
TenantNotFound: Tenant not found: supabase-realtime
```

### Root Cause

- `DB_AFTER_CONNECT_QUERY` was set to `SET search_path TO realtime`
- But the tenant table is in `_realtime` schema, not `realtime`
- Realtime couldn't find the tenant because it was looking in the wrong schema

### Solution

**Updated `docker-compose-supabase.yml`:**

```yaml
supabase-realtime:
  environment:
    DB_AFTER_CONNECT_QUERY: SET search_path TO _realtime # Changed from 'realtime'
```

### Result

✅ Realtime now correctly looks in `_realtime` schema and finds the tenant

---

## Issue 4: Kong 403 Forbidden for WebSocket Upgrades

### Problem

Kong was returning `403 Forbidden` for all WebSocket upgrade requests to Realtime.

### Error Messages (Browser Console)

```
WebSocket connection to 'ws://91.99.166.223:8000/realtime/v1/websocket?...' failed: There was a bad response from the server.
[Log] 🔄 [RealtimeWellsService] Subscription status: – "CHANNEL_ERROR"
```

### Error Messages (Kong Logs)

```
178.26.68.201 - - [timestamp] "GET /realtime/v1/websocket?... HTTP/1.1" 403 0
```

### Root Cause

1. Kong's `key-auth` plugin was configured for Realtime service
2. The plugin was rejecting requests because `anonymous` access wasn't configured
3. WebSocket upgrade requests need special handling in Kong

### Solution

**Updated `kong-simple.yml`:**

```yaml
services:
  - name: supabase-realtime
    plugins:
      - name: key-auth
        config:
          hide_credentials: false
          anonymous: anon # ← Added this to allow unauthenticated requests
```

**Also registered the anon key as a consumer:**

```yaml
consumers:
  - username: anon
    keyauth_credentials:
      - key: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### Result

✅ Kong now allows WebSocket connections through (no more 403 errors)

---

## Issue 5: Missing RLS Policies for Postgres Changes

### Problem

According to `postgres-changes.md`, RLS must be enabled and policies must allow `anon` and `authenticated` roles to read data. Otherwise Realtime cannot verify permissions when checking Postgres Changes.

### Error Messages

Potentially silent failures where Realtime connects but doesn't receive change events.

### Root Cause

- RLS was enabled but only SELECT policies existed (created for Realtime read access)
- Postgres Changes requires RLS policies to verify permissions for each change event

### Solution

**Created SQL script with RLS policies:**

```sql
-- Enable RLS
ALTER TABLE public.wells ENABLE ROW LEVEL SECURITY;

-- Create SELECT policies (for Realtime to verify permissions)
CREATE POLICY "Allow anon access to wells for Realtime"
    ON public.wells FOR SELECT TO anon USING (true);

CREATE POLICY "Allow authenticated access to wells"
    ON public.wells FOR SELECT TO authenticated USING (true);

-- Create INSERT policies (for well creation)
CREATE POLICY "Allow anon insert to wells"
    ON public.wells FOR INSERT TO anon WITH CHECK (true);

CREATE POLICY "Allow authenticated insert to wells"
    ON public.wells FOR INSERT TO authenticated WITH CHECK (true);
```

### Result

✅ RLS policies now allow both SELECT (for Realtime) and INSERT (for well creation)

---

## Issue 6: Missing INSERT Policies Blocking Well Creation

### Problem

After enabling RLS for Realtime, well creation stopped working. Backend was getting errors when trying to INSERT new wells.

### Error Messages (from backend logs)

```
⚠️ [LasDataProcessor] Failed to create well in database: ...
⚠️ [TrajectoryCommand] Failed to create well in database: ...
```

### Root Cause

- RLS policies only included SELECT policies (for Realtime to read)
- No INSERT policies existed, so `WellCreator.create_well()` was being blocked by RLS
- The `anon` role (used by PostgREST) couldn't INSERT into `wells` table

### Solution

**Added INSERT RLS policies:**

```sql
CREATE POLICY "Allow anon insert to wells"
    ON public.wells FOR INSERT TO anon WITH CHECK (true);

CREATE POLICY "Allow authenticated insert to wells"
    ON public.wells FOR INSERT TO authenticated WITH CHECK (true);
```

### Result

✅ Well creation now works during LAS, trajectory, and checkshot uploads

---

## Issue 7: Realtime Not Receiving Events (Connection Timing)

### Problem

Initially Realtime showed "Connection refused" errors shortly after restart, even though the service was running.

### Error Messages

```
connect() failed (111: Connection refused) while connecting to upstream
```

### Root Cause

- Temporary timing issue - Realtime container was still initializing when Kong tried to connect
- This was transient and resolved after a few seconds

### Solution

- No code change needed
- Service now starts successfully and Kong can connect once Realtime is fully initialized (~10-15 seconds)

### Result

✅ Connection works reliably after service initialization

---

## Summary of All Fixes Applied

### Database Configuration

**Files**: `sql/enable_realtime_wells.sql`, `sql/fix_realtime_complete.sql`

1. **Created `_realtime` schema** for Realtime tenant management:

   ```sql
   CREATE SCHEMA IF NOT EXISTS _realtime;
   GRANT ALL ON SCHEMA _realtime TO supabase_admin;
   ```

2. **Created tenant** with `external_id = 'supabase-realtime'`:

   ```sql
   INSERT INTO _realtime.tenants (id, name, external_id, jwt_secret, ...)
   SELECT gen_random_uuid(), 'supabase-realtime', 'supabase-realtime', jwt_secret, ...
   FROM _realtime.tenants WHERE external_id = 'realtime-dev';
   ```

3. **Copied extension** configuration for Postgres Changes:

   ```sql
   INSERT INTO _realtime.extensions (id, type, settings, tenant_external_id, ...)
   SELECT gen_random_uuid(), type, settings, 'supabase-realtime', ...
   FROM _realtime.extensions WHERE tenant_external_id = 'realtime-dev';
   ```

4. **Added wells to publication**:

   ```sql
   ALTER PUBLICATION supabase_realtime ADD TABLE public.wells;
   ```

5. **Enabled RLS** on `wells` table:

   ```sql
   ALTER TABLE public.wells ENABLE ROW LEVEL SECURITY;
   ```

6. **Created RLS policies**:
   - SELECT policies for `anon` and `authenticated` (for Realtime permission checks)
   - INSERT policies for `anon` and `authenticated` (for well creation during data ingestion)

### Docker Configuration

**File**: `docker-compose-supabase.yml`

1. **Added `RLIMIT_NOFILE: "10000"`** to Realtime service:

   ```yaml
   supabase-realtime:
     environment:
       RLIMIT_NOFILE: "10000" # Required for Erlang VM file descriptor limit
   ```

   - **Purpose**: Erlang/Elixir applications require this for socket connections
   - **Error if missing**: `/app/run.sh: line 6: RLIMIT_NOFILE: unbound variable`

2. **Added `SEED_SELF_HOST: "true"`** for self-hosted deployment:

   ```yaml
   SEED_SELF_HOST: "true" # Required for self-hosted Supabase deployments
   ```

   - **Purpose**: Tells Realtime to initialize in self-hosted mode
   - **Error if missing**: Realtime may not initialize tenant configuration correctly

3. **Fixed `DB_AFTER_CONNECT_QUERY`**: Changed from `realtime` → `_realtime`:

   ```yaml
   DB_AFTER_CONNECT_QUERY: SET search_path TO _realtime
   ```

   - **Purpose**: Sets PostgreSQL search path to `_realtime` schema where tenant table exists
   - **Error if wrong**: `TenantNotFound: Tenant not found: supabase-realtime`
   - **Critical**: Must be `_realtime` (underscore), not `realtime`

4. **Updated `DNS_NODES`**: Use quoted empty string format `"''"`:

   ```yaml
   DNS_NODES: "''" # Quoted empty string for self-hosted deployments
   ```

   - **Purpose**: Disables external DNS node discovery for self-hosted setup

5. **Service Configuration**:
   ```yaml
   supabase-realtime:
     image: supabase/realtime:v2.32.12
     container_name: mudrock-supabase-realtime
     ports:
       - "4001:4000" # External:Internal
     depends_on:
       - postgres
     networks:
       - mudrock-network
   ```

### Kong Configuration

**File**: `kong-simple.yml`

1. **Added `key-auth` plugin** for Realtime service:

   ```yaml
   services:
     - name: supabase-realtime
       plugins:
         - name: key-auth
           config:
             hide_credentials: false
   ```

   - **Purpose**: Validates API keys for Realtime WebSocket connections
   - **Required by**: Supabase's self-hosting guidelines

2. **Added `anonymous: anon`** to key-auth config:

   ```yaml
   config:
     hide_credentials: false
     anonymous: anon # ← CRITICAL: Allows unauthenticated requests with anon key
   ```

   - **Purpose**: Allows requests using the anonymous API key without explicit authentication
   - **Error if missing**: Kong returns `403 Forbidden` for WebSocket upgrade requests
   - **Key Point**: The `anon` consumer must exist and be registered

3. **Registered anon consumer** with the anon API key:

   ```yaml
   consumers:
     - username: anon
       keyauth_credentials:
         - key: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9... # Anon JWT token
   ```

   - **Purpose**: Maps the anonymous API key to a consumer that Kong recognizes
   - **JWT Token**: Generated by Supabase with `role: anon` claim

4. **Added trailing slash** to Realtime URL (`/socket/`):

   ```yaml
   url: http://supabase-realtime:4000/socket/ # Trailing slash required
   ```

   - **Purpose**: Ensures correct WebSocket routing
   - **Service Resolution**: Uses Docker service name `supabase-realtime`

5. **Set `preserve_host: true`**:

   ```yaml
   routes:
     - name: supabase-realtime-route
       preserve_host: true # Required for WebSocket upgrades
   ```

   - **Purpose**: Preserves the original Host header for WebSocket connections

6. **Complete Kong Service Configuration**:

   ```yaml
   services:
     - name: supabase-realtime
       url: http://supabase-realtime:4000/socket/
       routes:
         - name: supabase-realtime-route
           paths:
             - /realtime/v1/websocket
           strip_path: false
           preserve_host: true
       plugins:
         - name: key-auth
           config:
             hide_credentials: false
             anonymous: anon

   consumers:
     - username: anon
       keyauth_credentials:
         - key: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9... # Your anon JWT
   ```

### Frontend Configuration

**Files**:

- `src/lib/supabase/client.ts`
- `src/lib/services/realtime-wells-service.ts`
- `src/lib/state/postgres/postgres-wells-state.svelte.ts`
- `src/routes/home/+layout.svelte`

1. **Created singleton Supabase client** with proper initialization:

   ```typescript
   // Prevents multiple GoTrueClient instances
   let supabaseClient: SupabaseClient | null = null;
   let isInitialized = false;

   export function getSupabaseClient(): SupabaseClient {
     if (isInitialized && supabaseClient) {
       return supabaseClient;
     }
     // ... create client
     isInitialized = true;
     return supabaseClient;
   }
   ```

   - **Purpose**: Ensures only one Supabase client instance exists
   - **Error if not done**: "Multiple GoTrueClient instances detected"

2. **Implemented RealtimeWellsService** for managing subscriptions:

   ```typescript
   class RealtimeWellsService {
     private channel: RealtimeChannel | null = null;

     async connect() {
       this.channel = this.client
         .channel('wells-changes')
         .on('postgres_changes', {...}, this.handleChange)
         .subscribe();
     }
   }
   ```

   - **Purpose**: Manages WebSocket connection and handles database change events
   - **Pattern**: Singleton pattern prevents multiple instances

3. **Integrated with PostgresWellsState** global state:

   ```typescript
   export class PostgresWellsState {
     wells = $state<Well[]>([]);

     setWells(newWells: Well[]) {
       // Merge instead of replace to preserve realtime updates
       const merged = new Map();
       this.wells.forEach((w) => merged.set(w.id, w));
       newWells.forEach((w) => merged.set(w.id, w));
       this.wells = Array.from(merged.values());
     }
   }
   ```

   - **Purpose**: Provides reactive global state for wells across the application
   - **Key Feature**: Merging preserves realtime updates when database is refetched

4. **Added connection status monitoring**:

   ```typescript
   getRealtimeStatus(): boolean {
     return this.realtimeService?.isConnected ?? false;
   }
   ```

   - **Purpose**: Allows UI components to display connection status (🟢 Live / 🔴 Offline)

5. **Fixed lifecycle issues** with deferred initialization:

   ```typescript
   constructor() {
     requestAnimationFrame(() => {
       setTimeout(() => {
         this.realtimeService = realtimeWellsService;
         this.realtimeService.connect();
       }, 100);
     });
   }
   ```

   - **Purpose**: Prevents "Cannot access uninitialized variable" errors
   - **Error if not done**: `TypeError: render_fn is not a function`

6. **Added cleanup in layout**:

   ```typescript
   // src/routes/home/+layout.svelte
   import { onDestroy } from "svelte";

   onDestroy(() => {
     wellsState.disconnectRealtime();
   });
   ```

   - **Purpose**: Properly disconnects Realtime service on component destruction
   - **Prevents**: Memory leaks and resource conflicts

---

## Current Status

### ✅ **Working**

- Realtime service running and stable
- Tenant configuration correct
- Kong routing WebSocket connections
- Frontend successfully connecting and subscribing
- RLS policies allowing SELECT and INSERT operations
- Publication enabled for `wells` table

### ⚠️ **Known Limitations**

- Realtime events may not be received during initial connection (need to wait for subscription)
- Connection recovery after network interruption needs testing
- Browser background tab suspension may cause connection drops (expected behavior per Supabase docs)

### 🔄 **Next Steps**

1. Test well creation during data ingestion to verify Realtime events are received
2. Monitor connection stability over time
3. Consider adding reconnection logic for dropped connections
4. Test with multiple concurrent users

---

## Reference Files

- **SQL Fix Script**: `sql/fix_realtime_complete.sql`
- **Realtime Service**: `src/lib/services/realtime-wells-service.ts`
- **Global State**: `src/lib/state/postgres/postgres-wells-state.svelte.ts`
- **Supabase Client**: `src/lib/supabase/client.ts`
- **Docker Compose**: `docker-compose-supabase.yml`
- **Kong Config**: `kong-simple.yml`
- **Realtime Guide**: `markdown-guides/to-do/realtime-wells-reactive-to-postgres-update.md`
