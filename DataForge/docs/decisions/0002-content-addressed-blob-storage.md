# Use Content-Addressed Storage for Parquet Blobs

- **Status**: accepted
- **Date**: 2024-12-14
- **Decision-makers**: Engineering team
- **Consulted**: Data engineering experts
- **Informed**: Stakeholders

## Context and Problem Statement

DataForge stores well log curve data as Parquet files. These files can be large (MB range) and need to be synchronized between clients and server. How should we organize blob storage to ensure integrity, enable deduplication, and simplify sync?

## Decision Drivers

- **Data integrity**: Curve data is mission-critical; corruption must be detectable
- **Deduplication**: Same data shouldn't be stored multiple times
- **Sync efficiency**: Only transfer blobs that don't exist on target
- **Immutability**: Historical data should never be silently modified
- **Provenance**: Track origin and transformations of data

## Considered Options

1. **Content-addressed storage** (SHA-256 hash as filename)
2. **UUID-based storage** (random identifiers)
3. **Path-based storage** (hierarchical by well/curve)

## Decision Outcome

**Chosen option**: "Content-addressed storage", because it provides built-in integrity verification, automatic deduplication, and immutability guarantees that align with our data integrity requirements.

### Consequences

**Positive:**
- Integrity verification on every read (compare computed hash to filename)
- Automatic deduplication (same content = same hash = same file)
- Immutability enforced (can't modify without changing hash)
- Sync is simple (check if hash exists on target)
- Cache-friendly (identical content always resolves to same path)

**Negative:**
- Cannot modify files in place (must create new version)
- Garbage collection needed for orphaned blobs
- Filename not human-readable

## Confirmation

- Blob store tests verify hash integrity on read
- Sync tests confirm deduplication works across clients
- Integration tests verify corrupted blobs are detected

## Pros and Cons of Options

### Content-Addressed Storage (SHA-256)

Store blobs at path `blobs/{hash[0:2]}/{hash[2:4]}/{hash}.parquet`

- **Good**: Built-in integrity verification
- **Good**: Automatic deduplication
- **Good**: Immutable by design
- **Good**: Simple sync (just check if hash exists)
- **Good**: Industry standard pattern (Git, IPFS, Docker)
- **Bad**: Non-human-readable filenames
- **Bad**: Need garbage collection for orphans
- **Neutral**: Two-level directory nesting prevents too many files per directory

### UUID-Based Storage

Store blobs at path `blobs/{uuid}.parquet`

- **Good**: Simple implementation
- **Good**: Unique identifiers
- **Bad**: No integrity verification built-in
- **Bad**: No deduplication
- **Bad**: Sync requires tracking which UUIDs exist where

### Path-Based Storage

Store blobs at path `wells/{wellId}/curves/{curveId}.parquet`

- **Good**: Human-readable organization
- **Good**: Easy to browse in file manager
- **Bad**: No integrity verification
- **Bad**: No deduplication across wells
- **Bad**: Path conflicts if same curve imported twice
- **Bad**: Moves/renames break references

## More Information

**Storage path structure:**
```
blobs/
├── a3/
│   └── f2/
│       └── a3f2b8c9d1e4f5...parquet
└── 7b/
    └── 1c/
        └── 7b1c3d4e5f6a7...parquet
```

**Implementation files:**
- `crates/dataforge-core/src/blob.rs` - BlobStore implementation
- `crates/dataforge-storage/src/` - S3/MinIO/filesystem backends

**Related patterns:**
- Git object storage
- Docker image layers
- IPFS content addressing
