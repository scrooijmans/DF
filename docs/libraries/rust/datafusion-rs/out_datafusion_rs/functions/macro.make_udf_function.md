# Macro make_udf_function Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/macros.rs.html#74" class="src">Source</a>

``` rust
macro_rules! make_udf_function {
    ($UDF:ty, $NAME:ident) => { ... };
}
```

Expand description

Creates a singleton `ScalarUDF` of the `$UDF` function and a function named `$NAME` which returns that singleton.

This is used to ensure creating the list of `ScalarUDF` only happens once.
