# CachedStatement in duckdb - Rust

## Struct CachedStatement 

[Source](about:blank/src/duckdb/cache.rs.html#72-75)

```
pub struct CachedStatement<'conn> { /* private fields */ }
```

Expand description

Cacheable statement.

Statement will return automatically to the cache by default. If you want the statement to be discarded, call [`discard()`](about:blank/struct.CachedStatement.html#method.discard "method duckdb::CachedStatement::discard") on it.

[Source](about:blank/src/duckdb/cache.rs.html#103-118)
[§](#impl-CachedStatement%3C'_%3E)

[Source](about:blank/src/duckdb/cache.rs.html#115-117)

Discard the statement, preventing it from being returned to its [`Connection`](struct.Connection.html "struct duckdb::Connection")’s collection of cached statements.

[Source](about:blank/src/duckdb/column.rs.html#37-44)

Get all the column names in the result set of the prepared statement.

If associated DB schema can be altered concurrently, you should make sure that current statement has already been stepped once before calling this method.

##### [§](#caveats)Caveats

Panics if the query has not been [`execute`](about:blank/struct.Statement.html#method.execute "method duckdb::Statement::execute")d yet.

[Source](about:blank/src/duckdb/column.rs.html#76-78)

Return the number of columns in the result set returned by the prepared statement.

If associated DB schema can be altered concurrently, you should make sure that current statement has already been stepped once before calling this method.

##### [§](#example)Example

```
fn get_column_count(conn: &Connection) -> Result<usize> {
    let mut stmt = conn.prepare("SELECT id, name FROM people")?;

    // Option 1: Execute first, then get column count
    stmt.execute([])?;
    let count = stmt.column_count();

    // Option 2: Get column count from rows (avoids borrowing issues)
    let mut stmt2 = conn.prepare("SELECT id, name FROM people")?;
    let rows = stmt2.query([])?;
    let count2 = rows.as_ref().unwrap().column_count();

    Ok(count)
}
```

##### [§](#caveats-1)Caveats

Panics if the query has not been [`execute`](about:blank/struct.Statement.html#method.execute "method duckdb::Statement::execute")d yet.

[Source](about:blank/src/duckdb/column.rs.html#122-124)

Returns the name assigned to a particular column in the result set returned by the prepared statement.

If associated DB schema can be altered concurrently, you should make sure that current statement has already been stepped once before calling this method.

###### [§](#failure)Failure

Returns an `Error::InvalidColumnIndex` if `idx` is outside the valid column range for this row.

##### [§](#caveats-2)Caveats

Panics if the query has not been [`execute`](about:blank/struct.Statement.html#method.execute "method duckdb::Statement::execute")d yet or when column name is not valid UTF-8.

[Source](about:blank/src/duckdb/column.rs.html#143-153)

Returns the column index in the result set for a given column name.

If there is no AS clause then the name of the column is unspecified and may change from one release of DuckDB to the next.

If associated DB schema can be altered concurrently, you should make sure that current statement has already been stepped once before calling this method.

##### [§](#failure-1)Failure

Will return an `Error::InvalidColumnName` when there is no column with the specified `name`.

##### [§](#caveats-3)Caveats

Panics if the query has not been [`execute`](about:blank/struct.Statement.html#method.execute "method duckdb::Statement::execute")d yet.

[Source](about:blank/src/duckdb/column.rs.html#160-162)

Returns the declared data type of the column.

##### [§](#caveats-4)Caveats

Panics if the query has not been [`execute`](about:blank/struct.Statement.html#method.execute "method duckdb::Statement::execute")d yet.

[Source](about:blank/src/duckdb/statement.rs.html#63-66)

Execute the prepared statement.

On success, returns the number of rows that were changed or inserted or deleted.

###### [§](#example-1)Example

###### [§](#use-with-positional-parameters)Use with positional parameters

```
fn update_rows(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("UPDATE foo SET bar = 'baz' WHERE qux = ?")?;
    // The `duckdb::params!` macro is mostly useful when the parameters do not
    // all have the same type, or if there are more than 32 parameters
    // at once.
    stmt.execute(params![1i32])?;
    // However, it's not required, many cases are fine as:
    stmt.execute(&[&2i32])?;
    // Or even:
    stmt.execute([2i32])?;
    Ok(())
}
```

###### [§](#use-without-parameters)Use without parameters

```
fn delete_all(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("DELETE FROM users")?;
    stmt.execute([])?;
    Ok(())
}
```

##### [§](#failure-2)Failure

Will return `Err` if binding parameters fails, the executed statement returns rows (in which case `query` should be used instead), or the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/statement.rs.html#82-88)

Execute an INSERT.

##### [§](#note)Note

This function is a convenience wrapper around [`execute()`](about:blank/struct.Statement.html#method.execute "method duckdb::Statement::execute") intended for queries that insert a single item. It is possible to misuse this function in a way that it cannot detect, such as by calling it on a statement which _updates_ a single item rather than inserting one. Please don’t do that.

##### [§](#failure-3)Failure

Will return `Err` if no row is inserted or many rows are inserted.

[Source](about:blank/src/duckdb/statement.rs.html#107-110)

Execute the prepared statement, returning a handle to the resulting vector of arrow RecordBatch

###### [§](#example-2)Example

```
fn get_arrow_data(conn: &Connection) -> Result<Vec<RecordBatch>> {
    Ok(conn.prepare("SELECT * FROM test")?.query_arrow([])?.collect())
}
```

##### [§](#failure-4)Failure

Will return `Err` if binding parameters fails.

[Source](about:blank/src/duckdb/statement.rs.html#130-134)

Execute the prepared statement, returning a handle to the resulting vector of arrow RecordBatch in streaming way

###### [§](#example-3)Example

```
fn get_arrow_data(conn: &Connection, schema: SchemaRef) -> Result<Vec<RecordBatch>> {
    Ok(conn.prepare("SELECT * FROM test")?.stream_arrow([], schema)?.collect())
}
```

##### [§](#failure-5)Failure

Will return `Err` if binding parameters fails.

[Source](about:blank/src/duckdb/statement.rs.html#175-178)

Execute the prepared statement, returning a handle to the resulting vector of polars DataFrame.

###### [§](#example-4)Example

```

fn get_polars_dfs(conn: &Connection) -> Result<Vec<DataFrame>> {
    let dfs: Vec<DataFrame> = conn
        .prepare("SELECT * FROM test")?
        .query_polars([])?
        .collect();

    Ok(dfs)
}
```

To derive a DataFrame from Vec<DataFrame>, we can use function [polars_core::utils::accumulate_dataframes_vertical_unchecked](https://docs.rs/polars-core/latest/polars_core/utils/fn.accumulate_dataframes_vertical_unchecked.html).

```

fn get_polars_df(conn: &Connection) -> Result<DataFrame> {
    let mut stmt = conn.prepare("SELECT * FROM test")?;
    let pl = stmt.query_polars([])?;
    let df = accumulate_dataframes_vertical_unchecked(pl);

   Ok(df)
}
```

[Source](about:blank/src/duckdb/statement.rs.html#239-242)

Execute the prepared statement, returning a handle to the resulting rows.

Due to lifetime restricts, the rows handle returned by `query` does not implement the `Iterator` trait. Consider using [`query_map`](about:blank/struct.Statement.html#method.query_map "method duckdb::Statement::query_map") or [`query_and_then`](about:blank/struct.Statement.html#method.query_and_then "method duckdb::Statement::query_and_then") instead, which do.

###### [§](#example-5)Example

###### [§](#use-without-parameters-1)Use without parameters

```
fn get_names(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM people")?;
    let mut rows = stmt.query([])?;

    let mut names = Vec::new();
    while let Some(row) = rows.next()? {
        names.push(row.get(0)?);
    }

    Ok(names)
}
```

###### [§](#use-with-positional-parameters-1)Use with positional parameters

```
fn query(conn: &Connection, name: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM test where name = ?")?;
    let mut rows = stmt.query(duckdb::params![name])?;
    while let Some(row) = rows.next()? {
        // ...
    }
    Ok(())
}
```

Or, equivalently (but without the \[`params!`\] macro).

```
fn query(conn: &Connection, name: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM test where name = ?")?;
    let mut rows = stmt.query([name])?;
    while let Some(row) = rows.next()? {
        // ...
    }
    Ok(())
}
```

###### [§](#failure-6)Failure

Will return `Err` if binding parameters fails.

[Source](about:blank/src/duckdb/statement.rs.html#274-280)

Executes the prepared statement and maps a function over the resulting rows, returning an iterator over the mapped function results.

`f` is used to transform the _streaming_ iterator into a _standard_ iterator.

This is equivalent to `stmt.query(params)?.mapped(f)`.

###### [§](#example-6)Example

###### [§](#use-with-positional-params)Use with positional params

```
fn get_names(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM people")?;
    let rows = stmt.query_map([], |row| row.get(0))?;

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}
```

###### [§](#failure-7)Failure

Will return `Err` if binding parameters fails.

[Source](about:blank/src/duckdb/statement.rs.html#311-318)

Executes the prepared statement and maps a function over the resulting rows, where the function returns a `Result` with `Error` type implementing `std::convert::From<Error>` (so errors can be unified).

This is equivalent to `stmt.query(params)?.and_then(f)`.

###### [§](#example-7)Example

###### [§](#use-with-positional-params-1)Use with positional params

```
fn get_names(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM people WHERE id = ?")?;
    let rows = stmt.query_and_then(["one"], |row| row.get::<_, String>(0))?;

    let mut persons = Vec::new();
    for person_result in rows {
        persons.push(person_result?);
    }

    Ok(persons)
}
```

##### [§](#failure-8)Failure

Will return `Err` if binding parameters fails.

[Source](about:blank/src/duckdb/statement.rs.html#323-327)

Return `true` if a query in the SQL statement it executes returns one or more rows and `false` if the SQL returns an empty set.

[Source](about:blank/src/duckdb/statement.rs.html#344-350)

Convenience method to execute a query that is expected to return a single row.

If the query returns more than one row, all rows except the first are ignored.

Returns `Err(QueryReturnedNoRows)` if no results are returned. If the query truly is optional, you can call [`.optional()`](about:blank/trait.OptionalExt.html#tymethod.optional "method duckdb::OptionalExt::optional") on the result of this to get a `Result<Option<T>>` (requires that the trait `duckdb::OptionalExt` is imported).

##### [§](#failure-9)Failure

Will return `Err` if the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/statement.rs.html#366-377)

Convenience method to execute a query that is expected to return exactly one row.

Returns `Err(QueryReturnedMoreThanOneRow)` if the query returns more than one row.

Returns `Err(QueryReturnedNoRows)` if no results are returned. If the query truly is optional, you can call [`.optional()`](about:blank/trait.OptionalExt.html#tymethod.optional "method duckdb::OptionalExt::optional") on the result of this to get a `Result<Option<T>>` (requires that the trait `duckdb::OptionalExt` is imported).

##### [§](#failure-10)Failure

Will return `Err` if the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/statement.rs.html#381-383)

Return the row count

[Source](about:blank/src/duckdb/statement.rs.html#387-389)

Get next batch records in arrow-rs

[Source](about:blank/src/duckdb/statement.rs.html#393-395)

Get next batch records in arrow-rs in a streaming way

[Source](about:blank/src/duckdb/statement.rs.html#400-402)

Get next batch records in arrow2

[Source](about:blank/src/duckdb/statement.rs.html#428-430)

Return the number of parameters that can be bound to this statement.

[Source](about:blank/src/duckdb/statement.rs.html#470-474)

Low level API to directly bind a parameter to a given index.

Note that the index is one-based, that is, the first parameter index is 1 and not 0. This is consistent with the DuckDB API and the values given to parameters bound as `?NNN`.

The valid values for `one_based_col_index` begin at `1`, and end at [`Statement::parameter_count`](about:blank/struct.Statement.html#method.parameter_count "method duckdb::Statement::parameter_count"), inclusive.

##### [§](#caveats-5)Caveats

This should not generally be used, but is available for special cases such as:

- binding parameters where a gap exists.
- binding named and positional parameters in the same query.
- separating parameter binding from query execution.

Statements that have had their parameters bound this way should be queried or executed by [`Statement::raw_query`](about:blank/struct.Statement.html#method.raw_query "method duckdb::Statement::raw_query") or [`Statement::raw_execute`](about:blank/struct.Statement.html#method.raw_execute "method duckdb::Statement::raw_execute"). Other functions are not guaranteed to work.

##### [§](#example-8)Example

```
fn query(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM test WHERE name = ? AND value > ?2")?;
    stmt.raw_bind_parameter(1, "foo")?;
    stmt.raw_bind_parameter(2, 100)?;
    let mut rows = stmt.raw_query();
    while let Some(row) = rows.next()? {
        // ...
    }
    Ok(())
}
```

[Source](about:blank/src/duckdb/statement.rs.html#491-493)

Low level API to execute a statement given that all parameters were bound explicitly with the [`Statement::raw_bind_parameter`](about:blank/struct.Statement.html#method.raw_bind_parameter "method duckdb::Statement::raw_bind_parameter") API.

##### [§](#caveats-6)Caveats

Any unbound parameters will have `NULL` as their value.

This should not generally be used outside of special cases, and functions in the [`Statement::execute`](about:blank/struct.Statement.html#method.execute "method duckdb::Statement::execute") family should be preferred.

##### [§](#failure-11)Failure

Will return `Err` if the executed statement returns rows (in which case `query` should be used instead), or the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/statement.rs.html#508-510)

Low level API to get `Rows` for this query given that all parameters were bound explicitly with the [`Statement::raw_bind_parameter`](about:blank/struct.Statement.html#method.raw_bind_parameter "method duckdb::Statement::raw_bind_parameter") API.

##### [§](#caveats-7)Caveats

Any unbound parameters will have `NULL` as their value.

This should not generally be used outside of special cases, and functions in the [`Statement::query`](about:blank/struct.Statement.html#method.query "method duckdb::Statement::query") family should be preferred.

Note that if the SQL does not return results, [`Statement::raw_execute`](about:blank/struct.Statement.html#method.raw_execute "method duckdb::Statement::raw_execute") should be used instead.

[Source](about:blank/src/duckdb/statement.rs.html#517-519)

Returns the underlying schema of the prepared statement.

##### [§](#caveats-8)Caveats

Panics if the query has not been [`execute`](about:blank/struct.Statement.html#method.execute "method duckdb::Statement::execute")d yet.

[§](#impl-Freeze-for-CachedStatement%3C'conn%3E)

[§](#impl-RefUnwindSafe-for-CachedStatement%3C'conn%3E)

[§](#impl-Send-for-CachedStatement%3C'conn%3E)

[§](#impl-Sync-for-CachedStatement%3C'conn%3E)

[§](#impl-Unpin-for-CachedStatement%3C'conn%3E)

[§](#impl-UnwindSafe-for-CachedStatement%3C'conn%3E)
