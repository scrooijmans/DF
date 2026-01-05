Title: Reading and Writing Parquet Files – DuckDB

Description: Examples Read a single Parquet file: SELECT _ FROM 'test.parquet'; Figure out which columns/types are in a Parquet file: DESCRIBE SELECT _ FROM 'test.parquet'; Create a table from a Parquet file: CREATE TABLE test AS SELECT _ FROM 'test.parquet'; If the file does not end in .parquet, use the read_parquet function: SELECT _ FROM read_parquet('test.parq'); Use list parameter to read three Parquet files and treat them as a single table: SELECT _ FROM read_parquet(['file1.parquet', 'file2.parquet', 'file3.parquet']); Read all files that match the glob pattern: SELECT _ FROM 'test/\*.parquet'; Read all files that match the glob pattern, and include the filename…

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

Reading and Writing Parquet Files

## Examples

Read a single Parquet file:

```
SELECT * FROM 'test.parquet';
```

Figure out which columns/types are in a Parquet file:

```
DESCRIBE SELECT * FROM 'test.parquet';
```

Create a table from a Parquet file:

```
CREATE TABLE test AS
SELECT * FROM 'test.parquet';
```

If the file does not end in `.parquet`, use the `read_parquet` function:

```
SELECT *
FROM read_parquet('test.parq');
```

Use list parameter to read three Parquet files and treat them as a single table:

```
SELECT *
FROM read_parquet(['file1.parquet', 'file2.parquet', 'file3.parquet']);
```

Read all files that match the glob pattern:

```
SELECT *
FROM 'test/*.parquet';
```

Read all files that match the glob pattern, and include the `filename` virtual column that specifies which file each row came from (this column is available by default without a configuration options since DuckDB v1.3.0):

```
SELECT *, filename
FROM read_parquet('test/*.parquet');
```

Use a list of globs to read all Parquet files from two specific folders:

```
SELECT *
FROM read_parquet(['folder1/*.parquet', 'folder2/*.parquet']);
```

Read over HTTPS:

```
SELECT *
FROM read_parquet('https://some.url/some_file.parquet');
```

Query the metadata of a Parquet file:

```
SELECT *
FROM parquet_metadata('test.parquet');
```

Query the file metadata of a Parquet file:

```
SELECT *
FROM parquet_file_metadata('test.parquet');
```

Query the key-value metadata of a Parquet file:

```
SELECT *
FROM parquet_kv_metadata('test.parquet');
```

Query the schema of a Parquet file:

```
SELECT *
FROM parquet_schema('test.parquet');
```

Write the results of a query to a Parquet file using the default compression (Snappy):

```
COPY
(SELECT * FROM tbl)
TO 'result-snappy.parquet'
(FORMAT parquet);
```

Write the results from a query to a Parquet file with specific compression and row group size:

```
COPY
(FROM generate_series(100_000))
TO 'test.parquet'
(FORMAT parquet, COMPRESSION zstd, ROW_GROUP_SIZE 100_000);
```

Export the table contents of the entire database as parquet:

```
EXPORT DATABASE 'target_directory' (FORMAT parquet);
```

## Parquet Files

Parquet files are compressed columnar files that are efficient to load and process. DuckDB provides support for both reading and writing Parquet files in an efficient manner, as well as support for pushing filters and projections into the Parquet file scans.

> Parquet datasets differ based on the number of files, the size of individual files, the compression algorithm used, row group size, etc. These have a significant effect on performance. Please consult the Performance Guide for details.

## `read_parquet` Function

| Function                              | Description              | Example                                       |
| ------------------------------------- | ------------------------ | --------------------------------------------- |
| `read_parquet(path_or_list_of_paths)` | Read Parquet file(s)     | `SELECT * FROM read_parquet('test.parquet');` |
| `parquet_scan(path_or_list_of_paths)` | Alias for `read_parquet` | `SELECT * FROM parquet_scan('test.parquet');` |

If your file ends in `.parquet`, the function syntax is optional. The system will automatically infer that you are reading a Parquet file:

```
SELECT * FROM 'test.parquet';
```

Multiple files can be read at once by providing a glob or a list of files. Refer to the multiple files section for more information.

### Parameters

There are a number of options exposed that can be passed to the `read_parquet` function or the `COPY` statement.

| Name                | Description                                                                                                                                                                                                               | Type     | Default         |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- | --------------- |
| `binary_as_string`  | Parquet files generated by legacy writers do not correctly set the `UTF8` flag for strings, causing string columns to be loaded as `BLOB` instead. Set this to true to load binary columns as strings.                    | `BOOL`   | `false`         |
| `encryption_config` | Configuration for Parquet encryption.                                                                                                                                                                                     | `STRUCT` | \-              |
| `filename`          | Whether or not an extra `filename` column should be included in the result. Since DuckDB v1.3.0, the `filename` column is added automatically as a virtual column and this option is only kept for compatibility reasons. | `BOOL`   | `false`         |
| `file_row_number`   | Whether or not to include the `file_row_number` column.                                                                                                                                                                   | `BOOL`   | `false`         |
| `hive_partitioning` | Whether or not to interpret the path as a Hive partitioned path.                                                                                                                                                          | `BOOL`   | (auto-detected) |
| `union_by_name`     | Whether the columns of multiple schemas should be unified by name, rather than by position.                                                                                                                               | `BOOL`   | `false`         |

