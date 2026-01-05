# Struct IntervalYearMonthType Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#224-229" class="src">Source</a>

``` rust
pub struct IntervalYearMonthType {}
```

Expand description

32-bit “calendar” interval type: the number of whole months.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#impl-IntervalYearMonthType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#method.make_value" class="fn">make_value</a>( years: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, months: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, ) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>

Creates a IntervalYearMonthType::Native

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#arguments" class="doc-anchor">§</a>Arguments

- `years` - The number of years (+/-) represented in this interval
- `months` - The number of months (+/-) represented in this interval

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#method.to_months" class="fn">to_months</a>( i: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Turns a IntervalYearMonthType type into an i32 of months.

This operation is technically a no-op, it is included for comprehensiveness.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#arguments-1" class="doc-anchor">§</a>Arguments

- `i` - The IntervalYearMonthType::Native to convert

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#impl-ArrowPrimitiveType-for-IntervalYearMonthType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#associatedconstant.DATA_TYPE" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

the corresponding Arrow data type of this primitive type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Corresponding Rust native type for the primitive type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#method.default_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#method.default_value" class="fn">default_value</a>() -\> Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>

Returns a default value of this primitive type. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#method.default_value)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#impl-Debug-for-IntervalYearMonthType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.IntervalYearMonthType.html#blanket-implementations" class="anchor">§</a>
