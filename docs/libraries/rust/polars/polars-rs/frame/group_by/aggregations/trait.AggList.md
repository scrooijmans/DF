# Trait AggList Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/aggregations/agg_list.rs.html#7" class="src">Source</a>

``` rust
pub trait AggList {
    // Required method
    unsafe fn agg_list(&self, _groups: &GroupsType) -> Series;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#required-methods" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#tymethod.agg_list" class="fn">agg_list</a>(&self, \_groups: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#safety" class="doc-anchor">§</a>Safety

groups should be in bounds

## Implementors<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#impl-AggList-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html" class="trait" title="trait polars::prelude::aggregations::AggList">AggList</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#impl-AggList-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html" class="trait" title="trait polars::prelude::aggregations::AggList">AggList</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#impl-AggList-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html" class="trait" title="trait polars::prelude::aggregations::AggList">AggList</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#impl-AggList-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html" class="trait" title="trait polars::prelude::aggregations::AggList">AggList</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#impl-AggList-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html" class="trait" title="trait polars::prelude::aggregations::AggList">AggList</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#impl-AggList-for-ChunkedArray%3CStructType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html" class="trait" title="trait polars::prelude::aggregations::AggList">AggList</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#impl-AggList-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html" class="trait" title="trait polars::prelude::aggregations::AggList">AggList</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#impl-AggList-for-ChunkedArray%3CObjectType%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html" class="trait" title="trait polars::prelude::aggregations::AggList">AggList</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/frame/group_by/aggregations/trait.AggList.html#impl-AggList-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html" class="trait" title="trait polars::prelude::aggregations::AggList">AggList</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,
