# Trait EdgeType Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/lib.rs.html#592-594" class="src">Source</a>

``` rust
pub trait EdgeType {
    // Required method
    fn is_directed() -> bool;
}
```

Expand description

A graph’s edge type determines whether it has directed edges or not.

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html#tymethod.is_directed" class="fn">is_directed</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html#impl-EdgeType-for-Directed" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a> for <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>

<a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html#impl-EdgeType-for-Undirected" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a> for <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>
