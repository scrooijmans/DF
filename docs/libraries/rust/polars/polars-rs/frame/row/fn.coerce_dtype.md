# Function coerce_dtype Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/row/mod.rs.html#139" class="src">Source</a>

``` rust
pub fn coerce_dtype<A>(datatypes: &[A]) -> DataTypewhere
    A: Borrow<DataType>,
```

Expand description

Coerces a slice of datatypes into a single supertype.
