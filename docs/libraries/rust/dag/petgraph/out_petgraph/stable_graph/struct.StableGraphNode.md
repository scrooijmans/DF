# Struct StableGraphNode Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/stable_graph/mod.rs.html#165-168" class="src">Source</a>

``` rust
pub struct StableGraphNode<N, Ix> {
    pub index: NodeIndex<Ix>,
    pub weight: N,
}
```

## Fields<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html#fields" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html#structfield.index" class="anchor field">§</a>`index: `<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex"><code>NodeIndex</code></a>`<Ix>`<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html#structfield.weight" class="anchor field">§</a>`weight: N`

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html#impl-Clone-for-StableGraphNode%3CN,+Ix%3E" class="anchor">§</a>

### impl\<N, Ix\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html" class="struct" title="struct petgraph::stable_graph::StableGraphNode">StableGraphNode</a>\<N, Ix\>

where N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, Ix: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> Self

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html#impl-Debug-for-StableGraphNode%3CN,+Ix%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, Ix: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html" class="struct" title="struct petgraph::stable_graph::StableGraphNode">StableGraphNode</a>\<N, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html#blanket-implementations" class="anchor">§</a>
