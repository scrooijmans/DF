Title: Rust Client – DuckDB

Description: The latest stable version of the DuckDB Rust client is {{ site.current_duckdb_rust_version }}. Installation The DuckDB Rust client can be installed from crates.io. Please see the docs.rs for details. Basic API Usage duckdb-rs is an ergonomic wrapper based on the DuckDB C API, please refer to the README for details. Startup & Shutdown To use duckdb, you must first initialize a Connection handle using Connection::open(). Connection::open() takes as parameter the database file to read and write from. If the database file does not exist, it will be created (the file extension may be .db, .duckdb, or anything else). You can…

Search Shortcut cmd + k | ctrl + k

*   Installation
*   Documentation

*   Getting Started
*   Connect

*   Overview
*   Concurrency

*   Data Import

*   Overview
*   Data Sources
*   CSV Files

*   Overview
*   Auto Detection
*   Reading Faulty CSV Files
*   Tips

*   JSON Files

*   Overview
*   Creating JSON
*   Loading JSON
*   Writing JSON
*   JSON Type
*   JSON Functions
*   Format Settings
*   Installing and Loading
*   SQL to / from JSON
*   Caveats

*   Multiple Files

*   Overview
*   Combining Schemas

*   Parquet Files

*   Overview
*   Metadata
*   Encryption
*   Tips

*   Partitioning

*   Hive Partitioning
*   Partitioned Writes

*   Appender
*   INSERT Statements

*   Client APIs

*   Overview
*   Tertiary Clients
*   ADBC
*   C

*   Overview
*   Startup
*   Configuration
*   Query
*   Data Chunks
*   Vectors
*   Values
*   Types
*   Prepared Statements
*   Appender
*   Table Functions
*   Replacement Scans
*   API Reference

*   C++
*   CLI

*   Overview
*   Arguments
*   Dot Commands
*   Output Formats
*   Editing
*   Safe Mode
*   Autocomplete
*   Syntax Highlighting
*   Known Issues

*   Dart
*   Go
*   Java (JDBC)
*   Julia
*   Node.js (Deprecated)

*   Overview
*   API Reference

*   Node.js (Neo)

*   Overview

*   ODBC

*   Overview
*   Linux Setup
*   Windows Setup
*   macOS Setup
*   Configuration

*   PHP
*   Python

*   Overview
*   Data Ingestion
*   Conversion between DuckDB and Python
*   DB API
*   Relational API
*   Function API
*   Types API
*   Expression API
*   Spark API
*   API Reference
*   Known Python Issues

*   R
*   Rust
*   Swift
*   Wasm

*   Overview
*   Deploying DuckDB-Wasm
*   Instantiation
*   Data Ingestion
*   Query
*   Extensions

*   SQL

*   Introduction
*   Statements

*   Overview
*   ANALYZE
*   ALTER DATABASE
*   ALTER TABLE
*   ALTER VIEW
*   ATTACH and DETACH
*   CALL
*   CHECKPOINT
*   COMMENT ON
*   COPY
*   CREATE INDEX
*   CREATE MACRO
*   CREATE SCHEMA
*   CREATE SECRET
*   CREATE SEQUENCE
*   CREATE TABLE
*   CREATE VIEW
*   CREATE TYPE
*   DELETE
*   DESCRIBE
*   DROP
*   EXPORT and IMPORT DATABASE
*   INSERT
*   LOAD / INSTALL
*   MERGE INTO
*   PIVOT
*   Profiling
*   SELECT
*   SET / RESET
*   SET VARIABLE
*   SUMMARIZE
*   Transaction Management
*   UNPIVOT
*   UPDATE
*   USE
*   VACUUM

*   Query Syntax

*   SELECT
*   FROM and JOIN
*   WHERE
*   GROUP BY
*   GROUPING SETS
*   HAVING
*   ORDER BY
*   LIMIT and OFFSET
*   SAMPLE
*   Unnesting
*   WITH
*   WINDOW
*   QUALIFY
*   VALUES
*   FILTER
*   Set Operations
*   Prepared Statements

*   Data Types

