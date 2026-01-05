Title: Querying Parquet Metadata – DuckDB

Description: Parquet Metadata The parquet_metadata function can be used to query the metadata contained within a Parquet file, which reveals various internal details of the Parquet file such as the statistics of the different columns. This can be useful for figuring out what kind of skipping is possible in Parquet files, or even to obtain a quick overview of what the different columns contain: SELECT \* FROM parquet_metadata('test.parquet'); Below is a table of the columns returned by parquet_metadata. Field Type file_name VARCHAR row_group_id BIGINT row_group_num_rows BIGINT row_group_num_columns BIGINT row_group_bytes BIGINT column_id BIGINT file_offset BIGINT num_values BIGINT path_in_schema VARCHAR type VARCHAR stats_min…

Search Shortcut cmd + k | ctrl + k

- Installation
- Documentation

- Getting Started
- Connect

- Overview
- Concurrency

- Data Import

- Overview
- Data Sources
- CSV Files

- Overview
- Auto Detection
- Reading Faulty CSV Files
- Tips

- JSON Files

- Overview
- Creating JSON
- Loading JSON
- Writing JSON
- JSON Type
- JSON Functions
- Format Settings
- Installing and Loading
- SQL to / from JSON
- Caveats

- Multiple Files

- Overview
- Combining Schemas

- Parquet Files

- Overview
- Metadata
- Encryption
- Tips

- Partitioning

- Hive Partitioning
- Partitioned Writes

- Appender
- INSERT Statements

- Client APIs

- Overview
- Tertiary Clients
- ADBC
- C

- Overview
- Startup
- Configuration
- Query
- Data Chunks
- Vectors
- Values
- Types
- Prepared Statements
- Appender
- Table Functions
- Replacement Scans
- API Reference

- C++
- CLI

- Overview
- Arguments
- Dot Commands
- Output Formats
- Editing
- Safe Mode
- Autocomplete
- Syntax Highlighting
- Known Issues

- Dart
- Go
- Java (JDBC)
- Julia
- Node.js (Deprecated)

- Overview
- API Reference

- Node.js (Neo)

- Overview

- ODBC

- Overview
- Linux Setup
- Windows Setup
- macOS Setup
- Configuration

- PHP
- Python

- Overview
- Data Ingestion
- Conversion between DuckDB and Python
- DB API
- Relational API
- Function API
- Types API
- Expression API
- Spark API
- API Reference
- Known Python Issues

- R
- Rust
- Swift
- Wasm

- Overview
- Deploying DuckDB-Wasm
- Instantiation
- Data Ingestion
- Query
- Extensions

- SQL

- Introduction
- Statements

- Overview
- ANALYZE
- ALTER DATABASE
- ALTER TABLE
- ALTER VIEW
- ATTACH and DETACH
- CALL
- CHECKPOINT
- COMMENT ON
- COPY
- CREATE INDEX
- CREATE MACRO
- CREATE SCHEMA
- CREATE SECRET
- CREATE SEQUENCE
- CREATE TABLE
- CREATE VIEW
- CREATE TYPE
- DELETE
- DESCRIBE
- DROP
- EXPORT and IMPORT DATABASE
- INSERT
- LOAD / INSTALL
- MERGE INTO
- PIVOT
- Profiling
- SELECT
- SET / RESET
- SET VARIABLE
- SUMMARIZE
- Transaction Management
- UNPIVOT
- UPDATE
- USE
- VACUUM

- Query Syntax

- SELECT
- FROM and JOIN
- WHERE
- GROUP BY
- GROUPING SETS
- HAVING
- ORDER BY
- LIMIT and OFFSET
- SAMPLE
- Unnesting
- WITH
- WINDOW
- QUALIFY
- VALUES
- FILTER
- Set Operations
- Prepared Statements

- Data Types

- Overview
- Array
- Bitstring
- Blob
- Boolean
- Date
- Enum
- Interval
- List
- Literal Types
- Map
- NULL Values
- Numeric
- Struct
- Text
- Time
- Timestamp
- Time Zones
- Union
- Typecasting

- Expressions

- Overview
- CASE Expression
- Casting
- Collations
- Comparisons
- IN Operator
- Logical Operators
- Star Expression
- Subqueries
- TRY

- Functions

- Overview
- Aggregate Functions
- Array Functions
- Bitstring Functions
- Blob Functions
- Date Format Functions
- Date Functions
- Date Part Functions
- Enum Functions
- Interval Functions
- Lambda Functions
- List Functions
- Map Functions
- Nested Functions
- Numeric Functions
- Pattern Matching
- Regular Expressions
- Struct Functions
- Text Functions
- Time Functions
- Timestamp Functions
- Timestamp with Time Zone Functions
- Union Functions
- Utility Functions
- Window Functions

