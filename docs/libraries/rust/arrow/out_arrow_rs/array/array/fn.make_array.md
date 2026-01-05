# Function make_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/mod.rs.html#744" class="src">Source</a>

``` rust
pub fn make_array(data: ArrayData) -> Arc<dyn Array>
```

Expand description

Constructs an array using the input `data`. Returns a reference-counted `Array` instance.
