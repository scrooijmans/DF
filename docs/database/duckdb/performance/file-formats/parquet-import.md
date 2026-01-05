# Parquet Import – DuckDB

To read data from a Parquet file, use the `read_parquet` function in the `FROM` clause of a query:

```
SELECT * FROM read_parquet('input.parquet');

```

Alternatively, you can omit the `read_parquet` function and let DuckDB infer it from the extension:

```
SELECT * FROM 'input.parquet';

```

To create a new table using the result from a query, use [`CREATE TABLE ... AS SELECT` statement](about:/docs/stable/sql/statements/create_table.html#create-table--as-select-ctas):

```
CREATE TABLE new_tbl AS
    SELECT * FROM read_parquet('input.parquet');

```

To load data into an existing table from a query, use `INSERT INTO` from a `SELECT` statement:

```
INSERT INTO tbl
    SELECT * FROM read_parquet('input.parquet');

```

Alternatively, the `COPY` statement can also be used to load data from a Parquet file into an existing table:

```
COPY tbl FROM 'input.parquet' (FORMAT parquet);

```

## [Adjusting the Schema on the Fly](#adjusting-the-schema-on-the-fly)

You can load a Parquet file into a slightly different schema (e.g., different number of columns, more relaxed types) using the following trick.

Suppose we have a Parquet file with two columns, `c1` and `c2`:

```
COPY (FROM (VALUES (42, 43)) t(c1, c2))
TO 'f.parquet';

```

If we want to add another column `c3` that is not present in the file, we can run:

```
FROM (VALUES(NULL::VARCHAR, NULL, NULL)) t(c1, c2, c3)
WHERE false
UNION ALL BY NAME
FROM 'f.parquet';

```

The first `FROM` clause generates an empty tables with _three_ columns where `c1` is a `VARCHAR`. Then, we use `UNION ALL BY NAME` to union the Parquet file. The result here is:

```
┌─────────┬───────┬───────┐
│   c1    │  c2   │  c3   │
│ varchar │ int32 │ int32 │
├─────────┼───────┼───────┤
│ 42      │  43   │ NULL  │
└─────────┴───────┴───────┘

```

For additional options, see the [Parquet loading reference](https://duckdb.org/docs/stable/data/parquet/overview.html).

© 2025 DuckDB Foundation, Amsterdam NL
