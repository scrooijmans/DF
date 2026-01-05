# DuckdbConnectionManager in duckdb - Rust

## Struct DuckdbConnectionManager 

[Source](about:blank/src/duckdb/r2d2.rs.html#52-54)

```
pub struct DuckdbConnectionManager { /* private fields */ }
```

Expand description

An `r2d2::ManageConnection` for `duckdb::Connection`s.

[Source](about:blank/src/duckdb/r2d2.rs.html#56-100)
[§](#impl-DuckdbConnectionManager)

[Source](about:blank/src/duckdb/r2d2.rs.html#58-62)

Creates a new `DuckdbConnectionManager` from file.

[Source](about:blank/src/duckdb/r2d2.rs.html#64-68)

Creates a new `DuckdbConnectionManager` from file with flags.

[Source](about:blank/src/duckdb/r2d2.rs.html#71-75)

Creates a new `DuckdbConnectionManager` from memory.

[Source](about:blank/src/duckdb/r2d2.rs.html#78-82)

Creates a new `DuckdbConnectionManager` from memory with flags.

[Source](about:blank/src/duckdb/r2d2.rs.html#86-89)

Register a table function.

[Source](about:blank/src/duckdb/r2d2.rs.html#93-99)

Register a scalar function.

[Source](about:blank/src/duckdb/r2d2.rs.html#102-118)
[§](#impl-ManageConnection-for-DuckdbConnectionManager)

[Source](about:blank/src/duckdb/r2d2.rs.html#103)
[§](#associatedtype.Connection)

The connection type this manager deals with.

[Source](about:blank/src/duckdb/r2d2.rs.html#104)
[§](#associatedtype.Error)

The error type returned by `Connection`s.

[Source](about:blank/src/duckdb/r2d2.rs.html#106-109)
[§](#method.connect)

Attempts to create a new connection.

[Source](about:blank/src/duckdb/r2d2.rs.html#111-113)
[§](#method.is_valid)

Determines if the connection is still connected to the database. [Read more](https://docs.rs/r2d2/0.8.10/x86_64-unknown-linux-gnu/r2d2/trait.ManageConnection.html#tymethod.is_valid)

[Source](about:blank/src/duckdb/r2d2.rs.html#115-117)
[§](#method.has_broken)

_Quickly_ determines if the connection is no longer usable. [Read more](https://docs.rs/r2d2/0.8.10/x86_64-unknown-linux-gnu/r2d2/trait.ManageConnection.html#tymethod.has_broken)

[§](#impl-Freeze-for-DuckdbConnectionManager)

[§](#impl-RefUnwindSafe-for-DuckdbConnectionManager)

[§](#impl-Send-for-DuckdbConnectionManager)

[§](#impl-Sync-for-DuckdbConnectionManager)

[§](#impl-Unpin-for-DuckdbConnectionManager)

[§](#impl-UnwindSafe-for-DuckdbConnectionManager)
