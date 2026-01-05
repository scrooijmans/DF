# Database & Storage Inspection Tools for MVP

This document outlines tools for viewing, inspecting, and debugging databases, storage files, and application state when following ColaNode's architecture for the MudRock MVP.

## Overview

Based on ColaNode's tech stack, you'll need tools for:

1. **SQLite** (local client database)
2. **PostgreSQL** (server database, if used)
3. **Parquet files** (content-addressed storage)
4. **Yjs documents** (if collaboration is added)
5. **Object storage** (S3/MinIO/Filesystem)
6. **Application state** (debugging)

## SQLite Inspection Tools

### Desktop GUI Applications

#### 1. **DB Browser for SQLite** (Recommended)

**Platform**: Windows, macOS, Linux

**Features**:
- ✅ Free and open-source
- ✅ Visual table browser
- ✅ SQL query editor with syntax highlighting
- ✅ Export data to CSV, JSON, SQL
- ✅ Edit data directly in tables
- ✅ View database structure (schema)
- ✅ Execute SQL queries
- ✅ Browse indexes and triggers

**Installation**:
```bash
# macOS
brew install --cask db-browser-for-sqlite

# Windows
# Download from: https://sqlitebrowser.org/

# Linux
sudo apt install sqlitebrowser
```

**Usage**:
```
1. Open DB Browser for SQLite
2. File → Open Database → Select mudrock.db
3. Browse Data tab: View tables and data
4. Execute SQL tab: Run queries
5. Database Structure tab: View schema
```

#### 2. **TablePlus** (Premium)

**Platform**: Windows, macOS, Linux

**Features**:
- ✅ Beautiful UI
- ✅ Supports SQLite, PostgreSQL, MySQL, Redis, etc.
- ✅ Multiple database connections
- ✅ Query history
- ✅ Data export/import
- ✅ ⚠️ Paid (free tier available)

**Installation**:
```bash
# macOS
brew install --cask tableplus

# Download from: https://tableplus.com/
```

#### 3. **DBeaver** (Free, Open Source)

**Platform**: Windows, macOS, Linux

**Features**:
- ✅ Free and open-source
- ✅ Universal database tool (SQLite, PostgreSQL, MySQL, etc.)
- ✅ ER diagram generation
- ✅ SQL editor with autocomplete
- ✅ Data export/import
- ✅ ⚠️ Java-based (heavier)

**Installation**:
```bash
# macOS
brew install --cask dbeaver-community

# Download from: https://dbeaver.io/
```

### Command-Line Tools

#### 1. **sqlite3** (Built-in)

**Platform**: All platforms (comes with SQLite)

**Features**:
- ✅ Always available (bundled with SQLite)
- ✅ Interactive shell
- ✅ Execute SQL queries
- ✅ Export data
- ✅ ⚠️ Text-only interface

**Usage**:
```bash
# Open database
sqlite3 mudrock.db

# Execute queries
sqlite3 mudrock.db "SELECT * FROM wells LIMIT 10;"

# Export to CSV
sqlite3 mudrock.db ".mode csv" ".output wells.csv" "SELECT * FROM wells;"

# View schema
sqlite3 mudrock.db ".schema"

# List tables
sqlite3 mudrock.db ".tables"

# Interactive mode
sqlite3 mudrock.db
> .tables
> SELECT * FROM wells;
> .quit
```

#### 2. **sqlite-utils** (Python CLI)

**Platform**: All platforms (Python)

**Features**:
- ✅ Powerful CLI for SQLite operations
- ✅ JSON import/export
- ✅ Data transformation
- ✅ CSV import/export

**Installation**:
```bash
pip install sqlite-utils
```

**Usage**:
```bash
# View tables
sqlite-utils tables mudrock.db

# Query data
sqlite-utils mudrock.db "SELECT * FROM wells LIMIT 10"

# Export to JSON
sqlite-utils mudrock.db wells --json > wells.json

# Import from JSON
sqlite-utils insert mudrock.db wells data.json

# View schema
sqlite-utils schema mudrock.db
```

### VS Code Extensions

#### 1. **SQLite Viewer**

**Extension ID**: `qwtel.sqlite-viewer`

**Features**:
- ✅ View SQLite databases in VS Code
- ✅ Browse tables
- ✅ Execute queries
- ✅ Export data

**Usage**:
```
1. Install extension
2. Open .db file in VS Code
3. Click "Open Database" button
4. Browse tables and run queries
```

#### 2. **SQLite**

**Extension ID**: `alexcvzz.vscode-sqlite`

