# Function regexp_count_inner Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/regex/regexpcount.rs.html#263-273" class="src">Source</a>

``` rust
pub fn regexp_count_inner<'a, S>(
    values: S,
    regex_array: S,
    is_regex_scalar: bool,
    start_array: Option<&PrimitiveArray<Int64Type>>,
    is_start_scalar: bool,
    flags_array: Option<S>,
    is_flags_scalar: bool,
) -> Result<Arc<dyn Array>, ArrowError>where
    S: StringArrayType<'a>,
```
