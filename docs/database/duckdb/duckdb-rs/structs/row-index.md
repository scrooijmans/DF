# RowIndex in duckdb - Rust

[duckdb](index.html)

## Trait RowIndex 

[Source](about:blank/src/duckdb/row.rs.html#692-696)

```
pub trait RowIndex: Sealed {
    // Required method
    fn idx(&self, stmt: &Statement<'_>) -> Result<usize>;
}
```

Expand description

A trait implemented by types that can index into columns of a row.

It is only implemented for `usize` and `&str`.

## Required Methods[§](#required-methods)

[Source](about:blank/src/duckdb/row.rs.html#695)

#### fn [idx](#tymethod.idx)(&self, stmt: &[Statement](struct.Statement.html "struct duckdb::Statement")<'\_>) -> [Result](type.Result.html "type duckdb::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Returns the index of the appropriate column, or `None` if no such column exists.

## Implementations on Foreign Types[§](#foreign-impls)

[Source](about:blank/src/duckdb/row.rs.html#709-714)
[§](#impl-RowIndex-for-%26str)

### impl [RowIndex](trait.RowIndex.html "trait duckdb::RowIndex") for &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](about:blank/src/duckdb/row.rs.html#711-713)
[§](#method.idx)

#### fn [idx](#tymethod.idx)(&self, stmt: &[Statement](struct.Statement.html "struct duckdb::Statement")<'\_>) -> [Result](type.Result.html "type duckdb::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](about:blank/src/duckdb/row.rs.html#698-707)
[§](#impl-RowIndex-for-usize)

### impl [RowIndex](trait.RowIndex.html "trait duckdb::RowIndex") for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](about:blank/src/duckdb/row.rs.html#700-706)
[§](#method.idx-1)

#### fn [idx](#tymethod.idx)(&self, stmt: &[Statement](struct.Statement.html "struct duckdb::Statement")<'\_>) -> [Result](type.Result.html "type duckdb::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

## Implementors[§](#implementors)
