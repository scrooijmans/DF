# Trait GraphIndex Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/mod.rs.html#2279-2284" class="src">Source</a>

``` rust
pub trait GraphIndex: Copy { }
```

Expand description

A `GraphIndex` is a node or edge index.

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.GraphIndex.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.GraphIndex.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.GraphIndex.html#impl-GraphIndex-for-EdgeIndex%3CIx%3E" class="anchor">§</a>

### impl\<Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.GraphIndex.html" class="trait" title="trait petgraph::graph::GraphIndex">GraphIndex</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.GraphIndex.html#impl-GraphIndex-for-NodeIndex%3CIx%3E" class="anchor">§</a>

### impl\<Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.GraphIndex.html" class="trait" title="trait petgraph::graph::GraphIndex">GraphIndex</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>
