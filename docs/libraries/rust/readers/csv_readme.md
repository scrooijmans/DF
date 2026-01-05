Description: The `csv` crate provides a fast and flexible CSV reader and writer, with support for Serde.

Title: csv - Rust

Docs.rs

*   csv-1.3.1

*   csv 1.3.1
*   Permalink
*   Docs.rs crate page
*   Unlicense/MIT

*   Links
*   Homepage
*   Repository
*   crates.io
*   Source

*   Owners
*   BurntSushi

*   Dependencies
*   *   csv-core ^0.1.11 _normal_
*   itoa ^1 _normal_
*   ryu ^1 _normal_
*   serde ^1.0.55 _normal_
*   bstr ^1.7.0 _dev_
*   serde ^1.0.55 _dev_

*   Versions

*   **100%** of the crate is documented

*   Platform
*   i686-pc-windows-msvc
*   i686-unknown-linux-gnu
*   x86\_64-apple-darwin
*   x86\_64-pc-windows-msvc
*   x86\_64-unknown-linux-gnu
*   Feature flags

*   docs.rs
*   About docs.rs
*   Privacy policy

*   Rust
*   Rust website
*   The Book
*   Standard Library API Reference
*   Rust by Example
*   The Cargo Guide
*   Clippy Documentation

Crate csvCopy item path
=======================

Source

Expand description

The `csv` crate provides a fast and flexible CSV reader and writer, with support for Serde.

The tutorial is a good place to start if you’re new to Rust.

The cookbook will give you a variety of complete Rust programs that do CSV reading and writing.

§Brief overview
---------------

**If you’re new to Rust**, you might find the tutorial to be a good place to start.

The primary types in this crate are `Reader` and `Writer`, for reading and writing CSV data respectively. Correspondingly, to support CSV data with custom field or record delimiters (among many other things), you should use either a `ReaderBuilder` or a `WriterBuilder`, depending on whether you’re reading or writing CSV data.

Unless you’re using Serde, the standard CSV record types are `StringRecord` and `ByteRecord`. `StringRecord` should be used when you know your data to be valid UTF-8. For data that may be invalid UTF-8, `ByteRecord` is suitable.

Finally, the set of errors is described by the `Error` type.

The rest of the types in this crate mostly correspond to more detailed errors, position information, configuration knobs or iterator types.

§Setup
------

Run `cargo add csv` to add the latest version of the `csv` crate to your Cargo.toml.

If you want to use Serde’s custom derive functionality on your custom structs, then run `cargo add serde --features derive` to add the `serde` crate with its `derive` feature enabled to your `Cargo.toml`.

§Example
--------

This example shows how to read CSV data from stdin and print each record to stdout.

There are more examples in the cookbook.

```
use std::{error::Error, io, process};

fn example() -> Result<(), Box<dyn Error>> {
// Build the CSV reader and iterate over each record.
let mut rdr = csv::Reader::from_reader(io::stdin());
for result in rdr.records() {
// The iterator yields Result<StringRecord, Error>, so we check the
// error here.
let record = result?;
println!("{:?}", record);
}
Ok(())
}

fn main() {
if let Err(err) = example() {
println!("error running example: {}", err);
process::exit(1);
}
}
```

The above example can be run like so:

ⓘ

```
$ git clone git://github.com/BurntSushi/rust-csv
$ cd rust-csv
$ cargo run --example cookbook-read-basic < examples/data/smallpop.csv
```

§Example with Serde
-------------------

This example shows how to read CSV data from stdin into your own custom struct. By default, the member names of the struct are matched with the values in the header record of your CSV data.

```
use std::{error::Error, io, process};

#[derive(Debug, serde::Deserialize)]
struct Record {
city: String,
region: String,
country: String,
population: Option<u64>,
}

fn example() -> Result<(), Box<dyn Error>> {
let mut rdr = csv::Reader::from_reader(io::stdin());
for result in rdr.deserialize() {
// Notice that we need to provide a type hint for automatic
// deserialization.
let record: Record = result?;
println!("{:?}", record);
}
Ok(())
}

fn main() {
if let Err(err) = example() {
println!("error running example: {}", err);
process::exit(1);
}
}
```

The above example can be run like so:

ⓘ

```
$ git clone git://github.com/BurntSushi/rust-csv
$ cd rust-csv
$ cargo run --example cookbook-read-serde < examples/data/smallpop.csv
```

Modules§
--------

cookbook

A cookbook of examples for CSV reading and writing.

tutorial

A tutorial for handling CSV data in Rust.

Structs§
--------

ByteRecord

A single CSV record stored as raw bytes.

ByteRecordIter

A double-ended iterator over the fields in a byte record.

ByteRecordsIntoIter

An owned iterator over records as raw bytes.

ByteRecordsIter

A borrowed iterator over records as raw bytes.

DeserializeError

An Serde deserialization error.

DeserializeRecordsIntoIter

An owned iterator over deserialized records.

DeserializeRecordsIter

A borrowed iterator over deserialized records.

Error

An error that can occur when processing CSV data.

FromUtf8Error

A UTF-8 validation error during record conversion.

IntoInnerError

`IntoInnerError` occurs when consuming a `Writer` fails.

Position

A position in CSV data.

Reader

A already configured CSV reader.

ReaderBuilder

Builds a CSV reader with various configuration knobs.

StringRecord

A single CSV record stored as valid UTF-8 bytes.

StringRecordIter

An iterator over the fields in a string record.

StringRecordsIntoIter

An owned iterator over records as strings.

StringRecordsIter

A borrowed iterator over records as strings.

Utf8Error

A UTF-8 validation error.

Writer

An already configured CSV writer.

WriterBuilder

Builds a CSV writer with various configuration knobs.

Enums§
------

DeserializeErrorKind

The type of a Serde deserialization error.

ErrorKind

The specific type of an error.

QuoteStyle

The quoting style to use when writing CSV data.

Terminator

A record terminator.

Trim

The whitespace preservation behaviour when reading CSV data.

Functions§
----------

invalid\_option

A custom Serde deserializer for possibly invalid `Option<T>` fields.

Type Aliases§
-------------

Result

A type alias for `Result<T, csv::Error>`.