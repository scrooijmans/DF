# Struct LogicalPlanUdfOptions Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/options/mod.rs.html#350" class="src">Source</a>

``` rust
pub struct LogicalPlanUdfOptions {
    pub predicate_pd: bool,
    pub projection_pd: bool,
    pub fmt_str: &'static str,
}
```

Available on **crate feature `lazy`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#structfield.predicate_pd" class="anchor field">§</a>`predicate_pd: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

allow predicate pushdown optimizations

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#structfield.projection_pd" class="anchor field">§</a>`projection_pd: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

allow projection pushdown optimizations

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#structfield.fmt_str" class="anchor field">§</a>`fmt_str: &'static `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#impl-Clone-for-LogicalPlanUdfOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html" class="struct" title="struct polars::prelude::LogicalPlanUdfOptions">LogicalPlanUdfOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html" class="struct" title="struct polars::prelude::LogicalPlanUdfOptions">LogicalPlanUdfOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#impl-Debug-for-LogicalPlanUdfOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html" class="struct" title="struct polars::prelude::LogicalPlanUdfOptions">LogicalPlanUdfOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#impl-PartialEq-for-LogicalPlanUdfOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html" class="struct" title="struct polars::prelude::LogicalPlanUdfOptions">LogicalPlanUdfOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html" class="struct" title="struct polars::prelude::LogicalPlanUdfOptions">LogicalPlanUdfOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#impl-Copy-for-LogicalPlanUdfOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html" class="struct" title="struct polars::prelude::LogicalPlanUdfOptions">LogicalPlanUdfOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#impl-Eq-for-LogicalPlanUdfOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html" class="struct" title="struct polars::prelude::LogicalPlanUdfOptions">LogicalPlanUdfOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#impl-StructuralPartialEq-for-LogicalPlanUdfOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html" class="struct" title="struct polars::prelude::LogicalPlanUdfOptions">LogicalPlanUdfOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LogicalPlanUdfOptions.html#blanket-implementations" class="anchor">§</a>
