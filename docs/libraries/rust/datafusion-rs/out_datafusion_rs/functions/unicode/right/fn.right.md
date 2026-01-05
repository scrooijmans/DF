# Function right Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/unicode/right.rs.html#125" class="src">Source</a>

``` rust
pub fn right<T>(
    args: &[Arc<dyn Array>],
) -> Result<Arc<dyn Array>, DataFusionError>where
    T: OffsetSizeTrait,
```

Expand description

Returns last n characters in the string, or when n is negative, returns all but first \|n\| characters. right(‘abcde’, 2) = ‘de’ The implementation uses UTF-8 code points as characters
