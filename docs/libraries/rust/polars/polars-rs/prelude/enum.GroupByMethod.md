# Enum GroupByMethod Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/mod.rs.html#866" class="src">Source</a>

``` rust
pub enum GroupByMethod {
Show 16 variants    Min,
    NanMin,
    Max,
    NanMax,
    Median,
    Mean,
    First,
    Last,
    Sum,
    Groups,
    NUnique,
    Quantile(f64, QuantileMethod),
    Count {
        include_nulls: bool,
    },
    Implode,
    Std(u8),
    Var(u8),
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Min" class="anchor">§</a>

### Min

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.NanMin" class="anchor">§</a>

### NanMin

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Max" class="anchor">§</a>

### Max

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.NanMax" class="anchor">§</a>

### NanMax

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Median" class="anchor">§</a>

### Median

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Mean" class="anchor">§</a>

### Mean

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.First" class="anchor">§</a>

### First

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Last" class="anchor">§</a>

### Last

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Sum" class="anchor">§</a>

### Sum

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Groups" class="anchor">§</a>

### Groups

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.NUnique" class="anchor">§</a>

### NUnique

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Quantile" class="anchor">§</a>

### Quantile(<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Count" class="anchor">§</a>

### Count

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Count.field.include_nulls" class="anchor field">§</a>`include_nulls: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Implode" class="anchor">§</a>

### Implode

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Std" class="anchor">§</a>

### Std(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#variant.Var" class="anchor">§</a>

### Var(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>)

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#impl-Clone-for-GroupByMethod" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html" class="enum" title="enum polars::prelude::GroupByMethod">GroupByMethod</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html" class="enum" title="enum polars::prelude::GroupByMethod">GroupByMethod</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#impl-Debug-for-GroupByMethod" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html" class="enum" title="enum polars::prelude::GroupByMethod">GroupByMethod</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#impl-Display-for-GroupByMethod" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html" class="enum" title="enum polars::prelude::GroupByMethod">GroupByMethod</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#impl-From%3CIRAggExpr%3E-for-GroupByMethod" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/aexpr/enum.IRAggExpr.html" class="enum" title="enum polars_plan::plans::aexpr::IRAggExpr">IRAggExpr</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html" class="enum" title="enum polars::prelude::GroupByMethod">GroupByMethod</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/aexpr/enum.IRAggExpr.html" class="enum" title="enum polars_plan::plans::aexpr::IRAggExpr">IRAggExpr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html" class="enum" title="enum polars::prelude::GroupByMethod">GroupByMethod</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#impl-Copy-for-GroupByMethod" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html" class="enum" title="enum polars::prelude::GroupByMethod">GroupByMethod</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupByMethod.html#blanket-implementations" class="anchor">§</a>
