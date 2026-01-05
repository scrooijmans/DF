# Trait ChunkCompareEq Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#324" class="src">Source</a>

``` rust
pub trait ChunkCompareEq<Rhs> {
    type Item;

    // Required methods
    fn equal(&self, rhs: Rhs) -> Self::Item;
    fn equal_missing(&self, rhs: Rhs) -> Self::Item;
    fn not_equal(&self, rhs: Rhs) -> Self::Item;
    fn not_equal_missing(&self, rhs: Rhs) -> Self::Item;
}
```

Expand description

Compare [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray")’s and get a `boolean` mask that can be used to filter rows.

## <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#example" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
fn filter_all_ones(df: &DataFrame) -> PolarsResult<DataFrame> {
    let mask = df
    .column("column_a")?
    .as_materialized_series()
    .equal(1)?;

    df.filter(&mask)
}
```

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a>

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal" class="fn">equal</a>(&self, rhs: Rhs) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal_missing" class="fn">equal_missing</a>(&self, rhs: Rhs) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality where `None == None`.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal" class="fn">not_equal</a>(&self, rhs: Rhs) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal_missing" class="fn">not_equal_missing</a>(&self, rhs: Rhs) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality where `None == None`.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26Column%3E-for-Column" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26str%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26str%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26ChunkedArray%3CBinaryOffsetType%3E%3E-for-ChunkedArray%3CBinaryOffsetType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26ChunkedArray%3CBinaryType%3E%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26ChunkedArray%3CBooleanType%3E%3E-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26ChunkedArray%3CFixedSizeListType%3E%3E-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26ChunkedArray%3CListType%3E%3E-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26ChunkedArray%3CStringType%3E%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26ChunkedArray%3CStructType%3E%3E-for-ChunkedArray%3CStructType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26NullChunked%3E-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26Series%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-12" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26%5Bu8%5D%3E-for-ChunkedArray%3CBinaryOffsetType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-13" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26%5Bu8%5D%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-14" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3CRhs%3E-for-Series" class="anchor">§</a>

### impl\<Rhs\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<Rhs\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where Rhs: <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-15" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26str%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>: for\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::Native">Native</a>, Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-16" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26ChunkedArray%3CStringType%3E%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>: for\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::Native">Native</a>, Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-17" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26ChunkedArray%3CT%3E%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalOrdKernel.html" class="trait" title="trait polars_compute::comparisons::TotalOrdKernel">TotalOrdKernel</a>\<Scalar = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> + <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html" class="trait" title="trait polars_compute::comparisons::TotalEqKernel">TotalEqKernel</a>\<Scalar = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>, T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-18" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3C%26Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>: for\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-19" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#impl-ChunkCompareEq%3CRhs%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T, Rhs\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<Rhs\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>, Rhs: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html" class="trait" title="trait num_traits::cast::ToPrimitive">ToPrimitive</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalOrdKernel.html" class="trait" title="trait polars_compute::comparisons::TotalOrdKernel">TotalOrdKernel</a>\<Scalar = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> + <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/comparisons/trait.TotalEqKernel.html" class="trait" title="trait polars_compute::comparisons::TotalEqKernel">TotalEqKernel</a>\<Scalar = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item-20" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>
