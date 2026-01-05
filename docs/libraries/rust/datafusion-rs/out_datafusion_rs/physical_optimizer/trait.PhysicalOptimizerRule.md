# Trait PhysicalOptimizerRule Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/optimizer.rs.html#52" class="src">Source</a>

``` rust
pub trait PhysicalOptimizerRule: Debug {
    // Required methods
    fn optimize(
        &self,
        plan: Arc<dyn ExecutionPlan>,
        config: &ConfigOptions,
    ) -> Result<Arc<dyn ExecutionPlan>, DataFusionError>;
    fn name(&self) -> &str;
    fn schema_check(&self) -> bool;
}
```

Expand description

`PhysicalOptimizerRule` transforms one \[‘ExecutionPlan’\] into another which computes the same results, but in a potentially more efficient way.

Use [`SessionState::add_physical_optimizer_rule`](https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_physical_optimizer_rule) to register additional `PhysicalOptimizerRule`s.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.optimize" class="fn">optimize</a>( &self, plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Rewrite `plan` to an optimized form

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this optimizer rule

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.schema_check" class="fn">schema_check</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

A flag to indicate whether the physical planner should validate that the rule will not change the schema of the plan after the rewriting. Some of the optimization rules might change the nullable properties of the schema and should disable the schema check.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-AggregateStatistics" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/aggregate_statistics/struct.AggregateStatistics.html" class="struct" title="struct datafusion::physical_optimizer::aggregate_statistics::AggregateStatistics">AggregateStatistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-CoalesceAsyncExecInput" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_async_exec_input/struct.CoalesceAsyncExecInput.html" class="struct" title="struct datafusion::physical_optimizer::coalesce_async_exec_input::CoalesceAsyncExecInput">CoalesceAsyncExecInput</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-CoalesceBatches" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html" class="struct" title="struct datafusion::physical_optimizer::coalesce_batches::CoalesceBatches">CoalesceBatches</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-CombinePartialFinalAggregate" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/combine_partial_final_agg/struct.CombinePartialFinalAggregate.html" class="struct" title="struct datafusion::physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate">CombinePartialFinalAggregate</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-EnforceDistribution" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html" class="struct" title="struct datafusion::physical_optimizer::enforce_distribution::EnforceDistribution">EnforceDistribution</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-EnforceSorting" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html" class="struct" title="struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting">EnforceSorting</a>

Performs optimizations based upon a series of subrules. Refer to each subrule for detailed descriptions of the optimizations performed: Subrule application is ordering dependent.

Optimizer consists of 5 main parts which work sequentially

1.  [`ensure_sorting`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/fn.ensure_sorting.html "fn datafusion::physical_optimizer::enforce_sorting::ensure_sorting") Works down-to-top to be able to remove unnecessary [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")s, [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec")s add [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")s if necessary by a requirement and adjusts window operators.
2.  [`parallelize_sorts`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/fn.parallelize_sorts.html "fn datafusion::physical_optimizer::enforce_sorting::parallelize_sorts") (Optional, depends on the `repartition_sorts` configuration) Responsible to identify and remove unnecessary partition unifier operators such as [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec"), [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec") follows [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")s does possible simplifications.
3.  [`replace_with_order_preserving_variants()`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/replace_with_order_preserving_variants/fn.replace_with_order_preserving_variants.html "fn datafusion::physical_optimizer::enforce_sorting::replace_with_order_preserving_variants::replace_with_order_preserving_variants") Replaces with alternative operators, for example can merge a [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") and a [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec") into one [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec") or a [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") + [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec") combination into an order preserving [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec")
4.  [`sort_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/sort_pushdown/index.html "mod datafusion::physical_optimizer::enforce_sorting::sort_pushdown") Works top-down. Responsible to push down sort operators as deep as possible in the plan.
5.  `replace_with_partial_sort` Checks if it’s possible to replace [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")s with [`PartialSortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/partial_sort/struct.PartialSortExec.html "struct datafusion::physical_plan::sorts::partial_sort::PartialSortExec") operators

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-EnsureCooperative" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/ensure_coop/struct.EnsureCooperative.html" class="struct" title="struct datafusion::physical_optimizer::ensure_coop::EnsureCooperative">EnsureCooperative</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-FilterPushdown" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html" class="struct" title="struct datafusion::physical_optimizer::filter_pushdown::FilterPushdown">FilterPushdown</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-JoinSelection" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/join_selection/struct.JoinSelection.html" class="struct" title="struct datafusion::physical_optimizer::join_selection::JoinSelection">JoinSelection</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-LimitPushdown" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/struct.LimitPushdown.html" class="struct" title="struct datafusion::physical_optimizer::limit_pushdown::LimitPushdown">LimitPushdown</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-LimitedDistinctAggregation" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limited_distinct_aggregation/struct.LimitedDistinctAggregation.html" class="struct" title="struct datafusion::physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation">LimitedDistinctAggregation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-OutputRequirements" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html" class="struct" title="struct datafusion::physical_optimizer::output_requirements::OutputRequirements">OutputRequirements</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-ProjectionPushdown" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/projection_pushdown/struct.ProjectionPushdown.html" class="struct" title="struct datafusion::physical_optimizer::projection_pushdown::ProjectionPushdown">ProjectionPushdown</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-SanityCheckPlan" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/sanity_checker/struct.SanityCheckPlan.html" class="struct" title="struct datafusion::physical_optimizer::sanity_checker::SanityCheckPlan">SanityCheckPlan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-TopKAggregation" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/topk_aggregation/struct.TopKAggregation.html" class="struct" title="struct datafusion::physical_optimizer::topk_aggregation::TopKAggregation">TopKAggregation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#impl-PhysicalOptimizerRule-for-OptimizeAggregateOrder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/update_aggr_exprs/struct.OptimizeAggregateOrder.html" class="struct" title="struct datafusion::physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder">OptimizeAggregateOrder</a>
