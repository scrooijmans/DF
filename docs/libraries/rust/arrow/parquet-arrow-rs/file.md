Description: APIs for reading parquet data.

Title: parquet::file - Rust

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

## Module file

parquet

# Module file Copy item path

Source

Expand description

APIs for reading parquet data.

Provides access to file and row group readers and writers, record API, metadata, etc.

## §See Also:

- `SerializedFileReader` and `SerializedFileWriter` for reading / writing parquet
- `metadata`: for working with metadata such as schema
- `statistics`: for working with statistics in metadata

## §Example of writing a new file

```
use std::{fs, path::Path, sync::Arc};

use parquet::{
file::{
properties::WriterProperties,
writer::SerializedFileWriter,
},
schema::parser::parse_message_type,
};

let path = Path::new("/path/to/sample.parquet");

let message_type = "
message schema {
REQUIRED INT32 b;
}
";
let schema = Arc::new(parse_message_type(message_type).unwrap());
let file = fs::File::create(&path).unwrap();
let mut writer = SerializedFileWriter::new(file, schema, Default::default()).unwrap();
let mut row_group_writer = writer.next_row_group().unwrap();
while let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
// ... write values to a column writer
col_writer.close().unwrap()
}
row_group_writer.close().unwrap();
writer.close().unwrap();

let bytes = fs::read(&path).unwrap();
assert_eq!(&bytes[0..4], &[b'P', b'A', b'R', b'1']);
```

## §Example of reading an existing file

```
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::{fs::File, path::Path};

let path = Path::new("/path/to/sample.parquet");
if let Ok(file) = File::open(&path) {
let reader = SerializedFileReader::new(file).unwrap();

let parquet_metadata = reader.metadata();
assert_eq!(parquet_metadata.num_row_groups(), 1);

let row_group_reader = reader.get_row_group(0).unwrap();
assert_eq!(row_group_reader.num_columns(), 1);
}
```

## §Example of reading multiple files

```
use parquet::file::reader::SerializedFileReader;
use std::convert::TryFrom;

let paths = vec![
"/path/to/sample.parquet/part-1.snappy.parquet",
"/path/to/sample.parquet/part-2.snappy.parquet"
];
// Create a reader for each file and flat map rows
let rows = paths.iter()
.map(|p| SerializedFileReader::try_from(*p).unwrap())
.flat_map(|r| r.into_iter());

for row in rows {
println!("{}", row.unwrap());
}
```

## Modules§

column_crypto_metadata`encryption`

Column chunk encryption metadata

metadata

Parquet metadata API

page_encoding_stats

Per-page encoding information.

page_index

Page Index of “Column Index Layout to Support Page Skipping”

properties

Configuration via `WriterProperties` and `ReaderProperties`

reader

File reader API and methods to access file metadata, row group readers to read individual column chunks, or access record iterator.

serialized_reader

Contains implementations of the reader traits FileReader, RowGroupReader and PageReader Also contains implementations of the ChunkReader for files (with buffering) and byte arrays (RAM)

statistics

Contains definitions for working with Parquet statistics.

writer

Contains file writer API, and provides methods to write row groups and columns by using row group writers and column writers respectively.

## Constants§

FOOTER_SIZE

The length of the parquet footer in bytes
