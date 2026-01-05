# Enum RangeFunction Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/function_expr/range.rs.html#15" class="src">Source</a>

``` rust
pub enum RangeFunction {
    IntRange {
        step: i64,
        dtype: DataTypeExpr,
    },
    IntRanges {
        dtype: DataTypeExpr,
    },
    LinearSpace {
        closed: ClosedInterval,
    },
    LinearSpaces {
        closed: ClosedInterval,
        array_width: Option<usize>,
    },
    DateRange {
        interval: Duration,
        closed: ClosedWindow,
    },
    DateRanges {
        interval: Duration,
        closed: ClosedWindow,
    },
    DatetimeRange {
        interval: Duration,
        closed: ClosedWindow,
        time_unit: Option<TimeUnit>,
        time_zone: Option<TimeZone>,
    },
    DatetimeRanges {
        interval: Duration,
        closed: ClosedWindow,
        time_unit: Option<TimeUnit>,
        time_zone: Option<TimeZone>,
    },
    TimeRange {
        interval: Duration,
        closed: ClosedWindow,
    },
    TimeRanges {
        interval: Duration,
        closed: ClosedWindow,
    },
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.IntRange" class="anchor">§</a>

### IntRange

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.IntRange.field.step" class="anchor field">§</a>`step: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.IntRange.field.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr"><code>DataTypeExpr</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.IntRanges" class="anchor">§</a>

### IntRanges

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.IntRanges.field.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr"><code>DataTypeExpr</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.LinearSpace" class="anchor">§</a>

### LinearSpace

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.LinearSpace.field.closed" class="anchor field">§</a>`closed: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedInterval.html" class="enum" title="enum polars::prelude::ClosedInterval"><code>ClosedInterval</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.LinearSpaces" class="anchor">§</a>

### LinearSpaces

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.LinearSpaces.field.closed" class="anchor field">§</a>`closed: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedInterval.html" class="enum" title="enum polars::prelude::ClosedInterval"><code>ClosedInterval</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.LinearSpaces.field.array_width" class="anchor field">§</a>`array_width: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DateRange" class="anchor">§</a>

### DateRange

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DateRange.field.interval" class="anchor field">§</a>`interval: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration"><code>Duration</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DateRange.field.closed" class="anchor field">§</a>`closed: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedWindow.html" class="enum" title="enum polars::prelude::ClosedWindow"><code>ClosedWindow</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DateRanges" class="anchor">§</a>

### DateRanges

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DateRanges.field.interval" class="anchor field">§</a>`interval: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration"><code>Duration</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DateRanges.field.closed" class="anchor field">§</a>`closed: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedWindow.html" class="enum" title="enum polars::prelude::ClosedWindow"><code>ClosedWindow</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DatetimeRange" class="anchor">§</a>

### DatetimeRange

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DatetimeRange.field.interval" class="anchor field">§</a>`interval: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration"><code>Duration</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DatetimeRange.field.closed" class="anchor field">§</a>`closed: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedWindow.html" class="enum" title="enum polars::prelude::ClosedWindow"><code>ClosedWindow</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DatetimeRange.field.time_unit" class="anchor field">§</a>`time_unit: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit"><code>TimeUnit</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DatetimeRange.field.time_zone" class="anchor field">§</a>`time_zone: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone"><code>TimeZone</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DatetimeRanges" class="anchor">§</a>

### DatetimeRanges

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DatetimeRanges.field.interval" class="anchor field">§</a>`interval: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration"><code>Duration</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DatetimeRanges.field.closed" class="anchor field">§</a>`closed: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedWindow.html" class="enum" title="enum polars::prelude::ClosedWindow"><code>ClosedWindow</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DatetimeRanges.field.time_unit" class="anchor field">§</a>`time_unit: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit"><code>TimeUnit</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.DatetimeRanges.field.time_zone" class="anchor field">§</a>`time_zone: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone"><code>TimeZone</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.TimeRange" class="anchor">§</a>

### TimeRange

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.TimeRange.field.interval" class="anchor field">§</a>`interval: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration"><code>Duration</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.TimeRange.field.closed" class="anchor field">§</a>`closed: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedWindow.html" class="enum" title="enum polars::prelude::ClosedWindow"><code>ClosedWindow</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.TimeRanges" class="anchor">§</a>

### TimeRanges

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.TimeRanges.field.interval" class="anchor field">§</a>`interval: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration"><code>Duration</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#variant.TimeRanges.field.closed" class="anchor field">§</a>`closed: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedWindow.html" class="enum" title="enum polars::prelude::ClosedWindow"><code>ClosedWindow</code></a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#impl-Clone-for-RangeFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#impl-Debug-for-RangeFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#impl-Deserialize%3C&#39;de%3E-for-RangeFunction" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#impl-Display-for-RangeFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#impl-From%3CRangeFunction%3E-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#impl-Hash-for-RangeFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#impl-PartialEq-for-RangeFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#impl-Serialize-for-RangeFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#impl-StructuralPartialEq-for-RangeFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.RangeFunction.html" class="enum" title="enum polars::prelude::RangeFunction">RangeFunction</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.RangeFunction.html#blanket-implementations" class="anchor">§</a>
