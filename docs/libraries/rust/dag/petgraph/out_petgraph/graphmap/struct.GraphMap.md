# Struct GraphMap Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graphmap.rs.html#76-88" class="src">Source</a>

``` rust
pub struct GraphMap<N, E, Ty, S = RandomState>where
    S: BuildHasher,{ /* private fields */ }
```

Expand description

`GraphMap<N, E, Ty>` is a graph datastructure using an associative array of its node weights `N`.

It uses an combined adjacency list and sparse adjacency matrix representation, using **O(\|V\| + \|E\|)** space where V is the set of nodes and E is the set of edges, and allows testing for edge existence in constant time.

`GraphMap` is parameterized over:

- Associated data `N` for nodes and `E` for edges, called *weights*.
- The node weight `N` must implement `Copy` and will be used as node identifier, duplicated into several places in the data structure. It must be suitable as a hash table key (implementing `Eq + Hash`). The node type must also implement `Ord` so that the implementation can order the pair (`a`, `b`) for an edge connecting any two nodes `a` and `b`.
- `E` can be of arbitrary type.
- Edge type `Ty` that determines whether the graph edges are directed or undirected.

You can use the type aliases `UnGraphMap` and `DiGraphMap` for convenience.

`GraphMap` does not allow parallel edges, but self loops are allowed.

Depends on crate feature `graphmap` (default).

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.new" class="fn">new</a>() -\> Self

where S: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Create a new `GraphMap`

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.with_capacity" class="fn">with_capacity</a>(nodes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, edges: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

where S: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Create a new `GraphMap` with estimated capacity.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.with_capacity_and_hasher" class="fn">with_capacity_and_hasher</a>(nodes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, edges: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, hasher: S) -\> Self

