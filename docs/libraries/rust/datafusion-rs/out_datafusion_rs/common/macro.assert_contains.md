# Macro assert_contains Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/test_util.rs.html#163" class="src">Source</a>

``` rust
macro_rules! assert_contains {
    ($ACTUAL: expr, $EXPECTED: expr) => { ... };
}
```

Expand description

A macro to assert that one string is contained within another with a nice error message if they are not.

Usage: `assert_contains!(actual, expected)`

Is a macro so test error messages are on the same line as the failure;

Both arguments must be convertible into Strings ([`Into`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")\<[`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")\>)
