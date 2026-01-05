# Struct IntervalDayTimeType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#230-235" class="src">Source</a>

``` rust
pub struct IntervalDayTimeType {}
```

Expand description

“Calendar” interval type: days and milliseconds. See [`IntervalDayTime`](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html "struct arrow::datatypes::IntervalDayTime") for more details.

## Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#impl-IntervalDayTimeType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#method.make_value" class="fn">make_value</a>(days: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, milliseconds: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

Creates a IntervalDayTimeType::Native

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#arguments" class="doc-anchor">§</a>Arguments

- `days` - The number of days (+/-) represented in this interval
- `millis` - The number of milliseconds (+/-) represented in this interval

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#method.to_parts" class="fn">to_parts</a>(i: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct arrow::datatypes::IntervalDayTime">IntervalDayTime</a>) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>)

Turns a IntervalDayTimeType into a tuple of (days, milliseconds)

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#arguments-1" class="doc-anchor">§</a>Arguments

- `i` - The IntervalDayTimeType to convert

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#impl-ArrowPrimitiveType-for-IntervalDayTimeType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#associatedconstant.DATA_TYPE" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

the corresponding Arrow data type of this primitive type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

Corresponding Rust native type for the primitive type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#method.default_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value" class="fn">default_value</a>() -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

Returns a default value of this primitive type. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#impl-Debug-for-IntervalDayTimeType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html#blanket-implementations" class="anchor">§</a>
