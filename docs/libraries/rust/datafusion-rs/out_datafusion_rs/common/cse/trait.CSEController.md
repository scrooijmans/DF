# Trait CSEController Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/cse.rs.html#173" class="src">Source</a>

``` rust
pub trait CSEController {
    type Node;

    // Required methods
    fn conditional_children(
        node: &Self::Node,
    ) -> Option<(Vec<&Self::Node>, Vec<&Self::Node>)>;
    fn is_valid(node: &Self::Node) -> bool;
    fn is_ignored(&self, node: &Self::Node) -> bool;
    fn generate_alias(&self) -> String;
    fn rewrite(&mut self, node: &Self::Node, alias: &str) -> Self::Node;

    // Provided methods
    fn rewrite_f_down(&mut self, _node: &Self::Node) { ... }
    fn rewrite_f_up(&mut self, _node: &Self::Node) { ... }
}
```

Expand description

The [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") specific definition of elimination.

## Required Associated Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#associatedtype.Node" class="associatedtype">Node</a>

The type of the tree nodes.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#tymethod.conditional_children" class="fn">conditional_children</a>( node: &Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::cse::CSEController::Node">Node</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::cse::CSEController::Node">Node</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::cse::CSEController::Node">Node</a>\>)\>

Splits the children to normal and conditionally evaluated ones or returns `None` if all are always evaluated.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#tymethod.is_valid" class="fn">is_valid</a>(node: &Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::cse::CSEController::Node">Node</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#tymethod.is_ignored" class="fn">is_ignored</a>(&self, node: &Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::cse::CSEController::Node">Node</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#tymethod.generate_alias" class="fn">generate_alias</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#tymethod.rewrite" class="fn">rewrite</a>(&mut self, node: &Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::cse::CSEController::Node">Node</a>, alias: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::cse::CSEController::Node">Node</a>

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#method.rewrite_f_down" class="fn">rewrite_f_down</a>(&mut self, \_node: &Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::cse::CSEController::Node">Node</a>)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#method.rewrite_f_up" class="fn">rewrite_f_up</a>(&mut self, \_node: &Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::cse::CSEController::Node">Node</a>)

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html#implementors" class="anchor">§</a>
