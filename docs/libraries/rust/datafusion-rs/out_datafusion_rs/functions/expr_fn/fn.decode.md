# Function decode Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/encoding/mod.rs.html#29-37" class="src">Source</a>

``` rust
pub fn decode(input: Expr, encoding: Expr) -> Expr
```

Expand description

decode the `input`, using the `encoding`. encoding can be base64 or hex
