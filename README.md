# DataForge Platform

**Unified geoscience data platform combining DataForge and DataForge-Compute.**

This repository contains the complete DataForge ecosystem, including both the main DataForge application and the DataForge-Compute components.

## Project Structure

```
.
├── DataForge/              # Main DataForge application
│   ├── crates/            # Rust workspace crates
│   ├── src/               # SvelteKit frontend
│   ├── src-tauri/         # Tauri desktop app
│   └── docs/              # DataForge documentation
│
├── DataForge-Compute/     # Compute components
│   ├── src/               # SvelteKit frontend
│   ├── src-tauri/         # Tauri desktop app
│   └── docs/              # Compute documentation
│
├── docs/                  # Shared documentation
├── markdown-guides/       # Development guides
└── data/                  # Sample data files
```

## Components

### DataForge

**Offline-first geoscience data platform for enterprise teams.**

DataForge enables geoscientists to upload, visualize, and share well log data with colleagues while maintaining full offline functionality and data sovereignty.

See [DataForge/README.md](DataForge/README.md) for detailed information.

### DataForge-Compute

Compute and analysis components for the DataForge platform.

See [DataForge-Compute/README.md](DataForge-Compute/README.md) for more details.

## Getting Started

### Prerequisites

- **Rust** 1.75+ (install via [rustup](https://rustup.rs/))
- **Bun** 1.0+ (install via [bun.sh](https://bun.sh/))

### Development

Each component can be developed independently:

```bash
# Work on DataForge
cd DataForge
bun install
bun tauri dev

# Work on DataForge-Compute
cd DataForge-Compute
bun install
bun tauri dev
```

## License

MIT

