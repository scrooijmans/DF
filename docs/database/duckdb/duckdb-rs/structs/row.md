# Row in duckdb - Rust

```
pub struct Row<'stmt> { /* private fields */ }
```

Expand description

A single result row of a query.

[Source](about:blank/src/duckdb/row.rs.html#295-673)
[§](#impl-Row%3C'stmt%3E)

[Source](about:blank/src/duckdb/row.rs.html#308-310)

Get the value of a particular column of the result row.

###### [§](#failure)Failure

Panics if calling [`row.get(idx)`](about:blank/struct.Row.html#method.get "method duckdb::Row::get") would return an error, including:

- If the underlying DuckDB column type is not a valid type as a source for `T`
- If the underlying DuckDB integral value is outside the range representable by `T`
- If `idx` is outside the range of columns in the returned query

[Source](about:blank/src/duckdb/row.rs.html#328-342)

Get the value of a particular column of the result row.

###### [§](#failure-1)Failure

Returns an `Error::InvalidColumnType` if the underlying DuckDB column type is not a valid type as a source for `T`.

Returns an `Error::InvalidColumnIndex` if `idx` is outside the valid column range for this row.

Returns an `Error::InvalidColumnName` if `idx` is not a valid column name for this row.

If the result type is i128 (which requires the `i128_blob` feature to be enabled), and the underlying DuckDB column is a blob whose size is not 16 bytes, `Error::InvalidColumnType` will also be returned.

[Source](about:blank/src/duckdb/row.rs.html#359-366)

Get the value of a particular column of the result row as a `ValueRef`, allowing data to be read out of a row without copying.

This `ValueRef` is valid only as long as this Row, which is enforced by it’s lifetime. This means that while this method is completely safe, it can be somewhat difficult to use, and most callers will be better served by [`get`](about:blank/struct.Row.html#method.get "method duckdb::Row::get") or [`get_unwrap`](about:blank/struct.Row.html#method.get_unwrap "method duckdb::Row::get_unwrap").

###### [§](#failure-2)Failure

Returns an `Error::InvalidColumnIndex` if `idx` is outside the valid column range for this row.

Returns an `Error::InvalidColumnName` if `idx` is not a valid column name for this row.

[Source](about:blank/src/duckdb/row.rs.html#670-672)

Get the value of a particular column of the result row as a `ValueRef`, allowing data to be read out of a row without copying.

This `ValueRef` is valid only as long as this Row, which is enforced by it’s lifetime. This means that while this method is completely safe, it can be difficult to use, and most callers will be better served by [`get`](about:blank/struct.Row.html#method.get "method duckdb::Row::get") or [`get_unwrap`](about:blank/struct.Row.html#method.get_unwrap "method duckdb::Row::get_unwrap").

###### [§](#failure-3)Failure

Panics if calling [`row.get_ref(idx)`](about:blank/struct.Row.html#method.get_ref "method duckdb::Row::get_ref") would return an error, including:

- If `idx` is outside the range of columns in the returned query.
- If `idx` is not a valid column name for this row.

[Source](about:blank/src/duckdb/row.rs.html#675-679)
[§](#impl-AsRef%3CStatement%3C'stmt%3E%3E-for-Row%3C'stmt%3E)

[Source](about:blank/src/duckdb/row.rs.html#676-678)
[§](#method.as_ref)

Converts this type into a shared reference of the (usually inferred) input type.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-()>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-16)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-16)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(A,+B,+C,+D,+E,+F,+G,+H,+I,+J,+K,+L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(B,+C,+D,+E,+F,+G,+H,+I,+J,+K,+L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-1)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-1)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(C,+D,+E,+F,+G,+H,+I,+J,+K,+L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-2)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-2)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(D,+E,+F,+G,+H,+I,+J,+K,+L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-3)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-3)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(E,+F,+G,+H,+I,+J,+K,+L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-4)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-4)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(F,+G,+H,+I,+J,+K,+L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-5)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-5)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(G,+H,+I,+J,+K,+L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-6)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-6)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(H,+I,+J,+K,+L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-7)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-7)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(I,+J,+K,+L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-8)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-8)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(J,+K,+L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-9)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-9)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(K,+L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-10)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-10)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(L,+M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-11)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-11)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(M,+N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-12)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-12)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(N,+O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-13)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-13)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(O,+P)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-14)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-14)

Performs the conversion.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](<#impl-TryFrom%3C%26Row%3C'a%3E%3E-for-(P,)>)

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#associatedtype.Error-15)

The type returned in the event of a conversion error.

[Source](about:blank/src/duckdb/row.rs.html#748)
[§](#method.try_from-15)

Performs the conversion.

[§](#impl-Freeze-for-Row%3C'stmt%3E)

[§](#impl-RefUnwindSafe-for-Row%3C'stmt%3E)

[§](#impl-Send-for-Row%3C'stmt%3E)

[§](#impl-Sync-for-Row%3C'stmt%3E)

[§](#impl-Unpin-for-Row%3C'stmt%3E)

[§](#impl-UnwindSafe-for-Row%3C'stmt%3E)
