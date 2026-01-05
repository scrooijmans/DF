# Struct TarjanScc Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/scc/tarjan_scc.rs.html#15-20" class="src">Source</a>

``` rust
pub struct TarjanScc<N> { /* private fields */ }
```

Expand description

A reusable state for computing the *strongly connected components* using [Tarjan’s algorithm](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm).

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#impl-TarjanScc%3CN%3E" class="anchor">§</a>

### impl\<N\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html" class="struct" title="struct petgraph::algo::scc::tarjan_scc::TarjanScc">TarjanScc</a>\<N\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#method.new" class="fn">new</a>() -\> Self

Creates a new `TarjanScc`

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#method.run" class="fn">run</a>\<G, F\>(&mut self, g: G, f: F)

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a>\<NodeId = N\> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a>\<NodeId = N\> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a>\<NodeId = N\>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[N]</a>), N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Compute the *strongly connected components* using Algorithm 3 in [A Space-Efficient Algorithm for Finding Strongly Connected Components](https://homepages.ecs.vuw.ac.nz/~djp/files/P05.pdf) by David J. Pierce, which is a memory-efficient variation of [Tarjan’s algorithm](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm).

Calls `f` for each strongly connected component (scc). The order of node ids within each scc is arbitrary, but the order of the sccs is their postorder (reverse topological sort).

For an undirected graph, the sccs are simply the connected components.

This implementation is recursive and does one pass over the nodes.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#method.node_component_index" class="fn">node_component_index</a>\<G\>(&self, g: G, v: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a>\<NodeId = N\> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a>\<NodeId = N\>, N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

Returns the index of the component in which v has been assigned. Allows for using self as a lookup table for an scc decomposition produced by self.run().

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#impl-Debug-for-TarjanScc%3CN%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html" class="struct" title="struct petgraph::algo::scc::tarjan_scc::TarjanScc">TarjanScc</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#impl-Default-for-TarjanScc%3CN%3E" class="anchor">§</a>

### impl\<N\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html" class="struct" title="struct petgraph::algo::scc::tarjan_scc::TarjanScc">TarjanScc</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html#blanket-implementations" class="anchor">§</a>
