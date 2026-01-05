# Struct TimestampMillisecondType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#168-173" class="src">Source</a>

``` rust
pub struct TimestampMillisecondType {}
```

Expand description

Timestamp millisecond type with an optional timezone.

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#impl-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.add_year_months" class="fn">add_year_months</a>( timestamp: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Adds the given IntervalYearMonthType to an arrow TimestampMillisecondType

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#arguments" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.add_day_time" class="fn">add_day_time</a>( timestamp: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Adds the given IntervalDayTimeType to an arrow TimestampMillisecondType

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#arguments-1" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.add_month_day_nano" class="fn">add_month_day_nano</a>( timestamp: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Adds the given IntervalMonthDayNanoType to an arrow TimestampMillisecondType

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#arguments-2" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.subtract_year_months" class="fn">subtract_year_months</a>( timestamp: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Subtracts the given IntervalYearMonthType to an arrow TimestampMillisecondType

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#arguments-3" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.subtract_day_time" class="fn">subtract_day_time</a>( timestamp: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Subtracts the given IntervalDayTimeType to an arrow TimestampMillisecondType

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#arguments-4" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.subtract_month_day_nano" class="fn">subtract_month_day_nano</a>( timestamp: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Subtracts the given IntervalMonthDayNanoType to an arrow TimestampMillisecondType

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#arguments-5" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#impl-ArrowPrimitiveType-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#associatedconstant.DATA_TYPE" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

the corresponding Arrow data type of this primitive type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Corresponding Rust native type for the primitive type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.default_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value" class="fn">default_value</a>() -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

Returns a default value of this primitive type. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#impl-ArrowTimestampType-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html" class="trait" title="trait arrow::datatypes::ArrowTimestampType">ArrowTimestampType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#associatedconstant.UNIT" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#associatedconstant.UNIT" class="constant">UNIT</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum arrow::datatypes::TimeUnit">TimeUnit</a> = TimeUnit::Millisecond

The [`TimeUnit`](https://docs.rs/arrow/latest/arrow/datatypes/enum.TimeUnit.html "enum arrow::datatypes::TimeUnit") of this timestamp.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.make_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#tymethod.make_value" class="fn">make_value</a>(naive: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Creates a ArrowTimestampType::Native from the provided [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime") [Read more](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#tymethod.make_value)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#impl-Debug-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#impl-Parser-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.parse" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#tymethod.parse" class="fn">parse</a>(string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Parse a string to the native type

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.parse_formatted" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#method.parse_formatted" class="fn">parse_formatted</a>(string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Parse a string to the native type with a format string [Read more](https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#method.parse_formatted)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#impl-RandomTemporalValue-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

Available on **crate feature `test_utils`** only.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.value_range" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#tymethod.value_range" class="fn">value_range</a>() -\> impl <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleRange.html" class="trait" title="trait rand::distr::uniform::SampleRange">SampleRange</a>\<Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Range of values for a timestamp in milliseconds. The range begins at the start of the unix epoch and continues for 100 years.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.gen_range" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#method.gen_range" class="fn">gen_range</a>\<R: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html" class="trait" title="trait rand::rng::Rng">Rng</a>\>(rng: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut R</a>) -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

where Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html" class="trait" title="trait rand::distr::uniform::SampleUniform">SampleUniform</a>,

Generate a random value within the range of the type

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#method.random" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#method.random" class="fn">random</a>\<R: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html" class="trait" title="trait rand::rng::Rng">Rng</a>\>(rng: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut R</a>) -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

where Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html" class="trait" title="trait rand::distr::uniform::SampleUniform">SampleUniform</a>,

Generate a random value of the type

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#impl-ArrowTemporalType-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html#blanket-implementations" class="anchor">§</a>
