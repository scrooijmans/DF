# Function condensation Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#477-514" class="src">Source</a>

``` rust
pub fn condensation<N, E, Ty, Ix>(
    g: Graph<N, E, Ty, Ix>,
    make_acyclic: bool,
) -> Graph<Vec<N>, E, Ty, Ix>where
    Ty: EdgeType,
    Ix: IndexType,
```

Expand description

[Graph](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html "struct petgraph::graph::Graph") Condense every strongly connected component into a single node and return the result.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.condensation.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: an input [`Graph`](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html "struct petgraph::graph::Graph").
- `make_acyclic`: if `true`, self-loops and multi edges are ignored, guaranteeing that the output is acyclic.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.condensation.html#returns" class="doc-anchor">§</a>Returns

Returns a `Graph` with nodes `Vec<N>` representing strongly connected components.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.condensation.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**.
- Auxiliary space: **O(\|V\| + \|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.condensation.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::Graph;
use petgraph::algo::condensation;
use petgraph::prelude::*;

let mut graph : Graph<(),(),Directed> = Graph::new();
let a = graph.add_node(()); // node with no weight
let b = graph.add_node(());
let c = graph.add_node(());
let d = graph.add_node(());
let e = graph.add_node(());
let f = graph.add_node(());
let g = graph.add_node(());
let h = graph.add_node(());

graph.extend_with_edges(&[
    (a, b),
    (b, c),
    (c, d),
    (d, a),
    (b, e),
    (e, f),
    (f, g),
    (g, h),
    (h, e)
]);

// a ----> b ----> e ----> f
// ^       |       ^       |
// |       v       |       v
// d <---- c       h <---- g

let condensed_graph = condensation(graph,false);
let A = NodeIndex::new(0);
let B = NodeIndex::new(1);
assert_eq!(condensed_graph.node_count(), 2);
assert_eq!(condensed_graph.edge_count(), 9);
assert_eq!(condensed_graph.neighbors(A).collect::<Vec<_>>(), vec![A, A, A, A]);
assert_eq!(condensed_graph.neighbors(B).collect::<Vec<_>>(), vec![A, B, B, B, B]);
```

If `make_acyclic` is true, self-loops and multi edges are ignored:

``` rust
let acyclic_condensed_graph = condensation(graph, true);
let A = NodeIndex::new(0);
let B = NodeIndex::new(1);
assert_eq!(acyclic_condensed_graph.node_count(), 2);
assert_eq!(acyclic_condensed_graph.edge_count(), 1);
assert_eq!(acyclic_condensed_graph.neighbors(B).collect::<Vec<_>>(), vec![A]);
```
