Description: Parquet schema definitions and methods to print and parse schema.

Title: parquet::schema - Rust

Docs.rs

- parquet-56.2.0

- parquet 56.2.0
- Permalink
- Docs.rs crate page
- Apache-2.0

- Links
- Homepage
- Repository
- crates.io
- Source

- Owners
- andygrove
- sunchao
- kou
- kszucs
- nevi-me
- sadikovi
- alamb
- tustvold

- Dependencies
- - arrow-array ^56.2.0 _normal_ _optional_
- arrow-buffer ^56.2.0 _normal_ _optional_
- arrow-cast ^56.2.0 _normal_ _optional_
- arrow-csv ^56.2.0 _normal_ _optional_
- arrow-data ^56.2.0 _normal_ _optional_
- arrow-ipc ^56.2.0 _normal_ _optional_
- arrow-schema ^56.2.0 _normal_ _optional_
- arrow-select ^56.2.0 _normal_ _optional_
- base64 ^0.22 _normal_ _optional_
- brotli ^8.0 _normal_ _optional_
- bytes ^1.1 _normal_
- chrono ^0.4.40 _normal_
- clap ^4.1 _normal_ _optional_
- crc32fast ^1.4.2 _normal_ _optional_
- flate2 ^1.1 _normal_ _optional_
- futures ^0.3 _normal_ _optional_
- half ^2.1 _normal_
- hashbrown ^0.16 _normal_
- lz4*flex ^0.11 \_normal* _optional_
- num ^0.4 _normal_
- num-bigint ^0.4 _normal_
- object*store ^0.12.0 \_normal* _optional_
- parquet-variant ^0.1.0 _normal_ _optional_
- parquet-variant-compute ^0.1.0 _normal_ _optional_
- parquet-variant-json ^0.1.0 _normal_ _optional_
- paste ^1.0 _normal_
- ring ^0.17 _normal_ _optional_
- seq-macro ^0.3 _normal_
- serde ^1.0 _normal_ _optional_
- serde*json ^1.0 \_normal* _optional_
- simdutf8 ^0.1.5 _normal_ _optional_
- snap ^1.0 _normal_ _optional_
- thrift ^0.17 _normal_
- tokio ^1.0 _normal_ _optional_
- twox-hash ^2.0 _normal_
- zstd ^0.13 _normal_ _optional_
- arrow ^56.2.0 _dev_
- base64 ^0.22 _dev_
- brotli ^8.0 _dev_
- criterion ^0.5 _dev_
- flate2 ^1.0 _dev_
- insta ^1.43.1 _dev_
- lz4*flex ^0.11 \_dev*
- object*store ^0.12.0 \_dev*
- rand ^0.9 _dev_
- serde*json ^1.0 \_dev*
- snap ^1.0 _dev_
- sysinfo ^0.36.0 _dev_
- tempfile ^3.0 _dev_
- tokio ^1.0 _dev_
- zstd ^0.13 _dev_
- ahash ^0.8 _normal_
- ahash ^0.8 _normal_
- ring ^0.17 _normal_ _optional_

- Versions

- **100%** of the crate is documented

- Platform
- x86_64-unknown-linux-gnu
- Feature flags

- docs.rs
- About docs.rs
- Badges
- Builds
- Metadata
- Shorthand URLs
- Download
- Rustdoc JSON
- Build queue
- Privacy policy

- Rust
- Rust website
- The Book
- Standard Library API Reference
- Rust by Example
- The Cargo Guide
- Clippy Documentation

## Module schema

parquet

# Module schema Copy item path

Source

Expand description

Parquet schema definitions and methods to print and parse schema.

- `SchemaDescriptor` describes the data types of the columns stored in a file
- `ColumnDescriptor`: Describes the schema of a single (leaf) column.
- `ColumnPath`: Represents the location of a column in the schema (e.g. a nested field)

Parquet distinguishes between “logical” and “physical” data types. For instance, strings (logical type) are stored as byte arrays (physical type). Likewise, temporal types like dates, times, timestamps, etc. (logical type) are stored as integers (physical type).

## §Example

```
use parquet::{
basic::{ConvertedType, Repetition, Type as PhysicalType},
schema::{parser, printer, types::Type},
};
use std::sync::Arc;

// Create the following schema:
//
// message schema {
//   OPTIONAL BYTE_ARRAY a (UTF8);
//   REQUIRED INT32 b;
// }

let field_a = Type::primitive_type_builder("a", PhysicalType::BYTE_ARRAY)
.with_converted_type(ConvertedType::UTF8)
.with_repetition(Repetition::OPTIONAL)
.build()
.unwrap();

let field_b = Type::primitive_type_builder("b", PhysicalType::INT32)
.with_repetition(Repetition::REQUIRED)
.build()
.unwrap();

let schema = Type::group_type_builder("schema")
.with_fields(vec![Arc::new(field_a), Arc::new(field_b)])
.build()
.unwrap();

let mut buf = Vec::new();

// Print schema into buffer
printer::print_schema(&mut buf, &schema);

// Parse schema from the string
let string_schema = String::from_utf8(buf).unwrap();
let parsed_schema = parser::parse_message_type(&string_schema).unwrap();

assert_eq!(schema, parsed_schema);
```

## Modules§

parser

Parquet schema parser. Provides methods to parse and validate string message type into Parquet `Type`.

printer

Parquet schema printer. Provides methods to print Parquet file schema and list file metadata.

types

Contains structs and methods to build Parquet schema and schema descriptors.

visitor

Utilities to traverse against various parquet type.
