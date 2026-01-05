# Appender in duckdb - Rust

```
pub struct Appender<'conn> { /* private fields */ }
```

Expand description

Appender for fast import data

[Source](about:blank/src/duckdb/appender/arrow.rs.html#11-61)
[§](#impl-Appender%3C'_%3E)

[Source](about:blank/src/duckdb/appender/arrow.rs.html#30-60)

Append one record batch

###### [§](#example)Example

```
  use arrow::record_batch::RecordBatch;
fn insert_record_batch(conn: &Connection,record_batch:RecordBatch) -> Result<()> {
    let mut app = conn.appender("foo")?;
    app.append_record_batch(record_batch)?;
    Ok(())
}
```

##### [§](#failure)Failure

Will return `Err` if append column count not the same with the table schema

[Source](about:blank/src/duckdb/appender/mod.rs.html#19-159)
[§](#impl-Appender%3C'_%3E-1)

[Source](about:blank/src/duckdb/appender/mod.rs.html#37-46)

Append multiple rows from Iterator

###### [§](#example-1)Example

```
fn insert_rows(conn: &Connection) -> Result<()> {
    let mut app = conn.appender("foo")?;
    app.append_rows([[1, 2], [3, 4], [5, 6], [7, 8], [9, 10]])?;
    Ok(())
}
```

##### [§](#failure-1)Failure

Will return `Err` if append column count not the same with the table schema

[Source](about:blank/src/duckdb/appender/mod.rs.html#65-71)

Append one row

###### [§](#example-2)Example

```
fn insert_row(conn: &Connection) -> Result<()> {
    let mut app = conn.appender("foo")?;
    app.append_row([1, 2])?;
    Ok(())
}
```

##### [§](#failure-2)Failure

Will return `Err` if append column count not the same with the table schema

[Source](about:blank/src/duckdb/appender/mod.rs.html#153-158)

Flush data into DB

[§](#impl-Freeze-for-Appender%3C'conn%3E)

[§](#impl-RefUnwindSafe-for-Appender%3C'conn%3E)

[§](#impl-Send-for-Appender%3C'conn%3E)

[§](#impl-Sync-for-Appender%3C'conn%3E)

[§](#impl-Unpin-for-Appender%3C'conn%3E)

[§](#impl-UnwindSafe-for-Appender%3C'conn%3E)
