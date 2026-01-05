# Trait NamedFrom Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/named_from.rs.html#15" class="src">Source</a>

``` rust
pub trait NamedFrom<T, Phantom>where
    Phantom: ?Sized,{
    // Required method
    fn new(name: PlSmallStr, _: T) -> Self;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, \_: T) -\> Self

Initialize by name and values.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3C%26Series,+str%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CRange%3Ci32%3E,+Int32Type%3E-for-ChunkedArray%3CInt32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CRange%3Ci32%3E,+Int32Type%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CRange%3Ci64%3E,+Int64Type%3E-for-ChunkedArray%3CInt64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CRange%3Ci64%3E,+Int64Type%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CRange%3Cu32%3E,+UInt32Type%3E-for-ChunkedArray%3CUInt32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CRange%3Cu32%3E,+UInt32Type%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CRange%3Cu64%3E,+UInt64Type%3E-for-ChunkedArray%3CUInt64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CRange%3Cu64%3E,+UInt64Type%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5B%26str%5D%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5B%26str%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5B%26%5Bu8%5D%5D%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5B%26%5Bu8%5D%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BAnyValue%3C&#39;a%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BCow%3C&#39;a,+str%3E%5D%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BCow%3C&#39;a,+str%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BCow%3C&#39;a,+%5Bu8%5D%3E%5D%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BCow%3C&#39;a,+%5Bu8%5D%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3C%26str%3E%5D%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3C%26str%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3C%26%5Bu8%5D%3E%5D%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3C%26%5Bu8%5D%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CCow%3C&#39;a,+str%3E%3E%5D%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CCow%3C&#39;a,+str%3E%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CCow%3C&#39;a,+%5Bu8%5D%3E%3E%5D%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CCow%3C&#39;a,+%5Bu8%5D%3E%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3C%26%5BT%5D,+%26%5BT%5D%3E-for-ChunkedArray%3CObjectType%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+ListType%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+T%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a>,

For any [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") and [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series")

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cbool%3E%5D%3E-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cbool%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cf32%3E%5D%3E-for-ChunkedArray%3CFloat32Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cf32%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cf64%3E%5D%3E-for-ChunkedArray%3CFloat64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cf64%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Ci8%3E%5D%3E-for-ChunkedArray%3CInt8Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Ci8%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Ci16%3E%5D%3E-for-ChunkedArray%3CInt16Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Ci16%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Ci32%3E%5D%3E-for-ChunkedArray%3CInt32Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Ci32%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Ci64%3E%5D%3E-for-ChunkedArray%3CInt64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Ci64%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Ci128%3E%5D%3E-for-ChunkedArray%3CInt128Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Ci128%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cu8%3E%5D%3E-for-ChunkedArray%3CUInt8Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cu8%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cu16%3E%5D%3E-for-ChunkedArray%3CUInt16Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cu16%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cu32%3E%5D%3E-for-ChunkedArray%3CUInt32Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cu32%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cu64%3E%5D%3E-for-ChunkedArray%3CUInt64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3Cu64%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CString%3E%5D%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CString%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CVec%3Cu8%3E%3E%5D%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CVec%3Cu8%3E%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveDate%3E%5D%3E-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveDate%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveDateTime%3E%5D%3E-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveDateTime%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveTime%3E%5D%3E-for-Logical%3CTimeType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveTime%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CTimeDelta%3E%5D%3E-for-Logical%3CDurationType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CTimeDelta%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BOption%3CSeries%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bbool%5D%3E-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bbool%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bf32%5D%3E-for-ChunkedArray%3CFloat32Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bf32%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bf64%5D%3E-for-ChunkedArray%3CFloat64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bf64%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bi8%5D%3E-for-ChunkedArray%3CInt8Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bi8%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bi16%5D%3E-for-ChunkedArray%3CInt16Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bi16%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bi32%5D%3E-for-ChunkedArray%3CInt32Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bi32%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bi64%5D%3E-for-ChunkedArray%3CInt64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bi64%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bi128%5D%3E-for-ChunkedArray%3CInt128Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bi128%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bu8%5D%3E-for-ChunkedArray%3CUInt8Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bu8%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bu16%5D%3E-for-ChunkedArray%3CUInt16Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bu16%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bu32%5D%3E-for-ChunkedArray%3CUInt32Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bu32%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bu64%5D%3E-for-ChunkedArray%3CUInt64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5Bu64%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BString%5D%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BString%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BVec%3Cu8%3E%5D%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BVec%3Cu8%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BNaiveDate%5D%3E-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BNaiveDate%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BNaiveDateTime%5D%3E-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BNaiveDateTime%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BNaiveTime%5D%3E-for-Logical%3CTimeType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BNaiveTime%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BTimeDelta%5D%3E-for-Logical%3CDurationType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CT,+%5BTimeDelta%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#impl-NamedFrom%3CS,+%5BOption%3CT%3E%5D%3E-for-ChunkedArray%3CObjectType%3CT%3E%3E" class="anchor">§</a>

### impl\<T, S\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<S, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\]\>,
