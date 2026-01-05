# Function col Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/selectors.rs.html#27-29" class="src">Source</a>

``` rust
pub fn col<S>(name: S) -> Exprwhere
    S: Into<PlSmallStr>,
```

Available on **crate feature `lazy`** only.

Expand description

Create a Column Expression based on a column name.

## <a href="https://docs.rs/polars/latest/polars/prelude/fn.col.html#arguments" class="doc-anchor">§</a>Arguments

- `name` - A string slice that holds the name of the column. If a column with this name does not exist when the LazyFrame is collected, an error is returned.

## <a href="https://docs.rs/polars/latest/polars/prelude/fn.col.html#examples" class="doc-anchor">§</a>Examples

<a href="https://docs.rs/polars/latest/polars/prelude/fn.col.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
// select a column name
col("foo")
```

<a href="https://docs.rs/polars/latest/polars/prelude/fn.col.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
// select all columns by using a wildcard
col("*")
```

<a href="https://docs.rs/polars/latest/polars/prelude/fn.col.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
// select specific columns by writing a regular expression that starts with `^` and ends with `$`
// only if regex features is activated
col("^foo.*$")
```
