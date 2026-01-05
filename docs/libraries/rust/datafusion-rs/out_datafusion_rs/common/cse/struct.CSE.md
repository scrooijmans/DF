# Struct CSE Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/cse.rs.html#514" class="src">Source</a>

``` rust
pub struct CSE<N, C>where
    C: CSEController<Node = N>,{ /* private fields */ }
```

Expand description

The main entry point of Common Subexpression Elimination.

[`CSE`](https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html "struct datafusion::common::cse::CSE") requires a [`CSEController`](https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html "trait datafusion::common::cse::CSEController"), that defines how common subtrees of a particular [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") tree can be eliminated. The elimination process can be started with the [`CSE::extract_common_nodes()`](https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html#method.extract_common_nodes "method datafusion::common::cse::CSE::extract_common_nodes") method.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html#impl-CSE%3CN,+C%3E" class="anchor">§</a>

### impl\<N, C\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html" class="struct" title="struct datafusion::common::cse::CSE">CSE</a>\<N, C\>

where N: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html" class="trait" title="trait datafusion::common::tree_node::TreeNode">TreeNode</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html" class="trait" title="trait datafusion::common::cse::HashNode">HashNode</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html" class="trait" title="trait datafusion::common::cse::NormalizeEq">NormalizeEq</a>, C: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html" class="trait" title="trait datafusion::common::cse::CSEController">CSEController</a>\<Node = N\>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html#method.new" class="fn">new</a>(controller: C) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html" class="struct" title="struct datafusion::common::cse::CSE">CSE</a>\<N, C\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html#method.extract_common_nodes" class="fn">extract_common_nodes</a>( &mut self, nodes_list: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<N\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/enum.FoundCommonNodes.html" class="enum" title="enum datafusion::common::cse::FoundCommonNodes">FoundCommonNodes</a>\<N\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Extracts common [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode")s and rewrites `nodes_list`.

Returns [`FoundCommonNodes`](https://docs.rs/datafusion/50.2.0/datafusion/common/cse/enum.FoundCommonNodes.html "enum datafusion::common::cse::FoundCommonNodes") recording the result of the extraction.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html#blanket-implementations" class="anchor">§</a>
