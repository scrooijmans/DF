# Trait GraphRef Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/mod.rs.html#106" class="src">Source</a>

``` rust
pub trait GraphRef: Copy + GraphBase { }
```

Expand description

A copyable reference to a graph.

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html#impl-GraphRef-for-%26G" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;G</a>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>,

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html#impl-GraphRef-for-Reversed%3CG%3E" class="anchor">§</a>

### impl\<G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Reversed.html" class="struct" title="struct petgraph::visit::Reversed">Reversed</a>\<G\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html#impl-GraphRef-for-UndirectedAdaptor%3CG%3E" class="anchor">§</a>

### impl\<G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.UndirectedAdaptor.html" class="struct" title="struct petgraph::visit::UndirectedAdaptor">UndirectedAdaptor</a>\<G\>
