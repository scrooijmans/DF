# Trait ChunkCompareIneq Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#342" class="src">Source</a>

``` rust
pub trait ChunkCompareIneq<Rhs> {
    type Item;

    // Required methods
    fn gt(&self, rhs: Rhs) -> Self::Item;
    fn gt_eq(&self, rhs: Rhs) -> Self::Item;
    fn lt(&self, rhs: Rhs) -> Self::Item;
    fn lt_eq(&self, rhs: Rhs) -> Self::Item;
}
```

Expand description

Compare [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray")’s using inequality operators (`<`, `>=`, etc.) and get a `boolean` mask that can be used to filter rows.

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a>

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt" class="fn">gt</a>(&self, rhs: Rhs) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than comparison.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt_eq" class="fn">gt_eq</a>(&self, rhs: Rhs) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than or equal comparison.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt" class="fn">lt</a>(&self, rhs: Rhs) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than comparison.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt_eq" class="fn">lt_eq</a>(&self, rhs: Rhs) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than or equal comparison

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26Column%3E-for-Column" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26str%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26str%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26ChunkedArray%3CBinaryOffsetType%3E%3E-for-ChunkedArray%3CBinaryOffsetType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26ChunkedArray%3CBinaryType%3E%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26ChunkedArray%3CBooleanType%3E%3E-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26ChunkedArray%3CStringType%3E%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26NullChunked%3E-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26Series%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26%5Bu8%5D%3E-for-ChunkedArray%3CBinaryOffsetType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26%5Bu8%5D%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3CRhs%3E-for-Series" class="anchor">§</a>

### impl\<Rhs\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<Rhs\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where Rhs: <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-12" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26str%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-13" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26ChunkedArray%3CStringType%3E%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-14" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26ChunkedArray%3CT%3E%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalOrdKernel.html" class="trait" title="trait polars_compute::comparisons::TotalOrdKernel">TotalOrdKernel</a>\<Scalar = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> + <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html" class="trait" title="trait polars_compute::comparisons::TotalEqKernel">TotalEqKernel</a>\<Scalar = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>, T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-15" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3C%26Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>: for\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-16" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#impl-ChunkCompareIneq%3CRhs%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T, Rhs\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<Rhs\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>, Rhs: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html" class="trait" title="trait num_traits::cast::ToPrimitive">ToPrimitive</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalOrdKernel.html" class="trait" title="trait polars_compute::comparisons::TotalOrdKernel">TotalOrdKernel</a>\<Scalar = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> + <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html" class="trait" title="trait polars_compute::comparisons::TotalEqKernel">TotalEqKernel</a>\<Scalar = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item-17" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>
