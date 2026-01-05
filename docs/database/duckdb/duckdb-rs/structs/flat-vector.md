# FlatVector in duckdb::core - Rust

```
pub struct FlatVector { /* private fields */ }
```

Expand description

A flat vector

[Source](about:blank/src/duckdb/core/vector.rs.html#50-122)
[§](#impl-FlatVector)

[Source](about:blank/src/duckdb/core/vector.rs.html#56-58)

Returns the capacity of the vector

[Source](about:blank/src/duckdb/core/vector.rs.html#61-76)

Returns true if the row at the given index is null

[Source](about:blank/src/duckdb/core/vector.rs.html#79-81)

Returns an unsafe mutable pointer to the vector’s

[Source](about:blank/src/duckdb/core/vector.rs.html#84-86)

Returns a slice of the vector

[Source](about:blank/src/duckdb/core/vector.rs.html#89-91)

Returns a slice of the vector up to a certain length

[Source](about:blank/src/duckdb/core/vector.rs.html#94-96)

Returns a mutable slice of the vector

[Source](about:blank/src/duckdb/core/vector.rs.html#99-101)

Returns a mutable slice of the vector up to a certain length

[Source](about:blank/src/duckdb/core/vector.rs.html#104-106)

Returns the logical type of the vector

[Source](about:blank/src/duckdb/core/vector.rs.html#109-115)

Set row as null

[Source](about:blank/src/duckdb/core/vector.rs.html#118-121)

Copy data to the vector.

[§](#impl-Freeze-for-FlatVector)

[§](#impl-RefUnwindSafe-for-FlatVector)

[§](#impl-Send-for-FlatVector)

[§](#impl-Sync-for-FlatVector)

[§](#impl-Unpin-for-FlatVector)

[§](#impl-UnwindSafe-for-FlatVector)
