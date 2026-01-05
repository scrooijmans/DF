# Trait ChunkApproxNUnique Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#377" class="src">Source</a>

``` rust
pub trait ChunkApproxNUnique {
    // Required method
    fn approx_n_unique(&self) -> u32;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApproxNUnique.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApproxNUnique.html#tymethod.approx_n_unique" class="fn">approx_n_unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApproxNUnique.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApproxNUnique.html#impl-ChunkApproxNUnique-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApproxNUnique.html" class="trait" title="trait polars::prelude::ChunkApproxNUnique">ChunkApproxNUnique</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype" title="type polars::prelude::PolarsDataType::Physical">Physical</a>\<'a\>: for\<'a\> <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.TotalHash.html" class="trait" title="trait polars_utils::total_ord::TotalHash">TotalHash</a> + for\<'a\> <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.TotalEq.html" class="trait" title="trait polars_utils::total_ord::TotalEq">TotalEq</a> + for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + for\<'a\> <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.ToTotalOrd.html" class="trait" title="trait polars_utils::total_ord::ToTotalOrd">ToTotalOrd</a>, \<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype" title="type polars::prelude::PolarsDataType::Physical">Physical</a>\<'a\>\> as <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.ToTotalOrd.html" class="trait" title="trait polars_utils::total_ord::ToTotalOrd">ToTotalOrd</a>\>::<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.ToTotalOrd.html#associatedtype.TotalOrdItem" class="associatedtype" title="type polars_utils::total_ord::ToTotalOrd::TotalOrdItem">TotalOrdItem</a>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a>,
