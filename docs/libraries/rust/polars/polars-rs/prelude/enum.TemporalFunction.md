# Enum TemporalFunction Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/function_expr/datetime.rs.html#6" class="src">Source</a>

``` rust
pub enum TemporalFunction {
Show 42 variants    Millennium,
    Century,
    Year,
    IsLeapYear,
    IsoYear,
    Quarter,
    Month,
    DaysInMonth,
    Week,
    WeekDay,
    Day,
    OrdinalDay,
    Time,
    Date,
    Datetime,
    Duration(TimeUnit),
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    TotalDays,
    TotalHours,
    TotalMinutes,
    TotalSeconds,
    TotalMilliseconds,
    TotalMicroseconds,
    TotalNanoseconds,
    ToString(String),
    CastTimeUnit(TimeUnit),
    WithTimeUnit(TimeUnit),
    ConvertTimeZone(TimeZone),
    TimeStamp(TimeUnit),
    Truncate,
    BaseUtcOffset,
    DSTOffset,
    Round,
    Replace,
    ReplaceTimeZone(Option<TimeZone>, NonExistent),
    Combine(TimeUnit),
    DatetimeFunction {
        time_unit: TimeUnit,
        time_zone: Option<TimeZone>,
    },
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Millennium" class="anchor">§</a>

### Millennium

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Century" class="anchor">§</a>

### Century

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Year" class="anchor">§</a>

### Year

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.IsLeapYear" class="anchor">§</a>

### IsLeapYear

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.IsoYear" class="anchor">§</a>

### IsoYear

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Quarter" class="anchor">§</a>

### Quarter

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Month" class="anchor">§</a>

### Month

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.DaysInMonth" class="anchor">§</a>

### DaysInMonth

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Week" class="anchor">§</a>

### Week

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.WeekDay" class="anchor">§</a>

### WeekDay

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Day" class="anchor">§</a>

### Day

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.OrdinalDay" class="anchor">§</a>

### OrdinalDay

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Time" class="anchor">§</a>

### Time

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Date" class="anchor">§</a>

### Date

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Datetime" class="anchor">§</a>

### Datetime

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Duration" class="anchor">§</a>

### Duration(<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Hour" class="anchor">§</a>

### Hour

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Minute" class="anchor">§</a>

### Minute

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Second" class="anchor">§</a>

### Second

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Millisecond" class="anchor">§</a>

### Millisecond

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Microsecond" class="anchor">§</a>

### Microsecond

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Nanosecond" class="anchor">§</a>

### Nanosecond

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.TotalDays" class="anchor">§</a>

### TotalDays

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.TotalHours" class="anchor">§</a>

### TotalHours

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.TotalMinutes" class="anchor">§</a>

### TotalMinutes

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.TotalSeconds" class="anchor">§</a>

### TotalSeconds

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.TotalMilliseconds" class="anchor">§</a>

### TotalMilliseconds

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.TotalMicroseconds" class="anchor">§</a>

### TotalMicroseconds

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.TotalNanoseconds" class="anchor">§</a>

### TotalNanoseconds

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.ToString" class="anchor">§</a>

### ToString(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.CastTimeUnit" class="anchor">§</a>

### CastTimeUnit(<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.WithTimeUnit" class="anchor">§</a>

### WithTimeUnit(<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.ConvertTimeZone" class="anchor">§</a>

### ConvertTimeZone(<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.TimeStamp" class="anchor">§</a>

### TimeStamp(<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Truncate" class="anchor">§</a>

### Truncate

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.BaseUtcOffset" class="anchor">§</a>

### BaseUtcOffset

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.DSTOffset" class="anchor">§</a>

### DSTOffset

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Round" class="anchor">§</a>

### Round

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Replace" class="anchor">§</a>

### Replace

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.ReplaceTimeZone" class="anchor">§</a>

### ReplaceTimeZone(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.NonExistent.html" class="enum" title="enum polars::prelude::NonExistent">NonExistent</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.Combine" class="anchor">§</a>

### Combine(<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.DatetimeFunction" class="anchor">§</a>

### DatetimeFunction

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.DatetimeFunction.field.time_unit" class="anchor field">§</a>`time_unit: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit"><code>TimeUnit</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#variant.DatetimeFunction.field.time_zone" class="anchor field">§</a>`time_zone: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone"><code>TimeZone</code></a>`>`

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#impl-Clone-for-TemporalFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#impl-Debug-for-TemporalFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#impl-Deserialize%3C&#39;de%3E-for-TemporalFunction" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#impl-Display-for-TemporalFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#impl-Hash-for-TemporalFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#impl-PartialEq-for-TemporalFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#impl-Serialize-for-TemporalFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#impl-Eq-for-TemporalFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#impl-StructuralPartialEq-for-TemporalFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html" class="enum" title="enum polars::prelude::TemporalFunction">TemporalFunction</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.TemporalFunction.html#blanket-implementations" class="anchor">§</a>
