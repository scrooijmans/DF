# Struct Node Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/mod.rs.html#220-225" class="src">Source</a>

``` rust
pub struct Node<N, Ix = DefaultIx> {
    pub weight: N,
    /* private fields */
}
```

Expand description

The graph’s node type.

## Fields<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#fields" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#structfield.weight" class="anchor field">§</a>`weight: N`

Associated node data.

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#impl-Node%3CN,+Ix%3E" class="anchor">§</a>

### impl\<N, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html" class="struct" title="struct petgraph::graph::Node">Node</a>\<N, Ix\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#method.next_edge" class="fn">next_edge</a>(&self, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>

Accessor for data structure internals: the first edge in the given direction.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#impl-Clone-for-Node%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html" class="struct" title="struct petgraph::graph::Node">Node</a>\<E, Ix\>

where E: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, Ix: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> Self

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#impl-Debug-for-Node%3CN,+Ix%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, Ix: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html" class="struct" title="struct petgraph::graph::Node">Node</a>\<N, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Node.html#blanket-implementations" class="anchor">§</a>
