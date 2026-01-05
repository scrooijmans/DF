# Directly Reading Files – DuckDB

DuckDB allows directly reading files via the [`read_text`](#read_text) and [`read_blob`](#read_blob) functions. These functions accept a filename, a list of filenames or a glob pattern, and output the content of each file as a `VARCHAR` or `BLOB`, respectively, as well as additional metadata such as the file size and last modified time.

## [`read_text`](#read_text)

The `read_text` table function reads from the selected source(s) to a `VARCHAR`. Each file results in a single row with the `content` field holding the entire content of the respective file.

```
SELECT size, parse_path(filename), content
FROM read_text('test/sql/table_function/files/*.txt');

```

| size | parse_path(filename)                          | content          |
| ---- | --------------------------------------------- | ---------------- |
| 12   | [test, sql, table_function, files, one.txt]   | Hello World!     |
| 2    | [test, sql, table_function, files, three.txt] | 42               |
| 10   | [test, sql, table_function, files, two.txt]   | Foo Bar\nFöö Bär |

The file content is first validated to be valid UTF-8. If `read_text` attempts to read a file with invalid UTF-8, an error is thrown suggesting to use [`read_blob`](#read_blob) instead.

## [`read_blob`](#read_blob)

The `read_blob` table function reads from the selected source(s) to a `BLOB`:

```
SELECT size, content, filename
FROM read_blob('test/sql/table_function/files/*');

```

- size: 178
  - content: PK\x03\x04\x0A\x00\x00\x00\x00\x00\xACi=X\x14t\xCE\xC7\x0A…
  - filename: test/sql/table_function/files/four.blob
- size: 12
  - content: Hello World!
  - filename: test/sql/table_function/files/one.txt
- size: 2
  - content: 42
  - filename: test/sql/table_function/files/three.txt
- size: 10
  - content: F\xC3\xB6\xC3\xB6 B\xC3\xA4r
  - filename: test/sql/table_function/files/two.txt

## [Schema](#schema)

The schemas of the tables returned by `read_text` and `read_blob` are identical:

```
DESCRIBE FROM read_text('README.md');

```

| column_name   | column_type | null | key  | default | extra |
| ------------- | ----------- | ---- | ---- | ------- | ----- |
| filename      | VARCHAR     | YES  | NULL | NULL    | NULL  |
| content       | VARCHAR     | YES  | NULL | NULL    | NULL  |
| size          | BIGINT      | YES  | NULL | NULL    | NULL  |
| last_modified | TIMESTAMP   | YES  | NULL | NULL    | NULL  |

## [Hive Partitioning](#hive-partitioning)

Data can be read from [Hive partitioned](https://duckdb.org/docs/stable/data/partitioning/hive_partitioning.html) datasets.

```
SELECT *
FROM read_blob('data/parquet-testing/hive-partitioning/simple/**/*.parquet')
WHERE part IN ('a', 'b') AND date >= '2012-01-01';

```

- filename: …/part=a/date=2012-01-01/test.parquet
  - content: PAR1\x15\x00\x15\x14\x15\x18…
  - size: 266
  - last_modified: 2024-11-12 02:23:20+00
  - date: 2012-01-01
  - part: a
- filename: …/part=b/date=2013-01-01/test.parquet
  - content: PAR1\x15\x00\x15\x14\x15\x18…
  - size: 266
  - last_modified: 2024-11-12 02:23:20+00
  - date: 2013-01-01
  - part: b

In cases where the underlying filesystem is unable to provide some of this data due (e.g., because HTTPFS can't always return a valid timestamp), the cell is set to `NULL` instead.

## [Support for Projection Pushdown](#support-for-projection-pushdown)

The table functions also utilize projection pushdown to avoid computing properties unnecessarily. So you could e.g., use this to glob a directory full of huge files to get the file size in the size column, as long as you omit the content column the data won't be read into DuckDB.

© 2025 DuckDB Foundation, Amsterdam NL
