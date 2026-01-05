# Macro downcast_value Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#116" class="src">Source</a>

``` rust
macro_rules! downcast_value {
    ($Value: expr, $Type: ident) => { ... };
    ($Value: expr, $Type: ident, $T: tt) => { ... };
}
```

Expand description

Downcast an Arrow Array to a concrete type, return an `DataFusionError::Internal` if the cast is not possible. In normal usage of DataFusion the downcast should always succeed.

Example: `let array = downcast_value!(values, Int32Array)`
