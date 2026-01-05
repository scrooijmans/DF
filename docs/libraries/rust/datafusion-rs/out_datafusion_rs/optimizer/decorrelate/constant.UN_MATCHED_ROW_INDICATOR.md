# Constant UN_MATCHED_ROW_INDICATOR Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/decorrelate.rs.html#126" class="src">Source</a>

``` rust
pub const UN_MATCHED_ROW_INDICATOR: &'static str;
```

Expand description

Used to indicate the unmatched rows from the inner(subquery) table after the left out Join This is used to handle [the Count bug](https://github.com/apache/datafusion/issues/10553)
