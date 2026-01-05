# Enum FilterPushdownPhase Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/filter_pushdown.rs.html#46" class="src">Source</a>

``` rust
pub enum FilterPushdownPhase {
    Pre,
    Post,
}
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#variant.Pre" class="anchor">§</a>

### Pre

Pushdown that happens before most other optimizations. This pushdown allows static filters that do not reference any [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")s to be pushed down. Filters that reference an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") cannot be pushed down at this stage since the whole plan tree may be rewritten by other optimizations. Implementers are however allowed to modify the execution plan themselves during this phase, for example by returning a completely different [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") from [`ExecutionPlan::handle_child_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result "method datafusion::physical_plan::ExecutionPlan::handle_child_pushdown_result").

Pushdown of [`FilterExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/struct.FilterExec.html "struct datafusion::physical_plan::filter::FilterExec") into `DataSourceExec` is an example of a pre-pushdown. Unlike filter pushdown in the logical phase, which operates on the logical plan to push filters into the logical table scan, the `Pre` phase in the physical plan targets the actual physical scan, pushing filters down to specific data source implementations. For example, Parquet supports filter pushdown to reduce data read during scanning, while CSV typically does not.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#variant.Post" class="anchor">§</a>

### Post

Pushdown that happens after most other optimizations. This stage of filter pushdown allows filters that reference an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") to be pushed down. Since subsequent optimizations should not change the structure of the plan tree except for calling [`ExecutionPlan::with_new_children`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.with_new_children "method datafusion::physical_plan::ExecutionPlan::with_new_children") (which generally preserves internal references) it is safe for references between [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")s to be established at this stage.

This phase is used to link a [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") (with a TopK operator) or a [`HashJoinExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html "struct datafusion::physical_plan::joins::HashJoinExec") to a `DataSourceExec`.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#impl-Clone-for-FilterPushdownPhase" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#impl-Debug-for-FilterPushdownPhase" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#impl-Display-for-FilterPushdownPhase" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#impl-PartialEq-for-FilterPushdownPhase" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#impl-Copy-for-FilterPushdownPhase" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#impl-Eq-for-FilterPushdownPhase" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#impl-StructuralPartialEq-for-FilterPushdownPhase" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html#blanket-implementations" class="anchor">§</a>
