# Function catch_keyboard_interrupt Copy item path

<a href="https://docs.rs/polars-error/0.51.0/x86_64-unknown-linux-gnu/src/polars_error/signals.rs.html#77-79" class="src">Source</a>

``` rust
pub fn catch_keyboard_interrupt<R, F>(try_fn: F) -> Result<R, KeyboardInterrupt>where
    F: FnOnce() -> R + UnwindSafe,
```

Expand description

Runs the passed function, catching any KeyboardInterrupts if they occur while running the function.
