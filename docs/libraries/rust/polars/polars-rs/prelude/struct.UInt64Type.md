# Struct UInt64Type Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/mod.rs.html#212" class="src">Source</a>

``` rust
pub struct UInt64Type {}
```

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#impl-Clone-for-UInt64Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#impl-NamedFrom%3CRange%3Cu64%3E,+UInt64Type%3E-for-ChunkedArray%3CUInt64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#impl-NamedFrom%3CRange%3Cu64%3E,+UInt64Type%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#method.new-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#impl-PolarsDataType-for-UInt64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#associatedtype.Physical" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Physical" class="associatedtype">Physical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#associatedtype.OwnedPhysical" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.OwnedPhysical" class="associatedtype">OwnedPhysical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#associatedtype.ZeroablePhysical" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.ZeroablePhysical" class="associatedtype">ZeroablePhysical</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#associatedtype.Array" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype">Array</a> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#associatedtype.IsNested" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.IsNested" class="associatedtype">IsNested</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#associatedtype.HasViews" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.HasViews" class="associatedtype">HasViews</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#associatedtype.IsStruct" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.IsStruct" class="associatedtype">IsStruct</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#associatedtype.IsObject" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.IsObject" class="associatedtype">IsObject</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#method.get_static_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#tymethod.get_static_dtype" class="fn">get_static_dtype</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Returns the DataType variant associated with this PolarsDataType. Not implemented for types whose DataTypes have parameters.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#impl-PolarsNumericType-for-UInt64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#impl-PolarsPhysicalType-for-UInt64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#method.ca_into_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html#tymethod.ca_into_series" class="fn">ca_into_series</a>(ca: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#impl-Copy-for-UInt64Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#impl-PolarsIntegerType-for-UInt64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIntegerType.html" class="trait" title="trait polars::prelude::PolarsIntegerType">PolarsIntegerType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html#blanket-implementations" class="anchor">§</a>
