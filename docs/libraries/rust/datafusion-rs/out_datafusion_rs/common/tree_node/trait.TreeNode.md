# Trait TreeNode Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/tree_node.rs.html#95" class="src">Source</a>

``` rust
pub trait TreeNode: Sized {
    // Required methods
    fn apply_children<'n, F>(
        &'n self,
        f: F,
    ) -> Result<TreeNodeRecursion, DataFusionError>
       where F: FnMut(&'n Self) -> Result<TreeNodeRecursion, DataFusionError>;
    fn map_children<F>(self, f: F) -> Result<Transformed<Self>, DataFusionError>
       where F: FnMut(Self) -> Result<Transformed<Self>, DataFusionError>;

    // Provided methods
    fn visit<'n, V>(
        &'n self,
        visitor: &mut V,
    ) -> Result<TreeNodeRecursion, DataFusionError>
       where V: TreeNodeVisitor<'n, Node = Self> { ... }
    fn rewrite<R>(
        self,
        rewriter: &mut R,
    ) -> Result<Transformed<Self>, DataFusionError>
       where R: TreeNodeRewriter<Node = Self> { ... }
    fn apply<'n, F>(
        &'n self,
        f: F,
    ) -> Result<TreeNodeRecursion, DataFusionError>
       where F: FnMut(&'n Self) -> Result<TreeNodeRecursion, DataFusionError> { ... }
    fn transform<F>(self, f: F) -> Result<Transformed<Self>, DataFusionError>
       where F: FnMut(Self) -> Result<Transformed<Self>, DataFusionError> { ... }
    fn transform_down<F>(
        self,
        f: F,
    ) -> Result<Transformed<Self>, DataFusionError>
       where F: FnMut(Self) -> Result<Transformed<Self>, DataFusionError> { ... }
    fn transform_up<F>(self, f: F) -> Result<Transformed<Self>, DataFusionError>
       where F: FnMut(Self) -> Result<Transformed<Self>, DataFusionError> { ... }
    fn transform_down_up<FD, FU>(
        self,
        f_down: FD,
        f_up: FU,
    ) -> Result<Transformed<Self>, DataFusionError>
       where FD: FnMut(Self) -> Result<Transformed<Self>, DataFusionError>,
             FU: FnMut(Self) -> Result<Transformed<Self>, DataFusionError> { ... }
    fn exists<F>(&self, f: F) -> Result<bool, DataFusionError>
       where F: FnMut(&Self) -> Result<bool, DataFusionError> { ... }
}
```

Expand description

API for inspecting and rewriting tree data structures.

The `TreeNode` API is used to express algorithms separately from traversing the structure of `TreeNode`s, avoiding substantial code duplication.

