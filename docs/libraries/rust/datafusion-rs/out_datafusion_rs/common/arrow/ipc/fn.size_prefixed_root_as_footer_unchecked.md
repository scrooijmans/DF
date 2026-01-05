# Function size_prefixed_root_as_footer_unchecked Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/File.rs.html#479" class="src">Source</a>

``` rust
pub unsafe fn size_prefixed_root_as_footer_unchecked(buf: &[u8]) -> Footer<'_>
```

Expand description

Assumes, without verification, that a buffer of bytes contains a size prefixed Footer and returns it.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_footer_unchecked.html#safety" class="doc-anchor">§</a>Safety

Callers must trust the given bytes do indeed contain a valid size prefixed `Footer`.
