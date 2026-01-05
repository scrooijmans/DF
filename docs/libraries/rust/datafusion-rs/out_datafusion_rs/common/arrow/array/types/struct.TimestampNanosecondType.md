# Struct TimestampNanosecondType Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#180-185" class="src">Source</a>

``` rust
pub struct TimestampNanosecondType {}
```

Expand description

Timestamp nanosecond type with an optional timezone.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#impl-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.add_year_months" class="fn">add_year_months</a>( timestamp: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/timezone/struct.Tz.html" class="struct" title="struct datafusion::common::arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Adds the given IntervalYearMonthType to an arrow TimestampNanosecondType

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#arguments" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.add_day_time" class="fn">add_day_time</a>( timestamp: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/timezone/struct.Tz.html" class="struct" title="struct datafusion::common::arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Adds the given IntervalDayTimeType to an arrow TimestampNanosecondType

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#arguments-1" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.add_month_day_nano" class="fn">add_month_day_nano</a>( timestamp: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/timezone/struct.Tz.html" class="struct" title="struct datafusion::common::arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Adds the given IntervalMonthDayNanoType to an arrow TimestampNanosecondType

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#arguments-2" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.subtract_year_months" class="fn">subtract_year_months</a>( timestamp: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/timezone/struct.Tz.html" class="struct" title="struct datafusion::common::arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Subtracts the given IntervalYearMonthType to an arrow TimestampNanosecondType

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#arguments-3" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.subtract_day_time" class="fn">subtract_day_time</a>( timestamp: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/timezone/struct.Tz.html" class="struct" title="struct datafusion::common::arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Subtracts the given IntervalDayTimeType to an arrow TimestampNanosecondType

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#arguments-4" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.subtract_month_day_nano" class="fn">subtract_month_day_nano</a>( timestamp: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, tz: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/timezone/struct.Tz.html" class="struct" title="struct datafusion::common::arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Subtracts the given IntervalMonthDayNanoType to an arrow TimestampNanosecondType

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#arguments-5" class="doc-anchor">§</a>Arguments

- `timestamp` - The date on which to perform the operation
- `delta` - The interval to add
- `tz` - The timezone in which to interpret `timestamp`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#impl-ArrowPrimitiveType-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#associatedconstant.DATA_TYPE" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

the corresponding Arrow data type of this primitive type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Corresponding Rust native type for the primitive type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.default_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#method.default_value" class="fn">default_value</a>() -\> Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>

Returns a default value of this primitive type. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#method.default_value)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#impl-ArrowTimestampType-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowTimestampType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ArrowTimestampType">ArrowTimestampType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#associatedconstant.UNIT" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowTimestampType.html#associatedconstant.UNIT" class="constant">UNIT</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum datafusion::common::arrow::datatypes::TimeUnit">TimeUnit</a> = TimeUnit::Nanosecond

The [`TimeUnit`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.TimeUnit.html "enum datafusion::common::arrow::datatypes::TimeUnit") of this timestamp.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.make_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowTimestampType.html#tymethod.make_value" class="fn">make_value</a>(naive: <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Creates a ArrowTimestampType::Native from the provided [`NaiveDateTime`](https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime") [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowTimestampType.html#tymethod.make_value)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#impl-Debug-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#impl-Parser-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.parse" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#tymethod.parse" class="fn">parse</a>(string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Parse a string to the native type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.parse_formatted" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#method.parse_formatted" class="fn">parse_formatted</a>(string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Parse a string to the native type with a format string [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#method.parse_formatted)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#impl-ScalarType%3Ci64%3E-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/trait.ScalarType.html" class="trait" title="trait datafusion::scalar::ScalarType">ScalarType</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#method.scalar" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/trait.ScalarType.html#tymethod.scalar" class="fn">scalar</a>(r: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

returns a scalar from an optional T

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#impl-ArrowTemporalType-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/types/struct.TimestampNanosecondType.html#blanket-implementations" class="anchor">§</a>
