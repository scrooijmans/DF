# Trait TreeNodeRewriter Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/tree_node.rs.html#498" class="src">Source</a>

``` rust
pub trait TreeNodeRewriter: Sized {
    type Node: TreeNode;

    // Provided methods
    fn f_down(
        &mut self,
        node: Self::Node,
    ) -> Result<Transformed<Self::Node>, DataFusionError> { ... }
    fn f_up(
        &mut self,
        node: Self::Node,
    ) -> Result<Transformed<Self::Node>, DataFusionError> { ... }
}
```

Expand description

A [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) for recursively rewriting [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode")s via [`TreeNode::rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite "method datafusion::common::tree_node::TreeNode::rewrite").

For example you can implement this trait on a struct to rewrite `Expr` or `LogicalPlan` that needs to track state during the rewrite.

See [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") for more details on available APIs

When passed to [`TreeNode::rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite "method datafusion::common::tree_node::TreeNode::rewrite"), [`TreeNodeRewriter::f_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_down "method datafusion::common::tree_node::TreeNodeRewriter::f_down") and [`TreeNodeRewriter::f_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_up "method datafusion::common::tree_node::TreeNodeRewriter::f_up") are invoked recursively on the tree. See [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") for more details on controlling the traversal.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#return-value" class="doc-anchor">§</a>Return Value

The returns value of `f_up` and `f_down` specifies how the tree walk should proceed. See [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") for details. If an [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, the recursion stops immediately.

Note: If using the default implementations of [`TreeNodeRewriter::f_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_up "method datafusion::common::tree_node::TreeNodeRewriter::f_up") or [`TreeNodeRewriter::f_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_down "method datafusion::common::tree_node::TreeNodeRewriter::f_down") that do nothing, consider using [`TreeNode::transform_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up "method datafusion::common::tree_node::TreeNode::transform_up") or [`TreeNode::transform_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down "method datafusion::common::tree_node::TreeNode::transform_down") instead.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#see-also" class="doc-anchor">§</a>See Also:

- [`TreeNode::visit`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.visit "method datafusion::common::tree_node::TreeNode::visit") to inspect borrowed `TreeNode`s

## Required Associated Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype">Node</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html" class="trait" title="trait datafusion::common::tree_node::TreeNode">TreeNode</a>

The node type which is rewritable.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_down" class="fn">f_down</a>( &mut self, node: Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeRewriter::Node">Node</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeRewriter::Node">Node</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing down the tree before any children are rewritten. Default implementation returns the node as is and continues recursion.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_up" class="fn">f_up</a>( &mut self, node: Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeRewriter::Node">Node</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeRewriter::Node">Node</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing up the tree after all children have been rewritten. Default implementation returns the node as is and continues recursion.

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#impl-TreeNodeRewriter-for-TypeCoercionRewriter%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html" class="struct" title="struct datafusion::optimizer::analyzer::type_coercion::TypeCoercionRewriter">TypeCoercionRewriter</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node-1" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype">Node</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#impl-TreeNodeRewriter-for-PullUpCorrelatedExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html" class="struct" title="struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr">PullUpCorrelatedExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node-2" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype">Node</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#impl-TreeNodeRewriter-for-GuaranteeRewriter%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.GuaranteeRewriter.html" class="struct" title="struct datafusion::optimizer::simplify_expressions::GuaranteeRewriter">GuaranteeRewriter</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node-3" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype">Node</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#impl-TreeNodeRewriter-for-PhysicalExprSimplifier%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html" class="struct" title="struct datafusion::physical_expr::PhysicalExprSimplifier">PhysicalExprSimplifier</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node-4" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype">Node</a> = <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>
