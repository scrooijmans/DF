# Macro polars_warn Copy item path

<a href="https://docs.rs/polars-error/0.51.0/x86_64-unknown-linux-gnu/src/polars_error/warning.rs.html#29" class="src">Source</a>

``` rust
macro_rules! polars_warn {
    ($variant:ident, $fmt:literal $(, $arg:expr)* $(,)?) => { ... };
    ($fmt:literal, $($arg:expr)+) => { ... };
    ($($arg:tt)+) => { ... };
}
```
