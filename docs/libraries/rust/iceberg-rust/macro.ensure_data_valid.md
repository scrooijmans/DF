# Macro ensure_data_valid Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/error.rs.html#470-476" class="src">Source</a>

``` rust
macro_rules! ensure_data_valid {
    ($cond: expr, $fmt: literal, $($arg:tt)*) => { ... };
}
```

Expand description

Helper macro to check arguments.

Example:

Following example check `a > 0`, otherwise returns an error.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/macro.ensure_data_valid.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
use iceberg::check;
ensure_data_valid!(a > 0, "{} is not positive.", a);
```
