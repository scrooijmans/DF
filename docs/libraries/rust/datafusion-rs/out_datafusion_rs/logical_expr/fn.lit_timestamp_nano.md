# Function lit_timestamp_nano Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/literal.rs.html#49" class="src">Source</a>

``` rust
pub fn lit_timestamp_nano<T>(n: T) -> Exprwhere
    T: TimestampLiteral,
```

Expand description

Create a literal timestamp expression
