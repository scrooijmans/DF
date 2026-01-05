# Module eliminate_group_by_constant Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#48" class="src">Source</a>

Expand description

[`EliminateGroupByConstant`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_group_by_constant/struct.EliminateGroupByConstant.html "struct datafusion::optimizer::eliminate_group_by_constant::EliminateGroupByConstant") removes constant expressions from `GROUP BY` clause

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_group_by_constant/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_group_by_constant/struct.EliminateGroupByConstant.html" class="struct" title="struct datafusion::optimizer::eliminate_group_by_constant::EliminateGroupByConstant">EliminateGroupByConstant</a>  
Optimizer rule that removes constant expressions from `GROUP BY` clause and places additional projection on top of aggregation, to preserve original schema
