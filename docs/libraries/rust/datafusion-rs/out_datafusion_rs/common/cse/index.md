# Module cse Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#41" class="src">Source</a>

Expand description

Common Subexpression Elimination logic implemented in [`CSE`](https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html "struct datafusion::common::cse::CSE") can be controlled with a [`CSEController`](https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html "trait datafusion::common::cse::CSEController"), that defines how to eliminate common subtrees from a particular [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") tree.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html" class="struct" title="struct datafusion::common::cse::CSE">CSE</a>  
The main entry point of Common Subexpression Elimination.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/enum.FoundCommonNodes.html" class="enum" title="enum datafusion::common::cse::FoundCommonNodes">FoundCommonNodes</a>  
The result of potentially rewriting a list of [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode")s to eliminate common subtrees.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html" class="trait" title="trait datafusion::common::cse::CSEController">CSEController</a>  
The [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") specific definition of elimination.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html" class="trait" title="trait datafusion::common::cse::HashNode">HashNode</a>  
Hashes the direct content of an [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") without recursing into its children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html" class="trait" title="trait datafusion::common::cse::NormalizeEq">NormalizeEq</a>  
The `NormalizeEq` trait extends `Eq` and `Normalizeable` to provide a method for comparing normalized nodes in optimizations like Common Subexpression Elimination (CSE).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.Normalizeable.html" class="trait" title="trait datafusion::common::cse::Normalizeable">Normalizeable</a>  
The `Normalizeable` trait defines a method to determine whether a node can be normalized.
