# Module physical_planner Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/physical_planner.rs.html#18-3569" class="src">Source</a>

Expand description

Planner for [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") to [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html" class="struct" title="struct datafusion::physical_planner::DefaultPhysicalPlanner">DefaultPhysicalPlanner</a>  
Default single node physical query planner that converts a `LogicalPlan` to an `ExecutionPlan` suitable for execution.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.ExtensionPlanner.html" class="trait" title="trait datafusion::physical_planner::ExtensionPlanner">ExtensionPlanner</a>  
This trait exposes the ability to plan an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") out of a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html" class="trait" title="trait datafusion::physical_planner::PhysicalPlanner">PhysicalPlanner</a>  
Physical query planner that converts a `LogicalPlan` to an `ExecutionPlan` suitable for execution.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/fn.create_aggregate_expr_and_maybe_filter.html" class="fn" title="fn datafusion::physical_planner::create_aggregate_expr_and_maybe_filter">create_aggregate_expr_and_maybe_filter</a>  
Create an aggregate expression from a logical expression or an alias

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/fn.create_aggregate_expr_with_name_and_maybe_filter.html" class="fn" title="fn datafusion::physical_planner::create_aggregate_expr_with_name_and_maybe_filter">create_aggregate_expr_with_name_and_maybe_filter</a>  
Create an aggregate expression with a name from a logical expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/fn.create_window_expr.html" class="fn" title="fn datafusion::physical_planner::create_window_expr">create_window_expr</a>  
Create a window expression from a logical expression or an alias

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/fn.create_window_expr_with_name.html" class="fn" title="fn datafusion::physical_planner::create_window_expr_with_name">create_window_expr_with_name</a>  
Create a window expression with a name from a logical expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/fn.is_window_frame_bound_valid.html" class="fn" title="fn datafusion::physical_planner::is_window_frame_bound_valid">is_window_frame_bound_valid</a>  
Check if window bounds are valid after schema information is available, and window_frame bounds are casted to the corresponding column type. queries like: OVER (ORDER BY a RANGES BETWEEN 3 PRECEDING AND 5 PRECEDING) OVER (ORDER BY a RANGES BETWEEN INTERVAL ‘3 DAY’ PRECEDING AND ‘5 DAY’ PRECEDING) are rejected