**Features**:
- ✅ SQLite database explorer
- ✅ Query runner
- ✅ Table viewer

## PostgreSQL Inspection Tools

### Desktop GUI Applications

#### 1. **pgAdmin** (Official)

**Platform**: Windows, macOS, Linux

**Features**:
- ✅ Official PostgreSQL tool
- ✅ Full-featured database management
- ✅ Query tool
- ✅ ER diagram generation
- ✅ ⚠️ Heavy (web-based interface)

**Installation**:
```bash
# macOS
brew install --cask pgadmin4

# Download from: https://www.pgadmin.org/
```

#### 2. **TablePlus** (Also supports PostgreSQL)

**Features**:
- ✅ Same tool as SQLite
- ✅ Unified interface for multiple databases
- ✅ Beautiful UI

#### 3. **DBeaver** (Also supports PostgreSQL)

**Features**:
- ✅ Universal database tool
- ✅ Good for multi-database environments

### Command-Line Tools

#### 1. **psql** (Built-in)

**Platform**: All platforms (comes with PostgreSQL)

**Usage**:
```bash
# Connect to database
psql -h localhost -U postgres -d mudrock

# Execute queries
psql -h localhost -U postgres -d mudrock -c "SELECT * FROM wells LIMIT 10;"

# Interactive mode
psql -h localhost -U postgres -d mudrock
mudrock=# \dt          # List tables
mudrock=# \d wells     # Describe table
mudrock=# SELECT * FROM wells;
mudrock=# \q           # Quit
```

## Parquet File Inspection Tools

### Desktop GUI Applications

#### 1. **Parquet Viewer** (VS Code Extension)

**Extension ID**: `dvirgiln.parquet-viewer`

**Features**:
- ✅ View Parquet files in VS Code
- ✅ Browse columns and data
- ✅ Export to CSV/JSON
- ✅ View schema and metadata

**Usage**:
```
1. Install extension
2. Right-click .parquet file → "Open with Parquet Viewer"
3. Browse data and schema
```

#### 2. **Parquet Tools** (CLI)

**Platform**: All platforms (Python)

**Installation**:
```bash
pip install parquet-tools
```

**Usage**:
```bash
# View schema
parquet-tools schema file.parquet

# View data (first 10 rows)
parquet-tools head file.parquet

# View metadata
parquet-tools meta file.parquet

# Convert to CSV
parquet-tools csv file.parquet > output.csv
```

### Python Scripts

#### 1. **PyArrow** (Python Library)

**Usage**:
```python
import pyarrow.parquet as pq

# Read Parquet file
table = pq.read_table('file.parquet')

# View schema
print(table.schema)

# Convert to Pandas DataFrame
df = table.to_pandas()
print(df.head())

# View metadata
parquet_file = pq.ParquetFile('file.parquet')
print(parquet_file.metadata)
```

#### 2. **DuckDB** (CLI/Query Engine)

**Features**:
- ✅ Can read Parquet files directly
- ✅ SQL queries on Parquet
- ✅ Fast analytical queries

**Installation**:
```bash
# macOS
brew install duckdb

# Or download from: https://duckdb.org/
```

**Usage**:
```bash
# Interactive mode
duckdb

# Query Parquet file
SELECT * FROM 'file.parquet' LIMIT 10;

# Query with filters
SELECT DEPT, GR, RHOB 
FROM 'file.parquet' 
WHERE DEPT BETWEEN 1000 AND 2000;
```

### Rust Tools

#### 1. **parquet-cli** (Rust)

**Installation**:
```bash
cargo install parquet-cli
```

**Usage**:
```bash
# View schema
parquet schema file.parquet

# View data
parquet cat file.parquet

# View metadata
parquet meta file.parquet
```

## Yjs Document Inspection Tools

### Browser DevTools

#### 1. **IndexedDB Inspector**

**Features**:
- ✅ Built into Chrome/Edge DevTools
- ✅ View IndexedDB stores
- ✅ Inspect Yjs document data

**Usage**:
```
1. Open Chrome DevTools (F12)
2. Application tab → Storage → IndexedDB
3. Find Yjs store (usually named "yjs-documents" or similar)
4. Browse stored documents
```

### Node.js Scripts

#### 1. **Yjs Inspection Script**

```typescript
import * as Y from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

// Load Yjs document from IndexedDB
const ydoc = new Y.Doc();
const provider = new IndexeddbPersistence('document-id', ydoc);

provider.on('synced', () => {
  // Document loaded
  console.log('Document state:', ydoc.getText('content').toString());
  console.log('Document map:', ydoc.getMap('metadata').toJSON());
  
  // Export document state
  const state = Y.encodeStateAsUpdate(ydoc);
  console.log('Document state (binary):', state);
});
```

