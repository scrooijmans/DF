# Enum DatePart Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/temporal.rs.html#44" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum DatePart {
Show 16 variants    Quarter,
    Year,
    YearISO,
    Month,
    Week,
    WeekISO,
    Day,
    DayOfWeekSunday0,
    DayOfWeekMonday0,
    DayOfYear,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}
```

Expand description

Valid parts to extract from date/time/timestamp arrays.

See [`date_part`](https://docs.rs/arrow/latest/arrow/compute/fn.date_part.html "fn arrow::compute::date_part").

Marked as non-exhaustive as may expand to support more types of date parts in the future.

## Variants (Non-exhaustive)<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.Quarter" class="anchor">§</a>

### Quarter

Quarter of the year, in range `1..=4`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.Year" class="anchor">§</a>

### Year

Calendar year

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.YearISO" class="anchor">§</a>

### YearISO

ISO year, computed as per ISO 8601

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.Month" class="anchor">§</a>

### Month

Month in the year, in range `1..=12`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.Week" class="anchor">§</a>

### Week

week of the year, in range `1..=53`, computed as per ISO 8601

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.WeekISO" class="anchor">§</a>

### WeekISO

ISO week of the year, in range `1..=53`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.Day" class="anchor">§</a>

### Day

Day of the month, in range `1..=31`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.DayOfWeekSunday0" class="anchor">§</a>

### DayOfWeekSunday0

Day of the week, in range `0..=6`, where Sunday is `0`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.DayOfWeekMonday0" class="anchor">§</a>

### DayOfWeekMonday0

Day of the week, in range `0..=6`, where Monday is `0`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.DayOfYear" class="anchor">§</a>

### DayOfYear

Day of year, in range `1..=366`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.Hour" class="anchor">§</a>

### Hour

Hour of the day, in range `0..=23`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.Minute" class="anchor">§</a>

### Minute

Minute of the hour, in range `0..=59`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.Second" class="anchor">§</a>

### Second

Second of the minute, in range `0..=59`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.Millisecond" class="anchor">§</a>

### Millisecond

Millisecond of the second

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.Microsecond" class="anchor">§</a>

### Microsecond

Microsecond of the second

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#variant.Nanosecond" class="anchor">§</a>

### Nanosecond

Nanosecond of the second

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#impl-Clone-for-DatePart" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/enum.DatePart.html" class="enum" title="enum arrow::compute::DatePart">DatePart</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/enum.DatePart.html" class="enum" title="enum arrow::compute::DatePart">DatePart</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#impl-Debug-for-DatePart" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/enum.DatePart.html" class="enum" title="enum arrow::compute::DatePart">DatePart</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#impl-Display-for-DatePart" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/enum.DatePart.html" class="enum" title="enum arrow::compute::DatePart">DatePart</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#impl-PartialEq-for-DatePart" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/enum.DatePart.html" class="enum" title="enum arrow::compute::DatePart">DatePart</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/compute/enum.DatePart.html" class="enum" title="enum arrow::compute::DatePart">DatePart</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#impl-Copy-for-DatePart" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/enum.DatePart.html" class="enum" title="enum arrow::compute::DatePart">DatePart</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#impl-Eq-for-DatePart" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/enum.DatePart.html" class="enum" title="enum arrow::compute::DatePart">DatePart</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#impl-StructuralPartialEq-for-DatePart" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/enum.DatePart.html" class="enum" title="enum arrow::compute::DatePart">DatePart</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html#blanket-implementations" class="anchor">§</a>
