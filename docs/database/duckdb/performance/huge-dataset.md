Working with Huge Databases
This page contains information for working with huge DuckDB database files. While most DuckDB databases are well below 1 TB, in our 2024 user survey, 1% of respondents used DuckDB files of 2 TB or more (corresponding to roughly 10 TB of CSV files).

DuckDB's native database format supports huge database files without any practical restrictions, however, there are a few things to keep in mind when working with huge database files.

Object storage systems have lower limits on file sizes than block-based storage systems. For example, AWS S3 limits the file size to 5 TB.

Checkpointing a DuckDB database can be slow. For example, checkpointing after adding a few rows to a table in the TPC-H SF1000 database takes approximately 5 seconds.

On block-based storage, the file has a big effect on performance when working with large files. On Linux, DuckDB performs best with XFS on large files.

For storing large amounts of data, consider using the DuckLake lakehouse format.
