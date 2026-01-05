# Function set_warning_function Copy item path

<a href="https://docs.rs/polars-error/0.51.0/x86_64-unknown-linux-gnu/src/polars_error/warning.rs.html#12" class="src">Source</a>

``` rust
pub fn set_warning_function(function: fn(&str, PolarsWarning))
```

Expand description

Set the function that will be called by the `polars_warn!` macro. You can use this to set logging in polars.
