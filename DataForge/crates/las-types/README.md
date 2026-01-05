# las-types

Shared data structures and types for LAS (Log ASCII Standard) files.

## Description

This crate provides the core data structures and types used for working with LAS well log files. It includes:

- `LASFile` struct and related types
- Section structs (`CurveSection`, `WellSection`, `VersionSection`, `ParameterSection`)
- `HeaderItem`, `CurveDataColumn`, and related types
- Section parsing logic

## Usage

```rust
use las_types::{LASFile, CurveSection, WellSection};

// Create or parse LAS file structures
let las_file = LASFile::new();
```

## Dependencies

- serde - Serialization/deserialization
- regex - Regular expressions for parsing
- lazy_static - Static initialization
- thiserror - Error handling
- arrow - Apache Arrow data structures
- parquet - Parquet file format support

## License

MIT 