# Enum AggregateOrderSensitivity Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate_common/order.rs.html#20" class="src">Source</a>

``` rust
pub enum AggregateOrderSensitivity {
    Insensitive,
    HardRequirement,
    SoftRequirement,
    Beneficial,
}
```

Expand description

Represents the sensitivity of an aggregate expression to ordering.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#variant.Insensitive" class="anchor">§</a>

### Insensitive

Indicates that the aggregate expression is insensitive to ordering. Ordering at the input is not important for the result of the aggregator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#variant.HardRequirement" class="anchor">§</a>

### HardRequirement

Indicates that the aggregate expression has a hard requirement on ordering. The aggregator cannot produce a correct result unless its ordering requirement is satisfied.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#variant.SoftRequirement" class="anchor">§</a>

### SoftRequirement

Indicates that the aggregator is more efficient when the input is ordered but can still produce its result correctly regardless of the input ordering. This is similar to, but stronger than, [`Self::Beneficial`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#variant.Beneficial "variant datafusion::logical_expr::utils::AggregateOrderSensitivity::Beneficial").

Similarly to [`Self::HardRequirement`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#variant.HardRequirement "variant datafusion::logical_expr::utils::AggregateOrderSensitivity::HardRequirement"), when possible DataFusion will insert a `SortExec`, to reorder the input to match the SoftRequirement. However, when such a `SortExec` cannot be inserted, (for example, due to conflicting [`Self::HardRequirement`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#variant.HardRequirement "variant datafusion::logical_expr::utils::AggregateOrderSensitivity::HardRequirement") with other ordered aggregates in the query), the aggregate function will still execute, without the preferred order, unlike with [`Self::HardRequirement`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#variant.HardRequirement "variant datafusion::logical_expr::utils::AggregateOrderSensitivity::HardRequirement")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#variant.Beneficial" class="anchor">§</a>

### Beneficial

Indicates that ordering is beneficial for the aggregate expression in terms of evaluation efficiency. The aggregator can produce its result efficiently when its required ordering is satisfied; however, it can still produce the correct result (albeit less efficiently) when its required ordering is not met.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#impl-AggregateOrderSensitivity" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#method.is_insensitive" class="fn">is_insensitive</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#method.is_beneficial" class="fn">is_beneficial</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#method.hard_requires" class="fn">hard_requires</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#impl-Clone-for-AggregateOrderSensitivity" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#impl-Debug-for-AggregateOrderSensitivity" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#impl-PartialEq-for-AggregateOrderSensitivity" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#impl-Copy-for-AggregateOrderSensitivity" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#impl-Eq-for-AggregateOrderSensitivity" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#impl-StructuralPartialEq-for-AggregateOrderSensitivity" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html#blanket-implementations" class="anchor">§</a>
