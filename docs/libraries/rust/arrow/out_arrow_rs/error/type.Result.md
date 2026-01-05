# Type Alias Result Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/error.rs.html#23" class="src">Source</a>

``` rust
pub type Result<T> = Result<T, ArrowError>;
```

Expand description

A specialized `Result` type for Arrow operations.

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/error/type.Result.html#aliased-type" class="anchor">§</a>

``` rust
pub enum Result<T> {
    Ok(T),
    Err(ArrowError),
}
```

## Variants<a href="https://docs.rs/arrow/latest/arrow/error/type.Result.html#variants" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/error/type.Result.html#variant.Ok" class="anchor">§</a>1.0.0

### Ok(T)

Contains the success value

<a href="https://docs.rs/arrow/latest/arrow/error/type.Result.html#variant.Err" class="anchor">§</a>1.0.0

### Err(<a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>)

Contains the error value
