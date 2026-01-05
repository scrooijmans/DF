# Macro unwrap_or_internal_err Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/error.rs.html#734" class="src">Source</a>

``` rust
macro_rules! unwrap_or_internal_err {
    ($Value: ident) => { ... };
}
```

Expand description

Unwrap an `Option` if possible. Otherwise return an `DataFusionError::Internal`. In normal usage of DataFusion the unwrap should always succeed.

Example: `let values = unwrap_or_internal_err!(values)`
