# Module enforce_distribution Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#31" class="src">Source</a>

Expand description

EnforceDistribution optimizer rule inspects the physical plan with respect to distribution requirements and adds [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec")s to satisfy them when necessary. If increasing parallelism is beneficial (and also desirable according to the configuration), this rule increases partition counts in the physical plan.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html" class="struct" title="struct datafusion::physical_optimizer::enforce_distribution::EnforceDistribution">EnforceDistribution</a>  
The `EnforceDistribution` rule ensures that distribution requirements are met. In doing so, this rule will increase the parallelism in the plan by introducing repartitioning operators to the physical plan.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/fn.adjust_input_keys_ordering.html" class="fn" title="fn datafusion::physical_optimizer::enforce_distribution::adjust_input_keys_ordering">adjust_input_keys_ordering</a>  
When the physical planner creates the Joins, the ordering of join keys is from the original query. That might not match with the output partitioning of the join node’s children A Top-Down process will use this method to adjust children’s output partitioning based on the parent key reordering requirements:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/fn.ensure_distribution.html" class="fn" title="fn datafusion::physical_optimizer::enforce_distribution::ensure_distribution">ensure_distribution</a>  
This function checks whether we need to add additional data exchange operators to satisfy distribution requirements. Since this function takes care of such requirements, we should avoid manually adding data exchange operators in other places.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/fn.reorder_aggregate_keys.html" class="fn" title="fn datafusion::physical_optimizer::enforce_distribution::reorder_aggregate_keys">reorder_aggregate_keys</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/fn.reorder_join_keys_to_inputs.html" class="fn" title="fn datafusion::physical_optimizer::enforce_distribution::reorder_join_keys_to_inputs">reorder_join_keys_to_inputs</a>  
When the physical planner creates the Joins, the ordering of join keys is from the original query. That might not match with the output partitioning of the join node’s children This method will try to change the ordering of the join keys to match with the partitioning of the join nodes’ children. If it can not match with both sides, it will try to match with one, either the left side or the right side.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/fn.reorder_partitioned_join_keys.html" class="fn" title="fn datafusion::physical_optimizer::enforce_distribution::reorder_partitioned_join_keys">reorder_partitioned_join_keys</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/fn.replace_order_preserving_variants.html" class="fn" title="fn datafusion::physical_optimizer::enforce_distribution::replace_order_preserving_variants">replace_order_preserving_variants</a>  
Updates the [`DistributionContext`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/type.DistributionContext.html "type datafusion::physical_optimizer::enforce_distribution::DistributionContext") if preserving ordering while changing partitioning is not helpful or desirable.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/type.DistributionContext.html" class="type" title="type datafusion::physical_optimizer::enforce_distribution::DistributionContext">DistributionContext</a>  
Keeps track of distribution changing operators (like `RepartitionExec`, `SortPreservingMergeExec`, `CoalescePartitionsExec`) and their ancestors. Using this information, we can optimize distribution of the plan if/when necessary.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/type.PlanWithKeyRequirements.html" class="type" title="type datafusion::physical_optimizer::enforce_distribution::PlanWithKeyRequirements">PlanWithKeyRequirements</a>  
Keeps track of parent required key orderings.