This trait is implemented for plans ([`ExecutionPlan`](https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html), [`LogicalPlan`](https://docs.rs/datafusion-expr/latest/datafusion_expr/logical_plan/enum.LogicalPlan.html)) and expression trees ([`PhysicalExpr`](https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.PhysicalExpr.html), [`Expr`](https://docs.rs/datafusion-expr/latest/datafusion_expr/expr/enum.Expr.html)) as well as Plan+Payload combinations [`PlanContext`](https://docs.rs/datafusion/latest/datafusion/physical_plan/tree_node/struct.PlanContext.html) and [`ExprContext`](https://docs.rs/datafusion/latest/datafusion/physical_expr/tree_node/struct.ExprContext.html).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#overview" class="doc-anchor">§</a>Overview

There are three categories of TreeNode APIs:

1.  “Inspecting” APIs to traverse a tree of `&TreeNodes`: [`apply`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.apply "method datafusion_common::tree_node::TreeNode::apply::apply"), [`visit`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.visit "method datafusion_common::tree_node::TreeNode::visit::visit"), [`exists`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.exists "method datafusion_common::tree_node::TreeNode::exists::exists").

2.  “Transforming” APIs that traverse and consume a tree of `TreeNode`s producing possibly changed `TreeNode`s: [`transform`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform "method datafusion_common::tree_node::TreeNode::transform::transform"), [`transform_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up "method datafusion_common::tree_node::TreeNode::transform_up::transform_up"), [`transform_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down "method datafusion_common::tree_node::TreeNode::transform_down::transform_down"), [`transform_down_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down_up "method datafusion_common::tree_node::TreeNode::transform_down_up::transform_down_up"), and [`rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite "method datafusion_common::tree_node::TreeNode::rewrite::rewrite").

3.  Internal APIs used to implement the `TreeNode` API: [`apply_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.apply_children "method datafusion_common::tree_node::TreeNode::apply_children::apply_children"), and [`map_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.map_children "method datafusion_common::tree_node::TreeNode::map_children::map_children").

| Traversal Order | Inspecting | Transforming |
|----|----|----|
| top-down | [`apply`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.apply "method datafusion_common::tree_node::TreeNode::apply::apply"), [`exists`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.exists "method datafusion_common::tree_node::TreeNode::exists::exists") | [`transform_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down "method datafusion_common::tree_node::TreeNode::transform_down::transform_down") |
| bottom-up |  | [`transform`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform "method datafusion_common::tree_node::TreeNode::transform::transform") , [`transform_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up "method datafusion_common::tree_node::TreeNode::transform_up::transform_up") |
| combined with separate `f_down` and `f_up` closures |  | [`transform_down_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down_up "method datafusion_common::tree_node::TreeNode::transform_down_up::transform_down_up") |
| combined with `f_down()` and `f_up()` in an object | [`visit`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.visit "method datafusion_common::tree_node::TreeNode::visit::visit") | [`rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite "method datafusion_common::tree_node::TreeNode::rewrite::rewrite") |

**Note**:while there is currently no in-place mutation API that uses `&mut TreeNode`, the transforming APIs are efficient and optimized to avoid cloning.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#terminology" class="doc-anchor">§</a>Terminology

The following terms are used in this trait

- `f_down`: Invoked before any children of the current node are visited.
- `f_up`: Invoked after all children of the current node are visited.
- `f`: closure that is applied to the current node.
- `map_*`: applies a transformation to rewrite owned nodes
- `apply_*`: invokes a function on borrowed nodes
- `transform_`: applies a transformation to rewrite owned nodes

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.apply_children" class="fn">apply_children</a>\<'n, F\>( &'n self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&'n Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Low-level API used to implement other APIs.

If you want to implement the [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") trait for your own type, you should implement this method and [`Self::map_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.map_children "method datafusion_common::tree_node::TreeNode::map_children::map_children").

Users should use one of the higher level APIs described on [`Self`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode").

Description: Apply `f` to inspect node’s children (but not the node itself).

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.map_children" class="fn">map_children</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Low-level API used to implement other APIs.

If you want to implement the [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") trait for your own type, you should implement this method and [`Self::apply_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.apply_children "method datafusion_common::tree_node::TreeNode::apply_children::apply_children").

Users should use one of the higher level APIs described on [`Self`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode").

Description: Apply `f` to rewrite the node’s children (but not the node itself).

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.visit" class="fn">visit</a>\<'n, V\>( &'n self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeVisitor">TreeNodeVisitor</a>\<'n, Node = Self\>,

Visit the tree node with a [`TreeNodeVisitor`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html "trait datafusion::common::tree_node::TreeNodeVisitor"), performing a depth-first walk of the node and its children.

[`TreeNodeVisitor::f_down()`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#method.f_down "method datafusion::common::tree_node::TreeNodeVisitor::f_down") is called in top-down order (before children are visited), [`TreeNodeVisitor::f_up()`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#method.f_up "method datafusion::common::tree_node::TreeNodeVisitor::f_up") is called in bottom-up order (after children are visited).

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#return-value" class="doc-anchor">§</a>Return Value

Specifies how the tree walk ended. See [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") for details.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#see-also" class="doc-anchor">§</a>See Also:

- [`Self::apply`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.apply "method datafusion_common::tree_node::TreeNode::apply::apply") for inspecting nodes with a closure
- [`Self::rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite "method datafusion_common::tree_node::TreeNode::rewrite::rewrite") to rewrite owned `TreeNode`s

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#example" class="doc-anchor">§</a>Example

Consider the following tree structure:

``` text
ParentNode
   left: ChildNode1
   right: ChildNode2
```

Here, the nodes would be visited using the following order:

``` text
TreeNodeVisitor::f_down(ParentNode)
TreeNodeVisitor::f_down(ChildNode1)
TreeNodeVisitor::f_up(ChildNode1)
TreeNodeVisitor::f_down(ChildNode2)
TreeNodeVisitor::f_up(ChildNode2)
TreeNodeVisitor::f_up(ParentNode)
```

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite" class="fn">rewrite</a>\<R\>( self, rewriter: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut R</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where R: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a>\<Node = Self\>,

Rewrite the tree node with a [`TreeNodeRewriter`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html "trait datafusion::common::tree_node::TreeNodeRewriter"), performing a depth-first walk of the node and its children.

[`TreeNodeRewriter::f_down()`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_down "method datafusion::common::tree_node::TreeNodeRewriter::f_down") is called in top-down order (before children are visited), [`TreeNodeRewriter::f_up()`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_up "method datafusion::common::tree_node::TreeNodeRewriter::f_up") is called in bottom-up order (after children are visited).

Note: If using the default [`TreeNodeRewriter::f_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_up "method datafusion::common::tree_node::TreeNodeRewriter::f_up") or [`TreeNodeRewriter::f_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_down "method datafusion::common::tree_node::TreeNodeRewriter::f_down") that do nothing, consider using [`Self::transform_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down "method datafusion_common::tree_node::TreeNode::transform_down::transform_down") instead.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#return-value-1" class="doc-anchor">§</a>Return Value

The returns value specifies how the tree walk should proceed. See [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") for details. If an [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, the recursion stops immediately.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#see-also-1" class="doc-anchor">§</a>See Also

- [`Self::visit`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.visit "method datafusion_common::tree_node::TreeNode::visit::visit") for inspecting (without modification) `TreeNode`s
- [Self::transform_down_up](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down_up "method datafusion_common::tree_node::TreeNode::transform_down_up::transform_down_up") for a top-down (pre-order) traversal.
- [Self::transform_down](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down "method datafusion_common::tree_node::TreeNode::transform_down::transform_down") for a top-down (pre-order) traversal.
- [`Self::transform_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up "method datafusion_common::tree_node::TreeNode::transform_up::transform_up") for a bottom-up (post-order) traversal.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#example-1" class="doc-anchor">§</a>Example

Consider the following tree structure:

``` text
ParentNode
   left: ChildNode1
   right: ChildNode2
```

Here, the nodes would be visited using the following order:

``` text
TreeNodeRewriter::f_down(ParentNode)
TreeNodeRewriter::f_down(ChildNode1)
TreeNodeRewriter::f_up(ChildNode1)
TreeNodeRewriter::f_down(ChildNode2)
TreeNodeRewriter::f_up(ChildNode2)
TreeNodeRewriter::f_up(ParentNode)
```

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.apply" class="fn">apply</a>\<'n, F\>(&'n self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&'n Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Applies `f` to the node then each of its children, recursively (a top-down, pre-order traversal).

The return [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") controls the recursion and can cause an early return.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#see-also-2" class="doc-anchor">§</a>See Also

- [`Self::transform_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down "method datafusion_common::tree_node::TreeNode::transform_down::transform_down") for the equivalent transformation API.
- [`Self::visit`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.visit "method datafusion_common::tree_node::TreeNode::visit::visit") for both top-down and bottom up traversal.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform" class="fn">transform</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Recursively rewrite the node’s children and then the node using `f` (a bottom-up post-order traversal).

A synonym of [`Self::transform_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up "method datafusion_common::tree_node::TreeNode::transform_up::transform_up").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down" class="fn">transform_down</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Recursively rewrite the tree using `f` in a top-down (pre-order) fashion.

`f` is applied to the node first, and then its children.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#see-also-3" class="doc-anchor">§</a>See Also

- [`Self::transform_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up "method datafusion_common::tree_node::TreeNode::transform_up::transform_up") for a bottom-up (post-order) traversal.
- [Self::transform_down_up](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down_up "method datafusion_common::tree_node::TreeNode::transform_down_up::transform_down_up") for a combined traversal with closures
- [`Self::rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite "method datafusion_common::tree_node::TreeNode::rewrite::rewrite") for a combined traversal with a visitor

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up" class="fn">transform_up</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Recursively rewrite the node using `f` in a bottom-up (post-order) fashion.

`f` is applied to the node’s children first, and then to the node itself.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#see-also-4" class="doc-anchor">§</a>See Also

- [`Self::transform_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down "method datafusion_common::tree_node::TreeNode::transform_down::transform_down") top-down (pre-order) traversal.
- [Self::transform_down_up](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down_up "method datafusion_common::tree_node::TreeNode::transform_down_up::transform_down_up") for a combined traversal with closures
- [`Self::rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite "method datafusion_common::tree_node::TreeNode::rewrite::rewrite") for a combined traversal with a visitor

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down_up" class="fn">transform_down_up</a>\<FD, FU\>( self, f_down: FD, f_up: FU, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where FD: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>, FU: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Transforms the node using `f_down` while traversing the tree top-down (pre-order), and using `f_up` while traversing the tree bottom-up (post-order).

The method behaves the same as calling [`Self::transform_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down "method datafusion_common::tree_node::TreeNode::transform_down::transform_down") followed by [`Self::transform_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up "method datafusion_common::tree_node::TreeNode::transform_up::transform_up") on the same node. Use this method if you want to start the `f_up` process right where `f_down` jumps. This can make the whole process faster by reducing the number of `f_up` steps.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#see-also-5" class="doc-anchor">§</a>See Also

- [`Self::transform_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up "method datafusion_common::tree_node::TreeNode::transform_up::transform_up") for a bottom-up (post-order) traversal.
- [Self::transform_down](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down "method datafusion_common::tree_node::TreeNode::transform_down::transform_down") for a top-down (pre-order) traversal.
- [`Self::rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite "method datafusion_common::tree_node::TreeNode::rewrite::rewrite") for a combined traversal with a visitor

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#example-2" class="doc-anchor">§</a>Example

Consider the following tree structure:

``` text
ParentNode
   left: ChildNode1
   right: ChildNode2
```

The nodes are visited using the following order:

``` text
f_down(ParentNode)
f_down(ChildNode1)
f_up(ChildNode1)
f_down(ChildNode2)
f_up(ChildNode2)
f_up(ParentNode)
```

See [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") for more details on controlling the traversal.

If `f_down` or `f_up` returns [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err"), the recursion stops immediately.

Example:

``` text
                                              |   +---+
                                              |   | J |
                                              |   +---+
                                              |     |
                                              |   +---+
                 TreeNodeRecursion::Continue  |   | I |
                                              |   +---+
                                              |     |
                                              |   +---+
                                             \|/  | F |
                                              '   +---+
                                                 /     \ ___________________
                 When `f_down` is           +---+                           \ ---+
                 applied on node "E",       | E |                            | G |
                 it returns with "Jump".    +---+                            +---+
                                              |                                |
                                            +---+                            +---+
                                            | C |                            | H |
                                            +---+                            +---+
                                            /   \
                                       +---+     +---+
                                       | B |     | D |
                                       +---+     +---+
                                                   |
                                                 +---+
                                                 | A |
                                                 +---+

Instead of starting from leaf nodes, `f_up` starts from the node "E".
                                                  +---+
                                              |   | J |
                                              |   +---+
                                              |     |
                                              |   +---+
                                              |   | I |
                                              |   +---+
                                              |     |
                                             /    +---+
                                           /      | F |
                                         /        +---+
                                       /         /     \ ______________________
                                      |     +---+   .                          \ ---+
                                      |     | E |  /|\  After `f_down` jumps    | G |
                                      |     +---+   |   on node E, `f_up`       +---+
                                       \------| ---/   if applied on node E.      |
                                            +---+                               +---+
                                            | C |                               | H |
                                            +---+                               +---+
                                            /   \
                                       +---+     +---+
                                       | B |     | D |
                                       +---+     +---+
                                                   |
                                                 +---+
                                                 | A |
                                                 +---+
```

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.exists" class="fn">exists</a>\<F\>(&self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Returns true if `f` returns true for any node in the tree.

Stops recursion as soon as a matching node is found

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#impl-TreeNode-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html" class="trait" title="trait datafusion::common::tree_node::TreeNode">TreeNode</a> for <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html" class="trait" title="trait datafusion::common::tree_node::DynTreeNode">DynTreeNode</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Blanket implementation for any `Arc<T>` where `T` implements [`DynTreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html "trait datafusion::common::tree_node::DynTreeNode") (such as [`Arc<dyn PhysicalExpr>`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.apply_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.apply_children" class="fn">apply_children</a>\<'n, F\>( &'n self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&'n <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.map_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.map_children" class="fn">map_children</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#impl-TreeNode-for-LogicalPlan" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html" class="trait" title="trait datafusion::common::tree_node::TreeNode">TreeNode</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#impl-TreeNode-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html" class="trait" title="trait datafusion::common::tree_node::TreeNode">TreeNode</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Implementation of the [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") trait

This allows logical expressions (`Expr`) to be traversed and transformed Facilitates tasks such as optimization and rewriting during query planning.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#impl-TreeNode-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html" class="trait" title="trait datafusion::common::tree_node::TreeNode">TreeNode</a> for T

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html" class="trait" title="trait datafusion::common::tree_node::ConcreteTreeNode">ConcreteTreeNode</a>,
