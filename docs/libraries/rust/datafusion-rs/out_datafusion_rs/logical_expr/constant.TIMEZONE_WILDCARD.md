# Constant TIMEZONE_WILDCARD Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/signature.rs.html#39" class="src">Source</a>

``` rust
pub const TIMEZONE_WILDCARD: &'static str;
```

Expand description

Constant that is used as a placeholder for any valid timezone. This is used where a function can accept a timestamp type with any valid timezone, it exists to avoid the need to enumerate all possible timezones. See [`TypeSignature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html "enum datafusion::logical_expr::TypeSignature") for more details.

Type coercion always ensures that functions will be executed using timestamp arrays that have a valid time zone. Functions must never return results with this timezone.
