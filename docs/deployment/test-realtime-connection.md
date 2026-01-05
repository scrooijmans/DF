# Testing Supabase Realtime WebSocket Connection

## Prerequisites

After Dokploy redeploys with the updated Kong configuration, test the Realtime WebSocket connection.

## Quick HTTP Test

First, verify Kong can reach the Realtime service:

```bash
# Test Kong routing to Realtime
curl -v "http://91.99.166.223:8000/realtime/v1/" \
  -H "apikey: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlzcyI6InN1cGFiYXNlIiwiaWF0IjoxNzU5OTk3NDM0LCJleHAiOjE5MTc2Nzc0MzR9.z1kYdkPhGpmDpwStabAd47clRLHeEjHMszA_GUQ-rHk"

# Expected: Should connect to Realtime service (not "name resolution failed")
```

## Frontend Test

The frontend should automatically attempt to connect when:

1. The app launches
2. `PostgresWellsState` initializes
3. `RealtimeWellsService` calls `connect()`

### Check Browser Console

Look for these messages:

✅ **Success indicators:**

```
✅ [RealtimeWellsService] Connected to wells changes
🔄 [RealtimeWellsService] Subscription status: SUBSCRIBED
```

❌ **Failure indicators:**

```
❌ [RealtimeWellsService] Channel error
WebSocket connection failed: There was a bad response from the server
name resolution failed
```

## Kong Configuration Applied

The updated Kong config:

1. **Service URL: `http://mudrock-supabase-realtime:4000/socket`**
   - Uses container name for Docker network resolution
   - Includes `/socket` base path for Phoenix WebSocket

2. **Route Path**: `/realtime/v1/`
   - Strips prefix and routes remaining path
   - `/realtime/v1/websocket` → `/socket/websocket`

3. **Protocols**: `http`, `https`
   - Enables WebSocket upgrade support

4. **CORS Headers**: Includes `Upgrade` and `Connection`
   - Required for WebSocket handshake

## Node.js Test Script

If you have Node.js installed:

```bash
cd /Users/sc/Github/MudRock
npm install ws  # Install WebSocket library
node test-realtime-websocket.js
```

Expected output:

```
✅ WebSocket connection opened successfully!
📥 Received message: {...}
✅ Test completed successfully!
```

## Troubleshooting

### If "name resolution failed" persists:

1. Verify containers are on the same Docker network:

   ```bash
   docker network ls
   docker inspect mudrock-supabase-kong | grep NetworkMode
   docker inspect mudrock-supabase-realtime | grep NetworkMode
   ```

2. Check Kong logs:

   ```bash
   docker logs mudrock-supabase-kong --tail 50
   ```

3. Check Realtime logs:
   ```bash
   docker logs mudrock-supabase-realtime --tail 50
   ```

### If WebSocket upgrade fails:

1. Verify Kong loaded the new config:

   ```bash
   docker exec mudrock-supabase-kong kong config parse /var/lib/kong/kong.yml
   ```

2. Check if Realtime service is healthy:
   ```bash
   docker ps | grep realtime
   curl http://localhost:4001/api/tenants/health  # Direct access to Realtime
   ```

## Success Criteria

✅ Realtime connection is successful when:

- No "name resolution failed" errors
- WebSocket connection opens
- Subscription status is "SUBSCRIBED"
- No CHANNEL_ERROR in frontend console
- Wells data updates automatically when database changes
