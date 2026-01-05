# Trait ChunkShiftFill Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#612" class="src">Source</a>

``` rust
pub trait ChunkShiftFill<T, V>where
    T: PolarsDataType,{
    // Required method
    fn shift_and_fill(&self, periods: i64, fill_value: V) -> ChunkedArray<T>;
}
```

Expand description

Shift the values of a [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") by a number of periods.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShiftFill.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShiftFill.html#tymethod.shift_and_fill" class="fn">shift_and_fill</a>(&self, periods: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, fill_value: V) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

Shift the values by a given period and fill the parts that will be empty due to this operation with `fill_value`.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShiftFill.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShiftFill.html#impl-ChunkShiftFill%3CFixedSizeListType,+Option%3C%26Series%3E%3E-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShiftFill.html" class="trait" title="trait polars::prelude::ChunkShiftFill">ChunkShiftFill</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShiftFill.html#impl-ChunkShiftFill%3CListType,+Option%3C%26Series%3E%3E-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShiftFill.html" class="trait" title="trait polars::prelude::ChunkShiftFill">ChunkShiftFill</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShiftFill.html#impl-ChunkShiftFill%3CT,+Option%3C%3CT+as+PolarsDataType%3E::Physical%3C&#39;a%3E%3E%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShiftFill.html" class="trait" title="trait polars::prelude::ChunkShiftFill">ChunkShiftFill</a>\<T, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype" title="type polars::prelude::PolarsDataType::Physical">Physical</a>\<'a\>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\<IsNested = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>, IsObject = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFull.html" class="trait" title="trait polars::prelude::ChunkFull">ChunkFull</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype" title="type polars::prelude::PolarsDataType::Physical">Physical</a>\<'a\>\> + <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFullNull.html" class="trait" title="trait polars::prelude::ChunkFullNull">ChunkFullNull</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype" title="type polars::prelude::PolarsDataType::Physical">Physical</a>\<'b\>: for\<'b\> <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/trait.TotalOrd.html" class="trait" title="trait polars_utils::total_ord::TotalOrd">TotalOrd</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShiftFill.html#impl-ChunkShiftFill%3CObjectType%3CT%3E,+Option%3CObjectType%3CT%3E%3E%3E-for-ChunkedArray%3CObjectType%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShiftFill.html" class="trait" title="trait polars::prelude::ChunkShiftFill">ChunkShiftFill</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,
