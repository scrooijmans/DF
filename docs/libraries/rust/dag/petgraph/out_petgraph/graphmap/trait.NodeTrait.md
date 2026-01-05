# Trait NodeTrait Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graphmap.rs.html#99" class="src">Source</a>

``` rust
pub trait NodeTrait:
    Copy
    + Ord
    + Hash { }
```

Expand description

A trait group for `GraphMap`’s node identifier.

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html#impl-NodeTrait-for-N" class="anchor">§</a>

### impl\<N\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a> for N

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a>,
