# Function resolve_homedir Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/path_utils/mod.rs.html#125" class="src">Source</a>

``` rust
pub fn resolve_homedir(path: &dyn AsRef<Path>) -> PathBuf
```

Available on **crate feature `polars-io`** only.

Expand description

Replaces a “~” in the Path with the home directory.
