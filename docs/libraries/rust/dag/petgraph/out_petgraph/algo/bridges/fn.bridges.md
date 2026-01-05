# Function bridges Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/bridges.rs.html#54-124" class="src">Source</a>

``` rust
pub fn bridges<G>(graph: G) -> impl Iterator<Item = G::EdgeRef>where
    G: IntoNodeIdentifiers + IntoNeighbors + NodeIndexable + IntoEdgeReferences,
```

Expand description

Find all [bridges](https://en.wikipedia.org/wiki/Bridge_(graph_theory)) in a simple undirected graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bridges/fn.bridges.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: a simple undirected graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bridges/fn.bridges.html#returns" class="doc-anchor">§</a>Returns

- `impl Iterator`: the iterator of edge references `G::EdgeRef` representing the edges of the input graph that are bridges. The order of the edges is arbitrary.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bridges/fn.bridges.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bridges/fn.bridges.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::algo::bridges::bridges;
use petgraph::graph::UnGraph;
use petgraph::visit::EdgeRef;

// Create the following graph:
// 0----1    4
//      | __/|
// 5----2/---3

let mut g = UnGraph::new_undirected();
let n0 = g.add_node(());
let n1 = g.add_node(());
let n2 = g.add_node(());
let n3 = g.add_node(());
let n4 = g.add_node(());
let n5 = g.add_node(());
let e0 = g.add_edge(n0, n1, ());
let e1 = g.add_edge(n1, n2, ());
let e2 = g.add_edge(n2, n3, ());
let e3 = g.add_edge(n3, n4, ());
let e4 = g.add_edge(n2, n4, ());
let e5 = g.add_edge(n5, n2, ());

let bridges: Vec<_> = bridges(&g).map(|edge_ref| edge_ref.id()).collect();

// The bridges in this graph are the undirected edges {2, 5}, {1, 2}, {0, 1}.
assert_eq!(bridges, vec![e0, e1, e5]);
```
