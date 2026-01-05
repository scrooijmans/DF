# Module filter_pushdown Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#34" class="src">Source</a>

Expand description

Filter Pushdown Optimization Process

The filter pushdown mechanism involves four key steps:

1.  **Optimizer Asks Parent for a Filter Pushdown Plan**: The optimizer calls [`ExecutionPlan::gather_filters_for_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown "method datafusion::physical_plan::ExecutionPlan::gather_filters_for_pushdown") on the parent node, passing in parent predicates and phase. The parent node creates a [`FilterDescription`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterDescription.html "struct datafusion::physical_plan::filter_pushdown::FilterDescription") by inspecting its logic and children’s schemas, determining which filters can be pushed to each child.
2.  **Optimizer Executes Pushdown**: The optimizer recursively calls `push_down_filters` in this module on each child, passing the appropriate filters (`Vec<Arc<dyn PhysicalExpr>>`) for that child.
3.  **Optimizer Gathers Results**: The optimizer collects [`FilterPushdownPropagation`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html "struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation") results from children, containing information about which filters were successfully pushed down vs. unsupported.
4.  **Parent Responds**: The optimizer calls [`ExecutionPlan::handle_child_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result "method datafusion::physical_plan::ExecutionPlan::handle_child_pushdown_result") on the parent, passing a [`ChildPushdownResult`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildPushdownResult.html "struct datafusion::physical_plan::filter_pushdown::ChildPushdownResult") containing the aggregated pushdown outcomes. The parent decides how to handle filters that couldn’t be pushed down (e.g., keep them as FilterExec nodes).

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/filter_pushdown/struct.FilterPushdown.html" class="struct" title="struct datafusion::physical_optimizer::filter_pushdown::FilterPushdown">FilterPushdown</a>  
Attempts to recursively push given filters from the top of the tree into leaves.
