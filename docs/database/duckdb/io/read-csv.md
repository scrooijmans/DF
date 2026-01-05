Title: CSV Import – DuckDB

Description: Examples The following examples use the flights.csv file. Read a CSV file from disk, auto-infer options: SELECT * FROM 'flights.csv'; Use the read_csv function with custom options: SELECT * FROM read_csv('flights.csv', delim = '|', header = true, columns = { 'FlightDate': 'DATE', 'UniqueCarrier': 'VARCHAR', 'OriginCityName': 'VARCHAR', 'DestCityName': 'VARCHAR' }); Read a CSV from stdin, auto-infer options: cat flights.csv | duckdb -c "SELECT * FROM read_csv('/dev/stdin')" Read a CSV file into a table: CREATE TABLE ontime ( FlightDate DATE, UniqueCarrier VARCHAR, OriginCityName VARCHAR, DestCityName VARCHAR ); COPY ontime FROM 'flights.csv'; Alternatively, create a table without specifying the schema manually using a…

Search Shortcut cmd + k | ctrl + k

*   Installation
*   Documentation

*   Getting Started
*   Connect

*   Overview
*   Concurrency

*   Data Import and Export

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

*   Lakehouse Formats
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

Documentation / Data Import and Export / CSV Files  

CSV Import

Examples
--------

The following examples use the `flights.csv` file.

Read a CSV file from disk, auto-infer options:

```
SELECT * FROM 'flights.csv';
```

Use the `read_csv` function with custom options:

```
SELECT *
FROM read_csv('flights.csv',
delim = '|',
header = true,
columns = {
'FlightDate': 'DATE',
'UniqueCarrier': 'VARCHAR',
'OriginCityName': 'VARCHAR',
'DestCityName': 'VARCHAR'
});
```

Read a CSV from stdin, auto-infer options:

```
cat flights.csv | duckdb -c "SELECT * FROM read_csv('/dev/stdin')"
```

Read a CSV file into a table:

```
CREATE TABLE ontime (
FlightDate DATE,
UniqueCarrier VARCHAR,
OriginCityName VARCHAR,
DestCityName VARCHAR
);
COPY ontime FROM 'flights.csv';
```

Alternatively, create a table without specifying the schema manually using a `CREATE TABLE ... AS SELECT` statement:

```
CREATE TABLE ontime AS
SELECT * FROM 'flights.csv';
```

We can use the `FROM`\-first syntax to omit `SELECT *`.

```
CREATE TABLE ontime AS
FROM 'flights.csv';
```

CSV Loading
-----------

CSV loading, i.e., importing CSV files to the database, is a very common, and yet surprisingly tricky, task. While CSVs seem simple on the surface, there are a lot of inconsistencies found within CSV files that can make loading them a challenge. CSV files come in many different varieties, are often corrupt, and do not have a schema. The CSV reader needs to cope with all of these different situations.

The DuckDB CSV reader can automatically infer which configuration flags to use by analyzing the CSV file using the CSV sniffer. This will work correctly in most situations, and should be the first option attempted. In rare situations where the CSV reader cannot figure out the correct configuration it is possible to manually configure the CSV reader to correctly parse the CSV file. See the auto detection page for more information.

Parameters
----------

Below are parameters that can be passed to the `read_csv` function. Where meaningfully applicable, these parameters can also be passed to the `COPY` statement.

