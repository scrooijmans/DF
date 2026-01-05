# Type Alias SharedResult Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/error.rs.html#45" class="src">Source</a>

``` rust
pub type SharedResult<T> = Result<T, Arc<DataFusionError>>;
```

Expand description

Result type for operations that could result in an [DataFusionError](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError") and needs to be shared (wrapped into `Arc`).

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.SharedResult.html#aliased-type" class="anchor">§</a>

``` rust
pub enum SharedResult<T> {
    Ok(T),
    Err(Arc<DataFusionError>),
}
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.SharedResult.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.SharedResult.html#variant.Ok" class="anchor">§</a>1.0.0

### Ok(T)

Contains the success value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.SharedResult.html#variant.Err" class="anchor">§</a>1.0.0

### Err(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>)

Contains the error value
