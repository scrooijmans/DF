# Trait AsArray Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/array/mod.rs.html#18" class="src">Source</a>

``` rust
pub trait AsArray {
    // Required method
    fn as_array(&self) -> &ChunkedArray<FixedSizeListType>;
}
```

Available on **crate feature `polars-ops`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/array/trait.AsArray.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/array/trait.AsArray.html#tymethod.as_array" class="fn">as_array</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/array/trait.AsArray.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/array/trait.AsArray.html#impl-AsArray-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/array/trait.AsArray.html" class="trait" title="trait polars::prelude::array::AsArray">AsArray</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>
