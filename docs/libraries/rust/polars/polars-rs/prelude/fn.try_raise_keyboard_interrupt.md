# Function try_raise_keyboard_interrupt Copy item path

<a href="https://docs.rs/polars-error/0.51.0/x86_64-unknown-linux-gnu/src/polars_error/signals.rs.html#63" class="src">Source</a>

``` rust
pub fn try_raise_keyboard_interrupt()
```

Expand description

Checks if the keyboard interrupt flag is set, and if yes panics as a keyboard interrupt. This function is very cheap.
