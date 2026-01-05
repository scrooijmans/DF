# Function johnson Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/johnson.rs.html#95-132" class="src">Source</a>

``` rust
pub fn johnson<G, F, K>(
    graph: G,
    edge_cost: F,
) -> Result<HashMap<(G::NodeId, G::NodeId), K>, NegativeCycle>where
    G: IntoEdges + IntoNodeIdentifiers + NodeIndexable + Visitable,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> K,
    K: BoundedMeasure + Copy + Sub<K, Output = K>,
```

Expand description

[Johnson algorithm](https://en.wikipedia.org/wiki/Johnson%27s_algorithm) for all pairs shortest path problem.

Сompute the lengths of shortest paths in a weighted graph with positive or negative edge weights, but no negative cycles.

The time complexity of this implementation is **O(\|V\|\|E\|log(\|V\|) + \|V\|²\*log(\|V\|))**, which is faster than [`floyd_warshall`](https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/fn.floyd_warshall.html "fn petgraph::algo::floyd_warshall::floyd_warshall") on sparse graphs and slower on dense ones.

If you are working with a sparse graph that is guaranteed to have no negative weights, it’s preferable to run [`dijkstra`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html "fn petgraph::algo::dijkstra::dijkstra") several times.

There is also a parallel implementation `parallel_johnson`, available under the `rayon` feature.

### <a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.johnson.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: weighted graph.
- `edge_cost`: closure that returns cost of a particular edge.

### <a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.johnson.html#returns" class="doc-anchor">§</a>Returns

- `Err`: if graph contains negative cycle.
- `Ok`: `HashMap` of shortest distances.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.johnson.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\|\|E\|log(\|V\|) + \|V\|²log(\|V\|))** since the implementation is based on [`dijkstra`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html "fn petgraph::algo::dijkstra::dijkstra").
- Auxiliary space: **O(\|V\|² + \|V\|\|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.johnson.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::{prelude::*, Graph, Directed};
use petgraph::algo::johnson;
use std::collections::HashMap;

let mut graph: Graph<(), i32, Directed> = Graph::new();
let a = graph.add_node(());
let b = graph.add_node(());
let c = graph.add_node(());
let d = graph.add_node(());

graph.extend_with_edges(&[
   (a, b, 1),
   (a, c, 4),
   (a, d, 10),
   (b, c, 2),
   (b, d, 2),
   (c, d, 2)
]);

//     ----- b --------
//    |      ^         | 2
//    |    1 |    4    v
//  2 |      a ------> c
//    |   10 |         | 2
//    |      v         v
//     --->  d <-------

let expected_res: HashMap<(NodeIndex, NodeIndex), i32> = [
   ((a, a), 0), ((a, b), 1), ((a, c), 3), ((a, d), 3),
   ((b, b), 0), ((b, c), 2), ((b, d), 2),
   ((c, c), 0), ((c, d), 2),
   ((d, d), 0),
].iter().cloned().collect();


let res = johnson(&graph, |edge| {
    *edge.weight()
}).unwrap();

let nodes = [a, b, c, d];
for node1 in &nodes {
    for node2 in &nodes {
        assert_eq!(res.get(&(*node1, *node2)), expected_res.get(&(*node1, *node2)));
    }
}
```