*   Overview
*   Array
*   Bitstring
*   Blob
*   Boolean
*   Date
*   Enum
*   Interval
*   List
*   Literal Types
*   Map
*   NULL Values
*   Numeric
*   Struct
*   Text
*   Time
*   Timestamp
*   Time Zones
*   Union
*   Typecasting

*   Expressions

*   Overview
*   CASE Expression
*   Casting
*   Collations
*   Comparisons
*   IN Operator
*   Logical Operators
*   Star Expression
*   Subqueries
*   TRY

*   Functions

*   Overview
*   Aggregate Functions
*   Array Functions
*   Bitstring Functions
*   Blob Functions
*   Date Format Functions
*   Date Functions
*   Date Part Functions
*   Enum Functions
*   Interval Functions
*   Lambda Functions
*   List Functions
*   Map Functions
*   Nested Functions
*   Numeric Functions
*   Pattern Matching
*   Regular Expressions
*   Struct Functions
*   Text Functions
*   Time Functions
*   Timestamp Functions
*   Timestamp with Time Zone Functions
*   Union Functions
*   Utility Functions
*   Window Functions

*   Constraints
*   Indexes
*   Meta Queries

*   Information Schema
*   Metadata Functions

*   DuckDB's SQL Dialect

*   Overview
*   Indexing
*   Friendly SQL
*   Keywords and Identifiers
*   Order Preservation
*   PostgreSQL Compatibility
*   SQL Quirks

*   Samples

*   Configuration

*   Overview
*   Pragmas
*   Secrets Manager

*   Extensions

*   Overview
*   Installing Extensions
*   Advanced Installation Methods
*   Distributing Extensions
*   Versioning of Extensions
*   Troubleshooting of Extensions

*   Core Extensions

*   Overview
*   AutoComplete
*   Avro
*   AWS
*   Azure
*   Delta
*   DuckLake
*   Encodings
*   Excel
*   Full Text Search
*   httpfs (HTTP and S3)

*   Overview
*   HTTP(S) Support
*   Hugging Face
*   S3 API Support
*   Legacy Authentication Scheme for S3 API

*   Iceberg

*   Overview
*   Iceberg REST Catalogs
*   Amazon S3 Tables
*   Amazon SageMaker Lakehouse (AWS Glue)
*   Troubleshooting

*   ICU
*   inet
*   jemalloc
*   MySQL
*   PostgreSQL
*   Spatial

*   Overview
*   Function Reference
*   R-Tree Indexes
*   GDAL Integration

*   SQLite
*   TPC-DS
*   TPC-H
*   UI
*   VSS

*   Guides

*   Overview
*   Data Viewers

*   Tableau
*   CLI Charting with YouPlot

*   Database Integration

*   Overview
*   MySQL Import
*   PostgreSQL Import
*   SQLite Import

*   File Formats

*   Overview
*   CSV Import
*   CSV Export
*   Directly Reading Files
*   Excel Import
*   Excel Export
*   JSON Import
*   JSON Export
*   Parquet Import
*   Parquet Export
*   Querying Parquet Files
*   File Access with the file: Protocol

*   Network and Cloud Storage

*   Overview
*   HTTP Parquet Import
*   S3 Parquet Import
*   S3 Parquet Export
*   S3 Iceberg Import
*   S3 Express One
*   GCS Import
*   Cloudflare R2 Import
*   DuckDB over HTTPS / S3
*   Fastly Object Storage Import

*   Meta Queries

*   Describe Table
*   EXPLAIN: Inspect Query Plans
*   EXPLAIN ANALYZE: Profile Queries
*   List Tables
*   Summarize
*   DuckDB Environment

*   ODBC

*   ODBC Guide

*   Performance

*   Overview
*   Environment
*   Import
*   Schema
*   Indexing
*   Join Operations
*   File Formats
*   How to Tune Workloads
*   My Workload Is Slow
*   Benchmarks
*   Working with Huge Databases

*   Python

*   Installation
*   Executing SQL
*   Jupyter Notebooks
*   marimo Notebooks
*   SQL on Pandas
*   Import from Pandas
*   Export to Pandas
*   Import from Numpy
*   Export to Numpy
*   SQL on Arrow
*   Import from Arrow
*   Export to Arrow
*   Relational API on Pandas
*   Multiple Python Threads
*   Integration with Ibis
*   Integration with Polars
*   Using fsspec Filesystems

