Apache DataFusion
Crates.io Apache licensed Build Status Commit Activity Open Issues Discord chat Linkedin Crates.io MSRV

Website | API Docs | Chat

logo
DataFusion is an extensible query engine written in Rust that uses Apache Arrow as its in-memory format.

This crate provides libraries and binaries for developers building fast and feature rich database and analytic systems, customized to particular workloads. See use cases for examples. The following related subprojects target end users:

DataFusion Python offers a Python interface for SQL and DataFrame queries.
DataFusion Comet is an accelerator for Apache Spark based on DataFusion.
"Out of the box," DataFusion offers [SQL] and [Dataframe] APIs, excellent performance, built-in support for CSV, Parquet, JSON, and Avro, extensive customization, and a great community.

DataFusion features a full query planner, a columnar, streaming, multi-threaded, vectorized execution engine, and partitioned data sources. You can customize DataFusion at almost all points including additional data sources, query languages, functions, custom operators and more. See the Architecture section for more details.

Here are links to some important information

Project Site
Installation
Rust Getting Started
Rust DataFrame API
Rust API docs
Rust Examples
Python DataFrame API
Architecture
What can you do with this crate?
DataFusion is great for building projects such as domain specific query engines, new database platforms and data pipelines, query languages and more. It lets you start quickly from a fully working engine, and then customize those features specific to your use. Click Here to see a list known users.

Contributing to DataFusion
Please see the contributor guide and communication pages for more information.

Crate features
This crate has several features which can be specified in your Cargo.toml.

Default features:

nested_expressions: functions for working with nested type function such as array_to_string
compression: reading files compressed with xz2, bzip2, flate2, and zstd
crypto_expressions: cryptographic functions such as md5 and sha256
datetime_expressions: date and time functions such as to_timestamp
encoding_expressions: encode and decode functions
parquet: support for reading the Apache Parquet format
regex_expressions: regular expression functions, such as regexp_match
unicode_expressions: Include unicode aware functions such as character_length
unparser: enables support to reverse LogicalPlans back into SQL
recursive_protection: uses recursive for stack overflow protection.
Optional features:

avro: support for reading the Apache Avro format
backtrace: include backtrace information in error messages
parquet_encryption: support for using Parquet Modular Encryption
pyarrow: conversions between PyArrow and DataFusion types
serde: enable arrow-schema's serde feature
DataFusion API Evolution and Deprecation Guidelines
Public methods in Apache DataFusion evolve over time: while we try to maintain a stable API, we also improve the API over time. As a result, we typically deprecate methods before removing them, according to the deprecation guidelines.

Dependencies and Cargo.lock
Following the guidance on committing Cargo.lock files, this project commits its Cargo.lock file.

CI uses the committed Cargo.lock file, and dependencies are updated regularly using Dependabot PRs.
