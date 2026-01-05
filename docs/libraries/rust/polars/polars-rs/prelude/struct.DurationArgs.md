# Struct DurationArgs Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/temporal.rs.html#247" class="src">Source</a>

``` rust
pub struct DurationArgs {
    pub weeks: Expr,
    pub days: Expr,
    pub hours: Expr,
    pub minutes: Expr,
    pub seconds: Expr,
    pub milliseconds: Expr,
    pub microseconds: Expr,
    pub nanoseconds: Expr,
    pub time_unit: TimeUnit,
}
```

Available on **crate feature `lazy`** only.

Expand description

Arguments used by `duration` in order to produce an [`Expr`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html "enum polars::prelude::Expr") of [`Duration`](https://docs.rs/polars/latest/polars/prelude/struct.Duration.html "struct polars::prelude::Duration")

To construct a [`DurationArgs`](https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html "struct polars::prelude::DurationArgs"), use struct literal syntax with `..Default::default()` to leave unspecified fields at their default value of `lit(0)`, as demonstrated below.

``` rust
let args = DurationArgs {
    days: lit(5),
    hours: col("num_hours"),
    minutes: col("num_minutes"),
    ..Default::default()  // other fields are lit(0)
};
```

If you prefer builder syntax, `with_*` methods are also available.

``` rust
let args = DurationArgs::new().with_weeks(lit(42)).with_hours(lit(84));
```

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#structfield.weeks" class="anchor field">§</a>`weeks: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#structfield.days" class="anchor field">§</a>`days: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#structfield.hours" class="anchor field">§</a>`hours: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#structfield.minutes" class="anchor field">§</a>`minutes: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#structfield.seconds" class="anchor field">§</a>`seconds: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#structfield.milliseconds" class="anchor field">§</a>`milliseconds: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#structfield.microseconds" class="anchor field">§</a>`microseconds: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#structfield.nanoseconds" class="anchor field">§</a>`nanoseconds: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#structfield.time_unit" class="anchor field">§</a>`time_unit: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit"><code>TimeUnit</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#impl-DurationArgs" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Create a new [`DurationArgs`](https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html "struct polars::prelude::DurationArgs") with all fields set to `lit(0)`. Use the `with_*` methods to set the fields.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.with_hms" class="fn">with_hms</a>(self, hours: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, minutes: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, seconds: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Set `hours`, `minutes`, and `seconds`

Equivalent to:

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
self.with_hours(hours)
    .with_minutes(minutes)
    .with_seconds(seconds)
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.with_fractional_seconds" class="fn">with_fractional_seconds</a>( self, milliseconds: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, microseconds: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, nanoseconds: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Set `milliseconds`, `microseconds`, and `nanoseconds`

Equivalent to

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
self.with_milliseconds(milliseconds)
    .with_microseconds(microseconds)
    .with_nanoseconds(nanoseconds)
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.with_weeks" class="fn">with_weeks</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Set the weeks

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.with_days" class="fn">with_days</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Set the days

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.with_hours" class="fn">with_hours</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Set the hours

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.with_minutes" class="fn">with_minutes</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Set the minutes

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.with_seconds" class="fn">with_seconds</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Set the seconds

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.with_milliseconds" class="fn">with_milliseconds</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Set the milliseconds

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.with_microseconds" class="fn">with_microseconds</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Set the microseconds

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.with_nanoseconds" class="fn">with_nanoseconds</a>(self, n: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Set the nanoseconds

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#impl-Clone-for-DurationArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#impl-Debug-for-DurationArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#impl-Default-for-DurationArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html" class="struct" title="struct polars::prelude::DurationArgs">DurationArgs</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationArgs.html#blanket-implementations" class="anchor">§</a>
