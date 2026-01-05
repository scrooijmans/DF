# Connection in duckdb - Rust

```
pub struct Connection { /* private fields */ }
```

Expand description

A connection to a DuckDB database.

[Source](about:blank/src/duckdb/cache.rs.html#11-58)
[§](#impl-Connection)

[Source](about:blank/src/duckdb/cache.rs.html#39-41)

Prepare a SQL statement for execution, returning a previously prepared (but not currently in-use) statement if one is available. The returned statement will be cached for reuse by future calls to [`prepare_cached`](about:blank/struct.Connection.html#method.prepare_cached "method duckdb::Connection::prepare_cached") once it is dropped.

```
fn insert_new_people(conn: &Connection) -> Result<()> {
    {
        let mut stmt = conn.prepare_cached("INSERT INTO People (name) VALUES (?)")?;
        stmt.execute(["Joe Smith"])?;
    }
    {
        // This will return the same underlying DuckDB statement handle without
        // having to prepare it again.
        let mut stmt = conn.prepare_cached("INSERT INTO People (name) VALUES (?)")?;
        stmt.execute(["Bob Jones"])?;
    }
    Ok(())
}
```

##### [§](#failure)Failure

Will return `Err` if `sql` cannot be converted to a C-compatible string or if the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/cache.rs.html#49-51)

Set the maximum number of cached prepared statements this connection will hold. By default, a connection will hold a relatively small number of cached statements. If you need more, or know that you will not use cached statements, you can set the capacity manually using this method.

[Source](about:blank/src/duckdb/cache.rs.html#55-57)

Remove/finalize all prepared statements currently in the cache.

[Source](about:blank/src/duckdb/pragma.rs.html#144-247)
[§](#impl-Connection-1)

[Source](about:blank/src/duckdb/pragma.rs.html#149-156)

Query the current value of `pragma_name`.

Some pragmas will return multiple rows/values which cannot be retrieved with this method.

[Source](about:blank/src/duckdb/pragma.rs.html#159-171)

Query the current rows/values of `pragma_name`.

[Source](about:blank/src/duckdb/pragma.rs.html#178-203)

Query the current value(s) of `pragma_name` associated to `pragma_value`.

This method can be used with query-only pragmas which need an argument (e.g. `table_info('one_tbl')`) or pragmas which returns value(s) (e.g. `integrity_check`).

[Source](about:blank/src/duckdb/pragma.rs.html#209-223)

Set a new value to `pragma_name`.

Some pragmas will return the updated value which cannot be retrieved with this method.

[Source](about:blank/src/duckdb/pragma.rs.html#228-246)

Set a new value to `pragma_name` and return the updated value.

Only few pragmas automatically return the updated value.

[Source](about:blank/src/duckdb/transaction.rs.html#153-219)
[§](#impl-Connection-2)

[Source](about:blank/src/duckdb/transaction.rs.html#181-183)

Begin a new transaction with the default behavior (DEFERRED).

The transaction defaults to rolling back when it is dropped. If you want the transaction to commit, you must call [`commit`](about:blank/struct.Transaction.html#method.commit "method duckdb::Transaction::commit") or [`set_drop_behavior(DropBehavior: :Commit)`](about:blank/struct.Transaction.html#method.set_drop_behavior "method duckdb::Transaction::set_drop_behavior").

###### [§](#example)Example

```
fn perform_queries(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    do_queries_part_1(&tx)?; // tx causes rollback if this fails
    do_queries_part_2(&tx)?; // tx causes rollback if this fails

    tx.commit()
}
```

##### [§](#failure-1)Failure

Will return `Err` if the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/transaction.rs.html#216-218)

Begin a new transaction with the default behavior (DEFERRED).

Attempt to open a nested transaction will result in a DuckDB error. `Connection::transaction` prevents this at compile time by taking `&mut self`, but `Connection::unchecked_transaction()` may be used to defer the checking until runtime.

See [`Connection::transaction`](about:blank/struct.Connection.html#method.transaction "method duckdb::Connection::transaction") and [`Transaction::new_unchecked`](about:blank/struct.Transaction.html#method.new_unchecked "associated function duckdb::Transaction::new_unchecked") (which can be used if the default transaction behavior is undesirable).

###### [§](#example-1)Example

```
fn perform_queries(conn: Rc<Connection>) -> Result<()> {
    let tx = conn.unchecked_transaction()?;

    do_queries_part_1(&tx)?; // tx causes rollback if this fails
    do_queries_part_2(&tx)?; // tx causes rollback if this fails

    tx.commit()
}
```

##### [§](#failure-2)Failure

Will return `Err` if the underlying DuckDB call fails. The specific error returned if transactions are nested is currently unspecified.

[Source](about:blank/src/duckdb/vtab/mod.rs.html#134-153)
[§](#impl-Connection-3)

[Source](about:blank/src/duckdb/vtab/mod.rs.html#137-152)

Register the given TableFunction with the current db

[Source](about:blank/src/duckdb/vscalar/mod.rs.html#134-168)
[§](#impl-Connection-4)

[Source](about:blank/src/duckdb/vscalar/mod.rs.html#137-150)

Register the given ScalarFunction with default state

[Source](about:blank/src/duckdb/vscalar/mod.rs.html#154-167)

Register the given ScalarFunction with custom state

[Source](about:blank/src/duckdb/lib.rs.html#233-595)
[§](#impl-Connection-5)

[Source](about:blank/src/duckdb/lib.rs.html#255-257)

Open a new connection to a DuckDB database.

`Connection::open(path)` is equivalent to `Connection::open_with_flags(path, Config::default())`.

```
fn open_my_db() -> Result<()> {
    let path = "./my_db.db3";
    let db = Connection::open(&path)?;
    println!("{}", db.is_autocommit());
    Ok(())
}
```

##### [§](#failure-3)Failure

Will return `Err` if `path` cannot be converted to a C-compatible string or if the underlying DuckDB open call fails.

[Source](about:blank/src/duckdb/lib.rs.html#265-267)

Open a new connection to an in-memory DuckDB database.

##### [§](#failure-4)Failure

Will return `Err` if the underlying DuckDB open call fails.

[Source](about:blank/src/duckdb/lib.rs.html#278-284)

Open a new connection to an ffi database.

##### [§](#failure-5)Failure

Will return `Err` if the underlying DuckDB open call fails.

##### [§](#safety)Safety

Need to pass in a valid db instance

[Source](about:blank/src/duckdb/lib.rs.html#293-313)

Open a new connection to a DuckDB database.

##### [§](#failure-6)Failure

Will return `Err` if `path` cannot be converted to a C-compatible string or if the underlying DuckDB open call fails.

[Source](about:blank/src/duckdb/lib.rs.html#321-323)

Open a new connection to an in-memory DuckDB database.

##### [§](#failure-7)Failure

Will return `Err` if the underlying DuckDB open call fails.

[Source](about:blank/src/duckdb/lib.rs.html#345-347)

Convenience method to run multiple SQL statements (that cannot take any parameters).

###### [§](#example-2)Example

```
fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch("BEGIN;
                        CREATE TABLE foo(x INTEGER);
                        CREATE TABLE bar(y TEXT);
                        COMMIT;",
    )
}
```

##### [§](#failure-8)Failure

Will return `Err` if `sql` cannot be converted to a C-compatible string or if the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/lib.rs.html#385-387)

Convenience method to prepare and execute a single SQL statement.

On success, returns the number of rows that were changed or inserted or deleted.

###### [§](#example-3)Example

###### [§](#with-params)With params

```
fn update_rows(conn: &Connection) {
    match conn.execute("UPDATE foo SET bar = 'baz' WHERE qux = ?", [1i32]) {
        Ok(updated) => println!("{} rows were updated", updated),
        Err(err) => println!("update failed: {}", err),
    }
}
```

###### [§](#with-params-of-varying-types)With params of varying types

```
fn update_rows(conn: &Connection) {
    match conn.execute("UPDATE foo SET bar = ? WHERE qux = ?", params![&"baz", 1i32]) {
        Ok(updated) => println!("{} rows were updated", updated),
        Err(err) => println!("update failed: {}", err),
    }
}
```

##### [§](#failure-9)Failure

Will return `Err` if `sql` cannot be converted to a C-compatible string or if the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/lib.rs.html#391-393)

Returns the path to the database file, if one exists and is known.

[Source](about:blank/src/duckdb/lib.rs.html#423-429)

Convenience method to execute a query that is expected to return a single row.

###### [§](#example-4)Example

```
fn preferred_locale(conn: &Connection) -> Result<String> {
    conn.query_row(
        "SELECT value FROM preferences WHERE name='locale'",
        [],
        |row| row.get(0),
    )
}
```

If the query returns more than one row, all rows except the first are ignored.

Returns `Err(QueryReturnedNoRows)` if no results are returned. If the query truly is optional, you can call `.optional()` on the result of this to get a `Result<Option<T>>`.

##### [§](#failure-10)Failure

Will return `Err` if `sql` cannot be converted to a C-compatible string or if the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/lib.rs.html#457-468)

Convenience method to execute a query that is expected to return a single row, and execute a mapping via `f` on that returned row with the possibility of failure. The `Result` type of `f` must implement `std::convert::From<Error>`.

###### [§](#example-5)Example

```
fn preferred_locale(conn: &Connection) -> Result<String> {
    conn.query_row_and_then(
        "SELECT value FROM preferences WHERE name='locale'",
        [],
        |row| row.get(0),
    )
}
```

If the query returns more than one row, all rows except the first are ignored.

##### [§](#failure-11)Failure

Will return `Err` if `sql` cannot be converted to a C-compatible string or if the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/lib.rs.html#489-491)

Prepare a SQL statement for execution.

###### [§](#example-6)Example

```
fn insert_new_people(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("INSERT INTO People (name) VALUES (?)")?;
    stmt.execute(["Joe Smith"])?;
    stmt.execute(["Bob Jones"])?;
    Ok(())
}
```

##### [§](#failure-12)Failure

Will return `Err` if `sql` cannot be converted to a C-compatible string or if the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/lib.rs.html#510-512)

Create an Appender for fast import data default to use `DatabaseName::Main`

###### [§](#example-7)Example

```
fn insert_rows(conn: &Connection) -> Result<()> {
    let mut app = conn.appender("foo")?;
    app.append_rows([[1, 2], [3, 4], [5, 6], [7, 8], [9, 10]])?;
    Ok(())
}
```

##### [§](#failure-13)Failure

Will return `Err` if `table` not exists

[Source](about:blank/src/duckdb/lib.rs.html#530-532)

Create an Appender for fast import data

###### [§](#example-8)Example

```
fn insert_rows(conn: &Connection) -> Result<()> {
    let mut app = conn.appender_to_db("foo", &DatabaseName::Main.to_string())?;
    app.append_rows([[1, 2], [3, 4], [5, 6], [7, 8], [9, 10]])?;
    Ok(())
}
```

##### [§](#failure-14)Failure

Will return `Err` if `table` not exists

[Source](about:blank/src/duckdb/lib.rs.html#554-556)

Get a handle to interrupt long-running queries.

###### [§](#example-9)Example

```
fn run_query(conn: Connection) -> Result<()> {
  let interrupt_handle = conn.interrupt_handle();
  let join_handle = std::thread::spawn(move || { conn.execute("expensive query", []) });

  // Arbitrary wait for query to start
  std::thread::sleep(std::time::Duration::from_millis(100));

  interrupt_handle.interrupt();

  let query_result = join_handle.join().unwrap();
  assert!(query_result.is_err());

  Ok(())
}
```

[Source](about:blank/src/duckdb/lib.rs.html#569-572)

Close the DuckDB connection.

This is functionally equivalent to the `Drop` implementation for `Connection` except that on failure, it returns an error and the connection itself (presumably so closing can be attempted again).

##### [§](#failure-15)Failure

Will return `Err` if the underlying DuckDB call fails.

[Source](about:blank/src/duckdb/lib.rs.html#577-579)

Test for auto-commit mode. Autocommit mode is on by default.

[Source](about:blank/src/duckdb/lib.rs.html#582-589)

Creates a new connection to the already-opened database.

[Source](about:blank/src/duckdb/lib.rs.html#592-594)

Returns the version of the DuckDB library

[§](#impl-Freeze-for-Connection)

[§](#impl-RefUnwindSafe-for-Connection)

[§](#impl-Sync-for-Connection)

[§](#impl-Unpin-for-Connection)

[§](#impl-UnwindSafe-for-Connection)
