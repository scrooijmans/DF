# Realtime WebSocket Connection Fix Summary

## Issue

WebSocket connections to Realtime are failing with `CHANNEL_ERROR` and "bad response from the server" errors.

**Error observed:**
```
WebSocket connection to 'ws://91.99.166.223:8000/realtime/v1/websocket?apikey=...' failed: 
There was a bad response from the server.
```

## Root Cause

The Kong API Gateway configuration was missing explicit WebSocket protocol support (`ws` and `wss`), which is required for WebSocket upgrade handling in Kong.

## Solution Applied

### 1. Updated Kong Configuration (`kong-simple.yml`)

**Changes made:**
- Added `ws` and `wss` to the `protocols` list for the Realtime service route
- Added WebSocket-specific headers to CORS configuration:
  - `Sec-WebSocket-Key`
  - `Sec-WebSocket-Version`
  - `Sec-WebSocket-Protocol`
  - `Sec-WebSocket-Extensions`

**Updated configuration:**
```yaml
- name: supabase-realtime
  url: http://supabase-realtime:4000/socket/
  routes:
    - name: supabase-realtime-all
      strip_path: true
      paths:
        - /realtime/v1/
      protocols:
        - http
        - https
        - ws    # ✅ Added
        - wss   # ✅ Added
      preserve_host: true
  plugins:
    - name: cors
      config:
        headers:
          - Upgrade
          - Connection
          - Sec-WebSocket-Key          # ✅ Added
          - Sec-WebSocket-Version      # ✅ Added
          - Sec-WebSocket-Protocol     # ✅ Added
          - Sec-WebSocket-Extensions   # ✅ Added
    - name: key-auth
      config:
        hide_credentials: false
        anonymous: anon
```

### 2. Supabase Client Configuration

**File:** `src/lib/supabase/client.ts`

**Note:** The Supabase client automatically handles WebSocket connections. The client converts `http://` URLs to `ws://` for Realtime connections automatically. No code changes were needed here - the issue was entirely in Kong's WebSocket protocol support.

The existing configuration is correct:
```typescript
supabaseInstance = createClient(supabaseUrl, supabaseAnonKey, {
  realtime: {
    params: {
      eventsPerSecond: 10,
    },
  },
  // ... rest of config
});
```

## Deployment Steps

### 1. Redeploy Kong Configuration in Dokploy

The `kong-simple.yml` file has been updated but needs to be redeployed:

1. **Access Dokploy Dashboard**: `http://91.99.166.223:3000`
2. **Navigate to Supabase project** → Configuration → Volumes/Files
3. **Update `kong-simple.yml`** with the new configuration (or upload the updated file)
4. **Redeploy the service** to apply Kong configuration changes

**OR** manually update on VPS:

```bash
# SSH into VPS
ssh mudrock-enterprise

# Update Kong configuration
cd /path/to/supabase/volumes
# Edit or replace kong-simple.yml with updated version
# Restart Kong service
docker restart mudrock-supabase-kong
```

### 2. Verify Realtime Service Status

```bash
# On VPS, check if Realtime service is running
docker ps | grep realtime

# Check Realtime logs for errors
docker logs mudrock-supabase-realtime --tail 50
```

### 3. Test WebSocket Connection

**After redeploying Kong:**

1. **Refresh the MudRock app** - Realtime connection should now work
2. **Check browser console** for success messages:
   ```
   ✅ [RealtimeWellsService] Connected to wells changes
   🔄 [RealtimeWellsService] Subscription status: SUBSCRIBED
   ```

## Expected Behavior After Fix

✅ **Success indicators:**
- WebSocket connection establishes successfully
- Subscription status shows `SUBSCRIBED`
- Realtime events are received when database changes occur
- No `CHANNEL_ERROR` messages

❌ **If still failing, check:**
- Kong service logs: `docker logs mudrock-supabase-kong --tail 50`
- Realtime service logs: `docker logs mudrock-supabase-realtime --tail 50`
- Realtime tenant configuration in database
- RLS policies on target tables (e.g., `wells`)

## Related Files

- **Kong Config**: `kong-simple.yml`
- **Supabase Client**: `src/lib/supabase/client.ts`
- **Realtime Service**: `src/lib/services/realtime-wells-service.ts`
- **Documentation**: `markdown-guides/supabase/realtime/realtime-wells-reactive-to-postgres-update.md`

## Notes

- **Important**: Kong configuration changes require a service restart or redeploy to take effect
- **Self-hosted Supabase**: WebSocket connections go through Kong gateway, unlike hosted Supabase which uses direct WebSocket endpoints
- **Protocol Support**: Kong must explicitly support `ws` and `wss` protocols for WebSocket upgrades to work correctly

---

**Status**: ✅ Configuration updated, awaiting Kong redeployment on VPS

