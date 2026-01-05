# Module filter_pushdown Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#74" class="src">Source</a>

Expand description

Filter Pushdown Optimization Process

The filter pushdown mechanism involves four key steps:

1.  **Optimizer Asks Parent for a Filter Pushdown Plan**: The optimizer calls [`ExecutionPlan::gather_filters_for_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown "method datafusion::physical_plan::ExecutionPlan::gather_filters_for_pushdown") on the parent node, passing in parent predicates and phase. The parent node creates a [`FilterDescription`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterDescription.html "struct datafusion::physical_plan::filter_pushdown::FilterDescription") by inspecting its logic and children’s schemas, determining which filters can be pushed to each child.
2.  **Optimizer Executes Pushdown**: The optimizer recursively pushes down filters for each child, passing the appropriate filters (`Vec<Arc<dyn PhysicalExpr>>`) for that child.
3.  **Optimizer Gathers Results**: The optimizer collects [`FilterPushdownPropagation`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html "struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation") results from children, containing information about which filters were successfully pushed down vs. unsupported.
4.  **Parent Responds**: The optimizer calls [`ExecutionPlan::handle_child_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result "method datafusion::physical_plan::ExecutionPlan::handle_child_pushdown_result") on the parent, passing a [`ChildPushdownResult`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildPushdownResult.html "struct datafusion::physical_plan::filter_pushdown::ChildPushdownResult") containing the aggregated pushdown outcomes. The parent decides how to handle filters that couldn’t be pushed down (e.g., keep them as FilterExec nodes).

See also datafusion/physical-optimizer/src/filter_pushdown.rs.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildFilterDescription.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::ChildFilterDescription">ChildFilterDescription</a>  
Describes filter pushdown for a single child node.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildFilterPushdownResult.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::ChildFilterPushdownResult">ChildFilterPushdownResult</a>  
The result of pushing down a single parent filter into all children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildPushdownResult.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::ChildPushdownResult">ChildPushdownResult</a>  
The result of pushing down filters into a child node.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterDescription.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterDescription">FilterDescription</a>  
Describes how filters should be pushed down to children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>  
The result of pushing down filters into a node.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.PushedDownPredicate.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::PushedDownPredicate">PushedDownPredicate</a>  
The result of a plan for pushing down a filter into a child node. This contains references to filters so that nodes can mutate a filter before pushing it down to a child node (e.g. to adjust a projection) or can directly take ownership of filters that their children could not handle.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.PushedDown.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::PushedDown">PushedDown</a>  
Discriminant for the result of pushing down a filter into a child node.
