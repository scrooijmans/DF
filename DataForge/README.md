# DataForge

**Offline-first geoscience data platform for enterprise teams.**

DataForge enables geoscientists to upload, visualize, and share well log data with colleagues while maintaining full offline functionality and data sovereignty.

## Features

- **Offline-First**: Work without internet, sync when online
- **LAS File Support**: Parse LAS 1.2, 2.0, and 3.0 files
- **Content-Addressed Storage**: Parquet files stored by SHA-256 hash
- **Git-Like Sync**: Pull-based synchronization (not real-time CRDT)
- **Enterprise Deployment**: Self-hosted, air-gapped, or private cloud

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│  CLIENT (Tauri Desktop App)                                     │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────────┐ │
│  │ SvelteKit UI │  │ Rust Backend │  │ Local Storage          │ │
│  │ - Charts     │  │ - LAS Parser │  │ - SQLite (metadata)    │ │
│  │ - Data Grid  │  │ - Parquet    │  │ - Parquet (curves)     │ │
│  └──────────────┘  └──────────────┘  └────────────────────────┘ │
└───────────────────────────┬─────────────────────────────────────┘
                            │ REST API (when online)
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  SERVER (Self-Hosted)                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────────┐ │
│  │ REST API     │  │ SQLite DB    │  │ Blob Storage (S3)      │ │
│  └──────────────┘  └──────────────┘  └────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Project Structure

```
DataForge/
├── crates/
│   ├── dataforge-core/     # Core data types, DB, blob storage
│   ├── dataforge-sync/     # Git-like sync protocol
│   └── dataforge-storage/  # S3/local storage abstraction
├── src-tauri/              # Tauri desktop app
├── src/                    # SvelteKit frontend
└── docs/                   # Documentation
```

## Prerequisites

- **Rust** 1.75+ (install via [rustup](https://rustup.rs/))
- **Bun** 1.0+ (install via [bun.sh](https://bun.sh/))

## Quick Start

```bash
# Clone the repository
git clone https://github.com/scrooijmans/DataForge.git
cd DataForge

# Install frontend dependencies
bun install

# Build and run in development mode
bun tauri dev
```

## Development

### Build Core Crates

```bash
cargo build --workspace
```

### Run Tests

```bash
cargo test --workspace
```

### Build Desktop App

```bash
bun tauri build
```

## Deployment Options

| Option | Use Case | Complexity |
|--------|----------|------------|
| **Air-Gapped** | Classified environments | Single binary |
| **Private Cloud** | Remote team access | Docker Compose |
| **Hybrid** | Office + cloud backup | On-premise + S3 |

See [docs/architecture/MVP_IMPLEMENTATION_PLAN.md](docs/architecture/MVP_IMPLEMENTATION_PLAN.md) for detailed deployment guides.

## Reused Components

DataForge reuses battle-tested crates from the MudRock project:

- `las-parser` - LAS file parsing (LAS 1.2, 2.0, 3.0)
- `las-types` - LAS data structures
- `unit-conversions` - Petrophysics unit conversion

## License

MIT