- Constraints
- Indexes
- Meta Queries

- Information Schema
- Metadata Functions

- DuckDB's SQL Dialect

- Overview
- Indexing
- Friendly SQL
- Keywords and Identifiers
- Order Preservation
- PostgreSQL Compatibility
- SQL Quirks

- Samples

- Configuration

- Overview
- Pragmas
- Secrets Manager

- Extensions

- Overview
- Installing Extensions
- Advanced Installation Methods
- Distributing Extensions
- Versioning of Extensions
- Troubleshooting of Extensions

- Core Extensions

- Overview
- AutoComplete
- Avro
- AWS
- Azure
- Delta
- DuckLake
- Encodings
- Excel
- Full Text Search
- httpfs (HTTP and S3)

- Overview
- HTTP(S) Support
- Hugging Face
- S3 API Support
- Legacy Authentication Scheme for S3 API

- Iceberg

- Overview
- Iceberg REST Catalogs
- Amazon S3 Tables
- Amazon SageMaker Lakehouse (AWS Glue)
- Troubleshooting

- ICU
- inet
- jemalloc
- MySQL
- PostgreSQL
- Spatial

- Overview
- Function Reference
- R-Tree Indexes
- GDAL Integration

- SQLite
- TPC-DS
- TPC-H
- UI
- VSS

- Guides

- Overview
- Data Viewers

- Tableau
- CLI Charting with YouPlot

- Database Integration

- Overview
- MySQL Import
- PostgreSQL Import
- SQLite Import

- File Formats

- Overview
- CSV Import
- CSV Export
- Directly Reading Files
- Excel Import
- Excel Export
- JSON Import
- JSON Export
- Parquet Import
- Parquet Export
- Querying Parquet Files
- File Access with the file: Protocol

- Network and Cloud Storage

- Overview
- HTTP Parquet Import
- S3 Parquet Import
- S3 Parquet Export
- S3 Iceberg Import
- S3 Express One
- GCS Import
- Cloudflare R2 Import
- DuckDB over HTTPS / S3
- Fastly Object Storage Import

- Meta Queries

- Describe Table
- EXPLAIN: Inspect Query Plans
- EXPLAIN ANALYZE: Profile Queries
- List Tables
- Summarize
- DuckDB Environment

- ODBC

- ODBC Guide

- Performance

- Overview
- Environment
- Import
- Schema
- Indexing
- Join Operations
- File Formats
- How to Tune Workloads
- My Workload Is Slow
- Benchmarks
- Working with Huge Databases

- Python

- Installation
- Executing SQL
- Jupyter Notebooks
- marimo Notebooks
- SQL on Pandas
- Import from Pandas
- Export to Pandas
- Import from Numpy
- Export to Numpy
- SQL on Arrow
- Import from Arrow
- Export to Arrow
- Relational API on Pandas
- Multiple Python Threads
- Integration with Ibis
- Integration with Polars
- Using fsspec Filesystems

- SQL Editors

- DBeaver SQL IDE

- SQL Features

- AsOf Join
- Full-Text Search
- query and query_table Functions
- Timestamp Issues

- Snippets

- Creating Synthetic Data
- Dutch Railway Datasets
- Sharing Macros
- Analyzing a Git Repository
- Importing Duckbox Tables
- Copying an In-Memory Database to a File

- Troubleshooting

- Crashes
- Out of Memory Errors

- Glossary of Terms
- Browsing Offline

- Operations Manual

- Overview
- DuckDB's Footprint

- Files Created by DuckDB
- Gitignore for DuckDB
- Reclaiming Space

- Logging

- Overview

- Securing DuckDB

- Overview
- Embedding DuckDB
- Securing Extensions

- Non-Deterministic Behavior
- Limits
- DuckDB Docker Container

- Development

- DuckDB Repositories
- Profiling
- Building DuckDB

- Overview
- Build Configuration
- Building Extensions
- Android
- Linux
- macOS
- Raspberry Pi
- Windows
- Python
- R
- Troubleshooting
- Unofficial and Unsupported Platforms

- Benchmark Suite
- Testing

- Overview
- sqllogictest Introduction
- Writing Tests
- Debugging
- Result Verification
- Persistent Testing
- Loops
- Multiple Connections
- Catch

- Internals

- Overview
- Storage Versions and Format
- Execution Format
- Pivot

- Sitemap
- Live Demo

Documentation / Data Import / Parquet Files

Querying Parquet Metadata

## Parquet Metadata

The `parquet_metadata` function can be used to query the metadata contained within a Parquet file, which reveals various internal details of the Parquet file such as the statistics of the different columns. This can be useful for figuring out what kind of skipping is possible in Parquet files, or even to obtain a quick overview of what the different columns contain:

