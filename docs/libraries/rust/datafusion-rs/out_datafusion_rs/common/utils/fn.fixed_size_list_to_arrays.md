# Function fixed_size_list_to_arrays Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/utils/mod.rs.html#494" class="src">Source</a>

``` rust
pub fn fixed_size_list_to_arrays(a: &Arc<dyn Array>) -> Vec<Arc<dyn Array>>
```

Expand description

Helper function to convert a FixedSizeListArray into a vector of ArrayRefs.
