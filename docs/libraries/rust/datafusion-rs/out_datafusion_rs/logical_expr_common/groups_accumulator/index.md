# Module groups_accumulator Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/lib.rs.html#39" class="src">Source</a>

Expand description

Vectorized [`GroupsAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html "trait datafusion::logical_expr::GroupsAccumulator")

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/groups_accumulator/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/groups_accumulator/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr_common::groups_accumulator::EmitTo">EmitTo</a>  
Describes how many rows should be emitted during grouping.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/groups_accumulator/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/groups_accumulator/trait.GroupsAccumulator.html" class="trait" title="trait datafusion::logical_expr_common::groups_accumulator::GroupsAccumulator">GroupsAccumulator</a>  
`GroupsAccumulator` implements a single aggregate (e.g. AVG) and stores the state for *all* groups internally.