*   SQL Editors

*   DBeaver SQL IDE

*   SQL Features

*   AsOf Join
*   Full-Text Search
*   query and query\_table Functions
*   Timestamp Issues

*   Snippets

*   Creating Synthetic Data
*   Dutch Railway Datasets
*   Sharing Macros
*   Analyzing a Git Repository
*   Importing Duckbox Tables
*   Copying an In-Memory Database to a File

*   Troubleshooting

*   Crashes
*   Out of Memory Errors

*   Glossary of Terms
*   Browsing Offline

*   Operations Manual

*   Overview
*   DuckDB's Footprint

*   Files Created by DuckDB
*   Gitignore for DuckDB
*   Reclaiming Space

*   Logging

*   Overview

*   Securing DuckDB

*   Overview
*   Embedding DuckDB
*   Securing Extensions

*   Non-Deterministic Behavior
*   Limits
*   DuckDB Docker Container

*   Development

*   DuckDB Repositories
*   Profiling
*   Building DuckDB

*   Overview
*   Build Configuration
*   Building Extensions
*   Android
*   Linux
*   macOS
*   Raspberry Pi
*   Windows
*   Python
*   R
*   Troubleshooting
*   Unofficial and Unsupported Platforms

*   Benchmark Suite
*   Testing

*   Overview
*   sqllogictest Introduction
*   Writing Tests
*   Debugging
*   Result Verification
*   Persistent Testing
*   Loops
*   Multiple Connections
*   Catch

*   Internals

*   Overview
*   Storage Versions and Format
*   Execution Format
*   Pivot

*   Sitemap
*   Live Demo

Documentation / Client APIs  

Rust Client

> The latest stable version of the DuckDB Rust client is 1.4.0.

Installation
------------

The DuckDB Rust client can be installed from crates.io. Please see the docs.rs for details.

Basic API Usage
---------------

duckdb-rs is an ergonomic wrapper based on the DuckDB C API, please refer to the README for details.

### Startup & Shutdown

To use duckdb, you must first initialize a `Connection` handle using `Connection::open()`. `Connection::open()` takes as parameter the database file to read and write from. If the database file does not exist, it will be created (the file extension may be `.db`, `.duckdb`, or anything else). You can also use `Connection::open_in_memory()` to create an **in-memory database**. Note that for an in-memory database no data is persisted to disk (i.e., all data is lost when you exit the process).

```
use duckdb::{params, Connection, Result};
let conn = Connection::open_in_memory()?;
```

The `Connection` will automatically close the underlying db connection for you when it goes out of scope (via `Drop`). You can also explicitly close the `Connection` with `conn.close()`. This is not much difference between these in the typical case, but in case there is an error, you'll have the chance to handle it with the explicit close.

### Querying

SQL queries can be sent to DuckDB using the `execute()` method of connections, or we can also prepare the statement and then query on that.

```
#[derive(Debug)]
struct Person {
id: i32,
name: String,
data: Option<Vec<u8>>,
}

conn.execute(
"INSERT INTO person (name, data) VALUES (?, ?)",
params![me.name, me.data],
)?;

let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
let person_iter = stmt.query_map([], |row| {
Ok(Person {
id: row.get(0)?,
name: row.get(1)?,
data: row.get(2)?,
})
})?;

for person in person_iter {
println!("Found person {:?}", person.unwrap());
}
```

Appender
--------

The Rust client supports the DuckDB Appender API for bulk inserts. For example:

```
fn insert_rows(conn: &Connection) -> Result<()> {
let mut app = conn.appender("foo")?;
app.append_rows([[1, 2], [3, 4], [5, 6], [7, 8], [9, 10]])?;
Ok(())
}
```

##### About this page

*   See the repository of this feature on GitHub
*   Report content issue
*   See this page as Markdown
*   Edit this page on GitHub

© 2025 DuckDB Foundation, Amsterdam NL

Code of Conduct Trademark Use

##### In this article

*   Installation
*   Basic API Usage
*   Startup & Shutdown
*   Querying
*   Appender