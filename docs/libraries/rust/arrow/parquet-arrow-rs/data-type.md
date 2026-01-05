Description: Data types that connect Parquet physical types with their Rust-specific representations.

Title: parquet::data_type - Rust

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
- lz4_flex ^0.11 _normal_ _optional_
- num ^0.4 _normal_
- num-bigint ^0.4 _normal_
- object_store ^0.12.0 _normal_ _optional_
- parquet-variant ^0.1.0 _normal_ _optional_
- parquet-variant-compute ^0.1.0 _normal_ _optional_
- parquet-variant-json ^0.1.0 _normal_ _optional_
- paste ^1.0 _normal_
- ring ^0.17 _normal_ _optional_
- seq-macro ^0.3 _normal_
- serde ^1.0 _normal_ _optional_
- serde_json ^1.0 _normal_ _optional_
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
- lz4_flex ^0.11 _dev_
- object_store ^0.12.0 _dev_
- rand ^0.9 _dev_
- serde_json ^1.0 _dev_
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

## Module data_type

parquet

# Module data_type Copy item path

Source

Expand description

Data types that connect Parquet physical types with their Rust-specific representations.

## Structs§

BoolType

Parquet physical type: BoolType

ByteArray

Rust representation for BYTE_ARRAY and FIXED_LEN_BYTE_ARRAY Parquet physical types. Value is backed by a byte buffer.

ByteArrayType

Parquet physical type: ByteArrayType

DoubleType

Parquet physical type: DoubleType

FixedLenByteArray

Wrapper type for performance reasons, this represents `FIXED_LEN_BYTE_ARRAY` but in all other considerations behaves the same as `ByteArray`

FixedLenByteArrayType

Parquet physical type: FixedLenByteArrayType

FloatType

Parquet physical type: FloatType

Int96

Rust representation for logical type INT96, value is backed by an array of `u32`. The type only takes 12 bytes, without extra padding.

Int32Type

Parquet physical type: Int32Type

Int64Type

Parquet physical type: Int64Type

Int96Type

Parquet physical type: Int96Type

## Enums§

Decimal

Rust representation for Decimal values.

## Traits§

AsBytes

Converts an instance of data type to a slice of bytes as `u8`.

DataType

Contains the Parquet physical type information as well as the Rust primitive type presentation.

SliceAsBytes

Converts an slice of a data type to a slice of bytes.
