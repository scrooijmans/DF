# Struct DatetimeArgs Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/temporal.rs.html#33" class="src">Source</a>

``` rust
pub struct DatetimeArgs {
    pub year: Expr,
    pub month: Expr,
    pub day: Expr,
    pub hour: Expr,
    pub minute: Expr,
    pub second: Expr,
    pub microsecond: Expr,
    pub time_unit: TimeUnit,
    pub time_zone: Option<TimeZone>,
    pub ambiguous: Expr,
}
```

Available on **crate feature `lazy`** only.

Expand description

Arguments used by `datetime` in order to produce an [`Expr`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html "enum polars::prelude::Expr") of Datetime

Construct a [`DatetimeArgs`](https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html "struct polars::prelude::DatetimeArgs") with `DatetimeArgs::new(y, m, d)`. This will set the other time units to `lit(0)`. You can then set the other fields with the `with_*` methods, or use `with_hms` to set `hour`, `minute`, and `second` all at once.

## <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#examples" class="doc-anchor">§</a>Examples

``` rust
use polars_plan::prelude::*;
// construct a DatetimeArgs set to July 20, 1969 at 20:17
let args = DatetimeArgs::new(lit(1969), lit(7), lit(20)).with_hms(lit(20), lit(17), lit(0));
// or
let args = DatetimeArgs::new(lit(1969), lit(7), lit(20)).with_hour(lit(20)).with_minute(lit(17));

// construct a DatetimeArgs using existing columns
let args = DatetimeArgs::new(lit(2023), col("month"), col("day"));
```

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#structfield.year" class="anchor field">§</a>`year: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#structfield.month" class="anchor field">§</a>`month: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#structfield.day" class="anchor field">§</a>`day: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#structfield.hour" class="anchor field">§</a>`hour: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#structfield.minute" class="anchor field">§</a>`minute: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#structfield.second" class="anchor field">§</a>`second: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#structfield.microsecond" class="anchor field">§</a>`microsecond: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#structfield.time_unit" class="anchor field">§</a>`time_unit: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit"><code>TimeUnit</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#structfield.time_zone" class="anchor field">§</a>`time_zone: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone"><code>TimeZone</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#structfield.ambiguous" class="anchor field">§</a>`ambiguous: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#impl-DatetimeArgs" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.new" class="fn">new</a>(year: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, month: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, day: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

Construct a new `DatetimeArgs` set to `year`, `month`, `day`

Other fields default to `lit(0)`. Use the `with_*` methods to set them.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.with_hms" class="fn">with_hms</a>(self, hour: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, minute: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, second: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

Set `hour`, `minute`, and `second`

Equivalent to

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
self.with_hour(hour)
    .with_minute(minute)
    .with_second(second)
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.with_year" class="fn">with_year</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

Set the year

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.with_month" class="fn">with_month</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

Set the month

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.with_day" class="fn">with_day</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

Set the day

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.with_hour" class="fn">with_hour</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

Set the hour

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.with_minute" class="fn">with_minute</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

Set the minute

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.with_second" class="fn">with_second</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

Set the second

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.with_microsecond" class="fn">with_microsecond</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

Set the microsecond

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.with_time_unit" class="fn">with_time_unit</a>(self, time_unit: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.with_time_zone" class="fn">with_time_zone</a>(self, time_zone: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.with_ambiguous" class="fn">with_ambiguous</a>(self, ambiguous: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#impl-Clone-for-DatetimeArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#impl-Debug-for-DatetimeArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#impl-Default-for-DatetimeArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html" class="struct" title="struct polars::prelude::DatetimeArgs">DatetimeArgs</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeArgs.html#blanket-implementations" class="anchor">§</a>
