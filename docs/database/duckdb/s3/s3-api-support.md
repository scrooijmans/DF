Title: S3 API Support – DuckDB

Description: The httpfs extension supports reading/writing/globbing files on object storage servers using the S3 API. S3 offers a standard API to read and write to remote files (while regular http servers, predating S3, do not offer a common write API). DuckDB conforms to the S3 API, that is now common among industry storage providers. Platforms The httpfs filesystem is tested with AWS S3, Minio, Google Cloud, and lakeFS. Other services that implement the S3 API (such as Cloudflare R2) should also work, but not all features may be supported. The following table shows which parts of the S3 API are required…

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

Documentation / Core Extensions / httpfs (HTTP and S3)

S3 API Support

The `httpfs` extension supports reading/writing/globbing files on object storage servers using the S3 API. S3 offers a standard API to read and write to remote files (while regular http servers, predating S3, do not offer a common write API). DuckDB conforms to the S3 API, that is now common among industry storage providers.

## Platforms

The `httpfs` filesystem is tested with AWS S3, Minio, Google Cloud, and lakeFS. Other services that implement the S3 API (such as Cloudflare R2) should also work, but not all features may be supported.

The following table shows which parts of the S3 API are required for each `httpfs` feature.

| Feature            | Required S3 API features                   |
| ------------------ | ------------------------------------------ |
| Public file reads  | HTTP Range requests                        |
| Private file reads | Secret key or session token authentication |
| File glob          | ListObjectsV2                              |
| File writes        | Multipart upload                           |

## Configuration and Authentication

The preferred way to configure and authenticate to S3 endpoints is to use secrets. Multiple secret providers are available.

To migrate from the deprecated S3 API, use a defined secret with a profile. See the “Loading a Secret Based on a Profile” section.

### `config` Provider

The default provider, `config` (i.e., user-configured), allows access to the S3 bucket by manually providing a key. For example:

```
CREATE OR REPLACE SECRET secret (
TYPE s3,
PROVIDER config,
KEY_ID 'AKIAIOSFODNN7EXAMPLE',
SECRET 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY',
REGION 'us-east-1'
);
```

> Tip If you get an IO Error (`Connection error for HTTP HEAD`), configure the endpoint explicitly via `ENDPOINT 's3.your_region.amazonaws.com'`.

Now, to query using the above secret, simply query any `s3://` prefixed file:

```
SELECT *
FROM 's3://your-bucket/your_file.parquet';
```

### `credential_chain` Provider

The `credential_chain` provider allows automatically fetching credentials using mechanisms provided by the AWS SDK. For example, to use the AWS SDK default provider:

```
CREATE OR REPLACE SECRET secret (
TYPE s3,
PROVIDER credential_chain
);
```

Again, to query a file using the above secret, simply query any `s3://` prefixed file.

DuckDB also allows specifying a specific chain using the `CHAIN` keyword. This takes a semicolon-separated list (`a;b;c`) of providers that will be tried in order. For example:

```
CREATE OR REPLACE SECRET secret (
TYPE s3,
PROVIDER credential_chain,
CHAIN 'env;config'
);
```

The possible values for `CHAIN` are the following:

- `config`
- `sts`
- `sso`
- `env`
- `instance`
- `process`

The `credential_chain` provider also allows overriding the automatically fetched config. For example, to automatically load credentials, and then override the region, run:

```
CREATE OR REPLACE SECRET secret (
TYPE s3,
PROVIDER credential_chain,
CHAIN config,
REGION 'eu-west-1'
);
```

#### Loading a Secret Based on a Profile

To load credentials based on a profile which is not defined as a default from the `AWS_PROFILE` environment variable or as a default profile based on AWS SDK precedence, run:

```
CREATE OR REPLACE SECRET secret (
TYPE s3,
PROVIDER credential_chain,
CHAIN config,
PROFILE 'my_profile'
);
```

This approach is equivalent to the deprecated S3 API's's method `load_aws_credentials('⟨my_profile⟩')`.

### Overview of S3 Secret Parameters

Below is a complete list of the supported parameters that can be used for both the `config` and `credential_chain` providers:

