# Function bellman_ford Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/bellman_ford.rs.html#94-122" class="src">Source</a>

``` rust
pub fn bellman_ford<G>(
    g: G,
    source: G::NodeId,
) -> Result<Paths<G::NodeId, G::EdgeWeight>, NegativeCycle>where
    G: NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable,
    G::EdgeWeight: FloatMeasure,
```

Expand description

Compute shortest paths from node `source` to all other.

Using the [Bellman–Ford algorithm](https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm); negative edge costs are permitted, but the graph must not have a cycle of negative weights (in that case it will return an error).

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/fn.bellman_ford.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: graph with no negative cycle.
- `source`: the source node.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/fn.bellman_ford.html#returns" class="doc-anchor">§</a>Returns

- `Ok`: (if graph contains no negative cycle) a struct [`Paths`](https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html "struct petgraph::algo::bellman_ford::Paths") containing distances and predecessors along each shortest path. The vectors in [`Paths`](https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html "struct petgraph::algo::bellman_ford::Paths") are indexed by the graph’s node indices.
- `Err`: if graph contains negative cycle.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/fn.bellman_ford.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\|\|E\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/fn.bellman_ford.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::bellman_ford;
use petgraph::prelude::*;

let mut g = Graph::new();
let a = g.add_node(()); // node with no weight
let b = g.add_node(());
let c = g.add_node(());
let d = g.add_node(());
let e = g.add_node(());
let f = g.add_node(());
g.extend_with_edges(&[
    (0, 1, 2.0),
    (0, 3, 4.0),
    (1, 2, 1.0),
    (1, 5, 7.0),
    (2, 4, 5.0),
    (4, 5, 1.0),
    (3, 4, 1.0),
]);

// Graph represented with the weight of each edge
//
//     2       1
// a ----- b ----- c
// | 4     | 7     |
// d       f       | 5
// | 1     | 1     |
// \------ e ------/

let path = bellman_ford(&g, a);
assert!(path.is_ok());
let path = path.unwrap();
assert_eq!(path.distances, vec![    0.0,     2.0,    3.0,    4.0,     5.0,     6.0]);
assert_eq!(path.predecessors, vec![None, Some(a),Some(b),Some(a), Some(d), Some(e)]);

// Node f (indice 5) can be reach from a with a path costing 6.
// Predecessor of f is Some(e) which predecessor is Some(d) which predecessor is Some(a).
// Thus the path from a to f is a <-> d <-> e <-> f

let graph_with_neg_cycle = Graph::<(), f32, Undirected>::from_edges(&[
        (0, 1, -2.0),
        (0, 3, -4.0),
        (1, 2, -1.0),
        (1, 5, -25.0),
        (2, 4, -5.0),
        (4, 5, -25.0),
        (3, 4, -1.0),
]);

assert!(bellman_ford(&graph_with_neg_cycle, NodeIndex::new(0)).is_err());
```
