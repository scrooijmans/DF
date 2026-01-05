# Function print_columns Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/pretty.rs.html#168" class="src">Source</a>

``` rust
pub fn print_columns(
    col_name: &str,
    results: &[Arc<dyn Array>],
) -> Result<(), ArrowError>
```

Available on **crate feature `prettyprint`** only.

Expand description

Prints a visual representation of a list of column to stdout