| Name                     | Description                                                                           | Secret            | Type      | Default                                     |
| ------------------------ | ------------------------------------------------------------------------------------- | ----------------- | --------- | ------------------------------------------- |
| `ENDPOINT`               | Specify a custom S3 endpoint                                                          | `S3`, `GCS`, `R2` | `STRING`  | `s3.amazonaws.com` for `S3`,                |
| `KEY_ID`                 | The ID of the key to use                                                              | `S3`, `GCS`, `R2` | `STRING`  | \-                                          |
| `REGION`                 | The region for which to authenticate (should match the region of the bucket to query) | `S3`, `GCS`, `R2` | `STRING`  | `us-east-1`                                 |
| `SECRET`                 | The secret of the key to use                                                          | `S3`, `GCS`, `R2` | `STRING`  | \-                                          |
| `SESSION_TOKEN`          | Optionally, a session token can be passed to use temporary credentials                | `S3`, `GCS`, `R2` | `STRING`  | \-                                          |
| `URL_COMPATIBILITY_MODE` | Can help when URLs contain problematic characters                                     | `S3`, `GCS`, `R2` | `BOOLEAN` | `true`                                      |
| `URL_STYLE`              | Either `vhost` or `path`                                                              | `S3`, `GCS`, `R2` | `STRING`  | `vhost` for `S3`, `path` for `R2` and `GCS` |
| `USE_SSL`                | Whether to use HTTPS or HTTP                                                          | `S3`, `GCS`, `R2` | `BOOLEAN` | `true`                                      |
| `ACCOUNT_ID`             | The R2 account ID to use for generating the endpoint URL                              | `R2`              | `STRING`  | \-                                          |
| `KMS_KEY_ID`             | AWS KMS (Key Management Service) key for Server Side Encryption S3                    | `S3`              | `STRING`  | \-                                          |
| `REQUESTER_PAYS`         | Allows use of "requester pays" S3 buckets                                             | `S3`              | `BOOLEAN` | `false`                                     |

### Platform-Specific Secret Types

#### S3 Secrets

The httpfs extension supports Server Side Encryption via the AWS Key Management Service (KMS) on S3 using the `KMS_KEY_ID` option:

```
CREATE OR REPLACE SECRET secret (
TYPE s3,
PROVIDER credential_chain,
CHAIN config,
REGION 'eu-west-1',
KMS_KEY_ID 'arn:aws:kms:region:account_id:key/key_id',
SCOPE 's3://bucket-sub-path'
);
```

#### R2 Secrets

While Cloudflare R2 uses the regular S3 API, DuckDB has a special Secret type, `R2`, to make configuring it a bit simpler:

```
CREATE OR REPLACE SECRET secret (
TYPE r2,
KEY_ID 'AKIAIOSFODNN7EXAMPLE',
SECRET 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY',
ACCOUNT_ID 'my_account_id'
);
```

Note the addition of the `ACCOUNT_ID` which is used to generate the correct endpoint URL for you. Also note that `R2` Secrets can also use both the `CONFIG` and `credential_chain` providers. However, since DuckDB uses an AWS client internally, when using `credential_chain`, the client will search for AWS credentials in the standard AWS credential locations (environment variables, credential files, etc.). Therefore, your R2 credentials must be made available as AWS environment variables (`AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`) for the credential chain to work properly. Finally, `R2` secrets are only available when using URLs starting with `r2://`, for example:

```
SELECT *
FROM read_parquet('r2://some-file-that-uses-an-r2-secret.parquet');
```

#### GCS Secrets

While Google Cloud Storage is accessed by DuckDB using the S3 API, DuckDB has a special Secret type, `GCS`, to make configuring it a bit simpler:

```
CREATE OR REPLACE SECRET secret (
TYPE gcs,
KEY_ID 'my_hmac_access_id',
SECRET 'my_hmac_secret_key'
);
```

**Important**: The `KEY_ID` and `SECRET` values must be HMAC keys generated specifically for Google Cloud Storage interoperability. These are not the same as regular GCP service account keys or access tokens. You can create HMAC keys by following the Google Cloud documentation for managing HMAC keys.

