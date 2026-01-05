# Function try_schema_from_ipc_buffer Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/convert.rs.html#246" class="src">Source</a>

``` rust
pub fn try_schema_from_ipc_buffer(buffer: &[u8]) -> Result<Schema, ArrowError>
```

Expand description

Try deserialize the IPC format bytes into a schema