```
SELECT *
FROM parquet_metadata('test.parquet');
```

Below is a table of the columns returned by `parquet_metadata`.

| Field                      | Type            |
| -------------------------- | --------------- |
| file_name                  | VARCHAR         |
| row_group_id               | BIGINT          |
| row_group_num_rows         | BIGINT          |
| row_group_num_columns      | BIGINT          |
| row_group_bytes            | BIGINT          |
| column_id                  | BIGINT          |
| file_offset                | BIGINT          |
| num_values                 | BIGINT          |
| path_in_schema             | VARCHAR         |
| type                       | VARCHAR         |
| stats_min                  | VARCHAR         |
| stats_max                  | VARCHAR         |
| stats_null_count           | BIGINT          |
| stats_distinct_count       | BIGINT          |
| stats_min_value            | VARCHAR         |
| stats_max_value            | VARCHAR         |
| compression                | VARCHAR         |
| encodings                  | VARCHAR         |
| index_page_offset          | BIGINT          |
| dictionary_page_offset     | BIGINT          |
| data_page_offset           | BIGINT          |
| total_compressed_size      | BIGINT          |
| total_uncompressed_size    | BIGINT          |
| key_value_metadata         | MAP(BLOB, BLOB) |
| bloom_filter_offset        | BIGINT          |
| bloom_filter_length        | BIGINT          |
| min_is_exact               | BOOLEAN         |
| max_is_exact               | BOOLEAN         |
| row_group_compressed_bytes | BIGINT          |

## Parquet Schema

The `parquet_schema` function can be used to query the internal schema contained within a Parquet file. Note that this is the schema as it is contained within the metadata of the Parquet file. If you want to figure out the column names and types contained within a Parquet file it is easier to use `DESCRIBE`.

Fetch the column names and column types:

```
DESCRIBE SELECT * FROM 'test.parquet';
```

Fetch the internal schema of a Parquet file:

```
SELECT *
FROM parquet_schema('test.parquet');
```

Below is a table of the columns returned by `parquet_schema`.

| Field           | Type    |
| --------------- | ------- |
| file_name       | VARCHAR |
| name            | VARCHAR |
| type            | VARCHAR |
| type_length     | VARCHAR |
| repetition_type | VARCHAR |
| num_children    | BIGINT  |
| converted_type  | VARCHAR |
| scale           | BIGINT  |
| precision       | BIGINT  |
| field_id        | BIGINT  |
| logical_type    | VARCHAR |

## Parquet File Metadata

The `parquet_file_metadata` function can be used to query file-level metadata such as the format version and the encryption algorithm used:

```
SELECT *
FROM parquet_file_metadata('test.parquet');
```

Below is a table of the columns returned by `parquet_file_metadata`.

| Field                       | Type    |
| --------------------------- | ------- |
| file_name                   | VARCHAR |
| created_by                  | VARCHAR |
| num_rows                    | BIGINT  |
| num_row_groups              | BIGINT  |
| format_version              | BIGINT  |
| encryption_algorithm        | VARCHAR |
| footer_signing_key_metadata | VARCHAR |

## Parquet Key-Value Metadata

The `parquet_kv_metadata` function can be used to query custom metadata defined as key-value pairs:

```
SELECT *
FROM parquet_kv_metadata('test.parquet');
```

Below is a table of the columns returned by `parquet_kv_metadata`.

| Field     | Type    |
| --------- | ------- |
| file_name | VARCHAR |
| key       | BLOB    |
| value     | BLOB    |

## Bloom Filters

DuckDB supports Bloom filters for pruning the row groups that need to be read to answer highly selective queries. Currently, Bloom filters are supported for the following types:

- Integer types: `TINYINT`, `UTINYINT`, `SMALLINT`, `USMALLINT`, `INTEGER`, `UINTEGER`, `BIGINT`, `UBIGINT`
- Floating point types: `FLOAT`, `DOUBLE`
- `VARCHAR`
- `BLOB`

The `parquet_bloom_probe(filename, column_name, value)` function shows which row groups can excluded when filtering for a given value of a given column using the Bloom filter. For example:

```
FROM parquet_bloom_probe('my_file.parquet', 'my_col', 500);
```

| file_name       | row_group_id | bloom_filter_excludes |
| --------------- | ------------ | --------------------- |
| my_file.parquet | 0            | true                  |
| …               | …            | …                     |
| my_file.parquet | 9            | false                 |

##### About this page

- Report content issue
- See this page as Markdown
- Edit this page on GitHub

© 2025 DuckDB Foundation, Amsterdam NL

Code of Conduct Trademark Use

##### In this article

- Parquet Metadata
- Parquet Schema
- Parquet File Metadata
- Parquet Key-Value Metadata
- Bloom Filters