Note that the above secret will automatically have the correct Google Cloud Storage endpoint configured. Also note that `GCS` Secrets can also use both the `CONFIG` and `credential_chain` providers. However, since DuckDB uses an AWS client internally, when using `credential_chain`, the client will search for AWS credentials in the standard AWS credential locations (environment variables, credential files, etc.). Therefore, your GCS HMAC keys must be made available as AWS environment variables (`AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`) for the credential chain to work properly. Finally, `GCS` secrets are only available when using URLs starting with `gcs://` or `gs://`, for example:

```
SELECT *
FROM read_parquet('gcs://some/file/that/uses/a/gcs/secret.parquet');
```

## Reading

Reading files from S3 is now as simple as:

```
SELECT *
FROM 's3://your-bucket/filename.extension';
```

### Partial Reading

The `httpfs` extension supports partial reading from S3 buckets.

### Reading Multiple Files

Multiple files are also possible, for example:

```
SELECT *
FROM read_parquet([
's3://your-bucket/filename-1.parquet',
's3://your-bucket/filename-2.parquet'
]);
```

### Globbing

File globbing is implemented using the ListObjectsV2 API call and allows to use filesystem-like glob patterns to match multiple files, for example:

```
SELECT *
FROM read_parquet('s3://your-bucket/*.parquet');
```

This query matches all files in the root of the bucket with the Parquet extension.

Several features for matching are supported, such as `*` to match any number of any character, `?` for any single character or `[0-9]` for a single character in a range of characters:

```
SELECT count(*) FROM read_parquet('s3://your-bucket/folder*/100?/t[0-9].parquet');
```

A useful feature when using globs is the `filename` option, which adds a column named `filename` that encodes the file that a particular row originated from:

```
SELECT *
FROM read_parquet('s3://your-bucket/*.parquet', filename = true);
```

This could for example result in:

| column_a | column_b      | filename                       |
| -------- | ------------- | ------------------------------ |
| 1        | examplevalue1 | s3://bucket-name/file1.parquet |
| 2        | examplevalue1 | s3://bucket-name/file2.parquet |

### Hive Partitioning

DuckDB also offers support for the Hive partitioning scheme, which is available when using HTTP(S) and S3 endpoints.

## Writing

Writing to S3 uses the multipart upload API. This allows DuckDB to robustly upload files at high speed. Writing to S3 works for both CSV and Parquet:

```
COPY table_name TO 's3://your-bucket/filename.extension';
```

Partitioned copy to S3 also works:

```
COPY table TO 's3://your-bucket/partitioned' (
FORMAT parquet,
PARTITION_BY (part_col_a, part_col_b)
);
```

An automatic check is performed for existing files/directories, which is currently quite conservative (and on S3 will add a bit of latency). To disable this check and force writing, an `OVERWRITE_OR_IGNORE` flag is added:

```
COPY table TO 's3://your-bucket/partitioned' (
FORMAT parquet,
PARTITION_BY (part_col_a, part_col_b),
OVERWRITE_OR_IGNORE true
);
```

The naming scheme of the written files looks like this:

```
s3://your-bucket/partitioned/part_col_a=val/part_col_b=val/data_thread_number.parquet
```

### Configuration

Some additional configuration options exist for the S3 upload, though the default values should suffice for most use cases.

| Name                             | Description                                  |
| -------------------------------- | -------------------------------------------- |
| `s3_uploader_max_parts_per_file` | Used for part size calculation, see AWS docs |
| `s3_uploader_max_filesize`       | Used for part size calculation, see AWS docs |
| `s3_uploader_thread_limit`       | Maximum number of uploader threads           |

##### About this page

- Report content issue
- See this page as Markdown
- Edit this page on GitHub

© 2025 DuckDB Foundation, Amsterdam NL

Code of Conduct Trademark Use

##### In this article

- Platforms
- Configuration and Authentication
- config Provider
- credential_chain Provider
- Overview of S3 Secret Parameters
- Platform-Specific Secret Types
- Reading
- Partial Reading
- Reading Multiple Files
- Globbing
- Hive Partitioning
- Writing
- Configuration
