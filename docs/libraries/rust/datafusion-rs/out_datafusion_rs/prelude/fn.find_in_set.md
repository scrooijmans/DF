# Function find_in_set Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/unicode/mod.rs.html#56-108" class="src">Source</a>

``` rust
pub fn find_in_set(string: Expr, strlist: Expr) -> Expr
```

Expand description

Returns a value in the range of 1 to N if the string `str` is in the string list `strlist` consisting of N substrings
