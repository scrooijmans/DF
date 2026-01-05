Title: Environment – DuckDB

Description: The environment where DuckDB is run has an obvious impact on performance. This page focuses on the effects of the hardware configuration and the operating system used. Hardware Configuration CPU DuckDB works efficiently on both AMD64 (x86_64) and ARM64 (AArch64) CPU architectures. Memory Bestpractice Aim for 1-4 GB memory per thread. Minimum Required Memory As a rule of thumb, DuckDB requires a minimum of 125 MB of memory per thread. For example, if you use 8 threads, you need at least 1 GB of memory. If you are working in a memory-constrained environment, consider limiting the number of threads, e.g.,…

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

Environment

The environment where DuckDB is run has an obvious impact on performance. This page focuses on the effects of the hardware configuration and the operating system used.

## Hardware Configuration

### CPU

DuckDB works efficiently on both AMD64 (x86_64) and ARM64 (AArch64) CPU architectures.

### Memory

> Bestpractice Aim for 1-4 GB memory per thread.

#### Minimum Required Memory

As a rule of thumb, DuckDB requires a _minimum_ of 125 MB of memory per thread. For example, if you use 8 threads, you need at least 1 GB of memory. If you are working in a memory-constrained environment, consider limiting the number of threads, e.g., by issuing:

```
SET threads = 4;
```

#### Memory for Ideal Performance

The amount of memory required for ideal performance depends on several factors, including the dataset size and the queries to execute. Maybe surprisingly, the _queries_ have a larger effect on the memory requirement. Workloads containing large joins over many-to-many tables yield large intermediate datasets and thus require more memory for their evaluation to fully fit into the memory. As an approximation, aggregation-heavy workloads require 1-2 GB memory per thread and join-heavy workloads require 3-4 GB memory per thread.

#### Larger-than-Memory Workloads

DuckDB can process larger-than-memory workloads by spilling to disk. This is possible thanks to _out-of-core_ support for grouping, joining, sorting and windowing operators. Note that larger-than-memory workloads can be processed both in persistent mode and in in-memory mode as DuckDB still spills to disk in both modes.

### Local Disk

**Disk type.** DuckDB's disk-based mode is designed to work best with SSD and NVMe disks. While HDDs are supported, they will result in low performance, especially for write operations.

**Disk-based vs. in-memory storage.** Counter-intuitively, using a disk-based DuckDB instance can be faster than an in-memory instance due to compression. Read more in the “How to Tune Workloads” page.

**File systems.** On Linux, DuckDB performs best with the XFS file system but it also performs reasonably well with other file systems such as ext4. On Windows, we recommend using NTFS and avoiding FAT32.

> Note that DuckDB databases have built-in checksums, so integrity checks from the file system are not required to prevent data corruption.

### Network-Attached Disks

**Cloud disks.** DuckDB runs well on network-backed cloud disks such as AWS EBS for both read-only and read-write workloads.

**Network-attached storage.** Network-attached storage can serve DuckDB for read-only workloads. However, _it is not recommended to run DuckDB in read-write mode on network-attached storage (NAS)._ These setups include NFS, network drives such as SMB and Samba. Based on user reports, running read-write workloads on network-attached storage can result in slow and unpredictable performance, as well as spurious errors cased by the underlying file system.

> Warning Avoid running DuckDB in read-write mode on network-attached storage.

> Bestpractice Fast disks are important if your workload is larger than memory and/or fast data loading is important. Only use network-backed disks if they are reliable (e.g., cloud disks) and guarantee high IO.

## Operating System

We recommend using the latest stable version of operating systems: macOS, Windows, and Linux are all well-tested and DuckDB can run on them with high performance.

### Linux

DuckDB runs on all mainstream Linux distributions released in the last ≈5 years. If you don't have a particular preference, we recommended using Ubuntu Linux LTS due to its stability and the fact that most of DuckDB’s Linux test suite jobs run on Ubuntu workers.

#### glibc vs. musl libc

DuckDB can be built with both glibc (default) and musl libc (see the build guide). However, note that DuckDB binaries built with musl libc have lower performance. In practice, this can lead to a slowdown of more than 5× on compute-intensive workloads. Therefore, it's recommended to use a Linux distribution with glibc for performance-oriented workloads when running DuckDB.

## Memory Allocator

If you have a many-core CPU running on a system where DuckDB ships with `jemalloc` as the default memory allocator, consider enabling the allocator's background threads.

##### About this page

- Report content issue
- See this page as Markdown
- Edit this page on GitHub

© 2025 DuckDB Foundation, Amsterdam NL

Code of Conduct Trademark Use

##### In this article

- Hardware Configuration
- CPU
- Memory
- Local Disk
- Network-Attached Disks
- Operating System
- Linux
- Memory Allocator
