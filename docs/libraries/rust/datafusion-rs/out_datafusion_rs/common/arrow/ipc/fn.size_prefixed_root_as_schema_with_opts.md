# Function size_prefixed_root_as_schema_with_opts Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Schema.rs.html#5570-5573" class="src">Source</a>

``` rust
pub fn size_prefixed_root_as_schema_with_opts<'b, 'o>(
    opts: &'o VerifierOptions,
    buf: &'b [u8],
) -> Result<Schema<'b>, InvalidFlatbuffer>
```

Expand description

Verifies, with the given verifier options, that a buffer of bytes contains a size prefixed `Schema` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_schema_unchecked`.
