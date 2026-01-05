# las-parser

LAS (Log ASCII Standard) well log file parser.

## Description

This crate provides functionality for parsing LAS well log files. It includes:

- `LasParser` struct for parsing orchestration
- File reading and parsing logic
- High-level parsing API

## Usage

```rust
use las_parser::LasParser;

// Parse a LAS file
let parser = LasParser::new();
let las_file = parser.parse_file("path/to/file.las").await?;
```

## Dependencies

- las-types - Shared data structures and types
- serde - Serialization/deserialization
- tokio - Async runtime
- arrow - Apache Arrow data structures
- parquet - Parquet file format support

## License

MIT 