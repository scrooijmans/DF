# Struct Float32Type Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/mod.rs.html#219" class="src">Source</a>

``` rust
pub struct Float32Type {}
```

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#impl-Clone-for-Float32Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#impl-NumOpsDispatchCheckedInner-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html" class="trait" title="trait polars::series::arithmetic::checked::NumOpsDispatchCheckedInner">NumOpsDispatchCheckedInner</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#method.checked_div" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html#method.checked_div" class="fn">checked_div</a>( lhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Checked integer division. Computes self / rhs, returning None if rhs == 0 or the division results in overflow.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#method.checked_div_num" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/series/arithmetic/checked/trait.NumOpsDispatchCheckedInner.html#method.checked_div_num" class="fn">checked_div_num</a>\<T\>( lhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<Self\>, \_rhs: T, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html" class="trait" title="trait num_traits::cast::ToPrimitive">ToPrimitive</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#impl-PolarsDataType-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#associatedtype.Physical" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#associatedtype.OwnedPhysical" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#associatedtype.ZeroablePhysical" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#associatedtype.Array" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#associatedtype.IsNested" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#associatedtype.HasViews" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#associatedtype.IsStruct" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#associatedtype.IsObject" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#method.get_static_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#tymethod.get_static_dtype" class="fn">get_static_dtype</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Returns the DataType variant associated with this PolarsDataType. Not implemented for types whose DataTypes have parameters.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#impl-PolarsNumericType-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#impl-PolarsPhysicalType-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#method.ca_into_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html#tymethod.ca_into_series" class="fn">ca_into_series</a>(ca: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#impl-Copy-for-Float32Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#impl-PolarsFloatType-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsFloatType.html" class="trait" title="trait polars::prelude::PolarsFloatType">PolarsFloatType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Float32Type.html#blanket-implementations" class="anchor">§</a>
