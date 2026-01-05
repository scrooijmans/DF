# Struct StatisticsFlagsIM Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/flags.rs.html#7" class="src">Source</a>

``` rust
pub struct StatisticsFlagsIM { /* private fields */ }
```

Expand description

An interior mutable version of [`StatisticsFlags`](https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlags.html "struct polars::chunked_array::flags::StatisticsFlags")

## Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#impl-StatisticsFlagsIM" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlagsIM">StatisticsFlagsIM</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.new" class="fn">new</a>(value: <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlags.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlags">StatisticsFlags</a>) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlagsIM">StatisticsFlagsIM</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.empty" class="fn">empty</a>() -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlagsIM">StatisticsFlagsIM</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.get_mut" class="fn">get_mut</a>(&mut self) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlags.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlags">StatisticsFlags</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.set_mut" class="fn">set_mut</a>(&mut self, value: <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlags.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlags">StatisticsFlags</a>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.get" class="fn">get</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlags.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlags">StatisticsFlags</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.set" class="fn">set</a>(&self, value: <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlags.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlags">StatisticsFlags</a>)

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#impl-Clone-for-StatisticsFlagsIM" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlagsIM">StatisticsFlagsIM</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlagsIM">StatisticsFlagsIM</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#impl-Debug-for-StatisticsFlagsIM" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlagsIM">StatisticsFlagsIM</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#impl-From%3CStatisticsFlags%3E-for-StatisticsFlagsIM" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlags.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlags">StatisticsFlags</a>\> for <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlagsIM">StatisticsFlagsIM</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlags.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlags">StatisticsFlags</a>) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlagsIM">StatisticsFlagsIM</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#impl-PartialEq-for-StatisticsFlagsIM" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlagsIM">StatisticsFlagsIM</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlagsIM">StatisticsFlagsIM</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#impl-Eq-for-StatisticsFlagsIM" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlagsIM">StatisticsFlagsIM</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlagsIM.html#blanket-implementations" class="anchor">§</a>
