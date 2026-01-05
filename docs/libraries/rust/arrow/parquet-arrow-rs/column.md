Description: Low level column reader and writer APIs.

Title: parquet::column - Rust

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

## Module column

parquet

# Module column Copy item path

Source

Expand description

Low level column reader and writer APIs.

This API is designed for reading and writing column values, definition and repetition levels directly.

## §Example of writing and reading data

Data has the following format:

```
+---------------+
|         values|
+---------------+
|[1, 2]         |
|[3, null, null]|
+---------------+
```

The example uses column writer and reader APIs to write raw values, definition and repetition levels and read them to verify write/read correctness.

```

let path = Path::new("/path/to/column_sample.parquet");

// Writing data using column writer API.

let message_type = "
message schema {
optional group values (LIST) {
repeated group list {
optional INT32 element;
}
}
}
";
let schema = Arc::new(parse_message_type(message_type).unwrap());
let file = fs::File::create(path).unwrap();
let mut writer = SerializedFileWriter::new(file, schema, Default::default()).unwrap();

let mut row_group_writer = writer.next_row_group().unwrap();
while let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
col_writer
.typed::<Int32Type>()
.write_batch(&[1, 2, 3], Some(&[3, 3, 3, 2, 2]), Some(&[0, 1, 0, 1, 1]))
.unwrap();
col_writer.close().unwrap();
}
row_group_writer.close().unwrap();

writer.close().unwrap();

// Reading data using column reader API.

let file = fs::File::open(path).unwrap();
let reader = SerializedFileReader::new(file).unwrap();
let metadata = reader.metadata();

let mut values = vec![];
let mut def_levels = vec![];
let mut rep_levels = vec![];

for i in 0..metadata.num_row_groups() {
let row_group_reader = reader.get_row_group(i).unwrap();
let row_group_metadata = metadata.row_group(i);

for j in 0..row_group_metadata.num_columns() {
let mut column_reader = row_group_reader.get_column_reader(j).unwrap();
match column_reader {
// You can also use `get_typed_column_reader` method to extract typed reader.
ColumnReader::Int32ColumnReader(ref mut typed_reader) => {
let (records, values, levels) = typed_reader.read_records(
8, // maximum records to read
Some(&mut def_levels),
Some(&mut rep_levels),
&mut values,
).unwrap();
assert_eq!(records, 2);
assert_eq!(levels, 5);
assert_eq!(values, 3);
}
_ => {}
}
}
}

assert_eq!(values, vec![1, 2, 3]);
assert_eq!(def_levels, vec![3, 3, 3, 2, 2]);
assert_eq!(rep_levels, vec![0, 1, 0, 1, 1]);
```

## Modules§

page

Contains Parquet Page definitions and page reader interface.

reader

Contains column reader API.

writer

Contains column writer API.
