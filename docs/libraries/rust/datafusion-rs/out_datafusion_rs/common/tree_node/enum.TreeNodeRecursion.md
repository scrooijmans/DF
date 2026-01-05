# Enum TreeNodeRecursion Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/tree_node.rs.html#517" class="src">Source</a>

``` rust
pub enum TreeNodeRecursion {
    Continue,
    Jump,
    Stop,
}
```

Expand description

Controls how [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") recursions should proceed.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#variant.Continue" class="anchor">§</a>

### Continue

Continue recursion with the next node.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#variant.Jump" class="anchor">§</a>

### Jump

In top-down traversals, skip recursing into children but continue with the next node, which actually means pruning of the subtree.

In bottom-up traversals, bypass calling bottom-up closures till the next leaf node.

In combined traversals, if it is the `f_down` (pre-order) phase, execution “jumps” to the next `f_up` (post-order) phase by shortcutting its children. If it is the `f_up` (post-order) phase, execution “jumps” to the next `f_down` (pre-order) phase by shortcutting its parent nodes until the first parent node having unvisited children path.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#variant.Stop" class="anchor">§</a>

### Stop

Stop recursion.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#impl-TreeNodeRecursion" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#method.visit_children" class="fn">visit_children</a>\<F\>( self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>() -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Continues visiting nodes with `f` depending on the current [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") value and the fact that `f` is visiting the current node’s children.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#method.visit_sibling" class="fn">visit_sibling</a>\<F\>( self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>() -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Continues visiting nodes with `f` depending on the current [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") value and the fact that `f` is visiting the current node’s sibling.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#method.visit_parent" class="fn">visit_parent</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>() -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Continues visiting nodes with `f` depending on the current [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") value and the fact that `f` is visiting the current node’s parent.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#impl-Clone-for-TreeNodeRecursion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#impl-Debug-for-TreeNodeRecursion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#impl-PartialEq-for-TreeNodeRecursion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#impl-Copy-for-TreeNodeRecursion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#impl-StructuralPartialEq-for-TreeNodeRecursion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#blanket-implementations" class="anchor">§</a>
