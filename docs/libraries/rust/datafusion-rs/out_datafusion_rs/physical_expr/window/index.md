# Module window Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/lib.rs.html#44" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/struct.PlainAggregateWindowExpr.html" class="struct" title="struct datafusion::physical_expr::window::PlainAggregateWindowExpr">PlainAggregateWindowExpr</a>

A window expr that takes the form of an aggregate function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/struct.SlidingAggregateWindowExpr.html" class="struct" title="struct datafusion::physical_expr::window::SlidingAggregateWindowExpr">SlidingAggregateWindowExpr</a>

A window expr that takes the form of an aggregate function that can be incrementally computed over sliding windows.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/struct.StandardWindowExpr.html" class="struct" title="struct datafusion::physical_expr::window::StandardWindowExpr">StandardWindowExpr</a>

A window expr that takes the form of a [`StandardWindowFunctionExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html "trait datafusion::physical_expr::window::StandardWindowFunctionExpr").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/struct.WindowState.html" class="struct" title="struct datafusion::physical_expr::window::WindowState">WindowState</a>

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.StandardWindowFunctionExpr.html" class="trait" title="trait datafusion::physical_expr::window::StandardWindowFunctionExpr">StandardWindowFunctionExpr</a>  
Evaluates a window function by instantiating a `[PartitionEvaluator]` for calculating the function’s output in that partition.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/trait.WindowExpr.html" class="trait" title="trait datafusion::physical_expr::window::WindowExpr">WindowExpr</a>  
Common trait for [window function](https://en.wikipedia.org/wiki/Window_function_(SQL)) implementations

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/type.PartitionBatches.html" class="type" title="type datafusion::physical_expr::window::PartitionBatches">PartitionBatches</a>

The IndexMap (i.e. an ordered HashMap) where record batches are separated for each partition.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/type.PartitionKey.html" class="type" title="type datafusion::physical_expr::window::PartitionKey">PartitionKey</a>

Key for IndexMap for each unique partition

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/type.PartitionWindowAggStates.html" class="type" title="type datafusion::physical_expr::window::PartitionWindowAggStates">PartitionWindowAggStates</a>
