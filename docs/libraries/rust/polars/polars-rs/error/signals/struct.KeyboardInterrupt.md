# Struct KeyboardInterrupt Copy item path

<a href="https://docs.rs/polars-error/0.51.0/x86_64-unknown-linux-gnu/src/polars_error/signals.rs.html#8" class="src">Source</a>

``` rust
pub struct KeyboardInterrupt;
```

Expand description

Python hooks SIGINT to instead generate a KeyboardInterrupt exception. So we do the same to try and abort long-running computations and return to Python so that the Python exception can be generated.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/error/signals/struct.KeyboardInterrupt.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/error/signals/struct.KeyboardInterrupt.html#blanket-implementations" class="anchor">§</a>
