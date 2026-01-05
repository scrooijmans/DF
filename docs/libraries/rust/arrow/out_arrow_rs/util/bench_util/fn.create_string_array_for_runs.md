# Function create_string_array_for_runs Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#415-445" class="src">Source</a>

``` rust
pub fn create_string_array_for_runs(
    physical_array_len: usize,
    logical_array_len: usize,
    string_len: usize,
) -> Vec<String>
```

Available on **crate feature `test_utils`** only.

Expand description

Create string array to be used by run array builder. The string array will result in run array with physical length of `physical_array_len` and logical length of `logical_array_len`
