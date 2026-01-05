# Function coalesce_ranges Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/util.rs.html#105-136" class="src">Source</a>

``` rust
pub async fn coalesce_ranges<F, E, Fut>(
    ranges: &[Range<u64>],
    fetch: F,
    coalesce: u64,
) -> Result<Vec<Bytes>, E>where
    F: Send + FnMut(Range<u64>) -> Fut,
    E: Send,
    Fut: Future<Output = Result<Bytes, E>> + Send,
```

Expand description

Takes a function `fetch` that can fetch a range of bytes and uses this to fetch the provided byte `ranges`

To improve performance it will:

- Combine ranges less than `coalesce` bytes apart into a single call to `fetch`
- Make multiple `fetch` requests in parallel (up to maximum of 10)
