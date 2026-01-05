# Enum SetMonotonicity Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/udaf.rs.html#1221" class="src">Source</a>

``` rust
pub enum SetMonotonicity {
    Increasing,
    Decreasing,
    NotMonotonic,
}
```

Expand description

Indicates whether an aggregation function is monotonic as a set function. A set function is monotonically increasing if its value increases as its argument grows (as a set). Formally, `f` is a monotonically increasing set function if `f(S) >= f(T)` whenever `S` is a superset of `T`.

For example `COUNT` and `MAX` are monotonically increasing as their values always increase (or stay the same) as new values are seen. On the other hand, `MIN` is monotonically decreasing as its value always decreases or stays the same as new values are seen.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#variant.Increasing" class="anchor">§</a>

### Increasing

Aggregate value increases or stays the same as the input set grows.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#variant.Decreasing" class="anchor">§</a>

### Decreasing

Aggregate value decreases or stays the same as the input set grows.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#variant.NotMonotonic" class="anchor">§</a>

### NotMonotonic

Aggregate value may increase, decrease, or stay the same as the input set grows.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#impl-Clone-for-SetMonotonicity" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html" class="enum" title="enum datafusion::logical_expr::SetMonotonicity">SetMonotonicity</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html" class="enum" title="enum datafusion::logical_expr::SetMonotonicity">SetMonotonicity</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#impl-Debug-for-SetMonotonicity" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html" class="enum" title="enum datafusion::logical_expr::SetMonotonicity">SetMonotonicity</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#impl-PartialEq-for-SetMonotonicity" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html" class="enum" title="enum datafusion::logical_expr::SetMonotonicity">SetMonotonicity</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html" class="enum" title="enum datafusion::logical_expr::SetMonotonicity">SetMonotonicity</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#impl-StructuralPartialEq-for-SetMonotonicity" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html" class="enum" title="enum datafusion::logical_expr::SetMonotonicity">SetMonotonicity</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SetMonotonicity.html#blanket-implementations" class="anchor">§</a>
