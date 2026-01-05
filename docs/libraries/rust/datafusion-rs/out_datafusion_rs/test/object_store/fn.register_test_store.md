# Function register_test_store Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test/object_store.rs.html#41-44" class="src">Source</a>

``` rust
pub fn register_test_store(ctx: &SessionContext, files: &[(&str, u64)])
```

Available on **non-WebAssembly** only.

Expand description

Registers a test object store with the provided `ctx`
