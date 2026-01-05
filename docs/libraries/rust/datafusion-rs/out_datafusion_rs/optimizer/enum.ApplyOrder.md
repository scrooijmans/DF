# Enum ApplyOrder Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/optimizer.rs.html#209" class="src">Source</a>

``` rust
pub enum ApplyOrder {
    TopDown,
    BottomUp,
}
```

Expand description

Specifies how recursion for an `OptimizerRule` should be handled.

- `Some(apply_order)`: The Optimizer will recursively apply the rule to the plan.
- `None`: the rule must handle any required recursion itself.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#variant.TopDown" class="anchor">§</a>

### TopDown

Apply the rule to the node before its inputs

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#variant.BottomUp" class="anchor">§</a>

### BottomUp

Apply the rule to the node after its inputs

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#impl-Clone-for-ApplyOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#impl-Debug-for-ApplyOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#impl-PartialEq-for-ApplyOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#impl-Copy-for-ApplyOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#impl-StructuralPartialEq-for-ApplyOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html#blanket-implementations" class="anchor">§</a>
