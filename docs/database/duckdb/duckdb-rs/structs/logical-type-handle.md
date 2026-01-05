# LogicalTypeHandle in duckdb::core - Rust

## Struct LogicalTypeHandle 

[Source](about:blank/src/duckdb/core/logical_type.rs.html#113-115)

```
pub struct LogicalTypeHandle { /* private fields */ }
```

Expand description

[Source](about:blank/src/duckdb/core/logical_type.rs.html#161-292)
[§](#impl-LogicalTypeHandle)

[Source](about:blank/src/duckdb/core/logical_type.rs.html#168-174)

Creates a map type from its child type.

[Source](about:blank/src/duckdb/core/logical_type.rs.html#177-183)

Creates a list type from its child type.

[Source](about:blank/src/duckdb/core/logical_type.rs.html#186-192)

Creates an array type from its child type.

[Source](about:blank/src/duckdb/core/logical_type.rs.html#195-201)

Creates a decimal type from its `width` and `scale`.

[Source](about:blank/src/duckdb/core/logical_type.rs.html#205-207)

Retrieves the decimal width Returns 0 if the LogicalType is not a decimal

[Source](about:blank/src/duckdb/core/logical_type.rs.html#211-213)

Retrieves the decimal scale Returns 0 if the LogicalType is not a decimal

[Source](about:blank/src/duckdb/core/logical_type.rs.html#216-230)

Make a `LogicalType` for `struct`

[Source](about:blank/src/duckdb/core/logical_type.rs.html#233-247)

Make a `LogicalType` for `union`

[Source](about:blank/src/duckdb/core/logical_type.rs.html#250-253)

Logical type ID

[Source](about:blank/src/duckdb/core/logical_type.rs.html#256-263)

Logical type children num

[Source](about:blank/src/duckdb/core/logical_type.rs.html#268-279)

Logical type child name by idx

Panics if the logical type is not a struct or union

[Source](about:blank/src/duckdb/core/logical_type.rs.html#282-291)

Logical type child by idx

[§](#impl-Freeze-for-LogicalTypeHandle)

[§](#impl-RefUnwindSafe-for-LogicalTypeHandle)

[§](#impl-Send-for-LogicalTypeHandle)

[§](#impl-Sync-for-LogicalTypeHandle)

[§](#impl-Unpin-for-LogicalTypeHandle)

[§](#impl-UnwindSafe-for-LogicalTypeHandle)
