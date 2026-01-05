# Struct StructType Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/mod.rs.html#264" class="src">Source</a>

``` rust
pub struct StructType {}
```

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#impl-ChunkExpandAtIndex%3CStructType%3E-for-ChunkedArray%3CStructType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExpandAtIndex.html" class="trait" title="trait polars::prelude::ChunkExpandAtIndex">ChunkExpandAtIndex</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#method.new_from_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExpandAtIndex.html#tymethod.new_from_index" class="fn">new_from_index</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

Create a new ChunkedArray filled with values at that index.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#impl-ChunkShift%3CStructType%3E-for-ChunkedArray%3CStructType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShift.html" class="trait" title="trait polars::prelude::ChunkShift">ChunkShift</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#method.shift" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkShift.html#tymethod.shift" class="fn">shift</a>(&self, periods: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#impl-ChunkSort%3CStructType%3E-for-ChunkedArray%3CStructType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkSort.html" class="trait" title="trait polars::prelude::ChunkSort">ChunkSort</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#method.sort_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkSort.html#tymethod.sort_with" class="fn">sort_with</a>(&self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#method.sort" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkSort.html#tymethod.sort" class="fn">sort</a>(&self, descending: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

Returned a sorted `ChunkedArray`.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#method.arg_sort" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkSort.html#tymethod.arg_sort" class="fn">arg_sort</a>(&self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

Retrieve the indexes needed to sort this array.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#method.arg_sort_multiple" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkSort.html#method.arg_sort_multiple" class="fn">arg_sort_multiple</a>( &self, by: &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\], \_options: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Retrieve the indexes need to sort this and the other arrays.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#impl-ChunkZip%3CStructType%3E-for-ChunkedArray%3CStructType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkZip.html" class="trait" title="trait polars::prelude::ChunkZip">ChunkZip</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#method.zip_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkZip.html#tymethod.zip_with" class="fn">zip_with</a>( &self, mask: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create a new ChunkedArray with values from self where the mask evaluates `true` and values from `other` where the mask evaluates `false`

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#impl-PolarsDataType-for-StructType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#associatedtype.Physical" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#associatedtype.OwnedPhysical" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#associatedtype.ZeroablePhysical" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#associatedtype.Array" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#associatedtype.IsNested" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TrueT.html" class="struct" title="struct polars::prelude::TrueT">TrueT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#associatedtype.HasViews" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#associatedtype.IsStruct" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TrueT.html" class="struct" title="struct polars::prelude::TrueT">TrueT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#associatedtype.IsObject" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#method.get_static_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#tymethod.get_static_dtype" class="fn">get_static_dtype</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

where <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Returns the DataType variant associated with this PolarsDataType. Not implemented for types whose DataTypes have parameters.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#impl-PolarsPhysicalType-for-StructType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#method.ca_into_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html#tymethod.ca_into_series" class="fn">ca_into_series</a>(ca: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.StructType.html#blanket-implementations" class="anchor">§</a>
