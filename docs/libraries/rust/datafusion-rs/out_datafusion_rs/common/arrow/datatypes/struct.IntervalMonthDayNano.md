# Struct IntervalMonthDayNano Copy item path

<a href="https://docs.rs/arrow-buffer/56.0.0/x86_64-unknown-linux-gnu/src/arrow_buffer/interval.rs.html#70" class="src">Source</a>

``` rust
#[repr(C)]pub struct IntervalMonthDayNano {
    pub months: i32,
    pub days: i32,
    pub nanoseconds: i64,
}
```

Expand description

Value of an IntervalMonthDayNano array

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#representation" class="doc-anchor">§</a>Representation

This type is stored as a single 128 bit integer, interpreted as three different signed integral fields:

1.  The number of months (32 bits)
2.  The number days (32 bits)
3.  The number of nanoseconds (64 bits).

Nanoseconds does not allow for leap seconds.

Each field is independent (e.g. there is no constraint that the quantity of nanoseconds represents less than a day’s worth of time).

``` text
┌───────────────┬─────────────┬─────────────────────────────┐
│     Months    │     Days    │            Nanos            │
│   (32 bits)   │  (32 bits)  │          (64 bits)          │
└───────────────┴─────────────┴─────────────────────────────┘
0            32             64                           128 bit offset
```