## Partial Reading

DuckDB supports projection pushdown into the Parquet file itself. That is to say, when querying a Parquet file, only the columns required for the query are read. This allows you to read only the part of the Parquet file that you are interested in. This will be done automatically by DuckDB.

DuckDB also supports filter pushdown into the Parquet reader. When you apply a filter to a column that is scanned from a Parquet file, the filter will be pushed down into the scan, and can even be used to skip parts of the file using the built-in zonemaps. Note that this will depend on whether or not your Parquet file contains zonemaps.

Filter and projection pushdown provide significant performance benefits. See our blog post “Querying Parquet with Precision Using DuckDB” for more information.

## Inserts and Views

You can also insert the data into a table or create a table from the Parquet file directly. This will load the data from the Parquet file and insert it into the database:

Insert the data from the Parquet file in the table:

```
INSERT INTO people
SELECT * FROM read_parquet('test.parquet');
```

Create a table directly from a Parquet file:

```
CREATE TABLE people AS
SELECT * FROM read_parquet('test.parquet');
```

If you wish to keep the data stored inside the Parquet file, but want to query the Parquet file directly, you can create a view over the `read_parquet` function. You can then query the Parquet file as if it were a built-in table:

Create a view over the Parquet file:

```
CREATE VIEW people AS
SELECT * FROM read_parquet('test.parquet');
```

Query the Parquet file:

```
SELECT * FROM people;
```

## Writing to Parquet Files

DuckDB also has support for writing to Parquet files using the `COPY` statement syntax. See the `COPY` Statement page for details, including all possible parameters for the `COPY` statement.

Write a query to a Snappy-compressed Parquet file:

```
COPY
(SELECT * FROM tbl)
TO 'result-snappy.parquet'
(FORMAT parquet);
```

Write `tbl` to a zstd-compressed Parquet file:

```
COPY tbl
TO 'result-zstd.parquet'
(FORMAT parquet, COMPRESSION zstd);
```

Write `tbl` to a zstd-compressed Parquet file with the lowest compression level yielding the fastest compression:

```
COPY tbl
TO 'result-zstd.parquet'
(FORMAT parquet, COMPRESSION zstd, COMPRESSION_LEVEL 1);
```

Write to Parquet file with key-value metadata:

```
COPY (
SELECT
42 AS number,
true AS is_even
) TO 'kv_metadata.parquet' (
FORMAT parquet,
KV_METADATA {
number: 'Answer to life, universe, and everything',
is_even: 'not ''odd''' -- single quotes in values must be escaped
}
);
```

Write a CSV file to an uncompressed Parquet file:

```
COPY
'test.csv'
TO 'result-uncompressed.parquet'
(FORMAT parquet, COMPRESSION uncompressed);
```

Write a query to a Parquet file with zstd-compression and row group size:

```
COPY
(FROM generate_series(100_000))
TO 'row-groups-zstd.parquet'
(FORMAT parquet, COMPRESSION zstd, ROW_GROUP_SIZE 100_000);
```

Write data to an LZ4-compressed Parquet file:

```
COPY
(FROM generate_series(100_000))
TO 'result-lz4.parquet'
(FORMAT parquet, COMPRESSION lz4);
```

Or, equivalently:

```
COPY
(FROM generate_series(100_000))
TO 'result-lz4.parquet'
(FORMAT parquet, COMPRESSION lz4_raw);
```

Write data to a Brotli-compressed Parquet file:

```
COPY
(FROM generate_series(100_000))
TO 'result-brotli.parquet'
(FORMAT parquet, COMPRESSION brotli);
```

To configure the page size of Parquet file's dictionary pages, use the `STRING_DICTIONARY_PAGE_SIZE_LIMIT` option (default: 1 MB):

```
COPY
lineitem
TO 'lineitem-with-custom-dictionary-size.parquet'
(FORMAT parquet, STRING_DICTIONARY_PAGE_SIZE_LIMIT 100_000);
```

DuckDB's `EXPORT` command can be used to export an entire database to a series of Parquet files. See the “`EXPORT` statement” page for more details:

Export the table contents of the entire database as Parquet:

```
EXPORT DATABASE 'target_directory' (FORMAT parquet);
```

## Encryption

DuckDB supports reading and writing encrypted Parquet files.

## Supported Features

The list of supported Parquet features is available in the Parquet documentation's “Implementation status” page.

## Installing and Loading the Parquet Extension

The support for Parquet files is enabled via extension. The `parquet` extension is bundled with almost all clients. However, if your client does not bundle the `parquet` extension, the extension must be installed separately:

```
INSTALL parquet;
```

## Pages in This Section

##### About this page

- Report content issue
- See this page as Markdown
- Edit this page on GitHub

© 2025 DuckDB Foundation, Amsterdam NL

Code of Conduct Trademark Use

##### In this article

- Examples
- Parquet Files
- read_parquet Function
- Parameters
- Partial Reading
- Inserts and Views
- Writing to Parquet Files
- Encryption
- Supported Features
- Installing and Loading the Parquet Extension
