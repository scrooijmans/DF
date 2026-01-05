# Function spfa Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/spfa.rs.html#81-110" class="src">Source</a>

``` rust
pub fn spfa<G, F, K>(
    graph: G,
    source: G::NodeId,
    edge_cost: F,
) -> Result<Paths<G::NodeId, K>, NegativeCycle>where
    G: IntoEdges + IntoNodeIdentifiers + NodeIndexable,
    F: FnMut(G::EdgeRef) -> K,
    K: BoundedMeasure + Copy,
```

Expand description

Compute shortest paths from node `source` to all other.

Using the [Shortest Path Faster Algorithm](https://www.geeksforgeeks.org/shortest-path-faster-algorithm/). Compute shortest distances from node `source` to all other.

Compute shortest paths lengths in a weighted graph with positive or negative edge weights, but with no negative cycles.

### <a href="https://docs.rs/petgraph/latest/petgraph/algo/spfa/fn.spfa.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: weighted graph.
- `source`: the source vertex, for which we calculate the lengths of the shortest paths to all the others.
- `edge_cost`: closure that returns the cost of a particular edge.

### <a href="https://docs.rs/petgraph/latest/petgraph/algo/spfa/fn.spfa.html#returns" class="doc-anchor">§</a>Returns

- `Err`: if graph contains negative cycle.
- `Ok`: a pair of a vector of shortest distances and a vector of predecessors of each vertex along the shortest path.

### <a href="https://docs.rs/petgraph/latest/petgraph/algo/spfa/fn.spfa.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\|\|E\|)**, but it’s generally assumed that in the average case it is **O(\|E\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/spfa/fn.spfa.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::spfa;

let mut g = Graph::new();
let a = g.add_node(()); // node with no weight
let b = g.add_node(());
let c = g.add_node(());
let d = g.add_node(());
let e = g.add_node(());
let f = g.add_node(());
g.extend_with_edges(&[
    (0, 1, 3.0),
    (0, 3, 2.0),
    (1, 2, 1.0),
    (1, 5, 7.0),
    (2, 4, -4.0),
    (3, 4, -1.0),
    (4, 5, 1.0),
]);

// Graph represented with the weight of each edge.
//
//     3       1
// a ----- b ----- c
// | 2     | 7     |
// d       f       | -4
// | -1    | 1     |
// \------ e ------/

let path = spfa(&g, a, |edge| *edge.weight());
assert!(path.is_ok());
let path = path.unwrap();
assert_eq!(path.distances, vec![0.0 ,     3.0,     4.0,    2.0,     0.0,     1.0]);
assert_eq!(path.predecessors, vec![None, Some(a), Some(b), Some(a), Some(c), Some(e)]);


// Negative cycle.
let graph = Graph::<(), f32>::from_edges(&[
    (0, 1, 2.0), (1, 2, 2.0), (2, 0, -10.0)]);

assert!(spfa(&graph, 0.into(), |edge| *edge.weight()).is_err());
```
