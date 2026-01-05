Title: Parquet Tips – DuckDB

Description: Below is a collection of tips to help when dealing with Parquet files. Tips for Reading Parquet Files Use union*by_name When Loading Files with Different Schemas The union_by_name option can be used to unify the schema of files that have different or missing columns. For files that do not have certain columns, NULL values are filled in: SELECT * FROM read*parquet('flights*.parquet', union_by_name = true); Tips for Writing Parquet Files Using a glob pattern upon read or a Hive partitioning structure are good ways to transparently handle multiple files. Enabling PER_THREAD_OUTPUT If the final number of Parquet files is not important,…

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

Parquet Tips

Below is a collection of tips to help when dealing with Parquet files.

## Tips for Reading Parquet Files

### Use `union_by_name` When Loading Files with Different Schemas

The `union_by_name` option can be used to unify the schema of files that have different or missing columns. For files that do not have certain columns, `NULL` values are filled in:

```
SELECT *
FROM read_parquet('flights*.parquet', union_by_name = true);
```

## Tips for Writing Parquet Files

Using a glob pattern upon read or a Hive partitioning structure are good ways to transparently handle multiple files.

### Enabling `PER_THREAD_OUTPUT`

If the final number of Parquet files is not important, writing one file per thread can significantly improve performance:

```
COPY
(FROM generate_series(10_000_000))
TO 'test.parquet'
(FORMAT parquet, PER_THREAD_OUTPUT);
```

### Selecting a `ROW_GROUP_SIZE`

The `ROW_GROUP_SIZE` parameter specifies the minimum number of rows in a Parquet row group, with a minimum value equal to DuckDB's vector size, 2,048, and a default of 122,880. A Parquet row group is a partition of rows, consisting of a column chunk for each column in the dataset.

Compression algorithms are only applied per row group, so the larger the row group size, the more opportunities to compress the data. DuckDB can read Parquet row groups in parallel even within the same file and uses predicate pushdown to only scan the row groups whose metadata ranges match the `WHERE` clause of the query. However there is some overhead associated with reading the metadata in each group. A good approach would be to ensure that within each file, the total number of row groups is at least as large as the number of CPU threads used to query that file. More row groups beyond the thread count would improve the speed of highly selective queries, but slow down queries that must scan the whole file like aggregations.

To write a query to a Parquet file with a different row group size, run:

```
COPY
(FROM generate_series(100_000))
TO 'row-groups.parquet'
(FORMAT parquet, ROW_GROUP_SIZE 100_000);
```

### The `ROW_GROUPS_PER_FILE` Option

The `ROW_GROUPS_PER_FILE` parameter creates a new Parquet file if the current one has a specified number of row groups.

```
COPY
(FROM generate_series(100_000))
TO 'output-directory'
(FORMAT parquet, ROW_GROUP_SIZE 20_000, ROW_GROUPS_PER_FILE 2);
```

> If multiple threads are active, the number of row groups in a file may slightly exceed the specified number of row groups to limit the amount of locking – similarly to the behaviour of `FILE_SIZE_BYTES`. However, if `PER_THREAD_OUTPUT` is set, only one thread writes to each file, and it becomes accurate again.

See the Performance Guide on “File Formats” for more tips.

##### About this page

- Report content issue
- See this page as Markdown
- Edit this page on GitHub

© 2025 DuckDB Foundation, Amsterdam NL

Code of Conduct Trademark Use

##### In this article

- Tips for Reading Parquet Files
- Use union_by_name When Loading Files with Different Schemas
- Tips for Writing Parquet Files
- Enabling PER_THREAD_OUTPUT
- Selecting a ROW_GROUP_SIZE
- The ROW_GROUPS_PER_FILE Option
