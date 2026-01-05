# Function root_as_schema Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Schema.rs.html#5537" class="src">Source</a>

``` rust
pub fn root_as_schema(buf: &[u8]) -> Result<Schema<'_>, InvalidFlatbuffer>
```

Expand description

Verifies that a buffer of bytes contains a `Schema` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_schema_unchecked`.