| Name | Description | Type | Default |
| --- | --- | --- | --- |
| `all_varchar` | Skip type detection and assume all columns are of type `VARCHAR`. This option is only supported by the `read_csv` function. | `BOOL` | `false` |
| `allow_quoted_nulls` | Allow the conversion of quoted values to `NULL` values | `BOOL` | `true` |
| `auto_detect` | Auto detect CSV parameters. | `BOOL` | `true` |
| `auto_type_candidates` | Types that the sniffer uses when detecting column types. The `VARCHAR` type is always included as a fallback option. See example. | `TYPE[]` | default types |
| `buffer_size` | Size of the buffers used to read files, in bytes. Must be large enough to hold four lines and can significantly impact performance. | `BIGINT` | `16 * max_line_size` |
| `columns` | Column names and types, as a struct (e.g., `{'col1': 'INTEGER', 'col2': 'VARCHAR'}`). Using this option disables auto detection of the schema. | `STRUCT` | (empty) |
| `comment` | Character used to initiate comments. Lines starting with a comment character (optionally preceded by space characters) are completely ignored; other lines containing a comment character are parsed only up to that point. | `VARCHAR` | (empty) |
| `compression` | Method used to compress CSV files. By default this is detected automatically from the file extension (e.g., `t.csv.gz` will use gzip, `t.csv` will use `none`). Options are `none`, `gzip`, `zstd`. | `VARCHAR` | `auto` |
| `dateformat` | Date format used when parsing and writing dates. | `VARCHAR` | (empty) |
| `date_format` | Alias for `dateformat`; only available in the `COPY` statement. | `VARCHAR` | (empty) |
| `decimal_separator` | Decimal separator for numbers. | `VARCHAR` | `.` |
| `delim` | Delimiter character used to separate columns within each line, e.g., `,` `;` `\t`. The delimiter character can be up to 4 bytes, e.g., 🦆. Alias for `sep`. | `VARCHAR` | `,` |
| `delimiter` | Alias for `delim`; only available in the `COPY` statement. | `VARCHAR` | `,` |
| `escape` | String used to escape the `quote` character within quoted values. | `VARCHAR` | `"` |
| `encoding` | Encoding used by the CSV file. Options are `utf-8`, `utf-16`, `latin-1`. Not available in the `COPY` statement (which always uses `utf-8`). | `VARCHAR` | `utf-8` |
| `filename` | Add path of the containing file to each row, as a string column named `filename`. Relative or absolute paths are returned depending on the path or glob pattern provided to `read_csv`, not just filenames. Since DuckDB v1.3.0, the `filename` column is added automatically as a virtual column and this option is only kept for compatibility reasons. | `BOOL` | `false` |
| `force_not_null` | Do not match values in the specified columns against the `NULL` string. In the default case where the `NULL` string is empty, this means that empty values are read as zero-length strings instead of `NULL`s. | `VARCHAR[]` | `[]` |
| `header` | First line of each file contains the column names. | `BOOL` | `false` |
| `hive_partitioning` | Interpret the path as a Hive partitioned path. | `BOOL` | (auto-detected) |
| `ignore_errors` | Ignore any parsing errors encountered. | `BOOL` | `false` |
| `max_line_size` or `maximum_line_size`. Not available in the `COPY` statement. | Maximum line size, in bytes. | `BIGINT` | 2000000 |
| `names` or `column_names` | Column names, as a list. See example. | `VARCHAR[]` | (empty) |
| `new_line` | New line character(s). Options are `'\r'`,`'\n'`, or `'\r\n'`. The CSV parser only distinguishes between single-character and double-character line delimiters. Therefore, it does not differentiate between `'\r'` and `'\n'`. | `VARCHAR` | (empty) |
| `normalize_names` | Normalize column names. This removes any non-alphanumeric characters from them. Column names that are reserved SQL keywords are prefixed with an underscore character (`_`). | `BOOL` | `false` |
| `null_padding` | Pad the remaining columns on the right with `NULL` values when a line lacks columns. | `BOOL` | `false` |
| `nullstr` or `null` | Strings that represent a `NULL` value. | `VARCHAR` or `VARCHAR[]` | (empty) |
| `parallel` | Use the parallel CSV reader. | `BOOL` | `true` |
| `quote` | String used to quote values. | `VARCHAR` | `"` |
| `rejects_scan` | Name of the temporary table where information on faulty scans is stored. | `VARCHAR` | `reject_scans` |
| `rejects_table` | Name of the temporary table where information on faulty lines is stored. | `VARCHAR` | `reject_errors` |
| `rejects_limit` | Upper limit on the number of faulty lines per file that are recorded in the rejects table. Setting this to `0` means that no limit is applied. | `BIGINT` | `0` |
| `sample_size` | Number of sample lines for auto detection of parameters. | `BIGINT` | 20480 |
| `sep` | Delimiter character used to separate columns within each line, e.g., `,` `;` `\t`. The delimiter character can be up to 4 bytes, e.g., 🦆. Alias for `delim`. | `VARCHAR` | `,` |
| `skip` | Number of lines to skip at the start of each file. | `BIGINT` | 0 |
| `store_rejects` | Skip any lines with errors and store them in the rejects table. | `BOOL` | `false` |
| `strict_mode` | Enforces the strictness level of the CSV Reader. When set to `true`, the parser will throw an error upon encountering any issues. When set to `false`, the parser will attempt to read structurally incorrect files. It is important to note that reading structurally incorrect files can cause ambiguity; therefore, this option should be used with caution. | `BOOL` | `true` |
| `thousands` | Character used to identify thousands separators in numeric values. It must be a single character and different from the `decimal_separator` option. | `VARCHAR` | (empty) |
| `timestampformat` | Timestamp format used when parsing and writing timestamps. | `VARCHAR` | (empty) |
| `timestamp_format` | Alias for `timestampformat`; only available in the `COPY` statement. | `VARCHAR` | (empty) |
| `types` or `dtypes` or `column_types` | Column types, as either a list (by position) or a struct (by name). See example. | `VARCHAR[]` or `STRUCT` | (empty) |
| `union_by_name` | Align columns from different files by column name instead of position. Using this option increases memory consumption. | `BOOL` | `false` |