where S: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Create a new `GraphMap` with estimated capacity, and specified hasher.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-GraphMap%3CN,+E,+Ty,+S%3E-1" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.capacity" class="fn">capacity</a>(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Return the current node and edge capacity of the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.is_directed" class="fn">is_directed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether the graph has directed edges.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.from_edges" class="fn">from_edges</a>\<I\>(iterable: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, I::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>: <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E, NodeId = N\>, S: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Create a new `GraphMap` from an iterable of edges.

Node values are taken directly from the list. Edge weights `E` may either be specified in the list, or they are filled with default values.

Nodes are inserted automatically to match the edges.

``` rust
use petgraph::graphmap::UnGraphMap;

// Create a new undirected GraphMap.
// Use a type hint to have `()` be the edge weight type.
let gr = UnGraphMap::<_, ()>::from_edges(&[
    (0, 1), (0, 2), (0, 3),
    (1, 2), (1, 3),
    (2, 3),
]);
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.node_count" class="fn">node_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of nodes in the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.edge_count" class="fn">edge_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of edges in the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.clear" class="fn">clear</a>(&mut self)

Remove all nodes and edges

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.add_node" class="fn">add_node</a>(&mut self, n: N) -\> N

Add node `n` to the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.remove_node" class="fn">remove_node</a>(&mut self, n: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Remove node `n` from the graph.

Return `true` if it did exist.

Computes in **O(V)** time, due to the removal of edges with other nodes.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.contains_node" class="fn">contains_node</a>(&self, n: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return `true` if the node is contained in the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.add_edge" class="fn">add_edge</a>(&mut self, a: N, b: N, weight: E) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<E\>

Add an edge connecting `a` and `b` to the graph, with associated data `weight`. For a directed graph, the edge is directed from `a` to `b`.

Inserts nodes `a` and/or `b` if they aren’t already part of the graph.

Return `None` if the edge did not previously exist, otherwise, the associated data is updated and the old value is returned as `Some(old_weight)`.

``` rust
// Create a GraphMap with directed edges, and add one edge to it
use petgraph::graphmap::DiGraphMap;

let mut g = DiGraphMap::<_, _>::new();
g.add_edge("x", "y", -1);
assert_eq!(g.node_count(), 2);
assert_eq!(g.edge_count(), 1);
assert!(g.contains_edge("x", "y"));
assert!(!g.contains_edge("y", "x"));
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.remove_edge" class="fn">remove_edge</a>(&mut self, a: N, b: N) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<E\>

Remove edge from `a` to `b` from the graph and return the edge weight.

Return `None` if the edge didn’t exist.

``` rust
// Create a GraphMap with undirected edges, and add and remove an edge.
use petgraph::graphmap::UnGraphMap;

let mut g = UnGraphMap::<_, _>::new();
g.add_edge("x", "y", -1);

let edge_data = g.remove_edge("y", "x");
assert_eq!(edge_data, Some(-1));
assert_eq!(g.edge_count(), 0);
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.contains_edge" class="fn">contains_edge</a>(&self, a: N, b: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return `true` if the edge connecting `a` with `b` is contained in the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.nodes" class="fn">nodes</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Nodes.html" class="struct" title="struct petgraph::graphmap::Nodes">Nodes</a>\<'\_, N\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#" class="tooltip" data-notable-ty="Nodes&lt;&#39;_, N&gt;">ⓘ</a>

Return an iterator over the nodes of the graph.

Iterator element type is `N`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.par_nodes" class="fn">par_nodes</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParNodes.html" class="struct" title="struct petgraph::graphmap::ParNodes">ParNodes</a>\<'\_, N\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

Return a parallel iterator over the nodes of the graph.

Iterator element type is `N`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.neighbors" class="fn">neighbors</a>(&self, a: N) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Neighbors.html" class="struct" title="struct petgraph::graphmap::Neighbors">Neighbors</a>\<'\_, N, Ty\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#" class="tooltip" data-notable-ty="Neighbors&lt;&#39;_, N, Ty&gt;">ⓘ</a>

Return an iterator of all nodes with an edge starting from `a`.

- `Directed`: Outgoing edges from `a`.
- `Undirected`: All edges from or to `a`.

Produces an empty iterator if the node doesn’t exist.  
Iterator element type is `N`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.neighbors_directed" class="fn">neighbors_directed</a>( &self, a: N, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.NeighborsDirected.html" class="struct" title="struct petgraph::graphmap::NeighborsDirected">NeighborsDirected</a>\<'\_, N, Ty\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#" class="tooltip" data-notable-ty="NeighborsDirected&lt;&#39;_, N, Ty&gt;">ⓘ</a>

Return an iterator of all neighbors that have an edge between them and `a`, in the specified direction. If the graph’s edges are undirected, this is equivalent to *.neighbors(a)*.

- `Directed`, `Outgoing`: All edges from `a`.
- `Directed`, `Incoming`: All edges to `a`.
- `Undirected`: All edges from or to `a`.

Produces an empty iterator if the node doesn’t exist.  
Iterator element type is `N`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.edges" class="fn">edges</a>(&self, a: N) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Edges.html" class="struct" title="struct petgraph::graphmap::Edges">Edges</a>\<'\_, N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#" class="tooltip" data-notable-ty="Edges&lt;&#39;_, N, E, Ty, S&gt;">ⓘ</a>

Return an iterator of target nodes with an edge starting from `a`, paired with their respective edge weights.

- `Directed`: Outgoing edges from `a`.
- `Undirected`: All edges from or to `a`.

Produces an empty iterator if the node doesn’t exist.  
Iterator element type is `(N, N, &E)`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.edges_directed" class="fn">edges_directed</a>( &self, a: N, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.EdgesDirected.html" class="struct" title="struct petgraph::graphmap::EdgesDirected">EdgesDirected</a>\<'\_, N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#" class="tooltip" data-notable-ty="EdgesDirected&lt;&#39;_, N, E, Ty, S&gt;">ⓘ</a>

Return an iterator of target nodes with an edge starting from `a`, paired with their respective edge weights.

- `Directed`, `Outgoing`: All edges from `a`.
- `Directed`, `Incoming`: All edges to `a`.
- `Undirected`, `Outgoing`: All edges connected to `a`, with `a` being the source of each edge.
- `Undirected`, `Incoming`: All edges connected to `a`, with `a` being the target of each edge.

Produces an empty iterator if the node doesn’t exist.  
Iterator element type is `(N, N, &E)`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.edge_weight" class="fn">edge_weight</a>(&self, a: N, b: N) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>\>

Return a reference to the edge weight connecting `a` with `b`, or `None` if the edge does not exist in the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.edge_weight_mut" class="fn">edge_weight_mut</a>(&mut self, a: N, b: N) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>\>

Return a mutable reference to the edge weight connecting `a` with `b`, or `None` if the edge does not exist in the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.all_edges" class="fn">all_edges</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.AllEdges.html" class="struct" title="struct petgraph::graphmap::AllEdges">AllEdges</a>\<'\_, N, E, Ty\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#" class="tooltip" data-notable-ty="AllEdges&lt;&#39;_, N, E, Ty&gt;">ⓘ</a>

Return an iterator over all edges of the graph with their weight in arbitrary order.

Iterator element type is `(N, N, &E)`

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.all_edges_mut" class="fn">all_edges_mut</a>(&mut self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.AllEdgesMut.html" class="struct" title="struct petgraph::graphmap::AllEdgesMut">AllEdgesMut</a>\<'\_, N, E, Ty\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#" class="tooltip" data-notable-ty="AllEdgesMut&lt;&#39;_, N, E, Ty&gt;">ⓘ</a>

Return an iterator over all edges of the graph in arbitrary order, with a mutable reference to their weight.

Iterator element type is `(N, N, &mut E)`

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.par_all_edges" class="fn">par_all_edges</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdges.html" class="struct" title="struct petgraph::graphmap::ParAllEdges">ParAllEdges</a>\<'\_, N, E, Ty\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, E: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

Return a parallel iterator over all edges of the graph with their weight in arbitrary order.

Iterator element type is `(N, N, &E)`

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.par_all_edges_mut" class="fn">par_all_edges_mut</a>(&mut self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html" class="struct" title="struct petgraph::graphmap::ParAllEdgesMut">ParAllEdgesMut</a>\<'\_, N, E, Ty\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, E: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Return a parallel iterator over all edges of the graph in arbitrary order, with a mutable reference to their weight.

Iterator element type is `(N, N, &mut E)`

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.into_graph" class="fn">into_graph</a>\<Ix\>(self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Return a `Graph` that corresponds to this `GraphMap`.

1.  Note that node and edge indices in the `Graph` have nothing in common with the `GraphMap`s node weights `N`. The node weights `N` are used as node weights in the resulting `Graph`, too.
2.  Note that the index type is user-chosen.

Computes in **O(\|V\| + \|E\|)** time (average) where V is the set of nodes and E is the set of edges.

**Panics** if the number of nodes or edges does not fit with the resulting graph’s index type.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.from_graph" class="fn">from_graph</a>\<Ix\>(graph: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>) -\> Self

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>, E: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, S: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Creates a `GraphMap` that corresponds to the given `Graph`.

**Warning**: Nodes with the same weight are merged and only the last parallel edge is kept. Node and edge indices of the `Graph` are lost. Only use this function if the node weights are distinct and there are no parallel edges.

Computes in **O(\|V\| + \|E\|)** time (average).

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Arbitrary-for-GraphMap%3CN,+E,+Ty%3E" class="anchor">§</a>

### impl\<N, E, Ty\> <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Arbitrary.html" class="trait" title="trait quickcheck::arbitrary::Arbitrary">Arbitrary</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a> + <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Arbitrary.html" class="trait" title="trait quickcheck::arbitrary::Arbitrary">Arbitrary</a>, E: <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Arbitrary.html" class="trait" title="trait quickcheck::arbitrary::Arbitrary">Arbitrary</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static,

Available on **crate feature `graphmap`** only.

`Arbitrary` for `GraphMap` creates a graph by selecting a node count and a probability for each possible edge to exist.

The result will be simple graph or digraph, self loops possible, no parallel edges.

The exact properties of the produced graph is subject to change.

Requires crate features `"quickcheck"` and `"graphmap"`

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.arbitrary" class="anchor">§</a>

#### fn <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Arbitrary.html#tymethod.arbitrary" class="fn">arbitrary</a>\<G: <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Gen.html" class="trait" title="trait quickcheck::arbitrary::Gen">Gen</a>\>(g: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut G</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.shrink" class="anchor">§</a>

#### fn <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Arbitrary.html#method.shrink" class="fn">shrink</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = Self\>\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Build-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html" class="trait" title="trait petgraph::data::Build">Build</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

Available on **crate feature `graphmap`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.add_node-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#tymethod.add_node" class="fn">add_node</a>(&mut self, weight: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.add_edge-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#method.add_edge" class="fn">add_edge</a>( &mut self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, b: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, weight: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>\>

Add a new edge. If parallel edges (duplicate) are not allowed and the edge already exists, return `None`. [Read more](https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#method.add_edge)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.update_edge" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#tymethod.update_edge" class="fn">update_edge</a>( &mut self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, b: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, weight: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>, ) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>

Add or update the edge from `a` to `b`. Return the id of the affected edge. [Read more](https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#tymethod.update_edge)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Clone-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, E: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, Ty: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, S\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Create-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html" class="trait" title="trait petgraph::data::Create">Create</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Available on **crate feature `graphmap`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.with_capacity-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html#tymethod.with_capacity" class="fn">with_capacity</a>(nodes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, edges: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Data-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.NodeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype">NodeWeight</a> = N

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.EdgeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype">EdgeWeight</a> = E

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-DataMap-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMap.html" class="trait" title="trait petgraph::data::DataMap">DataMap</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.edge_weight-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMap.html#tymethod.edge_weight" class="fn">edge_weight</a>(&self, id: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.node_weight" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMap.html#tymethod.node_weight" class="fn">node_weight</a>(&self, id: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Debug-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, E: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Default-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Create a new empty `GraphMap`.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Deserialize%3C&#39;de%3E-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<'de, N, E, Ty, S\> <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a> + <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\>, E: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Available on **crate feature `serde-1`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>(deserializer: D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, D::<a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>,

Deserializes into a new `GraphMap` from the same format as the standard `Graph`. Needs feature `serde-1`.

**Warning**: When deserializing a graph that was not originally a `GraphMap`, the restrictions from [`from_graph`](https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.from_graph) apply.

Note: The edge weights have to be `Clone` for this to work.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-EdgeCount-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeCount.html" class="trait" title="trait petgraph::visit::EdgeCount">EdgeCount</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.edge_count-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeCount.html#tymethod.edge_count" class="fn">edge_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of edges in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-EdgeIndexable-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.edge_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.edge_bound" class="fn">edge_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return an upper bound of the edge indices in the graph (suitable for the size of a bitmap).

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.to_index-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.to_index" class="fn">to_index</a>(&self, ix: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert `a` to an integer index.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.from_index-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.from_index" class="fn">from_index</a>(&self, ix: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>

Convert `i` to an edge index. `i` must be a valid value in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Extend%3CItem%3E-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, Item, S\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<Item\> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where Item: <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E, NodeId = N\>, N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

Extend the graph from an iterable of edges.

Nodes are inserted automatically to match the edges.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<I\>(&mut self, iterable: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = Item\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-FromElements-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html" class="trait" title="trait petgraph::data::FromElements">FromElements</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Available on **crate feature `graphmap`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.from_elements" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html#method.from_elements" class="fn">from_elements</a>\<I\>(iterable: I) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>, Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>\>\>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-FromGraph6-for-GraphMap%3CIx,+(),+Undirected,+S%3E" class="anchor">§</a>

### impl\<Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.FromGraph6.html" class="trait" title="trait petgraph::graph6::FromGraph6">FromGraph6</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<Ix, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>, S\>

Available on **crate feature `graphmap`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.from_graph6_string" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.FromGraph6.html#tymethod.from_graph6_string" class="fn">from_graph6_string</a>(graph6_string: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-FromIterator%3CItem%3E-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, Item, S\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<Item\> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where Item: <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E, NodeId = N\>, N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Create a new `GraphMap` from an iterable of edges.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iterable: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = Item\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-GetAdjacencyMatrix-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

The `GraphMap` keeps an adjacency matrix internally.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.AdjMatrix" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

The associated adjacency matrix type

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.adjacency_matrix" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.adjacency_matrix" class="fn">adjacency_matrix</a>(&self)

Create the adjacency matrix

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.is_adjacent" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.is_adjacent" class="fn">is_adjacent</a>(&self, \_: &<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, a: N, b: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if there is an edge from `a` to `b`, false otherwise. [Read more](https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.is_adjacent)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-GraphBase-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.NodeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = N

node identifier

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.EdgeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(N, N)</a>

edge identifier

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-GraphProp-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.EdgeType" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#associatedtype.EdgeType" class="associatedtype">EdgeType</a> = Ty

The kind of edges in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.is_directed-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#method.is_directed" class="fn">is_directed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Index%3C(N,+N)%3E-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(N, N)</a>\> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

Index `GraphMap` by node pairs to access edge weights.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = E

The returned type after indexing.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.index" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(N, N)</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-IndexMut%3C(N,+N)%3E-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html" class="trait" title="trait core::ops::index::IndexMut">IndexMut</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(N, N)</a>\> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

Index `GraphMap` by node pairs to access edge weights.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.index_mut" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut" class="fn">index_mut</a>(&mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(N, N)</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-IntoEdgeReferences-for-%26GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<'a, N, E: 'a, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a> + 'a, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.EdgeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype">EdgeRef</a> = (N, N, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a E</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.EdgeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype">EdgeReferences</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.AllEdges.html" class="struct" title="struct petgraph::graphmap::AllEdges">AllEdges</a>\<'a, N, E, Ty\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.edge_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#tymethod.edge_references" class="fn">edge_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeReferences">EdgeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-IntoEdges-for-%26GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<'a, N, E: 'a, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html" class="trait" title="trait petgraph::visit::IntoEdges">IntoEdges</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a> + 'a, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.Edges" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype">Edges</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Edges.html" class="struct" title="struct petgraph::graphmap::Edges">Edges</a>\<'a, N, E, Ty, S\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.edges-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#tymethod.edges" class="fn">edges</a>(self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype" title="type petgraph::visit::IntoEdges::Edges">Edges</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-IntoEdgesDirected-for-%26GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<'a, N, E: 'a, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html" class="trait" title="trait petgraph::visit::IntoEdgesDirected">IntoEdgesDirected</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a> + 'a, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.EdgesDirected" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#associatedtype.EdgesDirected" class="associatedtype">EdgesDirected</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.EdgesDirected.html" class="struct" title="struct petgraph::graphmap::EdgesDirected">EdgesDirected</a>\<'a, N, E, Ty, S\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.edges_directed-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#tymethod.edges_directed" class="fn">edges_directed</a>(self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#associatedtype.EdgesDirected" class="associatedtype" title="type petgraph::visit::IntoEdgesDirected::EdgesDirected">EdgesDirected</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-IntoNeighbors-for-%26GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<'a, N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + 'a, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.Neighbors" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype">Neighbors</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Neighbors.html" class="struct" title="struct petgraph::graphmap::Neighbors">Neighbors</a>\<'a, N, Ty\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.neighbors-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#tymethod.neighbors" class="fn">neighbors</a>(self, n: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype" title="type petgraph::visit::IntoNeighbors::Neighbors">Neighbors</a>

Return an iterator of the neighbors of node `a`.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-IntoNeighborsDirected-for-%26GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<'a, N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + 'a, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.NeighborsDirected" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#associatedtype.NeighborsDirected" class="associatedtype">NeighborsDirected</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.NeighborsDirected.html" class="struct" title="struct petgraph::graphmap::NeighborsDirected">NeighborsDirected</a>\<'a, N, Ty\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.neighbors_directed-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#tymethod.neighbors_directed" class="fn">neighbors_directed</a>(self, n: N, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#associatedtype.NeighborsDirected" class="associatedtype" title="type petgraph::visit::IntoNeighborsDirected::NeighborsDirected">NeighborsDirected</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-IntoNodeIdentifiers-for-%26GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<'a, N, E: 'a, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.NodeIdentifiers" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype">NodeIdentifiers</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.NodeIdentifiers.html" class="struct" title="struct petgraph::graphmap::NodeIdentifiers">NodeIdentifiers</a>\<'a, N, E, Ty\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.node_identifiers" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#tymethod.node_identifiers" class="fn">node_identifiers</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype" title="type petgraph::visit::IntoNodeIdentifiers::NodeIdentifiers">NodeIdentifiers</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-IntoNodeReferences-for-%26GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<'a, N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.NodeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeRef" class="associatedtype">NodeRef</a> = (N, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a N</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.NodeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype">NodeReferences</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.NodeReferences.html" class="struct" title="struct petgraph::graphmap::NodeReferences">NodeReferences</a>\<'a, N, E, Ty\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.node_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#tymethod.node_references" class="fn">node_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype" title="type petgraph::visit::IntoNodeReferences::NodeReferences">NodeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-NodeCount-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html" class="trait" title="trait petgraph::visit::NodeCount">NodeCount</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.node_count-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html#tymethod.node_count" class="fn">node_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-NodeIndexable-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.node_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.node_bound" class="fn">node_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return an upper bound of the node indices in the graph (suitable for the size of a bitmap).

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.to_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.to_index" class="fn">to_index</a>(&self, ix: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert `a` to an integer index.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.from_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.from_index" class="fn">from_index</a>(&self, ix: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

Convert `i` to a node index. `i` must be a valid value in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Serialize-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a> + <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a>, E: <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Available on **crate feature `serde-1`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<Ser\>(&self, serializer: Ser) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Ser::<a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde_core::ser::Serializer::Ok">Ok</a>, Ser::<a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::ser::Serializer::Error">Error</a>\>

where Ser: <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html" class="trait" title="trait serde_core::ser::Serializer">Serializer</a>,

Serializes the given `GraphMap` into the same format as the standard `Graph`. Needs feature `serde-1`.

Note: the graph has to be `Clone` for this to work.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-ToGraph6-for-GraphMap%3CN,+E,+Undirected,+S%3E" class="anchor">§</a>

### impl\<N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html" class="trait" title="trait petgraph::graph6::ToGraph6">ToGraph6</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>, S\>

Available on **crate feature `graphmap`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.graph6_string" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html#tymethod.graph6_string" class="fn">graph6_string</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-Visitable-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#associatedtype.Map" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype">Map</a> = <a href="https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<N\>

The associated map type

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.visit_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.visit_map" class="fn">visit_map</a>(&self) -\> <a href="https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<N\>

Create a new visitor map

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#method.reset_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.reset_map" class="fn">reset_map</a>(&self, map: &mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>)

Reset the visitor map (and resize to new size of graph if needed)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#impl-NodeCompactIndexable-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCompactIndexable.html" class="trait" title="trait petgraph::visit::NodeCompactIndexable">NodeCompactIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html#blanket-implementations" class="anchor">§</a>