### VS Code Extensions

#### 1. **IndexedDB Explorer**

**Extension ID**: `ms-vscode.vscode-json`

**Features**:
- ✅ View IndexedDB in VS Code (limited)
- ✅ Better: Use browser DevTools

## Object Storage Inspection Tools

### S3/MinIO Tools

#### 1. **MinIO Client (mc)**

**Platform**: All platforms

**Features**:
- ✅ Official MinIO client
- ✅ S3-compatible
- ✅ Browse buckets and objects
- ✅ Upload/download files

**Installation**:
```bash
# macOS
brew install minio/stable/mc

# Download from: https://min.io/download
```

**Usage**:
```bash
# Configure MinIO
mc alias set mudrock http://localhost:9000 minioadmin minioadmin

# List buckets
mc ls mudrock

# List objects in bucket
mc ls mudrock/mudrock-data/blobs/

# Download file
mc cp mudrock/mudrock-data/blobs/a3/f2/file.parquet ./local-file.parquet

# Upload file
mc cp ./local-file.parquet mudrock/mudrock-data/blobs/a3/f2/
```

#### 2. **AWS CLI** (S3-compatible)

**Platform**: All platforms

**Features**:
- ✅ Works with S3 and S3-compatible storage (MinIO)
- ✅ Browse buckets
- ✅ Upload/download files

**Installation**:
```bash
# macOS
brew install awscli

# Or: pip install awscli
```

**Usage**:
```bash
# Configure (for MinIO)
aws configure set aws_access_key_id minioadmin
aws configure set aws_secret_access_key minioadmin
aws configure set default.region us-east-1

# List buckets
aws --endpoint-url http://localhost:9000 s3 ls

# List objects
aws --endpoint-url http://localhost:9000 s3 ls s3://mudrock-data/blobs/

# Download file
aws --endpoint-url http://localhost:9000 s3 cp s3://mudrock-data/blobs/a3/f2/file.parquet ./local-file.parquet
```

#### 3. **MinIO Console** (Web UI)

**Features**:
- ✅ Web-based UI for MinIO
- ✅ Browse buckets and objects
- ✅ Upload/download files
- ✅ View object metadata

**Access**:
```
1. Start MinIO with console enabled
2. Open http://localhost:9001
3. Login with MinIO credentials
4. Browse buckets and objects
```

### Filesystem Tools

#### 1. **File Explorer** (Native)

**Platform**: All platforms

**Features**:
- ✅ Browse local filesystem
- ✅ View file sizes and dates
- ✅ Open files with default applications

**Usage**:
```
Navigate to storage directory:
- macOS: ~/Library/Application Support/mudrock/blobs/
- Windows: %APPDATA%\mudrock\blobs\
- Linux: ~/.local/share/mudrock/blobs/
```

#### 2. **Tree** (CLI)

**Platform**: All platforms

**Features**:
- ✅ Visual directory tree
- ✅ See file structure at a glance

**Installation**:
```bash
# macOS
brew install tree

# Linux
sudo apt install tree
```

**Usage**:
```bash
# View directory tree
tree ~/Library/Application\ Support/mudrock/blobs/

# With file sizes
tree -h ~/Library/Application\ Support/mudrock/blobs/
```

## Application State Inspection Tools

### Browser DevTools

#### 1. **React DevTools** (If using React)

**Features**:
- ✅ Inspect React component state
- ✅ View props and hooks
- ✅ Performance profiling

**Installation**:
- Chrome Extension: React Developer Tools
- Firefox Extension: React Developer Tools

#### 2. **Svelte DevTools** (For Svelte)

**Features**:
- ✅ Inspect Svelte component state
- ✅ View reactive variables
- ✅ Component tree

**Installation**:
```bash
npm install -D @sveltejs/vite-plugin-svelte
```

### VS Code Extensions

#### 1. **Debugger for Chrome/Firefox**

**Extension ID**: `msjsdiag.debugger-for-chrome`

**Features**:
- ✅ Debug JavaScript/TypeScript in VS Code
- ✅ Set breakpoints
- ✅ Inspect variables
- ✅ Step through code

### Rust Debugging

#### 1. **VS Code Rust Analyzer**

**Extension ID**: `rust-lang.rust-analyzer`

**Features**:
- ✅ Rust language support
- ✅ Debugging with LLDB/GDB
- ✅ Code completion and navigation

