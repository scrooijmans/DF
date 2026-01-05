# Trait AsList Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/list/mod.rs.html#29" class="src">Source</a>

``` rust
pub trait AsList {
    // Required method
    fn as_list(&self) -> &ChunkedArray<ListType>;
}
```

Available on **crate feature `polars-ops`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsList.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AsList.html#tymethod.as_list" class="fn">as_list</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsList.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsList.html#impl-AsList-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.AsList.html" class="trait" title="trait polars::prelude::AsList">AsList</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>
