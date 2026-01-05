# Function get_temp_file Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/test_util.rs.html#41-62" class="src">Source</a>

``` rust
pub fn get_temp_file(file_name: &str, content: &[u8]) -> File
```

Available on **crate feature `test_utils`** only.

Expand description

Returns file handle for a temp file in ‘target’ directory with a provided content

TODO: Originates from `parquet` utils, can be merged in \[ARROW-4064\]
