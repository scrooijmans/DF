# Macro downcast_arg Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/macros.rs.html#140" class="src">Source</a>

``` rust
macro_rules! downcast_arg {
    ($ARG:expr, $ARRAY_TYPE:ident) => { ... };
}
```

Expand description

Downcast an argument to a specific array type, returning an internal error if the cast fails

\$ARG: ArrayRef \$ARRAY_TYPE: the type of array to cast the argument to
