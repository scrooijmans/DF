# Module physical_optimizer Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#797" class="src">Source</a>

Expand description

re-export of [`datafusion_physical_optimizer`](https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_optimizer/index.html "mod datafusion_physical_optimizer") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/aggregate_statistics/index.html" class="mod" title="mod datafusion::physical_optimizer::aggregate_statistics">aggregate_statistics</a>

Utilizing exact statistics from sources to avoid scanning data

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_async_exec_input/index.html" class="mod" title="mod datafusion::physical_optimizer::coalesce_async_exec_input">coalesce_async_exec_input</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/index.html" class="mod" title="mod datafusion::physical_optimizer::coalesce_batches">coalesce_batches</a>

CoalesceBatches optimizer that groups batches together rows in bigger batches to avoid overhead with small batches

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/combine_partial_final_agg/index.html" class="mod" title="mod datafusion::physical_optimizer::combine_partial_final_agg">combine_partial_final_agg</a>

CombinePartialFinalAggregate optimizer rule checks the adjacent Partial and Final AggregateExecs and try to combine them if necessary

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/index.html" class="mod" title="mod datafusion::physical_optimizer::enforce_distribution">enforce_distribution</a>

EnforceDistribution optimizer rule inspects the physical plan with respect to distribution requirements and adds [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec")s to satisfy them when necessary. If increasing parallelism is beneficial (and also desirable according to the configuration), this rule increases partition counts in the physical plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/index.html" class="mod" title="mod datafusion::physical_optimizer::enforce_sorting">enforce_sorting</a>

EnforceSorting optimizer rule inspects the physical plan with respect to local sorting requirements and does the following:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/ensure_coop/index.html" class="mod" title="mod datafusion::physical_optimizer::ensure_coop">ensure_coop</a>

The [`EnsureCooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/ensure_coop/struct.EnsureCooperative.html "struct datafusion::physical_optimizer::ensure_coop::EnsureCooperative") optimizer rule inspects the physical plan to find all portions of the plan that will not yield cooperatively. It will insert `CooperativeExec` nodes where appropriate to ensure execution plans always yield cooperatively.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/index.html" class="mod" title="mod datafusion::physical_optimizer::filter_pushdown">filter_pushdown</a>

Filter Pushdown Optimization Process

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/join_selection/index.html" class="mod" title="mod datafusion::physical_optimizer::join_selection">join_selection</a>

The [`JoinSelection`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/join_selection/struct.JoinSelection.html "struct datafusion::physical_optimizer::join_selection::JoinSelection") rule tries to modify a given plan so that it can accommodate infinite sources and utilize statistical information (if there is any) to obtain more performant plans. To achieve the first goal, it tries to transform a non-runnable query (with the given infinite sources) into a runnable query by replacing pipeline-breaking join operations with pipeline-friendly ones. To achieve the second goal, it selects the proper `PartitionMode` and the build side using the available statistics for hash joins.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/index.html" class="mod" title="mod datafusion::physical_optimizer::limit_pushdown">limit_pushdown</a>

[`LimitPushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/struct.LimitPushdown.html "struct datafusion::physical_optimizer::limit_pushdown::LimitPushdown") pushes `LIMIT` down through `ExecutionPlan`s to reduce data transfer as much as possible.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limited_distinct_aggregation/index.html" class="mod" title="mod datafusion::physical_optimizer::limited_distinct_aggregation">limited_distinct_aggregation</a>

A special-case optimizer rule that pushes limit into a grouped aggregation which has no aggregate expressions or sorting requirements

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/optimizer/index.html" class="mod" title="mod datafusion::physical_optimizer::optimizer">optimizer</a>

Physical optimizer traits

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/index.html" class="mod" title="mod datafusion::physical_optimizer::output_requirements">output_requirements</a>

The GlobalOrderRequire optimizer rule either:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/projection_pushdown/index.html" class="mod" title="mod datafusion::physical_optimizer::projection_pushdown">projection_pushdown</a>

This file implements the `ProjectionPushdown` physical optimization rule. The function [`remove_unnecessary_projections`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.remove_unnecessary_projections.html "fn datafusion::physical_plan::projection::remove_unnecessary_projections") tries to push down all projections one by one if the operator below is amenable to this. If a projection reaches a source, it can even disappear from the plan entirely.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/index.html" class="mod" title="mod datafusion::physical_optimizer::pruning">pruning</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/sanity_checker/index.html" class="mod" title="mod datafusion::physical_optimizer::sanity_checker">sanity_checker</a>

The [SanityCheckPlan](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/sanity_checker/struct.SanityCheckPlan.html "struct datafusion::physical_optimizer::sanity_checker::SanityCheckPlan") rule ensures that a given plan can accommodate its infinite sources, if there are any. It will reject non-runnable query plans that use pipeline-breaking operators on infinite input(s). In addition, it will check if all order and distribution requirements of a plan are satisfied by its children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/topk_aggregation/index.html" class="mod" title="mod datafusion::physical_optimizer::topk_aggregation">topk_aggregation</a>

An optimizer rule that detects aggregate operations that could use a limited bucket count

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/update_aggr_exprs/index.html" class="mod" title="mod datafusion::physical_optimizer::update_aggr_exprs">update_aggr_exprs</a>

An optimizer rule that checks ordering requirements of aggregate expressions and modifies the expressions to work more efficiently if possible.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/utils/index.html" class="mod" title="mod datafusion::physical_optimizer::utils">utils</a>

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a>  
`PhysicalOptimizerRule` transforms one \[‘ExecutionPlan’\] into another which computes the same results, but in a potentially more efficient way.