Please see the [Arrow Spec](https://github.com/apache/arrow/blob/081b4022fe6f659d8765efc82b3f4787c5039e3c/format/Schema.fbs#L409-L415) for more details

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#note-on-comparing-and-ordering-for-calendar-types" class="doc-anchor">§</a>Note on Comparing and Ordering for Calendar Types

Values of `IntervalMonthDayNano` are compared using their binary representation, which can lead to surprising results.

Spans of time measured in calendar units are not fixed in absolute size (e.g. number of seconds) which makes defining comparisons and ordering non trivial. For example `1 month` is 28 days for February but `1 month` is 31 days in December.

This makes the seemingly simple operation of comparing two intervals complicated in practice. For example is `1 month` more or less than `30 days`? The answer depends on what month you are talking about.

This crate defines comparisons for calendar types using their binary representation which is fast and efficient, but leads to potentially surprising results.

For example a `IntervalMonthDayNano` of `1 month` will compare as **greater** than a `IntervalMonthDayNano` of `100 days` because the binary representation of `1 month` is larger than the binary representation of 100 days.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#structfield.months" class="anchor field">§</a>`months: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

Number of months

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#structfield.days" class="anchor field">§</a>`days: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

Number of days

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#structfield.nanoseconds" class="anchor field">§</a>`nanoseconds: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

Number of nanoseconds

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The additive identity i.e. `0`.

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The multiplicative identity, i.e. `1`.

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedconstant.MINUS_ONE" class="constant">MINUS_ONE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The multiplicative inverse, i.e. `-1`.

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedconstant.MAX" class="constant">MAX</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The maximum value that can be represented

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedconstant.MIN" class="constant">MIN</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The minimum value that can be represented

#### pub const fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.new" class="fn">new</a>( months: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, days: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, nanoseconds: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Create a new [`IntervalMonthDayNano`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html "struct datafusion::common::arrow::datatypes::IntervalMonthDayNano")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.wrapping_abs" class="fn">wrapping_abs</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Computes the absolute value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.checked_abs" class="fn">checked_abs</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>

Computes the absolute value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.wrapping_neg" class="fn">wrapping_neg</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Negates the value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.checked_neg" class="fn">checked_neg</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>

Negates the value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.wrapping_add" class="fn">wrapping_add</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Performs wrapping addition

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.checked_add" class="fn">checked_add</a>( self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>

Performs checked addition

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.wrapping_sub" class="fn">wrapping_sub</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Performs wrapping subtraction

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.checked_sub" class="fn">checked_sub</a>( self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>

Performs checked subtraction

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.wrapping_mul" class="fn">wrapping_mul</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Performs wrapping multiplication

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.checked_mul" class="fn">checked_mul</a>( self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>

Performs checked multiplication

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.wrapping_div" class="fn">wrapping_div</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Performs wrapping division

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.checked_div" class="fn">checked_div</a>( self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>

Performs checked division

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.wrapping_rem" class="fn">wrapping_rem</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Performs wrapping remainder

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.checked_rem" class="fn">checked_rem</a>( self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>

Performs checked remainder

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.wrapping_pow" class="fn">wrapping_pow</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Performs wrapping exponentiation

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.checked_pow" class="fn">checked_pow</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>

Performs checked exponentiation

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Add%3C%26IntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html" class="trait" title="trait datafusion::prelude::Add">Add</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.add-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#tymethod.add" class="fn">add</a>( self, rhs: &'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html" class="trait" title="trait datafusion::prelude::Add">Add</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#tymethod.add)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Add%3C%26IntervalMonthDayNano%3E-for-IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html" class="trait" title="trait datafusion::prelude::Add">Add</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.add-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#tymethod.add" class="fn">add</a>( self, rhs: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html" class="trait" title="trait datafusion::prelude::Add">Add</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#tymethod.add)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Add%3CIntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html" class="trait" title="trait datafusion::prelude::Add">Add</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.add-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#tymethod.add" class="fn">add</a>( self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html" class="trait" title="trait datafusion::prelude::Add">Add</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#tymethod.add)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Add-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html" class="trait" title="trait datafusion::prelude::Add">Add</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.add" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html" class="trait" title="trait datafusion::prelude::Add">Add</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#tymethod.add)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-AddAssign-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html" class="trait" title="trait core::ops::arith::AddAssign">AddAssign</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.add_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign" class="fn">add_assign</a>(&mut self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>)

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-ArrowNativeType-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.from_usize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(\_: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>

Convert native integer type from usize [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.as_usize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert to usize according to the [`as`](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast) operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.usize_as" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Convert from usize according to the [`as`](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast) operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.to_usize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Convert native type to usize. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.to_isize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

Convert native type to isize. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.to_i64" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Convert native type to i64. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.get_byte_width" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html#method.get_byte_width" class="fn">get_byte_width</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the byte width of this native type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-ArrowNativeTypeOp-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait datafusion::common::arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedconstant.ZERO-1" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> = IntervalMonthDayNano::ZERO

The additive identity

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedconstant.ONE-1" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> = IntervalMonthDayNano::ONE

The multiplicative identity

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedconstant.MIN_TOTAL_ORDER" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> = IntervalMonthDayNano::MIN

The minimum value and identity for the `max` aggregation. Note that the aggregation uses the total order predicate for floating point values, which means that this value is a negative NaN.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedconstant.MAX_TOTAL_ORDER" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> = IntervalMonthDayNano::MAX

The maximum value and identity for the `min` aggregation. Note that the aggregation uses the total order predicate for floating point values, which means that this value is a positive NaN.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.add_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>( self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Checked addition operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.add_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Wrapping addition operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.sub_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>( self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Checked subtraction operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.sub_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Wrapping subtraction operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.mul_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>( self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Checked multiplication operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.mul_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Wrapping multiplication operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.div_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>( self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Checked division operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.div_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Wrapping division operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.mod_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>( self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Checked remainder operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.mod_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Wrapping remainder operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.neg_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Checked negation operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.pow_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Checked exponentiation operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.pow_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Wrapping exponentiation operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.neg_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Wrapping negation operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.is_zero" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if zero else false

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.compare" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

Compare operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.is_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Equality operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.is_ne" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#method.is_ne" class="fn">is_ne</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Not equal operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.is_lt" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#method.is_lt" class="fn">is_lt</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Less than operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.is_le" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#method.is_le" class="fn">is_le</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Less than equals operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.is_gt" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#method.is_gt" class="fn">is_gt</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Greater than operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.is_ge" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html#method.is_ge" class="fn">is_ge</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Greater than equals operation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Clone-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Debug-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Default-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Div%3C%26IntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html" class="trait" title="trait datafusion::prelude::Div">Div</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-15" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.div-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#tymethod.div" class="fn">div</a>( self, rhs: &'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html" class="trait" title="trait datafusion::prelude::Div">Div</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#tymethod.div)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Div%3C%26IntervalMonthDayNano%3E-for-IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html" class="trait" title="trait datafusion::prelude::Div">Div</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-14" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.div-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#tymethod.div" class="fn">div</a>( self, rhs: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html" class="trait" title="trait datafusion::prelude::Div">Div</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#tymethod.div)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Div%3CIntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html" class="trait" title="trait datafusion::prelude::Div">Div</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-13" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.div-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#tymethod.div" class="fn">div</a>( self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html" class="trait" title="trait datafusion::prelude::Div">Div</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#tymethod.div)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Div-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html" class="trait" title="trait datafusion::prelude::Div">Div</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-12" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.div" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#tymethod.div" class="fn">div</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html" class="trait" title="trait datafusion::prelude::Div">Div</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#tymethod.div)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-DivAssign-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html" class="trait" title="trait core::ops::arith::DivAssign">DivAssign</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.div_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign" class="fn">div_assign</a>(&mut self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>)

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Hash-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-HashValue-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/hash_utils/trait.HashValue.html" class="trait" title="trait datafusion::common::hash_utils::HashValue">HashValue</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.hash_one" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/hash_utils/trait.HashValue.html#tymethod.hash_one" class="fn">hash_one</a>(&self, state: &<a href="https://docs.rs/ahash/0.8.12/x86_64-unknown-linux-gnu/ahash/random_state/struct.RandomState.html" class="struct" title="struct ahash::random_state::RandomState">RandomState</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Mul%3C%26IntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html" class="trait" title="trait datafusion::prelude::Mul">Mul</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-11" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.mul-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#tymethod.mul" class="fn">mul</a>( self, rhs: &'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html" class="trait" title="trait datafusion::prelude::Mul">Mul</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Mul%3C%26IntervalMonthDayNano%3E-for-IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html" class="trait" title="trait datafusion::prelude::Mul">Mul</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-10" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.mul-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#tymethod.mul" class="fn">mul</a>( self, rhs: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html" class="trait" title="trait datafusion::prelude::Mul">Mul</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Mul%3CIntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html" class="trait" title="trait datafusion::prelude::Mul">Mul</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-9" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.mul-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#tymethod.mul" class="fn">mul</a>( self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html" class="trait" title="trait datafusion::prelude::Mul">Mul</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Mul-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html" class="trait" title="trait datafusion::prelude::Mul">Mul</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-8" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.mul" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#tymethod.mul" class="fn">mul</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html" class="trait" title="trait datafusion::prelude::Mul">Mul</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-MulAssign-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html" class="trait" title="trait core::ops::arith::MulAssign">MulAssign</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.mul_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign" class="fn">mul_assign</a>(&mut self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>)

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Neg-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-20" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.neg" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#tymethod.neg" class="fn">neg</a>(self) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

Performs the unary `-` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#tymethod.neg)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Ord-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-PartialEq-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-PartialOrd-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Rem%3C%26IntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-19" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.rem-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#tymethod.rem" class="fn">rem</a>( self, rhs: &'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

Performs the `%` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Rem%3C%26IntervalMonthDayNano%3E-for-IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-18" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.rem-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#tymethod.rem" class="fn">rem</a>( self, rhs: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

Performs the `%` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Rem%3CIntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-17" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.rem-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#tymethod.rem" class="fn">rem</a>( self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

Performs the `%` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Rem-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-16" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.rem" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#tymethod.rem" class="fn">rem</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

Performs the `%` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-RemAssign-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html" class="trait" title="trait core::ops::arith::RemAssign">RemAssign</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.rem_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign" class="fn">rem_assign</a>(&mut self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>)

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Sub%3C%26IntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-7" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.sub-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#tymethod.sub" class="fn">sub</a>( self, rhs: &'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Sub%3C%26IntervalMonthDayNano%3E-for-IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-6" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.sub-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#tymethod.sub" class="fn">sub</a>( self, rhs: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Sub%3CIntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-5" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.sub-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#tymethod.sub" class="fn">sub</a>( self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Sub-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.sub" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-SubAssign-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html" class="trait" title="trait core::ops::arith::SubAssign">SubAssign</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#method.sub_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign" class="fn">sub_assign</a>(&mut self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>)

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Copy-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-Eq-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#impl-StructuralPartialEq-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html#blanket-implementations" class="anchor">§</a>
