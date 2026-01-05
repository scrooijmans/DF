Description: API for reading/writing Arrow `RecordBatch`es and `Array`s to/from Parquet Files.

Title: parquet::arrow - Rust

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

## Module arrow

parquet

# Module arrow Copy item path

Source

Available on **crate feature `arrow`** only.

Expand description

API for reading/writing Arrow `RecordBatch`es and `Array`s to/from Parquet Files.

See the crate-level documentation for more details on other APIs

## §Schema Conversion

These APIs ensure that data in Arrow `RecordBatch`es written to Parquet are read back as `RecordBatch`es with the exact same types and values.

Parquet and Arrow have different type systems, and there is not always a one to one mapping between the systems. For example, data stored as a Parquet `BYTE_ARRAY` can be read as either an Arrow `BinaryViewArray` or `BinaryArray`.

To recover the original Arrow types, the writers in this module add a “hint” to the metadata in the `ARROW_SCHEMA_META_KEY` key which records the original Arrow schema. The metadata hint follows the same convention as arrow-cpp based implementations such as `pyarrow`. The reader looks for the schema hint in the metadata to determine Arrow types, and if it is not present, infers the Arrow schema from the Parquet schema.

In situations where the embedded Arrow schema is not compatible with the Parquet schema, the Parquet schema takes precedence and no error is raised. See #1663

You can also control the type conversion process in more detail using:

- `ArrowSchemaConverter` control the conversion of Arrow types to Parquet types.

- `ArrowReaderOptions::with_schema` to explicitly specify your own Arrow schema hint to use when reading Parquet, overriding any metadata that may be present.

## §Example: Writing Arrow `RecordBatch` to Parquet file

```
let ids = Int32Array::from(vec![1, 2, 3, 4]);
let vals = Int32Array::from(vec![5, 6, 7, 8]);
let batch = RecordBatch::try_from_iter(vec![
("id", Arc::new(ids) as ArrayRef),
("val", Arc::new(vals) as ArrayRef),
]).unwrap();

let file = tempfile().unwrap();

// WriterProperties can be used to set Parquet file options
let props = WriterProperties::builder()
.set_compression(Compression::SNAPPY)
.build();

let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props)).unwrap();

writer.write(&batch).expect("Writing batch");

// writer must be closed to write footer
writer.close().unwrap();
```

## §Example: Reading Parquet file into Arrow `RecordBatch`

```
let file = File::open("data.parquet").unwrap();

let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
println!("Converted arrow schema is: {}", builder.schema());

let mut reader = builder.build().unwrap();

let record_batch = reader.next().unwrap().unwrap();

println!("Read {} records.", record_batch.num_rows());
```

## §Example: Reading non-uniformly encrypted parquet file into arrow record batch

Note: This requires the experimental `encryption` feature to be enabled at compile time.

```
let file = File::open(path).unwrap();

// Define the AES encryption keys required required for decrypting the footer metadata
// and column-specific data. If only a footer key is used then it is assumed that the
// file uses uniform encryption and all columns are encrypted with the footer key.
// If any column keys are specified, other columns without a key provided are assumed
// to be unencrypted
let footer_key = "0123456789012345".as_bytes(); // Keys are 128 bits (16 bytes)
let column_1_key = "1234567890123450".as_bytes();
let column_2_key = "1234567890123451".as_bytes();

let decryption_properties = FileDecryptionProperties::builder(footer_key.to_vec())
.with_column_key("double_field", column_1_key.to_vec())
.with_column_key("float_field", column_2_key.to_vec())
.build()
.unwrap();

let options = ArrowReaderOptions::default()
.with_file_decryption_properties(decryption_properties);
let reader_metadata = ArrowReaderMetadata::load(&file, options.clone()).unwrap();
let file_metadata = reader_metadata.metadata().file_metadata();
assert_eq!(50, file_metadata.num_rows());

let mut reader = ParquetRecordBatchReaderBuilder::try_new_with_options(file, options)
.unwrap()
.build()
.unwrap();

let record_batch = reader.next().unwrap().unwrap();
assert_eq!(50, record_batch.num_rows());
```

## Re-exports§

`pub use self::arrow_writer::ArrowWriter;`

`pub use self::async_reader::ParquetRecordBatchStreamBuilder;``async`

`pub use self::async_writer::AsyncArrowWriter;``async`

## Modules§

arrow_reader

Contains reader which reads parquet data into arrow `RecordBatch`

arrow_writer

Contains writer which writes arrow data into parquet data.

async_reader`async`

`async` API for reading Parquet files as `RecordBatch`es

async_writer`async`

`async` API for writing `RecordBatch`es to Parquet files

## Structs§

ArrowSchemaConverter

Converter for Arrow schema to Parquet schema

FieldLevels

Schema information necessary to decode a parquet file as arrow `Fields`

ProjectionMask

A `ProjectionMask` identifies a set of columns within a potentially nested schema to project

## Constants§

ARROW_SCHEMA_META_KEY

Schema metadata key used to store serialized Arrow schema

PARQUET_FIELD_ID_META_KEY

The value of this metadata key, if present on `Field::metadata`, will be used to populate `BasicTypeInfo::id`

## Functions§

add_encoded_arrow_schema_to_metadata

Mutates writer metadata by storing the encoded Arrow schema hint in `ARROW_SCHEMA_META_KEY`.

encode_arrow_schema

Encodes the Arrow schema into the IPC format, and base64 encodes it

parquet_column

Lookups up the parquet column by name

parquet_to_arrow_field_levels

Convert a parquet `SchemaDescriptor` to `FieldLevels`

parquet_to_arrow_schema

Convert Parquet schema to Arrow schema including optional metadata

parquet_to_arrow_schema_by_columns

Convert parquet schema to arrow schema including optional metadata, only preserving some leaf columns.
