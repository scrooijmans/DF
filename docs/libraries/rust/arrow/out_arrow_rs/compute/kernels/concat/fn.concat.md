# Function concat Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/concat.rs.html#371" class="src">Source</a>

``` rust
pub fn concat(arrays: &[&dyn Array]) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Concatenate multiple [Array](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") of the same type into a single [ArrayRef](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef").
