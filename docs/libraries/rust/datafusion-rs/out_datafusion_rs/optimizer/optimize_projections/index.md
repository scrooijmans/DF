# Module optimize_projections Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#56" class="src">Source</a>

Expand description

[`OptimizeProjections`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimize_projections/struct.OptimizeProjections.html "struct datafusion::optimizer::optimize_projections::OptimizeProjections") identifies and eliminates unused columns

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimize_projections/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimize_projections/struct.OptimizeProjections.html" class="struct" title="struct datafusion::optimizer::optimize_projections::OptimizeProjections">OptimizeProjections</a>  
Optimizer rule to prune unnecessary columns from intermediate schemas inside the [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan"). This rule:

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimize_projections/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimize_projections/fn.is_projection_unnecessary.html" class="fn" title="fn datafusion::optimizer::optimize_projections::is_projection_unnecessary">is_projection_unnecessary</a>  
Projection is unnecessary, when
