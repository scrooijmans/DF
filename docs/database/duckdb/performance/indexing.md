Title: Indexing – DuckDB

Description: DuckDB has two types of indexes: zonemaps and ART indexes. Zonemaps DuckDB automatically creates zonemaps (also known as min-max indexes) for the columns of all general-purpose data types. Operations like predicate pushdown into scan operators and computing aggregations use zonemaps. If a filter criterion (like WHERE column1 = 123) is in use, DuckDB can skip any row group whose min-max range does not contain that filter value (e.g., it can omit a block with a min-max range of 1000 to 2000 when comparing for = 123 or < 400). The Effect of Ordering on Zonemaps The more ordered the data…

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

Documentation / Guides / Performance

Indexing

DuckDB has two types of indexes: zonemaps and ART indexes.

## Zonemaps

DuckDB automatically creates zonemaps (also known as min-max indexes) for the columns of all general-purpose data types. Operations like predicate pushdown into scan operators and computing aggregations use zonemaps. If a filter criterion (like `WHERE column1 = 123`) is in use, DuckDB can skip any row group whose min-max range does not contain that filter value (e.g., it can omit a block with a min-max range of 1000 to 2000 when comparing for `= 123` or `< 400`).

### The Effect of Ordering on Zonemaps

The more ordered the data within a column, the more valuable the zonemap indexes will be. For example, a column could contain a random number on every row in the worst case. Then, DuckDB will likely be unable to skip any row groups. If you query specific columns with selective filters, it is best to pre-order data by those columns when inserting it. Even an imperfect ordering will still be helpful. The best case of ordered data commonly arises with `DATETIME` columns.

### Microbenchmark: The Effect of Ordering

For an example, let’s repeat the microbenchmark for timestamps with an ordered timestamp column using an ascending order vs. an unordered one.

| Column type | Ordered | Storage size | Query time |
| ----------- | ------- | ------------ | ---------- |
| `DATETIME`  | yes     | 1.3 GB       | 0.6 s      |
| `DATETIME`  | no      | 3.3 GB       | 0.9 s      |

The results show that simply keeping the column order allows for improved compression, yielding a 2.5× smaller storage size. It also allows the computation to be 1.5× faster.

### Ordered Integers

Another practical way to exploit ordering is to use the `INTEGER` type with automatic increments rather than `UUID` for columns queried using selective filters. In a scenario where a table contains out-of-order `UUID`s, DuckDB has to scan many row groups to find a specific `UUID` value. An ordered `INTEGER` column allows skipping all row groups except those containing the value.

## ART Indexes

DuckDB allows defining Adaptive Radix Tree (ART) indexes in two ways. First, such an index is created implicitly for columns with `PRIMARY KEY`, `FOREIGN KEY`, and `UNIQUE` constraints. Second, explicitly running the `CREATE INDEX` statement creates an ART index on the target column(s).

The tradeoffs of having an ART index on a column are as follows:

1.  ART indexes enable constraint checking during changes (inserts, updates, and deletes).
2.  Changes on indexed tables perform worse than their non-indexed counterparts. That is because of index maintenance for these operations.
3.  For some use cases, _single-column ART indexes_ improve the performance of highly selective queries using the indexed column.

An ART index does not affect the performance of join, aggregation, and sorting queries.

### ART Index Scans

ART index scans probe a single-column ART index for the requested data instead of scanning a table sequentially. Probing can improve the performance of some queries. DuckDB will try to use an index scan for equality and `IN(...)` conditions. It also pushes dynamic filters, e.g., from hash joins, into the scan, allowing dynamic index scans on these filters.

Indexes are only eligible for index scans if they index a single column without expressions. E.g., the following index is eligible for index scans:

```
CREATE INDEX idx ON tbl (col1);
```

E.g., the following two indexes are **NOT** eligible for index scans:

```
CREATE INDEX idx_multi_column ON tbl (col1, col2);
CREATE INDEX idx_expr ON tbl (col1 + 1);
```

The default threshold for index scans is `MAX(2048, 0.001 * table_cardinality)`. You can configure this threshold via `index_scan_percentage` and `index_scan_max_count`, or disable them by setting these values to zero. When in doubt, use `EXPLAIN ANALYZE` to verify that your query plan uses the index scan.

### Indexes and Memory

DuckDB registers index memory through its buffer manager. However, these index buffers are not yet buffer-managed. That means DuckDB does not yet destroy any index buffers if it has to evict memory. Thus, indexes can take up a significant portion of DuckDB's available memory, potentially affecting the performance of memory-intensive queries. Re-attaching (`DETACH` + `ATTACH`) the database containing indexes can mitigate this effect, as we deserialize index memory lazily. Disabling index scans and re-attaching after changes can further decrease the impact of indexes on DuckDB's available memory.

### Indexes and Opening Databases

Indexes are serialized to disk and deserialized lazily, i.e., when reopening the database. Operations using the index will only load the required parts of the index. Therefore, having an index will not cause any slowdowns when opening an existing database.

> Bestpractice We recommend following these guidelines:
>
> - Only use primary keys, foreign keys, or unique constraints, if these are necessary for enforcing constraints on your data.
> - Do not define explicit indexes unless you have highly selective queries and enough memory available.
> - If you define an ART index, do so after bulk loading the data to the table. Adding an index prior to loading, either explicitly or via primary/foreign keys, is detrimental to load performance.

##### About this page

- Report content issue
- See this page as Markdown
- Edit this page on GitHub

© 2025 DuckDB Foundation, Amsterdam NL

Code of Conduct Trademark Use

##### In this article

- Zonemaps
- The Effect of Ordering on Zonemaps
- Microbenchmark: The Effect of Ordering
- Ordered Integers
- ART Indexes
- ART Index Scans
- Indexes and Memory
- Indexes and Opening Databases
