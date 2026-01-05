# Trait TreeNodeVisitor Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/tree_node.rs.html#458" class="src">Source</a>

``` rust
pub trait TreeNodeVisitor<'n>: Sized {
    type Node: TreeNode;

    // Provided methods
    fn f_down(
        &mut self,
        _node: &'n Self::Node,
    ) -> Result<TreeNodeRecursion, DataFusionError> { ... }
    fn f_up(
        &mut self,
        _node: &'n Self::Node,
    ) -> Result<TreeNodeRecursion, DataFusionError> { ... }
}
```

Expand description

A [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) for recursively inspecting [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode")s via [`TreeNode::visit`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.visit "method datafusion::common::tree_node::TreeNode::visit").

See [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") for more details on available APIs

When passed to [`TreeNode::visit`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.visit "method datafusion::common::tree_node::TreeNode::visit"), [`TreeNodeVisitor::f_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#method.f_down "method datafusion::common::tree_node::TreeNodeVisitor::f_down") and [`TreeNodeVisitor::f_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#method.f_up "method datafusion::common::tree_node::TreeNodeVisitor::f_up") are invoked recursively on the tree. See [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") for more details on controlling the traversal.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#return-value" class="doc-anchor">§</a>Return Value

The returns value of `f_up` and `f_down` specifies how the tree walk should proceed. See [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") for details. If an [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, the recursion stops immediately.

Note: If using the default implementations of [`TreeNodeVisitor::f_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#method.f_up "method datafusion::common::tree_node::TreeNodeVisitor::f_up") or [`TreeNodeVisitor::f_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#method.f_down "method datafusion::common::tree_node::TreeNodeVisitor::f_down") that do nothing, consider using [`TreeNode::apply`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.apply "method datafusion::common::tree_node::TreeNode::apply") instead.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#see-also" class="doc-anchor">§</a>See Also:

- [`TreeNode::rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite "method datafusion::common::tree_node::TreeNode::rewrite") to rewrite owned `TreeNode`s

## Required Associated Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#associatedtype.Node" class="associatedtype">Node</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html" class="trait" title="trait datafusion::common::tree_node::TreeNode">TreeNode</a>

The node type which is visitable.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#method.f_down" class="fn">f_down</a>( &mut self, \_node: &'n Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeVisitor::Node">Node</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing down the tree, before any children are visited. Default implementation continues the recursion.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#method.f_up" class="fn">f_up</a>( &mut self, \_node: &'n Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeVisitor::Node">Node</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing up the tree after children are visited. Default implementation continues the recursion.

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#impl-TreeNodeVisitor%3C&#39;n%3E-for-GraphvizVisitor%3C&#39;_,+&#39;_%3E" class="anchor">§</a>

### impl\<'n\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeVisitor">TreeNodeVisitor</a>\<'n\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.GraphvizVisitor.html" class="struct" title="struct datafusion::logical_expr::logical_plan::display::GraphvizVisitor">GraphvizVisitor</a>\<'\_, '\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#associatedtype.Node-1" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#associatedtype.Node" class="associatedtype">Node</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#impl-TreeNodeVisitor%3C&#39;n%3E-for-IndentVisitor%3C&#39;_,+&#39;_%3E" class="anchor">§</a>

### impl\<'n\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeVisitor">TreeNodeVisitor</a>\<'n\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html" class="struct" title="struct datafusion::logical_expr::logical_plan::display::IndentVisitor">IndentVisitor</a>\<'\_, '\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#associatedtype.Node-2" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#associatedtype.Node" class="associatedtype">Node</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#impl-TreeNodeVisitor%3C&#39;n%3E-for-PgJsonVisitor%3C&#39;_,+&#39;_%3E" class="anchor">§</a>

### impl\<'n\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeVisitor">TreeNodeVisitor</a>\<'n\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.PgJsonVisitor.html" class="struct" title="struct datafusion::logical_expr::logical_plan::display::PgJsonVisitor">PgJsonVisitor</a>\<'\_, '\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#associatedtype.Node-3" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#associatedtype.Node" class="associatedtype">Node</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>
