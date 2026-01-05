# Trait ChunkUnique Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#359" class="src">Source</a>

``` rust
pub trait ChunkUnique {
    // Required methods
    fn unique(&self) -> Result<Self, PolarsError>
       where Self: Sized;
    fn arg_unique(&self) -> Result<ChunkedArray<UInt32Type>, PolarsError>;

    // Provided method
    fn n_unique(&self) -> Result<usize, PolarsError> { ... }
}
```

Expand description

Get unique values in a `ChunkedArray`

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html#tymethod.unique" class="fn">unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Get unique values of a ChunkedArray

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html#tymethod.arg_unique" class="fn">arg_unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get first index of the unique values in a `ChunkedArray`. This Vec is sorted.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html#method.n_unique" class="fn">n_unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Number of unique values in the `ChunkedArray`

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html#impl-ChunkUnique-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html" class="trait" title="trait polars::prelude::ChunkUnique">ChunkUnique</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html#impl-ChunkUnique-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html" class="trait" title="trait polars::prelude::ChunkUnique">ChunkUnique</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html#impl-ChunkUnique-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html" class="trait" title="trait polars::prelude::ChunkUnique">ChunkUnique</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html#impl-ChunkUnique-for-ChunkedArray%3CObjectType%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html" class="trait" title="trait polars::prelude::ChunkUnique">ChunkUnique</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html#impl-ChunkUnique-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkUnique.html" class="trait" title="trait polars::prelude::ChunkUnique">ChunkUnique</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>: <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.TotalHash.html" class="trait" title="trait polars_utils::total_ord::TotalHash">TotalHash</a> + <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.TotalEq.html" class="trait" title="trait polars_utils::total_ord::TotalEq">TotalEq</a> + <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.ToTotalOrd.html" class="trait" title="trait polars_utils::total_ord::ToTotalOrd">ToTotalOrd</a>, \<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a> as <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.ToTotalOrd.html" class="trait" title="trait polars_utils::total_ord::ToTotalOrd">ToTotalOrd</a>\>::<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.ToTotalOrd.html#associatedtype.TotalOrdItem" class="associatedtype" title="type polars_utils::total_ord::ToTotalOrd::TotalOrdItem">TotalOrdItem</a>: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>: for\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>,
