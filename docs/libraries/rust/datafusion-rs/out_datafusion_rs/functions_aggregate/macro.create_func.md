# Macro create_func Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/macros.rs.html#65" class="src">Source</a>

``` rust
macro_rules! create_func {
    ($UDAF:ty, $AGGREGATE_UDF_FN:ident) => { ... };
    ($UDAF:ty, $AGGREGATE_UDF_FN:ident, $CREATE:expr) => { ... };
}
```
