# Macro polars_err Copy item path

<a href="https://docs.rs/polars-error/0.51.0/x86_64-unknown-linux-gnu/src/polars_error/lib.rs.html#331" class="src">Source</a>

``` rust
macro_rules! polars_err {
    ($variant:ident: $fmt:literal $(, $arg:expr)* $(,)?) => { ... };
    ($variant:ident: $fmt:literal $(, $arg:expr)*, hint = $hint:literal) => { ... };
    ($variant:ident: $err:expr $(,)?) => { ... };
    (expr = $expr:expr, $variant:ident: $err:expr $(,)?) => { ... };
    (expr = $expr:expr, $variant:ident: $fmt:literal, $($arg:tt)+) => { ... };
    (op = $op:expr, got = $arg:expr, expected = $expected:expr) => { ... };
    (opq = $op:ident, got = $arg:expr, expected = $expected:expr) => { ... };
    (un_impl = $op:ident) => { ... };
    (op = $op:expr, $arg:expr) => { ... };
    (op = $op:expr, $arg:expr, hint = $hint:literal) => { ... };
    (op = $op:expr, $lhs:expr, $rhs:expr) => { ... };
    (op = $op:expr, $arg1:expr, $arg2:expr, $arg3:expr) => { ... };
    (opidx = $op:expr, idx = $idx:expr, $arg:expr) => { ... };
    (oos = $($tt:tt)+) => { ... };
    (nyi = $($tt:tt)+) => { ... };
    (opq = $op:ident, $arg:expr) => { ... };
    (opq = $op:ident, $lhs:expr, $rhs:expr) => { ... };
    (bigidx, ctx = $ctx:expr, size = $size:expr) => { ... };
    (append) => { ... };
    (extend) => { ... };
    (unpack) => { ... };
    (not_in_enum,value=$value:expr,categories=$categories:expr) => { ... };
    (string_cache_mismatch) => { ... };
    (duplicate = $name:expr) => { ... };
    (duplicate_field = $name:expr) => { ... };
    (col_not_found = $name:expr) => { ... };
    (mismatch, col=$name:expr, expected=$expected:expr, found=$found:expr) => { ... };
    (oob = $idx:expr, $len:expr) => { ... };
    (agg_len = $agg_len:expr, $groups_len:expr) => { ... };
    (parse_fmt_idk = $dtype:expr) => { ... };
    (length_mismatch = $operation:expr, $lhs:expr, $rhs:expr) => { ... };
    (length_mismatch = $operation:expr, $lhs:expr, $rhs:expr, argument = $argument:expr, argument_idx = $argument_idx:expr) => { ... };
    (assertion_error = $objects:expr, $detail:expr, $lhs:expr, $rhs:expr) => { ... };
    (to_datetime_tz_mismatch) => { ... };
}
```
