# InterruptHandle in duckdb - Rust

## Struct InterruptHandle 

[Source](about:blank/src/duckdb/inner_connection.rs.html#213-215)

```
pub struct InterruptHandle { /* private fields */ }
```

Expand description

A handle that allows interrupting long-running queries.

[Source](about:blank/src/duckdb/inner_connection.rs.html#220-244)
[§](#impl-InterruptHandle)

[Source](about:blank/src/duckdb/inner_connection.rs.html#235-243)

Interrupt the query currently running on the connection this handle was obtained from. The interrupt will cause that query to fail with `Error::DuckDBFailure`. If the connection was dropped after obtaining this interrupt handle, calling this method results in a noop.

See [`crate::Connection::interrupt_handle`](about:blank/struct.Connection.html#method.interrupt_handle "method duckdb::Connection::interrupt_handle") for an example.

[Source](about:blank/src/duckdb/inner_connection.rs.html#217)
[§](#impl-Send-for-InterruptHandle)

[Source](about:blank/src/duckdb/inner_connection.rs.html#218)
[§](#impl-Sync-for-InterruptHandle)

[§](#impl-Freeze-for-InterruptHandle)

[§](#impl-RefUnwindSafe-for-InterruptHandle)

[§](#impl-Unpin-for-InterruptHandle)

[§](#impl-UnwindSafe-for-InterruptHandle)
