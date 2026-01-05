# Function floyd_warshall_path Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/floyd_warshall.rs.html#201-231" class="src">Source</a>

``` rust
pub fn floyd_warshall_path<G, F, K>(
    graph: G,
    edge_cost: F,
) -> Result<(HashMap<(G::NodeId, G::NodeId), K>, Vec<Vec<Option<usize>>>), NegativeCycle>where
    G: NodeCompactIndexable + IntoEdgeReferences + IntoNodeIdentifiers + GraphProp,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> K,
    K: BoundedMeasure + Copy,
```

Expand description

[Floyd–Warshall algorithm](https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm) is an algorithm for all pairs shortest path problem

Compute all pairs shortest paths in a weighted graph with positive or negative edge weights (but with no negative cycles). Returns HashMap of shortest path lengths. Additionally, returns HashMap of intermediate nodes along shortest path for indicated edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/fn.floyd_warshall_path.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: graph with no negative cycle
- `edge_cost`: closure that returns cost of a particular edge

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/fn.floyd_warshall_path.html#returns" class="doc-anchor">§</a>Returns

- `Ok`: (if graph contains no negative cycle) a hashmap containing all pairs shortest path distances and a hashmap for all pairs shortest paths
- `Err`: if graph contains negative cycle.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/fn.floyd_warshall_path.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\|³)**
- Auxiliary space: **O(\|V\|²)**

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/fn.floyd_warshall_path.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::{prelude::*, Graph, Directed};
use petgraph::algo::floyd_warshall::floyd_warshall_path;
use std::collections::HashMap;

let mut graph: Graph<(), (), Directed> = Graph::new();
let a = graph.add_node(());
let b = graph.add_node(());
let c = graph.add_node(());
let d = graph.add_node(());

graph.extend_with_edges(&[
   (a, b),
   (a, c),
   (a, d),
   (b, c),
   (b, d),
   (c, d)
]);

let weight_map: HashMap<(NodeIndex, NodeIndex), i32> = [
   ((a, a), 0), ((a, b), 1), ((a, c), 4), ((a, d), 10),
   ((b, b), 0), ((b, c), 2), ((b, d), 2),
   ((c, c), 0), ((c, d), 2)
].iter().cloned().collect();
//     ----- b --------
//    |      ^         | 2
//    |    1 |    4    v
//  2 |      a ------> c
//    |   10 |         | 2
//    |      v         v
//     --->  d <-------

let inf = std::i32::MAX;
let expected_res: HashMap<(NodeIndex, NodeIndex), i32> = [
   ((a, a), 0), ((a, b), 1), ((a, c), 3), ((a, d), 3),
   ((b, a), inf), ((b, b), 0), ((b, c), 2), ((b, d), 2),
   ((c, a), inf), ((c, b), inf), ((c, c), 0), ((c, d), 2),
   ((d, a), inf), ((d, b), inf), ((d, c), inf), ((d, d), 0),
].iter().cloned().collect();


let (res, prev) = floyd_warshall_path(&graph, |edge| {
    if let Some(weight) = weight_map.get(&(edge.source(), edge.target())) {
        *weight
    } else {
        inf
    }
}).unwrap();

assert_eq!(prev[0][2], Some(1));

let nodes = [a, b, c, d];
for node1 in &nodes {
    for node2 in &nodes {
        assert_eq!(res.get(&(*node1, *node2)).unwrap(), expected_res.get(&(*node1, *node2)).unwrap());
    }
}
```
