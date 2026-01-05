# Struct Date64Type Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#192-199" class="src">Source</a>

``` rust
pub struct Date64Type {}
```

Expand description

64-bit date type: the elapsed time since UNIX epoch in milliseconds (64 bits). Values must be divisible by `86_400_000`. See [`DataType::Date64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Date64 "variant arrow::datatypes::DataType::Date64") for more details.

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#impl-Date64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.to_naive_date" class="fn">to_naive_date</a>(i: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>

👎Deprecated since 56.0.0: Use to_naive_date_opt instead.

Converts an arrow Date64Type into a chrono::NaiveDate

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments" class="doc-anchor">§</a>Arguments

- `i` - The Date64Type to convert

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.to_naive_date_opt" class="fn">to_naive_date_opt</a>( i: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>

Converts an arrow Date64Type into a chrono::NaiveDateTime if it fits in the range that chrono::NaiveDateTime can represent. Returns `None` if the calculation would overflow or underflow.

This function is able to handle dates ranging between 1677-09-21 (-9,223,372,800,000) and 2262-04-11 (9,223,286,400,000).

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-1" class="doc-anchor">§</a>Arguments

- `i` - The Date64Type to convert

Returns `Some(NaiveDateTime)` if it fits, `None` otherwise.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.from_naive_date" class="fn">from_naive_date</a>( d: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>, ) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

Converts a chrono::NaiveDate into an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-2" class="doc-anchor">§</a>Arguments

- `d` - The NaiveDate to convert

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.add_year_months" class="fn">add_year_months</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

👎Deprecated since 56.0.0: Use `add_year_months_opt` instead, which returns an Option to handle overflow.

Adds the given IntervalYearMonthType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-3" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to add

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.add_year_months_opt" class="fn">add_year_months_opt</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Adds the given IntervalYearMonthType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-4" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to add

Returns `Some(Date64Type)` if it fits, `None` otherwise.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.add_day_time" class="fn">add_day_time</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

👎Deprecated since 56.0.0: Use `add_day_time_opt` instead, which returns an Option to handle overflow.

Adds the given IntervalDayTimeType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-5" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to add

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.add_day_time_opt" class="fn">add_day_time_opt</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Adds the given IntervalDayTimeType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-6" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to add

Returns `Some(Date64Type)` if it fits, `None` otherwise.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.add_month_day_nano" class="fn">add_month_day_nano</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

👎Deprecated since 56.0.0: Use `add_month_day_nano_opt` instead, which returns an Option to handle overflow.

Adds the given IntervalMonthDayNanoType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-7" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to add

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.add_month_day_nano_opt" class="fn">add_month_day_nano_opt</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Adds the given IntervalMonthDayNanoType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-8" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to add

Returns `Some(Date64Type)` if it fits, `None` otherwise.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.subtract_year_months" class="fn">subtract_year_months</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

👎Deprecated since 56.0.0: Use `subtract_year_months_opt` instead, which returns an Option to handle overflow.

Subtract the given IntervalYearMonthType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-9" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to subtract

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.subtract_year_months_opt" class="fn">subtract_year_months_opt</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Subtract the given IntervalYearMonthType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-10" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to subtract

Returns `Some(Date64Type)` if it fits, `None` otherwise.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.subtract_day_time" class="fn">subtract_day_time</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

👎Deprecated since 56.0.0: Use `subtract_day_time_opt` instead, which returns an Option to handle overflow.

Subtract the given IntervalDayTimeType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-11" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to subtract

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.subtract_day_time_opt" class="fn">subtract_day_time_opt</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Subtract the given IntervalDayTimeType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-12" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to subtract

Returns `Some(Date64Type)` if it fits, `None` otherwise.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.subtract_month_day_nano" class="fn">subtract_month_day_nano</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

👎Deprecated since 56.0.0: Use `subtract_month_day_nano_opt` instead, which returns an Option to handle overflow.

Subtract the given IntervalMonthDayNanoType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-13" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to subtract

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.subtract_month_day_nano_opt" class="fn">subtract_month_day_nano_opt</a>( date: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, delta: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Subtract the given IntervalMonthDayNanoType to an arrow Date64Type

##### <a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#arguments-14" class="doc-anchor">§</a>Arguments

- `date` - The date on which to perform the operation
- `delta` - The interval to subtract

Returns `Some(Date64Type)` if it fits, `None` otherwise.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#impl-ArrowPrimitiveType-for-Date64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#associatedconstant.DATA_TYPE" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a> = DataType::Date64

the corresponding Arrow data type of this primitive type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Corresponding Rust native type for the primitive type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.default_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value" class="fn">default_value</a>() -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

Returns a default value of this primitive type. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#impl-Debug-for-Date64Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#impl-Parser-for-Date64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.parse" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#tymethod.parse" class="fn">parse</a>(string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Parse a string to the native type

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.parse_formatted" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#method.parse_formatted" class="fn">parse_formatted</a>(string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Parse a string to the native type with a format string [Read more](https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#method.parse_formatted)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#impl-RandomTemporalValue-for-Date64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>

Available on **crate feature `test_utils`** only.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.value_range" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#tymethod.value_range" class="fn">value_range</a>() -\> impl <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleRange.html" class="trait" title="trait rand::distr::uniform::SampleRange">SampleRange</a>\<Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Range of values representing the elapsed time since UNIX epoch in milliseconds. The range begins at the start of the unix epoch and continues for 100 years.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.gen_range" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#method.gen_range" class="fn">gen_range</a>\<R: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html" class="trait" title="trait rand::rng::Rng">Rng</a>\>(rng: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut R</a>) -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

where Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html" class="trait" title="trait rand::distr::uniform::SampleUniform">SampleUniform</a>,

Generate a random value within the range of the type

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#method.random" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#method.random" class="fn">random</a>\<R: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html" class="trait" title="trait rand::rng::Rng">Rng</a>\>(rng: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut R</a>) -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

where Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html" class="trait" title="trait rand::distr::uniform::SampleUniform">SampleUniform</a>,

Generate a random value of the type

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#impl-ArrowTemporalType-for-Date64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html#blanket-implementations" class="anchor">§</a>
