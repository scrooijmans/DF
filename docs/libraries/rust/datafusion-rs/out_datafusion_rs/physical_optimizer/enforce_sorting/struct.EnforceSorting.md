# Struct EnforceSorting Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/enforce_sorting/mod.rs.html#79" class="src">Source</a>

``` rust
pub struct EnforceSorting {}
```

Expand description

This rule inspects [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")’s in the given physical plan in order to remove unnecessary sorts, and optimize sort performance across the plan.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#impl-EnforceSorting" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html" class="struct" title="struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting">EnforceSorting</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html" class="struct" title="struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting">EnforceSorting</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#impl-Debug-for-EnforceSorting" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html" class="struct" title="struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting">EnforceSorting</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#impl-Default-for-EnforceSorting" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html" class="struct" title="struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting">EnforceSorting</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html" class="struct" title="struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting">EnforceSorting</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#impl-PhysicalOptimizerRule-for-EnforceSorting" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html" class="struct" title="struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting">EnforceSorting</a>

Performs optimizations based upon a series of subrules. Refer to each subrule for detailed descriptions of the optimizations performed: Subrule application is ordering dependent.

Optimizer consists of 5 main parts which work sequentially

1.  [`ensure_sorting`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/fn.ensure_sorting.html "fn datafusion::physical_optimizer::enforce_sorting::ensure_sorting") Works down-to-top to be able to remove unnecessary [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")s, [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec")s add [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")s if necessary by a requirement and adjusts window operators.
2.  [`parallelize_sorts`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/fn.parallelize_sorts.html "fn datafusion::physical_optimizer::enforce_sorting::parallelize_sorts") (Optional, depends on the `repartition_sorts` configuration) Responsible to identify and remove unnecessary partition unifier operators such as [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec"), [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec") follows [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")s does possible simplifications.
3.  [`replace_with_order_preserving_variants()`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/replace_with_order_preserving_variants/fn.replace_with_order_preserving_variants.html "fn datafusion::physical_optimizer::enforce_sorting::replace_with_order_preserving_variants::replace_with_order_preserving_variants") Replaces with alternative operators, for example can merge a [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") and a [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec") into one [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec") or a [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") + [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec") combination into an order preserving [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec")
4.  [`sort_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/sort_pushdown/index.html "mod datafusion::physical_optimizer::enforce_sorting::sort_pushdown") Works top-down. Responsible to push down sort operators as deep as possible in the plan.
5.  `replace_with_partial_sort` Checks if it’s possible to replace [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")s with [`PartialSortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/partial_sort/struct.PartialSortExec.html "struct datafusion::physical_plan::sorts::partial_sort::PartialSortExec") operators

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#method.optimize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.optimize" class="fn">optimize</a>( &self, plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Rewrite `plan` to an optimized form

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this optimizer rule

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#method.schema_check" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.schema_check" class="fn">schema_check</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

A flag to indicate whether the physical planner should validate that the rule will not change the schema of the plan after the rewriting. Some of the optimization rules might change the nullable properties of the schema and should disable the schema check.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html#blanket-implementations" class="anchor">§</a>
