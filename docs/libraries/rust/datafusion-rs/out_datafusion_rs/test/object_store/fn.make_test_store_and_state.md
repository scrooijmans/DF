# Function make_test_store_and_state Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test/object_store.rs.html#47-62" class="src">Source</a>

``` rust
pub fn make_test_store_and_state(
    files: &[(&str, u64)],
) -> (Arc<InMemory>, SessionState)
```

Available on **non-WebAssembly** only.

Expand description

Create a test object store with the provided files
