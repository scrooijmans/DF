# Supabase MCP Setup and Limitations

## Overview

This document summarizes our investigation, setup attempts, and findings regarding Model Context Protocol (MCP) integration with our self-hosted Supabase instance running on Hetzner VPS via Docker Compose.

## Setup Attempted

### Initial Goal

Enable Supabase MCP server to work with our self-hosted Docker Compose deployment, allowing AI agents (Cursor) to interact with our Supabase database.

### Configuration Steps Taken

1. **Kong API Gateway Configuration**
   - Added MCP endpoint route: `/mcp` → `http://mudrock-supabase-studio:3000/api/mcp`
   - Configured IP restriction plugin to allow:
     - `127.0.0.1` (localhost)
     - `::1` (IPv6 localhost)
     - `172.30.6.1` (Docker bridge gateway IP)
   - Enabled CORS plugin
   - File: `kong-simple.yml`

2. **Docker Compose Configuration**
   - Fixed Studio healthcheck to use port 3000 (was incorrectly using 9999)
   - File: `docker-compose-supabase.yml`

3. **SSH Tunnel Setup**
   - Created tunnel script: `scripts/infrastructure/start-mcp-tunnel.sh` (to be created)
   - Forwarding: `localhost:8080` → VPS `localhost:8000` (Kong gateway)

4. **MCP Client Configuration**
   - Configured `~/.cursor/mcp.json`:
     ```json
     {
       "mcpServers": {
         "supabase-self-hosted": {
           "url": "http://localhost:8080/mcp"
         }
       }
     }
     ```

## Current Status

### ✅ What's Working

1. **Kong Configuration**: Correctly configured and deployed
   - MCP route is active
   - IP restrictions properly set
   - CORS enabled

2. **Network Connectivity**:
   - Kong successfully forwards requests to Studio
   - SSH tunnel configuration is correct
   - Docker bridge networking functional

3. **Port Configuration**: Fixed mismatches
   - Studio healthcheck: port 3000 ✅
   - Kong MCP route: port 3000 ✅

### ❌ What's NOT Working

**Studio MCP Endpoint Returns 404**

- Kong successfully forwards `/mcp` requests to Studio
- Studio responds with HTTP 404 (Not Found)
- This indicates Studio doesn't expose the `/api/mcp` endpoint

**Error Response:**

```
HTTP 404 - Studio 404 page HTML
```

## Root Cause Analysis

### Why Supabase MCP Doesn't Work

The official Supabase documentation states:

> "We're adding official MCP support for **local Supabase instances created through the CLI**"

**Key Finding**: Supabase MCP remote transport is only available for:

1. **CLI-Managed Local Instances**
   - Setup: `supabase start`
   - MCP endpoint: `http://localhost:54321/mcp`
   - Uses unified Supabase CLI gateway

2. **Hosted Supabase**
   - MCP endpoint: `https://mcp.supabase.com/mcp`
   - Requires Supabase cloud account

3. **NOT Available For**:
   - ❌ Standalone Docker Compose deployments
   - ❌ Custom Docker setups (like ours)

### Evidence

1. **Studio Container Investigation**
   - Searched Studio container: No MCP-related files found
   - Studio API routes exist (`/api/platform/*`, `/api/ai/*`)
   - `/api/mcp` route does not exist

2. **Official Documentation**
   - `remote-mcp.md`: Mentions only CLI (`localhost:54321`) or hosted
   - `self-hosting-mcp.md`: Assumes CLI-managed setup
   - No documentation for Docker Compose MCP setup

