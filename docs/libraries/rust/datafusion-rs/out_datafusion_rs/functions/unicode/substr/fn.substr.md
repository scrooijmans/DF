# Function substr Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/unicode/substr.rs.html#184" class="src">Source</a>

``` rust
pub fn substr(
    args: &[Arc<dyn Array>],
) -> Result<Arc<dyn Array>, DataFusionError>
```

Expand description

Extracts the substring of string starting at the start’th character, and extending for count characters if that is specified. (Same as substring(string from start for count).) substr(‘alphabet’, 3) = ‘phabet’ substr(‘alphabet’, 3, 2) = ‘ph’ The implementation uses UTF-8 code points as characters
