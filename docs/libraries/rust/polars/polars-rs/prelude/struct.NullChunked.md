# Struct NullChunked Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/implementations/null.rs.html#20" class="src">Source</a>

``` rust
pub struct NullChunked { /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#impl-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#impl-AggList-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html" class="trait" title="trait polars::prelude::aggregations::AggList">AggList</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.agg_list" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html#tymethod.agg_list" class="fn">agg_list</a>(&self, groups: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/aggregations/trait.AggList.html#tymethod.agg_list)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#impl-ChunkAnyValue-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html" class="trait" title="trait polars::prelude::ChunkAnyValue">ChunkAnyValue</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.get_any_value_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html#tymethod.get_any_value_unchecked" class="fn">get_any_value_unchecked</a>(&self, \_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

Get a single value. Beware this is slow. If you need to use this slightly performant, cast Categorical to UInt32 [Read more](https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html#tymethod.get_any_value_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.get_any_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAnyValue.html#tymethod.get_any_value" class="fn">get_any_value</a>(&self, \_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get a single value. Beware this is slow.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#impl-ChunkCompareEq%3C%26NullChunked%3E-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.equal" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal" class="fn">equal</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.equal_missing" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal_missing" class="fn">equal_missing</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.not_equal" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal" class="fn">not_equal</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.not_equal_missing" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal_missing" class="fn">not_equal_missing</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#impl-ChunkCompareIneq%3C%26NullChunked%3E-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.gt" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt" class="fn">gt</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.gt_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt_eq" class="fn">gt_eq</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than or equal comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.lt" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt" class="fn">lt</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.lt_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt_eq" class="fn">lt_eq</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than or equal comparison

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#impl-ChunkNestingUtils-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html" class="trait" title="trait polars::prelude::ChunkNestingUtils">ChunkNestingUtils</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.propagate_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#tymethod.propagate_nulls" class="fn">propagate_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>

Propagate nulls of nested datatype to all levels of nesting.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.trim_lists_to_normalized_offsets" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#tymethod.trim_lists_to_normalized_offsets" class="fn">trim_lists_to_normalized_offsets</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>

Trim all lists of unused start and end elements recursively.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.find_validity_mismatch" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#tymethod.find_validity_mismatch" class="fn">find_validity_mismatch</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, idxs: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)

Find the indices of the values where the validity mismatches. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#tymethod.find_validity_mismatch)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#impl-Clone-for-NullChunked" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#impl-IntoSeries-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.into_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#tymethod.into_series" class="fn">into_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.is_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#method.is_series" class="fn">is_series</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#impl-SeriesTrait-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

Name of series.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.rename" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.rename" class="fn">rename</a>(&mut self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)

Rename the Series.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.chunks" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.chunks" class="fn">chunks</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>

Underlying chunks.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.chunks_mut" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.chunks_mut" class="fn">chunks_mut</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>

Underlying chunks. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.chunks_mut)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.chunk_lengths" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.chunk_lengths" class="fn">chunk_lengths</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/map/struct.Map.html" class="struct" title="struct core::iter::adapters::map::Map">Map</a>\<<a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'\_, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive">fn</a>(&<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#" class="tooltip" data-notable-ty="Map&lt;Iter&lt;&#39;_, Box&lt;dyn Array&gt;&gt;, fn(&amp;Box&lt;dyn Array&gt;) -&gt; usize&gt;">ⓘ</a>

Get the lengths of the underlying chunks

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.take" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take" class="fn">take</a>( &self, indices: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take from `self` at the indexes given by `idx`. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.take_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take_unchecked" class="fn">take_unchecked</a>(&self, indices: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take from `self` at the indexes given by `idx`. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.take_slice" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take_slice" class="fn">take_slice</a>(&self, indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take from `self` at the indexes given by `idx`. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.take_slice_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take_slice_unchecked" class="fn">take_slice_unchecked</a>(&self, indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take from `self` at the indexes given by `idx`. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take_slice_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.len-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get length of series.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.has_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.has_nulls" class="fn">has_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return if any the chunks in this [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") have nulls.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.rechunk" class="fn">rechunk</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Aggregate all chunks to a contiguous array of memory.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.drop_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.drop_nulls" class="fn">drop_nulls</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Drop all null values and return a new Series.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.cast" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.cast" class="fn">cast</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, \_cast_options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Count the null values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.unique" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.unique" class="fn">unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get unique values in the Series.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.n_unique" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.n_unique" class="fn">n_unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get unique values in the Series. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.n_unique)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.arg_unique" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.arg_unique" class="fn">arg_unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get first indexes of unique values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.new_from_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.new_from_index" class="fn">new_from_index</a>(&self, \_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Create a new Series filled with values from the given index. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.new_from_index)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.get_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.get_unchecked" class="fn">get_unchecked</a>(&self, \_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

Get a single value by index. Don’t use this operation for loops as a runtime cast is needed for every iteration. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.get_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.slice" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Get a zero copy view of the data. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.split_at" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.split_at" class="fn">split_at</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> (<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>)

Get a zero copy view of the data. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.split_at)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.sort_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.sort_with" class="fn">sort_with</a>(&self, \_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.arg_sort" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.arg_sort" class="fn">arg_sort</a>(&self, \_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

Retrieve the indexes needed for a sort.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.is_null" class="fn">is_null</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

Get a mask of the null values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.is_not_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.is_not_null" class="fn">is_not_null</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

Get a mask of the non-null values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.reverse" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.reverse" class="fn">reverse</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

return a Series in reversed order

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.filter" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.filter" class="fn">filter</a>( &self, filter: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Filter by boolean mask. This operation clones data.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.shift" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.shift" class="fn">shift</a>(&self, \_periods: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Shift the values by a given period and fill the parts that will be empty due to this operation with `Nones`. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.shift)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.append" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.append" class="fn">append</a>(&mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.append_owned" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.append_owned" class="fn">append_owned</a>(&mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.approx_n_unique" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.approx_n_unique" class="fn">approx_n_unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.clone_inner" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.clone_inner" class="fn">clone_inner</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a>\>

Clone inner ChunkedArray and wrap in a new Arc

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.find_validity_mismatch-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.find_validity_mismatch" class="fn">find_validity_mismatch</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, idxs: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)

Find the indices of elements where the null masks are different recursively.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Get a hold of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray"), [`Logical`](https://docs.rs/polars/latest/polars/prelude/struct.Logical.html "struct polars::prelude::Logical") or `NullChunked` as an `Any` trait reference.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Get a hold of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray"), [`Logical`](https://docs.rs/polars/latest/polars/prelude/struct.Logical.html "struct polars::prelude::Logical") or `NullChunked` as an `Any` trait mutable reference.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.as_phys_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.as_phys_any" class="fn">as_phys_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Get a hold of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") or `NullChunked` as an `Any` trait reference. This pierces through `Logical` types to get the underlying physical array.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.as_arc_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.as_arc_any" class="fn">as_arc_any</a>(self: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.field" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.field" class="fn">field</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\>

Get field (used in schema)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Get datatype of series.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.n_chunks" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.n_chunks" class="fn">n_chunks</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Number of chunks in this Series

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrink the capacity of this array to fit its length.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.limit" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.limit" class="fn">limit</a>(&self, num_elements: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take `num_elements` from the top as a zero copy view.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.is_empty-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if Series is empty.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.rechunk_validity" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.rechunk_validity" class="fn">rechunk_validity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method._sum_as_f64" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method._sum_as_f64" class="fn">_sum_as_f64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

Returns the sum of the array as an f64.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.mean" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.mean" class="fn">mean</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

Returns the mean value in the array Returns an option because the array is nullable.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.std" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.std" class="fn">std</a>(&self, \_ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

Returns the std value in the array Returns an option because the array is nullable.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.var" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.var" class="fn">var</a>(&self, \_ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

Returns the var value in the array Returns an option because the array is nullable.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.median" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.median" class="fn">median</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

Returns the median value in the array Returns an option because the array is nullable.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.trim_lists_to_normalized_offsets-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.trim_lists_to_normalized_offsets" class="fn">trim_lists_to_normalized_offsets</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>

Trim all lists of unused start and end elements recursively. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.trim_lists_to_normalized_offsets)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.propagate_nulls-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.propagate_nulls" class="fn">propagate_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>

Propagate down nulls in nested types. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.propagate_nulls)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.get" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.get" class="fn">get</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get a single value by index. Don’t use this operation for loops as a runtime cast is needed for every iteration.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.as_single_ptr" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.as_single_ptr" class="fn">as_single_ptr</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Rechunk and return a pointer to the start of the Series. Only implemented for numeric types

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.sum_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.sum_reduce" class="fn">sum_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the sum of the Series as a new Scalar. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.sum_reduce)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.max_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.max_reduce" class="fn">max_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the max of the Series as a new Series of length 1.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.min_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.min_reduce" class="fn">min_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the min of the Series as a new Series of length 1.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.median_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.median_reduce" class="fn">median_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the median of the Series as a new Series of length 1.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.var_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.var_reduce" class="fn">var_reduce</a>(&self, \_ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the variance of the Series as a new Series of length 1.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.std_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.std_reduce" class="fn">std_reduce</a>(&self, \_ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the standard deviation of the Series as a new Series of length 1.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.quantile_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.quantile_reduce" class="fn">quantile_reduce</a>( &self, \_quantile: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, \_method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the quantile of the ChunkedArray as a new Series of length 1.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.and_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.and_reduce" class="fn">and_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the bitwise AND of the Series as a new Series of length 1,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.or_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.or_reduce" class="fn">or_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the bitwise OR of the Series as a new Series of length 1,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.xor_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.xor_reduce" class="fn">xor_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the bitwise XOR of the Series as a new Series of length 1,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.first" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.first" class="fn">first</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

Get the first element of the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") as a [`Scalar`](https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html "struct polars::prelude::Scalar") [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.first)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.last" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.last" class="fn">last</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

Get the last element of the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") as a [`Scalar`](https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html "struct polars::prelude::Scalar") [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.last)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.get_object" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.get_object" class="fn">get_object</a>(&self, \_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html" class="trait" title="trait polars::chunked_array::object::PolarsObjectSafe">PolarsObjectSafe</a> + 'static)\>

Get the value at this index as a downcastable Any trait ref.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.get_object_chunked_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.get_object_chunked_unchecked" class="fn">get_object_chunked_unchecked</a>( &self, \_chunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html" class="trait" title="trait polars::chunked_array::object::PolarsObjectSafe">PolarsObjectSafe</a> + 'static)\>

Get the value at this index as a downcastable Any trait ref. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.get_object_chunked_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.checked_div" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.checked_div" class="fn">checked_div</a>(&self, \_rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.rolling_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.rolling_map" class="fn">rolling_map</a>( &self, \_f: &dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>, \_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a custom function over a rolling/ moving window of the array. This has quite some dynamic dispatch, so prefer rolling_min, max, mean, sum over this.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#impl-VecHash-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.VecHash.html" class="trait" title="trait polars::prelude::VecHash">VecHash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.vec_hash" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.VecHash.html#tymethod.vec_hash" class="fn">vec_hash</a>( &self, random_state: <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.SeedableRandomState.html" class="struct" title="struct foldhash::quality::SeedableRandomState">SeedableRandomState</a>, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Compute the hash for all values in the array.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#method.vec_hash_combine" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.VecHash.html#tymethod.vec_hash_combine" class="fn">vec_hash_combine</a>( &self, random_state: <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.SeedableRandomState.html" class="struct" title="struct foldhash::quality::SeedableRandomState">SeedableRandomState</a>, hashes: &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html#blanket-implementations" class="anchor">§</a>
