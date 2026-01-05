# Function unqualified_field_not_found Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/error.rs.html#925" class="src">Source</a>

``` rust
pub fn unqualified_field_not_found(
    name: &str,
    schema: &DFSchema,
) -> DataFusionError
```

Expand description

Convenience wrapper over [`field_not_found`](https://docs.rs/datafusion/50.2.0/datafusion/common/fn.field_not_found.html "fn datafusion::common::field_not_found") for when there is no qualifier
