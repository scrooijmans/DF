# Function make_partition Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test/mod.rs.html#223-232" class="src">Source</a>

``` rust
pub fn make_partition(sz: i32) -> RecordBatch
```

Available on **non-WebAssembly** only.

Expand description

Return a RecordBatch with a single Int32 array with values (0..sz)
