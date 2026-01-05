# Module tree_node Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/lib.rs.html#76" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/tree_node/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/tree_node/struct.ExprContext.html" class="struct" title="struct datafusion::physical_expr::tree_node::ExprContext">ExprContext</a>  
A node object encapsulating a [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") node with a payload. Since there are two ways to access child plans—directly from the plan and through child nodes—it’s recommended to perform mutable operations via [`Self::update_expr_from_children`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/tree_node/struct.ExprContext.html#method.update_expr_from_children "method datafusion::physical_expr_common::tree_node::ExprContext::update_expr_from_children").
