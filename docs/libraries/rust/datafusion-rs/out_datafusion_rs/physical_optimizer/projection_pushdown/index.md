# Module projection_pushdown Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#40" class="src">Source</a>

Expand description

This file implements the `ProjectionPushdown` physical optimization rule. The function [`remove_unnecessary_projections`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.remove_unnecessary_projections.html "fn datafusion::physical_plan::projection::remove_unnecessary_projections") tries to push down all projections one by one if the operator below is amenable to this. If a projection reaches a source, it can even disappear from the plan entirely.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/projection_pushdown/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/projection_pushdown/struct.ProjectionPushdown.html" class="struct" title="struct datafusion::physical_optimizer::projection_pushdown::ProjectionPushdown">ProjectionPushdown</a>  
This rule inspects `ProjectionExec`’s in the given physical plan and tries to remove or swap with its child.
