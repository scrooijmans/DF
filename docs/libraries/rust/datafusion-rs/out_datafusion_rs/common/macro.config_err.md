# Macro config_err Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/error.rs.html#820" class="src">Source</a>

``` rust
macro_rules! config_err {
    ($($args:expr),* $(; diagnostic = $DIAG:expr)?) => { ... };
}
```

Expand description

Macro wraps Err(`$ERR`) to add backtrace feature
