# Function concat_elements_dyn Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/concat_elements.rs.html#176" class="src">Source</a>

``` rust
pub fn concat_elements_dyn(
    left: &dyn Array,
    right: &dyn Array,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Returns the elementwise concatenation of [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")s.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/concat_elements/fn.concat_elements_dyn.html#errors" class="doc-anchor">§</a>Errors

This function errors if the arrays are of different types.
