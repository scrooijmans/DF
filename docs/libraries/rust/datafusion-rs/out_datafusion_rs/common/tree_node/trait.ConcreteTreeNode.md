# Trait ConcreteTreeNode Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/tree_node.rs.html#1298" class="src">Source</a>

``` rust
pub trait ConcreteTreeNode: Sized {
    // Required methods
    fn children(&self) -> &[Self];
    fn take_children(self) -> (Self, Vec<Self>);
    fn with_new_children(
        self,
        children: Vec<Self>,
    ) -> Result<Self, DataFusionError>;
}
```

Expand description

Instead of implementing [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode"), it’s recommended to implement a [`ConcreteTreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html "trait datafusion::common::tree_node::ConcreteTreeNode") for trees that contain nodes with payloads. This approach ensures safe execution of algorithms involving payloads, by enforcing rules for detaching and reattaching child nodes.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html#tymethod.children" class="fn">children</a>(&self) -\> &\[Self\]

Provides read-only access to child nodes.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html#tymethod.take_children" class="fn">take_children</a>(self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self\>)

Detaches the node from its children, returning the node itself and its detached children.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html#tymethod.with_new_children" class="fn">with_new_children</a>(self, children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Reattaches updated child nodes to the node, returning the updated node.

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html#impl-ConcreteTreeNode-for-ExprContext%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html" class="trait" title="trait datafusion::common::tree_node::ConcreteTreeNode">ConcreteTreeNode</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/tree_node/struct.ExprContext.html" class="struct" title="struct datafusion::physical_expr_common::tree_node::ExprContext">ExprContext</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html#impl-ConcreteTreeNode-for-PlanContext%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html" class="trait" title="trait datafusion::common::tree_node::ConcreteTreeNode">ConcreteTreeNode</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/tree_node/struct.PlanContext.html" class="struct" title="struct datafusion::physical_plan::tree_node::PlanContext">PlanContext</a>\<T\>
