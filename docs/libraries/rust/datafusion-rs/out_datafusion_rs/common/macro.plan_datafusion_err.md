# Macro plan_datafusion_err Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/error.rs.html#808" class="src">Source</a>

``` rust
macro_rules! plan_datafusion_err {
    ($($args:expr),* $(; diagnostic = $DIAG:expr)?) => { ... };
}
```

Expand description

Macro wraps `$ERR` to add backtrace feature
