# Type Alias Result Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/error.rs.html#25" class="src">Source</a>

``` rust
pub type Result<T> = Result<T, Error>;
```

Expand description

Result that is a wrapper of `Result<T, iceberg::Error>`

## Aliased Type<a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html#aliased-type" class="anchor">§</a>

``` rust
pub enum Result<T> {
    Ok(T),
    Err(Error),
}
```

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html#variant.Ok" class="anchor">§</a>1.0.0

### Ok(T)

Contains the success value

<a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html#variant.Err" class="anchor">§</a>1.0.0

### Err(<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>)

Contains the error value
