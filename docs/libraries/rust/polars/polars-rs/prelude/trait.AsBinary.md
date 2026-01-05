# Trait AsBinary Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/binary/mod.rs.html#6" class="src">Source</a>

``` rust
pub trait AsBinary {
    // Required method
    fn as_binary(&self) -> &ChunkedArray<BinaryType>;
}
```

Available on **crate feature `polars-ops`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsBinary.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AsBinary.html#tymethod.as_binary" class="fn">as_binary</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsBinary.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsBinary.html#impl-AsBinary-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.AsBinary.html" class="trait" title="trait polars::prelude::AsBinary">AsBinary</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>
