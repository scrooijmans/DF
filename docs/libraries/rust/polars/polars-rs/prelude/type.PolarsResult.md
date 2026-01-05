# Type Alias PolarsResult Copy item path

<a href="https://docs.rs/polars-error/0.51.0/x86_64-unknown-linux-gnu/src/polars_error/lib.rs.html#203" class="src">Source</a>

``` rust
pub type PolarsResult<T> = Result<T, PolarsError>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/type.PolarsResult.html#aliased-type" class="anchor">§</a>

``` rust
pub enum PolarsResult<T> {
    Ok(T),
    Err(PolarsError),
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/type.PolarsResult.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/type.PolarsResult.html#variant.Ok" class="anchor">§</a>1.0.0

### Ok(T)

Contains the success value

<a href="https://docs.rs/polars/latest/polars/prelude/type.PolarsResult.html#variant.Err" class="anchor">§</a>1.0.0

### Err(<a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>)

Contains the error value
