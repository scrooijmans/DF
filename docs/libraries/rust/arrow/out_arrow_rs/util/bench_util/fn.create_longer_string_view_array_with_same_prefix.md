# Function create_longer_string_view_array_with_same_prefix Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#145-150" class="src">Source</a>

``` rust
pub fn create_longer_string_view_array_with_same_prefix(
    size: usize,
    null_density: f32,
) -> StringViewArray
```

Available on **crate feature `test_utils`** only.

Expand description

Creates longer string view array with same prefix, the prefix should be larger than 4 bytes, and the string length should be larger than 12 bytes so that we can compare the StringArray performance with StringViewArray, because StringViewArray has 4 bytes inline for view
