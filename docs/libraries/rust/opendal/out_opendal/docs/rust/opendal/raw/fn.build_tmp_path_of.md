# Function build_tmp_path_of Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/path.rs.html#225-255" class="src">Source</a>

``` rust
pub fn build_tmp_path_of(path: &str) -> String
```

Expand description

Build a temporary path of a file path.

`build_tmp_path_of` appends a dot following a random generated postfix. Donâ€™t use it with a path to a folder.
