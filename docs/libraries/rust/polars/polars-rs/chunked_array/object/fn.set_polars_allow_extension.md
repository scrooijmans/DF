# Function set_polars_allow_extension Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/extension/mod.rs.html#23" class="src">Source</a>

``` rust
pub fn set_polars_allow_extension(toggle: bool)
```

Expand description

Control whether extension types may be created.

If the environment variable POLARS_ALLOW_EXTENSION is set, this function has no effect.