3. **Testing Results**
   - `/api/platform/profile`: Returns "Unauthorized" (endpoint exists)
   - `/api/mcp`: Returns 404 (endpoint doesn't exist)

## Limitations

### Supabase MCP Limitations

1. **Deployment Method Restriction**
   - Only works with CLI-managed instances or hosted Supabase
   - Does not support standalone Docker Compose

2. **Studio Version Limitation**
   - Current version: `supabase/studio:2025.04.21-sha-173cc56`
   - This version doesn't expose MCP endpoint in Docker Compose
   - Even with correct configuration, endpoint doesn't exist

3. **Architecture Difference**
   - CLI-managed: Uses unified gateway on port 54321
   - Docker Compose: Separate services, no unified gateway
   - MCP endpoint appears to be CLI-specific feature

### Configuration Limitations

1. **Port Confusion**
   - Studio runs on multiple ports (3000 for web, potentially others for API)
   - Healthcheck originally used wrong port (9999)
   - Fixed to use port 3000 as per official docs

2. **IP Restriction Complexity**
   - Required Docker bridge gateway IP (`172.30.6.1`)
   - Must be updated if Docker network changes
   - SSH tunnel adds complexity

## Alternative Solution: PostgREST MCP

Since Supabase MCP isn't available for Docker Compose, **PostgREST MCP works perfectly**:

### Why PostgREST MCP Works

- ✅ Direct database connection (doesn't need Supabase platform)
- ✅ Works with any PostgreSQL + PostgREST setup
- ✅ Simpler authentication
- ✅ Already functional in our setup

### PostgREST MCP Configuration

```json
{
  "mcpServers": {
    "postgrest-mcp": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-postgres",
        "postgresql://postgres:PASSWORD@localhost:5432/postgres"
      ]
    }
  }
}
```

**Note**: Requires SSH tunnel for database access or PostgreSQL port exposure.

## Files Modified

1. **`kong-simple.yml`**
   - Added MCP service route
   - Configured IP restrictions
   - Port: 3000

2. **`docker-compose-supabase.yml`**
   - Fixed Studio healthcheck port (9999 → 3000)

3. **`~/.cursor/mcp.json`**
   - Added Supabase MCP server configuration (currently non-functional)

## Deployment Status

- ✅ Kong configuration deployed
- ✅ Docker Compose changes deployed
- ✅ IP restrictions active
- ✅ Port configuration fixed
- ❌ Studio MCP endpoint not available

## Recommendations

### Option 1: Use PostgREST MCP (Recommended)

**Pros:**

- ✅ Works with current setup
- ✅ Simpler configuration
- ✅ Direct database access
- ✅ No platform dependencies

**Cons:**

- ❌ No Supabase-specific features (doc search, project management)
- ❌ Requires database connection string
- ❌ No Edge Functions deployment

### Option 2: Switch to Supabase CLI

If you need full Supabase MCP features:

- Run `supabase start` instead of Docker Compose
- Access MCP at `http://localhost:54321/mcp`
- **Downside**: Can't use current Docker Compose setup

### Option 3: Wait for Docker Compose Support

Supabase is working on better self-hosted MCP support, but it's not available yet for standalone Docker Compose deployments.

## Conclusion

**Current Status**: Supabase MCP is **not available** for our Docker Compose deployment. All configuration is correct, but Studio doesn't expose the MCP endpoint in this deployment method.

**Best Path Forward**: Use **PostgREST MCP** which provides the core database interaction capabilities and works perfectly with our current setup.

## References

- [Supabase Remote MCP Documentation](/docs/supabase/mcp/remote-mcp.md)
- [Supabase Self-Hosting MCP Guide](/docs/supabase/mcp/self-hosting-mcp.md)
- [Why PostgREST vs Supabase MCP](/docs/supabase/mcp/why-postgrest-vs-supabase-mcp.md)
- [MCP Findings](/docs/supabase/mcp/mcp-findings.md)

## Timeline

- **Initial Setup**: Attempted to configure Supabase MCP for Docker Compose
- **Configuration**: Added Kong routes, IP restrictions, SSH tunnel
- **Port Fixes**: Fixed Studio healthcheck and Kong route port mismatches
- **Investigation**: Discovered Studio doesn't expose MCP endpoint
- **Conclusion**: Supabase MCP not available for Docker Compose deployments

---

**Last Updated**: October 31, 2025  
**Status**: Configuration complete, but endpoint unavailable due to deployment method limitation
