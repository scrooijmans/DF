# MCP Setup Cleanup Summary

## Changes Removed

### ❌ Removed: Non-Functional MCP Route
**File**: `kong-simple.yml`
- **What**: Removed MCP endpoint service configuration (lines 118-139)
- **Why**: MCP endpoint doesn't exist in Studio for Docker Compose deployments
- **Impact**: No functional change - route was returning 404 anyway

## Changes Kept

### ✅ Kept: Studio Healthcheck Port Fix
**File**: `docker-compose-supabase.yml`
- **What**: Healthcheck uses port 3000 (was incorrectly using 9999)
- **Why**: Studio actually runs on port 3000 internally - this is the correct configuration
- **Impact**: Healthcheck now accurately monitors Studio health

**Before**:
```yaml
'node -e "fetch(''http://localhost:9999/api/platform/profile'')...'
```

**After**:
```yaml
'node -e "fetch(''http://localhost:3000/api/platform/profile'')...'
```

## Rationale

1. **MCP Route**: Non-functional, just adds clutter. Removing it keeps configuration clean.

2. **Healthcheck Fix**: This was actually a bug fix, not MCP-specific. Studio runs on port 3000, so the healthcheck should check port 3000. This improves reliability regardless of MCP.

## Net Result

- Cleaner Kong configuration (no unused routes)
- Correct Studio healthcheck (monitoring actual port)
- No functional impact (MCP wasn't working anyway)

