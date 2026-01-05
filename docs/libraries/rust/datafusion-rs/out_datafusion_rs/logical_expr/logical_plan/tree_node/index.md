# Module tree_node Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/mod.rs.html#27" class="src">Source</a>

Expand description

[`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") based visiting and rewriting for [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s

Visiting (read only) APIs

- [`LogicalPlan::visit`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.visit "method datafusion::logical_expr::LogicalPlan::visit"): recursively visit the node and all of its inputs
- [`LogicalPlan::visit_with_subqueries`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.visit_with_subqueries "method datafusion::logical_expr::LogicalPlan::visit_with_subqueries"): recursively visit the node and all of its inputs, including subqueries
- [`LogicalPlan::apply_children`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.apply_children "method datafusion::logical_expr::LogicalPlan::apply_children"): recursively visit all inputs of this node
- [`LogicalPlan::apply_expressions`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.apply_expressions "method datafusion::logical_expr::LogicalPlan::apply_expressions"): (non recursively) visit all expressions of this node
- [`LogicalPlan::apply_subqueries`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.apply_subqueries "method datafusion::logical_expr::LogicalPlan::apply_subqueries"): (non recursively) visit all subqueries of this node
- [`LogicalPlan::apply_with_subqueries`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.apply_with_subqueries "method datafusion::logical_expr::LogicalPlan::apply_with_subqueries"): recursively visit all inputs and embedded subqueries.

Rewriting (update) APIs:

- [`LogicalPlan::exists`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.exists "method datafusion::logical_expr::LogicalPlan::exists"): search for an expression in a plan
- [`LogicalPlan::rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.rewrite "method datafusion::logical_expr::LogicalPlan::rewrite"): recursively rewrite the node and all of its inputs
- [`LogicalPlan::map_children`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.map_children "method datafusion::logical_expr::LogicalPlan::map_children"): recursively rewrite all inputs of this node
- [`LogicalPlan::map_expressions`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.map_expressions "method datafusion::logical_expr::LogicalPlan::map_expressions"): (non recursively) visit all expressions of this node
- [`LogicalPlan::map_subqueries`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.map_subqueries "method datafusion::logical_expr::LogicalPlan::map_subqueries"): (non recursively) rewrite all subqueries of this node
- [`LogicalPlan::rewrite_with_subqueries`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.rewrite_with_subqueries "method datafusion::logical_expr::LogicalPlan::rewrite_with_subqueries"): recursively rewrite the node and all of its inputs, including subqueries

(Re)creation APIs (these require substantial cloning and thus are slow):

- [`LogicalPlan::with_new_exprs`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.with_new_exprs "method datafusion::logical_expr::LogicalPlan::with_new_exprs"): Create a new plan with different expressions
- [`LogicalPlan::expressions`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.expressions "method datafusion::logical_expr::LogicalPlan::expressions"): Return a copy of the plan’s expressions
