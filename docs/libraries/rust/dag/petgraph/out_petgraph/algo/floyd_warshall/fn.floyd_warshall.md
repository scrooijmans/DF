# Function floyd_warshall Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/floyd_warshall.rs.html#91-120" class="src">Source</a>

``` rust
pub fn floyd_warshall<G, F, K>(
    graph: G,
    edge_cost: F,
) -> Result<HashMap<(G::NodeId, G::NodeId), K>, NegativeCycle>where
    G: NodeCompactIndexable + IntoEdgeReferences + IntoNodeIdentifiers + GraphProp,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> K,
    K: BoundedMeasure + Copy,
```

Expand description

[Floyd–Warshall algorithm](https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm) is an algorithm for all pairs shortest path problem

Compute the length of each shortest path in a weighted graph with positive or negative edge weights (but with no negative cycles).

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/fn.floyd_warshall.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: graph with no negative cycle.
- `edge_cost`: closure that returns cost of a particular edge.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/fn.floyd_warshall.html#returns" class="doc-anchor">§</a>Returns

- `Ok`: (if graph contains no negative cycle) a [`hashbrown::HashMap`](https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap") containing all pairs shortest paths.
- `Err`: if graph contains negative cycle.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/fn.floyd_warshall.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\|³)**.
- Auxiliary space: **O(\|V\|²)**.

where **\|V\|** is the number of nodes.

**Note**: If the graph is sparse (the number of edges is quite small), consider using [`johnson`](https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.johnson.html "fn petgraph::algo::johnson::johnson")’s algorithm, which is likely to show better performance in such cases.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/fn.floyd_warshall.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::{prelude::*, Graph, Directed};
use petgraph::algo::floyd_warshall;
use hashbrown::HashMap;

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

let inf = core::i32::MAX;
let expected_res: HashMap<(NodeIndex, NodeIndex), i32> = [
   ((a, a), 0), ((a, b), 1), ((a, c), 3), ((a, d), 3),
   ((b, a), inf), ((b, b), 0), ((b, c), 2), ((b, d), 2),
   ((c, a), inf), ((c, b), inf), ((c, c), 0), ((c, d), 2),
   ((d, a), inf), ((d, b), inf), ((d, c), inf), ((d, d), 0),
].iter().cloned().collect();


let res = floyd_warshall(&graph, |edge| {
    if let Some(weight) = weight_map.get(&(edge.source(), edge.target())) {
        *weight
    } else {
        inf
    }
}).unwrap();

let nodes = [a, b, c, d];
for node1 in &nodes {
    for node2 in &nodes {
        assert_eq!(res.get(&(*node1, *node2)).unwrap(), expected_res.get(&(*node1, *node2)).unwrap());
    }
}
```
