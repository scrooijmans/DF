# Rows in duckdb - Rust

```
pub struct Rows<'stmt> { /* private fields */ }
```

Expand description

An handle for the resulting rows of a query.

[Source](about:blank/src/duckdb/row.rs.html#24-130)
[§](#impl-Rows%3C'stmt%3E)

[Source](about:blank/src/duckdb/row.rs.html#46-49)

Attempt to get the next row from the query. Returns `Ok(Some(Row))` if there is another row, `Err(...)` if there was an error getting the next row, and `Ok(None)` if all rows have been retrieved.

###### [§](#note)Note

This interface is not compatible with Rust’s `Iterator` trait, because the lifetime of the returned row is tied to the lifetime of `self`. This is a fallible “streaming iterator”. For a more natural interface, consider using [`query_map`](about:blank/struct.Statement.html#method.query_map "method duckdb::Statement::query_map") or [`query_and_then`](about:blank/struct.Statement.html#method.query_and_then "method duckdb::Statement::query_and_then") instead, which return types that implement `Iterator`.

[Source](about:blank/src/duckdb/row.rs.html#76-81)

Map over this `Rows`, converting it to a [`Map`](struct.Map.html "struct duckdb::Map"), which implements `FallibleIterator`.

**Note:** This method requires the closure to return `duckdb::Result<B>`. If you need to use custom error types, consider using [`and_then`](about:blank/struct.Rows.html#method.and_then "method duckdb::Rows::and_then") instead, which allows any error type that implements `From<duckdb::Error>`.

```
use fallible_iterator::FallibleIterator;
fn query(stmt: &mut Statement) -> Result<Vec<i64>> {
    let rows = stmt.query([])?;
    rows.map(|r| r.get(0)).collect()
}
```

[Source](about:blank/src/duckdb/row.rs.html#86-91)

Map over this `Rows`, converting it to a [`MappedRows`](struct.MappedRows.html "struct duckdb::MappedRows"), which implements `Iterator`.

[Source](about:blank/src/duckdb/row.rs.html#97-102)

Map over this `Rows` with a fallible function, converting it to a [`AndThenRows`](struct.AndThenRows.html "struct duckdb::AndThenRows"), which implements `Iterator` (instead of `FallibleStreamingIterator`).

[Source](about:blank/src/duckdb/row.rs.html#127-129)

Access the underlying statement

This method provides a way to access the `Statement` that created these `Rows` without additional borrowing conflicts. This is particularly useful when you need to access statement metadata (like column count or names) while iterating over results.

##### [§](#example)Example

```
fn process_results(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name FROM people")?;
    let mut rows = stmt.query([])?;

    let column_count = rows.as_ref().unwrap().column_count();
    println!("Processing {} columns", column_count);

    while let Some(row) = rows.next()? {
        // Process row...
    }
    Ok(())
}
```

[Source](about:blank/src/duckdb/row.rs.html#243-285)
[§](#impl-FallibleStreamingIterator-for-Rows%3C'stmt%3E)

`FallibleStreamingIterator` differs from the standard library’s `Iterator` in two ways:

- each call to `next` (sqlite3_step) can fail.
- returned `Row` is valid until `next` is called again or `Statement` is reset or finalized.

While these iterators cannot be used with Rust `for` loops, `while let` loops offer a similar level of ergonomics:

```
fn query(stmt: &mut Statement) -> Result<()> {
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        // scan columns value
    }
    Ok(())
}
```

[Source](about:blank/src/duckdb/row.rs.html#244)
[§](#associatedtype.Error)

The error type of iteration.

[Source](about:blank/src/duckdb/row.rs.html#245)
[§](#associatedtype.Item)

The type being iterated over.

[Source](about:blank/src/duckdb/row.rs.html#248-279)
[§](#method.advance)

Advances the iterator to the next position. [Read more](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/fallible_streaming_iterator/trait.FallibleStreamingIterator.html#tymethod.advance)

[Source](about:blank/src/duckdb/row.rs.html#282-284)
[§](#method.get)

Returns the current element. [Read more](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/fallible_streaming_iterator/trait.FallibleStreamingIterator.html#tymethod.get)

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#51)
[§](#method.next-1)

Advances the iterator, returning the next element. [Read more](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/fallible_streaming_iterator/trait.FallibleStreamingIterator.html#method.next)

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#58)
[§](#method.size_hint)

Returns bounds on the number of remaining elements in the iterator.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#64-67)
[§](#method.all)

Determines if all elements of the iterator satisfy a predicate.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#79-82)
[§](#method.any)

Determines if any elements of the iterator satisfy a predicate.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#92-94)
[§](#method.by_ref)

Borrows an iterator, rather than consuming it. [Read more](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/fallible_streaming_iterator/trait.FallibleStreamingIterator.html#method.by_ref)

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#101-103)
[§](#method.count)

Returns the number of remaining elements in the iterator.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#114-117)
[§](#method.filter)

Returns an iterator which filters elements by a predicate.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#124-127)
[§](#method.find)

Returns the first element of the iterator which satisfies a predicate.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#145-148)
[§](#method.for_each)

Calls a closure on each element of an iterator.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#158-160)
[§](#method.fuse)

Returns an iterator which is well-behaved at the beginning and end of iteration.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#170-173)
[§](#method.map-1)

Returns an iterator which applies a transform to elements.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#187-190)
[§](#method.map_ref)

Returns an iterator which applies a transform to elements. [Read more](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/fallible_streaming_iterator/trait.FallibleStreamingIterator.html#method.map_ref)

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#197-200)
[§](#method.map_err)

Returns an iterator that applies a transform to errors.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#207)
[§](#method.nth)

Returns the `nth` element of the iterator.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#219-222)
[§](#method.position)

Returns the position of the first element matching a predicate.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#236-238)
[§](#method.skip)

Returns an iterator which skips the first `n` elements.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#245-248)
[§](#method.skip_while)

Returns an iterator which skips the first sequence of elements matching a predicate.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#259-261)
[§](#method.take)

Returns an iterator which only returns the first `n` elements.

[Source](https://docs.rs/fallible-streaming-iterator/0.1.9/x86_64-unknown-linux-gnu/src/fallible_streaming_iterator/lib.rs.html#272-275)
[§](#method.take_while)

Returns an iterator which only returns the first sequence of elements matching a predicate.

[§](#impl-Freeze-for-Rows%3C'stmt%3E)

[§](#impl-RefUnwindSafe-for-Rows%3C'stmt%3E)

[§](#impl-Send-for-Rows%3C'stmt%3E)

[§](#impl-Sync-for-Rows%3C'stmt%3E)

[§](#impl-Unpin-for-Rows%3C'stmt%3E)

[§](#impl-UnwindSafe-for-Rows%3C'stmt%3E)
