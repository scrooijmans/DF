# Trait DynTreeNode Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/tree_node.rs.html#1247" class="src">Source</a>

``` rust
pub trait DynTreeNode {
    // Required methods
    fn arc_children(&self) -> Vec<&Arc<Self>>;
    fn with_new_arc_children(
        &self,
        arc_self: Arc<Self>,
        new_children: Vec<Arc<Self>>,
    ) -> Result<Arc<Self>, DataFusionError>;
}
```

Expand description

Helper trait for implementing [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") that have children stored as `Arc`s. If some trait object, such as `dyn T`, implements this trait, its related `Arc<dyn T>` will automatically implement [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode").

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html#tymethod.arc_children" class="fn">arc_children</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self\>\>

Returns all children of the specified `TreeNode`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html#tymethod.with_new_arc_children" class="fn">with_new_arc_children</a>( &self, arc_self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self\>, new_children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Constructs a new node with the specified children.

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html#impl-DynTreeNode-for-dyn+PhysicalExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html" class="trait" title="trait datafusion::common::tree_node::DynTreeNode">DynTreeNode</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html#impl-DynTreeNode-for-dyn+ExecutionPlan" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html" class="trait" title="trait datafusion::common::tree_node::DynTreeNode">DynTreeNode</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>
