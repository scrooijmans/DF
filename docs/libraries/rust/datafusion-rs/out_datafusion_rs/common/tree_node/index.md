# Module tree_node Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#59" class="src">Source</a>

Expand description

[`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") for visiting and rewriting expression and plan trees

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>  
Result of tree walk / transformation APIs

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>  
Controls how [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") recursions should proceed.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html" class="trait" title="trait datafusion::common::tree_node::ConcreteTreeNode">ConcreteTreeNode</a>  
Instead of implementing [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode"), it’s recommended to implement a [`ConcreteTreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.ConcreteTreeNode.html "trait datafusion::common::tree_node::ConcreteTreeNode") for trees that contain nodes with payloads. This approach ensures safe execution of algorithms involving payloads, by enforcing rules for detaching and reattaching child nodes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html" class="trait" title="trait datafusion::common::tree_node::DynTreeNode">DynTreeNode</a>  
Helper trait for implementing [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") that have children stored as `Arc`s. If some trait object, such as `dyn T`, implements this trait, its related `Arc<dyn T>` will automatically implement [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TransformedResult.html" class="trait" title="trait datafusion::common::tree_node::TransformedResult">TransformedResult</a>  
Transformation helper to access [`Transformed`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html "struct datafusion::common::tree_node::Transformed") fields in a [`Result`](https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html "type datafusion::error::Result") easily.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html" class="trait" title="trait datafusion::common::tree_node::TreeNode">TreeNode</a>  
API for inspecting and rewriting tree data structures.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeContainer">TreeNodeContainer</a>  
[`TreeNodeContainer`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html "trait datafusion::common::tree_node::TreeNodeContainer") contains elements that a function can be applied on or mapped. The elements of the container are siblings so the continuation rules are similar to [`TreeNodeRecursion::visit_sibling`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#method.visit_sibling "method datafusion::common::tree_node::TreeNodeRecursion::visit_sibling") / [`Transformed::transform_sibling`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.transform_sibling "method datafusion::common::tree_node::Transformed::transform_sibling").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeIterator.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeIterator">TreeNodeIterator</a>  
Transformation helper to process a sequence of iterable tree nodes that are siblings.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRefContainer.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRefContainer">TreeNodeRefContainer</a>  
[`TreeNodeRefContainer`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRefContainer.html "trait datafusion::common::tree_node::TreeNodeRefContainer") contains references to elements that a function can be applied on. The elements of the container are siblings so the continuation rules are similar to [`TreeNodeRecursion::visit_sibling`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#method.visit_sibling "method datafusion::common::tree_node::TreeNodeRecursion::visit_sibling").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a>  
A [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) for recursively rewriting [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode")s via [`TreeNode::rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite "method datafusion::common::tree_node::TreeNode::rewrite").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeVisitor">TreeNodeVisitor</a>  
A [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) for recursively inspecting [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode")s via [`TreeNode::visit`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.visit "method datafusion::common::tree_node::TreeNode::visit").
