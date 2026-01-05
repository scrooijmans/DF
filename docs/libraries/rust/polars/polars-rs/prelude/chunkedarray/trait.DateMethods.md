# Trait DateMethods Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/date.rs.html#16" class="src">Source</a>

``` rust
pub trait DateMethods: AsDate {
    // Required method
    fn parse_from_str_slice(
        name: PlSmallStr,
        v: &[&str],
        fmt: &str,
    ) -> Logical<DateType, Int32Type>;

    // Provided methods
    fn year(&self) -> ChunkedArray<Int32Type> { ... }
    fn is_leap_year(&self) -> ChunkedArray<BooleanType> { ... }
    fn iso_year(&self) -> ChunkedArray<Int32Type> { ... }
    fn quarter(&self) -> ChunkedArray<Int8Type> { ... }
    fn month(&self) -> ChunkedArray<Int8Type> { ... }
    fn days_in_month(&self) -> ChunkedArray<Int8Type> { ... }
    fn week(&self) -> ChunkedArray<Int8Type> { ... }
    fn day(&self) -> ChunkedArray<Int8Type> { ... }
    fn ordinal(&self) -> ChunkedArray<Int16Type> { ... }
    fn new_from_parts(
        year: &ChunkedArray<Int32Type>,
        month: &ChunkedArray<Int8Type>,
        day: &ChunkedArray<Int8Type>,
        name: PlSmallStr,
    ) -> Result<Logical<DateType, Int32Type>, PolarsError> { ... }
}
```

Available on **crate feature `temporal`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#tymethod.parse_from_str_slice" class="fn">parse_from_str_slice</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\], fmt: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#method.year" class="fn">year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

Extract month from underlying NaiveDate representation. Returns the year number in the calendar date.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#method.is_leap_year" class="fn">is_leap_year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

Extract year from underlying NaiveDate representation. Returns whether the year is a leap year.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#method.iso_year" class="fn">iso_year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

This year number might not match the calendar year number.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#method.quarter" class="fn">quarter</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract month from underlying NaiveDateTime representation. Quarters range from 1 to 4.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#method.month" class="fn">month</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract month from underlying NaiveDateTime representation. Returns the month number starting from 1.

The return value ranges from 1 to 12.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#method.days_in_month" class="fn">days_in_month</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Returns the number of days in the month of the underlying NaiveDate representation.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#method.week" class="fn">week</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Returns the ISO week number starting from 1. The return value ranges from 1 to 53. (The last week of year differs by years.)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#method.day" class="fn">day</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract day from underlying NaiveDate representation. Returns the day of month starting from 1.

The return value ranges from 1 to 31. (The last day of month differs by months.)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#method.ordinal" class="fn">ordinal</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>

Returns the day of year starting from 1.

The return value ranges from 1 to 366. (The last day of year differs by years.)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#method.new_from_parts" class="fn">new_from_parts</a>( year: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, month: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, day: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Construct a date ChunkedArray from individual time components.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/trait.DateMethods.html#impl-DateMethods-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html" class="trait" title="trait polars::prelude::DateMethods">DateMethods</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>
