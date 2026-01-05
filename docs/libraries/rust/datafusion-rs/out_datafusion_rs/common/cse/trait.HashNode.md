# Trait HashNode Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/cse.rs.html#43" class="src">Source</a>

``` rust
pub trait HashNode {
    // Required method
    fn hash_node<H>(&self, state: &mut H)
       where H: Hasher;
}
```

Expand description

Hashes the direct content of an [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") without recursing into its children.

This method is useful to incrementally compute hashes, such as in [`CSE`](https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html "struct datafusion::common::cse::CSE") which builds a deep hash of a node and its descendants during the bottom-up phase of the first traversal and so avoid computing the hash of the node and then the hash of its descendants separately.

If a node doesn’t have any children then the value returned by `hash_node()` is similar to ’.hash()\`, but not necessarily returns the same value.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html#tymethod.hash_node" class="fn">hash_node</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html#impl-HashNode-for-Arc%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html" class="trait" title="trait datafusion::common::cse::HashNode">HashNode</a> for <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html" class="trait" title="trait datafusion::common::cse::HashNode">HashNode</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html#method.hash_node" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html#tymethod.hash_node" class="fn">hash_node</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html#impl-HashNode-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html" class="trait" title="trait datafusion::common::cse::HashNode">HashNode</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>
