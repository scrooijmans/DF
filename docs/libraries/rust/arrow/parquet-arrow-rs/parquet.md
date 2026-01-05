Description: Automatically generated code from the Parquet thrift definition.

Title: parquet::format - Rust

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

## Module format

parquet

# Module format Copy item path

Source

Expand description

Automatically generated code from the Parquet thrift definition.

This module code generated from parquet.thrift. See crate::file for more information on reading Parquet encoded data.

See `crate::file` for easier to use APIs.

## Structs§

AesGcmCtrV1

AesGcmV1

BloomFilterHeader

Bloom filter header is stored at beginning of Bloom filter data of each column and followed by its bitset.

BoundaryOrder

Enum to annotate whether lists of min/max elements inside ColumnIndex are ordered and if so, in which direction.

BoundingBox

Bounding box for GEOMETRY or GEOGRAPHY type in the representation of min/max value pair of coordinates from each axis.

BsonType

Embedded BSON logical type annotation

ColumnChunk

ColumnIndex

Optional statistics for each data page in a ColumnChunk.

ColumnMetaData

Description for column metadata

CompressionCodec

Supported compression algorithms.

ConvertedType

DEPRECATED: Common types used by frameworks(e.g. hive, pig) using parquet. ConvertedType is superseded by LogicalType. This enum should not be extended.

DataPageHeader

Data page header

DataPageHeaderV2

New page format allowing reading levels without decompressing the data Repetition and definition levels are uncompressed The remaining section containing the data is compressed if is_compressed is true

DateType

DecimalType

Decimal logical type annotation

DictionaryPageHeader

The dictionary page must be placed at the first position of the column chunk if it is partly or completely dictionary encoded. At most one dictionary page can be placed in a column chunk.

EdgeInterpolationAlgorithm

Edge interpolation algorithm for Geography logical type

Encoding

Encodings supported by Parquet. Not all encodings are valid for all types. These enums are also used to specify the encoding of definition and repetition levels. See the accompanying doc for the details of the more complicated encodings.

EncryptionWithColumnKey

EncryptionWithFooterKey

EnumType

FieldRepetitionType

Representation of Schemas

FileCryptoMetaData

Crypto metadata for files with encrypted footer \*

FileMetaData

Description for file metadata

Float16Type

GeographyType

Embedded Geography logical type annotation

GeometryType

Embedded Geometry logical type annotation

GeospatialStatistics

Statistics specific to Geometry and Geography logical types

IndexPageHeader

IntType

Integer logical type annotation

JsonType

Embedded JSON logical type annotation

KeyValue

Wrapper struct to store key values

ListType

MapType

MicroSeconds

MilliSeconds

Time units for logical types

NanoSeconds

NullType

Logical type to annotate a column that is always null.

OffsetIndex

Optional offsets for each data page in a ColumnChunk.

PageEncodingStats

statistics of a given page type and encoding

PageHeader

PageLocation

PageType

RowGroup

SchemaElement

Represents a element inside a schema definition.

SizeStatistics

A structure for capturing metadata for estimating the unencoded, uncompressed size of data written. This is useful for readers to estimate how much memory is needed to reconstruct data in their memory model and for fine grained filter pushdown on nested structures (the histograms contained in this structure can help determine the number of nulls at a particular nesting level and maximum length of lists).

SortingColumn

Sort order within a RowGroup of a leaf column

SplitBlockAlgorithm

Block-based algorithm type annotation. \*

Statistics

Statistics per row group and per page All fields are optional.

StringType

Empty structs to use as logical type annotations

TimeType

Time logical type annotation

TimestampType

Timestamp logical type annotation

Type

Types supported by Parquet. These types are intended to be used in combination with the encodings to control the on disk storage format. For example INT16 is not included as a type since a good encoding of INT32 would handle this.

TypeDefinedOrder

Empty struct to signal the order defined by the physical or logical type

UUIDType

Uncompressed

The compression used in the Bloom filter.

VariantType

Embedded Variant logical type annotation

XxHash

Hash strategy type annotation. xxHash is an extremely fast non-cryptographic hash algorithm. It uses 64 bits version of xxHash.

## Enums§

BloomFilterAlgorithm

BloomFilterCompression

BloomFilterHash

ColumnCryptoMetaData

ColumnOrder

EncryptionAlgorithm

LogicalType

TimeUnit
4