**Usage**:
```json
// .vscode/launch.json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Tauri",
      "cargo": {
        "args": ["build", "--manifest-path=src-tauri/Cargo.toml"],
        "filter": {
          "name": "mudrock",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

## Recommended Tool Stack for MVP

### Daily Development

1. **DB Browser for SQLite** - Primary SQLite inspection tool
2. **VS Code Parquet Viewer** - Quick Parquet file inspection
3. **MinIO Console** - Browse object storage
4. **Browser DevTools** - Inspect IndexedDB (if using Yjs)

### Advanced Debugging

1. **DBeaver** - Multi-database tool (SQLite + PostgreSQL)
2. **DuckDB CLI** - Query Parquet files with SQL
3. **sqlite-utils** - Command-line SQLite operations
4. **VS Code Debugger** - Debug Rust and TypeScript

### Production/Server

1. **psql** - PostgreSQL command-line (if using PostgreSQL)
2. **MinIO Client (mc)** - Server-side object storage management
3. **AWS CLI** - S3-compatible operations

## Quick Reference Commands

### SQLite

```bash
# Open database
sqlite3 mudrock.db

# View all tables
.tables

# View schema
.schema

# Query data
SELECT * FROM wells LIMIT 10;

# Export to CSV
.mode csv
.output wells.csv
SELECT * FROM wells;
.output stdout

# Exit
.quit
```

### Parquet Files

```bash
# View schema (Python)
python -c "import pyarrow.parquet as pq; print(pq.read_table('file.parquet').schema)"

# View data (Python)
python -c "import pyarrow.parquet as pq; print(pq.read_table('file.parquet').to_pandas().head())"

# Query with DuckDB
duckdb -c "SELECT * FROM 'file.parquet' LIMIT 10;"
```

### MinIO/S3

```bash
# List buckets
mc ls mudrock

# List objects
mc ls mudrock/mudrock-data/blobs/ --recursive

# Download file
mc cp mudrock/mudrock-data/blobs/a3/f2/file.parquet ./

# Upload file
mc cp ./file.parquet mudrock/mudrock-data/blobs/a3/f2/
```

## Integration with Development Workflow

### VS Code Workspace Setup

```json
// .vscode/settings.json
{
  "files.associations": {
    "*.parquet": "parquet",
    "*.db": "sqlite"
  },
  "sqlite.databasePath": "${workspaceFolder}/data/mudrock.db"
}
```

### Recommended Extensions

```json
// .vscode/extensions.json
{
  "recommendations": [
    "qwtel.sqlite-viewer",
    "dvirgiln.parquet-viewer",
    "rust-lang.rust-analyzer",
    "ms-vscode.vscode-json"
  ]
}
```

## Troubleshooting Common Issues

### SQLite Database Locked

**Problem**: Database is locked by another process

**Solution**:
```bash
# Check what's using the database
lsof mudrock.db

# Kill the process if needed
kill -9 <PID>
```

### Parquet File Corrupted

**Problem**: Can't read Parquet file

**Solution**:
```python
import pyarrow.parquet as pq

try:
    table = pq.read_table('file.parquet')
except Exception as e:
    print(f"Error: {e}")
    # Try to read metadata only
    try:
        metadata = pq.read_metadata('file.parquet')
        print(f"Metadata: {metadata}")
    except:
        print("File is corrupted")
```

### MinIO Connection Issues

**Problem**: Can't connect to MinIO

**Solution**:
```bash
# Test connection
mc admin info mudrock

# Check MinIO status
curl http://localhost:9000/minio/health/live

# View MinIO logs
docker logs minio
```

## Summary

| Tool | Purpose | Platform | Cost |
|------|---------|----------|------|
| **DB Browser for SQLite** | SQLite inspection | All | Free |
| **TablePlus** | Multi-database GUI | All | Paid (free tier) |
| **DBeaver** | Universal DB tool | All | Free |
| **Parquet Viewer (VS Code)** | Parquet inspection | VS Code | Free |
| **DuckDB** | Query Parquet files | CLI | Free |
| **MinIO Console** | Object storage UI | Web | Free |
| **MinIO Client (mc)** | Object storage CLI | All | Free |
| **Browser DevTools** | IndexedDB/Yjs inspection | Browser | Free |

**Recommended MVP Setup**:
1. **DB Browser for SQLite** - Primary database tool
2. **VS Code Parquet Viewer** - Quick Parquet inspection
3. **MinIO Console** - Object storage browsing
4. **DuckDB CLI** - Advanced Parquet queries

This tool stack provides comprehensive inspection capabilities for all components of the MVP architecture.