> Tip DuckDB's CSV reader supports `UTF-8` (default), `UTF-16` and `Latin-1` encodings as well as many other `encoding` options natively through the `encoding` extension, for details see All Supported Encodings. To convert files with different encodings, we recommend using the `iconv` command-line tool.
> 
> ```
> iconv -f ISO-8859-2 -t UTF-8 input.csv > input-utf-8.csv
> ```

### `auto_type_candidates` Details

The `auto_type_candidates` option lets you specify the data types that should be considered by the CSV reader for column data type detection. Usage example:

```
SELECT * FROM read_csv('csv_file.csv', auto_type_candidates = ['BIGINT', 'DATE']);
```

The default value for the `auto_type_candidates` option is `['SQLNULL', 'BOOLEAN', 'BIGINT', 'DOUBLE', 'TIME', 'DATE', 'TIMESTAMP', 'VARCHAR']`.

CSV Functions
-------------

The `read_csv` automatically attempts to figure out the correct configuration of the CSV reader using the CSV sniffer. It also automatically deduces types of columns. If the CSV file has a header, it will use the names found in that header to name the columns. Otherwise, the columns will be named `column0, column1, column2, ...`. An example with the `flights.csv` file:

```
SELECT * FROM read_csv('flights.csv');
```

| FlightDate | UniqueCarrier | OriginCityName | DestCityName |
| --- | --- | --- | --- |
| 1988-01-01 | AA | New York, NY | Los Angeles, CA |
| 1988-01-02 | AA | New York, NY | Los Angeles, CA |
| 1988-01-03 | AA | New York, NY | Los Angeles, CA |

The path can either be a relative path (relative to the current working directory) or an absolute path.

We can use `read_csv` to create a persistent table as well:

```
CREATE TABLE ontime AS
SELECT * FROM read_csv('flights.csv');
DESCRIBE ontime;
```

| column\_name | column\_type | null | key | default | extra |
| --- | --- | --- | --- | --- | --- |
| FlightDate | DATE | YES | NULL | NULL | NULL |
| UniqueCarrier | VARCHAR | YES | NULL | NULL | NULL |
| OriginCityName | VARCHAR | YES | NULL | NULL | NULL |
| DestCityName | VARCHAR | YES | NULL | NULL | NULL |

```
SELECT * FROM read_csv('flights.csv', sample_size = 20_000);
```

If we set `delim` / `sep`, `quote`, `escape`, or `header` explicitly, we can bypass the automatic detection of this particular parameter:

```
SELECT * FROM read_csv('flights.csv', header = true);
```

Multiple files can be read at once by providing a glob or a list of files. Refer to the multiple files section for more information.

Writing Using the `COPY` Statement
----------------------------------

The `COPY` statement can be used to load data from a CSV file into a table. This statement has the same syntax as the one used in PostgreSQL. To load the data using the `COPY` statement, we must first create a table with the correct schema (which matches the order of the columns in the CSV file and uses types that fit the values in the CSV file). `COPY` detects the CSV's configuration options automatically.

```
CREATE TABLE ontime (
flightdate DATE,
uniquecarrier VARCHAR,
origincityname VARCHAR,
destcityname VARCHAR
);
COPY ontime FROM 'flights.csv';
SELECT * FROM ontime;
```

| flightdate | uniquecarrier | origincityname | destcityname |
| --- | --- | --- | --- |
| 1988-01-01 | AA | New York, NY | Los Angeles, CA |
| 1988-01-02 | AA | New York, NY | Los Angeles, CA |
| 1988-01-03 | AA | New York, NY | Los Angeles, CA |

If we want to manually specify the CSV format, we can do so using the configuration options of `COPY`.

```
CREATE TABLE ontime (flightdate DATE, uniquecarrier VARCHAR, origincityname VARCHAR, destcityname VARCHAR);
COPY ontime FROM 'flights.csv' (DELIMITER '|', HEADER);
SELECT * FROM ontime;
```

Reading Faulty CSV Files
------------------------

DuckDB supports reading erroneous CSV files. For details, see the Reading Faulty CSV Files page.

Order Preservation
------------------

The CSV reader respects the `preserve_insertion_order` configuration option to preserve insertion order. When `true` (the default), the order of the rows in the result set returned by the CSV reader is the same as the order of the corresponding lines read from the file(s). When `false`, there is no guarantee that the order is preserved.

Writing CSV Files
-----------------

DuckDB can write CSV files using the `COPY ... TO` statement.

Pages in This Section
---------------------

##### About this page

*   Report content issue
*   See this page as Markdown
*   Edit this page on GitHub

© 2025 DuckDB Foundation, Amsterdam NL

Code of Conduct Trademark Use

##### In this article

*   Examples
*   CSV Loading
*   Parameters
*   auto\_type\_candidates Details
*   CSV Functions
*   Writing Using the COPY Statement
*   Reading Faulty CSV Files
*   Order Preservation
*   Writing CSV Files