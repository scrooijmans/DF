# Function table_with_sequence Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test/mod.rs.html#209-220" class="src">Source</a>

``` rust
pub fn table_with_sequence(
    seq_start: i32,
    seq_end: i32,
) -> Result<Arc<dyn TableProvider>>
```

Available on **non-WebAssembly** only.

Expand description

Return a new table provider that has a single Int32 column with values between `seq_start` and `seq_end`
