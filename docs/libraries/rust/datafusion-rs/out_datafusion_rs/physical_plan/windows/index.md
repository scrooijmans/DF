# Module windows Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#90" class="src">Source</a>

Expand description

Physical expressions for window functions

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.BoundedWindowAggExec.html" class="struct" title="struct datafusion::physical_plan::windows::BoundedWindowAggExec">BoundedWindowAggExec</a>  
Window execution plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.PlainAggregateWindowExpr.html" class="struct" title="struct datafusion::physical_plan::windows::PlainAggregateWindowExpr">PlainAggregateWindowExpr</a>  
A window expr that takes the form of an aggregate function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.StandardWindowExpr.html" class="struct" title="struct datafusion::physical_plan::windows::StandardWindowExpr">StandardWindowExpr</a>  
A window expr that takes the form of a [`StandardWindowFunctionExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html "trait datafusion::physical_expr::window::StandardWindowFunctionExpr").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowAggExec.html" class="struct" title="struct datafusion::physical_plan::windows::WindowAggExec">WindowAggExec</a>  
Window execution plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowUDFExpr.html" class="struct" title="struct datafusion::physical_plan::windows::WindowUDFExpr">WindowUDFExpr</a>  
Implements [`StandardWindowFunctionExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html "trait datafusion::physical_expr::window::StandardWindowFunctionExpr") for [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF")

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/trait.WindowExpr.html" class="trait" title="trait datafusion::physical_plan::windows::WindowExpr">WindowExpr</a>  
Common trait for [window function](https://en.wikipedia.org/wiki/Window_function_(SQL)) implementations

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/fn.create_udwf_window_expr.html" class="fn" title="fn datafusion::physical_plan::windows::create_udwf_window_expr">create_udwf_window_expr</a>  
Creates a `StandardWindowFunctionExpr` suitable for a user defined window function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/fn.create_window_expr.html" class="fn" title="fn datafusion::physical_plan::windows::create_window_expr">create_window_expr</a>  
Create a physical expression for window function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/fn.get_best_fitting_window.html" class="fn" title="fn datafusion::physical_plan::windows::get_best_fitting_window">get_best_fitting_window</a>  
Constructs the best-fitting windowing operator (a `WindowAggExec` or a `BoundedWindowExec`) for the given `input` according to the specifications of `window_exprs` and `physical_partition_keys`. Here, best-fitting means not requiring additional sorting and/or partitioning for the given input.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/fn.get_ordered_partition_by_indices.html" class="fn" title="fn datafusion::physical_plan::windows::get_ordered_partition_by_indices">get_ordered_partition_by_indices</a>  
This function calculates the indices such that when partition by expressions reordered with the indices resulting expressions define a preset for existing ordering. For instance, if input is ordered by a, b, c and PARTITION BY b, a is used, this vector will be \[1, 0\]. It means that when we iterate b, a columns with the order \[1, 0\] resulting vector (a, b) is a preset of the existing ordering (a, b, c).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/fn.get_window_mode.html" class="fn" title="fn datafusion::physical_plan::windows::get_window_mode">get_window_mode</a>  
Compares physical ordering (output ordering of the `input` operator) with `partitionby_exprs` and `orderby_keys` to decide whether existing ordering is sufficient to run the current window operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/fn.schema_add_window_field.html" class="fn" title="fn datafusion::physical_plan::windows::schema_add_window_field">schema_add_window_field</a>  
Build field from window function and add it into schema
