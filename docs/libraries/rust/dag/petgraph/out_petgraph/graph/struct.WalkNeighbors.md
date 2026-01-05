# Struct WalkNeighbors Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/mod.rs.html#2347-2350" class="src">Source</a>

``` rust
pub struct WalkNeighbors<Ix> { /* private fields */ }
```

Expand description

A “walker” object that can be used to step through the edge list of a node.

Created with [`.detach()`](https://docs.rs/petgraph/latest/petgraph/graph/struct.Neighbors.html#method.detach).

The walker does not borrow from the graph, so it lets you step through neighbors or incident edges while also mutating graph weights, as in the following example:

``` rust
use petgraph::{Graph, Incoming};
use petgraph::visit::Dfs;

let mut gr = Graph::new();
let a = gr.add_node(0.);
let b = gr.add_node(0.);
let c = gr.add_node(0.);
gr.add_edge(a, b, 3.);
gr.add_edge(b, c, 2.);
gr.add_edge(c, b, 1.);

// step through the graph and sum incoming edges into the node weight
let mut dfs = Dfs::new(&gr, a);
while let Some(node) = dfs.next(&gr) {
    // use a detached neighbors walker
    let mut edges = gr.neighbors_directed(node, Incoming).detach();
    while let Some(edge) = edges.next_edge(&gr) {
        gr[node] += gr[edge];
    }
}

// check the result
assert_eq!(gr[a], 0.);
assert_eq!(gr[b], 4.);
assert_eq!(gr[c], 2.);
```

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html#impl-WalkNeighbors%3CIx%3E" class="anchor">§</a>

### impl\<Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html" class="struct" title="struct petgraph::graph::WalkNeighbors">WalkNeighbors</a>\<Ix\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html#method.next" class="fn">next</a>\<N, E, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>\>( &mut self, g: &<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>)\>

Step to the next edge and its endpoint node in the walk for graph `g`.

The next node indices are always the others than the starting point where the `WalkNeighbors` value was created. For an `Outgoing` walk, the target nodes, for an `Incoming` walk, the source nodes of the edge.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html#method.next_node" class="fn">next_node</a>\<N, E, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>\>( &mut self, g: &<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html#method.next_edge" class="fn">next_edge</a>\<N, E, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>\>( &mut self, g: &<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>\>

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html#impl-Clone-for-WalkNeighbors%3CIx%3E" class="anchor">§</a>

### impl\<Ix\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html" class="struct" title="struct petgraph::graph::WalkNeighbors">WalkNeighbors</a>\<Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> Self

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.WalkNeighbors.html#blanket-implementations" class="anchor">§</a>
