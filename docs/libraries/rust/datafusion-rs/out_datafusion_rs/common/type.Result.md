# Type Alias Result Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/error.rs.html#42" class="src">Source</a>

``` rust
pub type Result<T, E = DataFusionError> = Result<T, E>;
```

Expand description

Result type for operations that could result in an [DataFusionError](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError")

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.Result.html#aliased-type" class="anchor">§</a>

``` rust
pub enum Result<T, E = DataFusionError> {
    Ok(T),
    Err(E),
}
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.Result.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.Result.html#variant.Ok" class="anchor">§</a>1.0.0

### Ok(T)

Contains the success value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.Result.html#variant.Err" class="anchor">§</a>1.0.0

### Err(E)

Contains the error value
