# DataForge Sync Server Testing Guide

This guide explains how to set up and run a local sync server for testing DataForge's synchronization features on macOS.

## Overview

DataForge includes a built-in **test sync server** that you can run locally. This server:
- Stores data in-memory (no persistence between restarts)
- Runs on `http://localhost:3000` by default
- Provides all the endpoints needed for testing sync functionality
- No Docker required - just Rust/Cargo

## Prerequisites

Before running the sync server, ensure you have:

1. **Rust toolchain** installed (includes Cargo)
   ```bash
   # Check if Rust is installed
   rustc --version
   cargo --version

   # If not installed, install via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **DataForge repository** cloned
   ```bash
   cd /path/to/DataForge
   ```

## Quick Start

### Step 1: Start the Test Server

Open a terminal and run:

```bash
# From the DataForge project root
cargo run --bin dataforge-test-server
```

You should see output like:

```
Starting DataForge test server on http://0.0.0.0:3000
Press Ctrl+C to stop

Endpoints:
  GET  /api/health       - Health check
  POST /api/sync/push    - Push changes
  POST /api/sync/pull    - Pull changes
  POST /api/blobs/urls   - Get blob URLs
  GET  /api/blobs/:hash  - Download blob
  PUT  /api/blobs/:hash  - Upload blob
```

### Step 2: Configure DataForge to Use the Test Server

1. Open DataForge application
2. Navigate to **Settings > Sync**
3. Enter the server URL: `http://localhost:3000`
4. Click **Test Connection**

If successful, you'll see: "Connection successful! Server version: X.X.X"

### Step 3: Test Sync Operations

1. **Create some data** in DataForge (import a LAS file, create a well)
2. Click **Sync Now** in the sync settings page
3. Watch the server terminal for sync activity logs

## Server Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/health` | GET | Health check - returns "OK" |
| `/api/sync/push` | POST | Push local changes to server |
| `/api/sync/pull` | POST | Pull remote changes from server |
| `/api/blobs/urls` | POST | Get upload/download URLs for blobs |
| `/api/blobs/:hash` | GET | Download a blob by hash |
| `/api/blobs/:hash` | PUT | Upload a blob by hash |

## Manual Testing with curl

You can test the server directly:

```bash
# Health check
curl http://localhost:3000/api/health
# Expected: OK

# Push changes (example)
curl -X POST http://localhost:3000/api/sync/push \
  -H "Content-Type: application/json" \
  -d '{
    "workspace_id": "test-workspace",
    "from_version": 0,
    "changes": [],
    "pending_blobs": []
  }'

# Pull changes (example)
curl -X POST http://localhost:3000/api/sync/pull \
  -H "Content-Type: application/json" \
  -d '{
    "workspace_id": "test-workspace",
    "from_version": 0,
    "limit": 100
  }'
```

## Troubleshooting

### "Connection failed: Network error"

This error means the DataForge app cannot reach the sync server.

**Causes & Solutions:**

1. **Server not running**
   - Ensure `cargo run --bin dataforge-test-server` is running in another terminal
   - Check for compilation errors in the terminal output

2. **Wrong URL**
   - Verify the URL is exactly `http://localhost:3000` (not `https`)
   - Don't add trailing slashes

3. **Port already in use**
   ```bash
   # Check if port 3000 is in use
   lsof -i :3000

   # Kill process using port 3000 (if needed)
   lsof -ti:3000 | xargs kill -9
   ```

4. **Firewall blocking**
   - macOS shouldn't block localhost, but check System Settings > Privacy & Security > Firewall

### "Failed to compile test server"

If cargo fails to build:

```bash
# Clean and rebuild
cargo clean
cargo build --bin dataforge-test-server

# If dependency issues, try updating
cargo update
```

### Server crashes on startup

Check for port conflicts:

```bash
# Use a different port
PORT=3001 cargo run --bin dataforge-test-server
# Then update DataForge settings to use http://localhost:3001
```

## Advanced: Running with Docker (Optional)

While the test server doesn't require Docker, you can containerize it for team testing:

### Dockerfile

Create `Dockerfile.sync-server` in the project root:

```dockerfile
# Build stage
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

RUN cargo build --release --bin dataforge-test-server

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/dataforge-test-server /usr/local/bin/

EXPOSE 3000

CMD ["dataforge-test-server"]
```

### Build and Run

```bash
# Build the Docker image
docker build -f Dockerfile.sync-server -t dataforge-sync-server .

# Run the container
docker run -p 3000:3000 dataforge-sync-server
```

### Docker Compose (for persistent testing)

Create `docker-compose.sync.yml`:

```yaml
version: '3.8'

services:
  sync-server:
    build:
      context: .
      dockerfile: Dockerfile.sync-server
    ports:
      - "3000:3000"
    restart: unless-stopped
    environment:
      - RUST_LOG=info
```

Run with:
```bash
docker-compose -f docker-compose.sync.yml up
```

## Server Architecture

The test server is an in-memory implementation for development. It:

- **Does NOT persist data** - All data is lost when the server stops
- **Does NOT authenticate** - No auth headers required
- **Stores changes in memory** - Using a simple vector
- **Stores blobs in a HashMap** - Key is SHA-256 hash

For production, you would deploy a proper server with:
- Database backend (PostgreSQL/SQLite)
- S3/MinIO for blob storage
- Authentication middleware
- HTTPS/TLS

## File Locations

| File | Purpose |
|------|---------|
| `crates/dataforge-sync/src/bin/test_server.rs` | Test server implementation |
| `crates/dataforge-sync/src/protocol.rs` | Sync protocol types |
| `crates/dataforge-sync/src/client.rs` | HTTP client for sync |
| `src/lib/stores/sync.svelte.ts` | Frontend sync store |
| `src-tauri/src/sync_commands.rs` | Tauri sync commands |

## Next Steps

After successfully testing locally:

1. **Test with multiple clients** - Run DataForge on another machine pointing to the same server
2. **Test offline scenarios** - Stop the server, make changes, restart, sync
3. **Test conflict resolution** - Make conflicting changes on two clients
4. **Review sync architecture** - See `markdown-guides/DataForge/sync/DF-sync-architecture.md`

## Summary

| Step | Command/Action |
|------|---------------|
| 1. Start server | `cargo run --bin dataforge-test-server` |
| 2. Verify running | `curl http://localhost:3000/api/health` |
| 3. Configure app | Settings > Sync > Enter `http://localhost:3000` |
| 4. Test connection | Click "Test Connection" button |
| 5. Sync data | Click "Sync Now" |

The test server is perfect for local development and testing. For production sync, you'll need to deploy a proper backend server with persistence and authentication.
