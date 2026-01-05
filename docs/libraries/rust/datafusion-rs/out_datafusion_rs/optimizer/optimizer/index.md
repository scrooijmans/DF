# Module optimizer Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#57" class="src">Source</a>

Expand description

[`Optimizer`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html "struct datafusion::optimizer::Optimizer") and [`OptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html "trait datafusion::optimizer::OptimizerRule")

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimizer/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::optimizer::Optimizer">Optimizer</a>  
A rule-based optimizer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::optimizer::OptimizerContext">OptimizerContext</a>  
A standalone [`OptimizerConfig`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html "trait datafusion::optimizer::OptimizerConfig") that can be used independently of DataFusion’s config management

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimizer/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::optimizer::ApplyOrder">ApplyOrder</a>  
Specifies how recursion for an `OptimizerRule` should be handled.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimizer/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimizer/trait.OptimizerConfig.html" class="trait" title="trait datafusion::optimizer::optimizer::OptimizerConfig">OptimizerConfig</a>  
Options to control the DataFusion Optimizer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::optimizer::OptimizerRule">OptimizerRule</a>  
`OptimizerRule`s transforms one [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") into another which computes the same results, but in a potentially more efficient way. If there are no suitable transformations for the input plan, the optimizer should simply return it unmodified.
