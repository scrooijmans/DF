# Function parallel_johnson Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/johnson.rs.html#205-248" class="src">Source</a>

``` rust
pub fn parallel_johnson<G, F, K>(
    graph: G,
    edge_cost: F,
) -> Result<HashMap<(G::NodeId, G::NodeId), K>, NegativeCycle>where
    G: IntoEdges + IntoNodeIdentifiers + NodeIndexable + Visitable + Sync,
    G::NodeId: Eq + Hash + Send,
    F: Fn(G::EdgeRef) -> K + Sync,
    K: BoundedMeasure + Copy + Sub<K, Output = K> + Send + Sync,
```

Expand description

[Johnson algorithm](https://en.wikipedia.org/wiki/Johnson%27s_algorithm) implementation for all pairs shortest path problem, parallelizing the [`dijkstra`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html "fn petgraph::algo::dijkstra::dijkstra") calls with `rayon`.

Сompute the lengths of shortest paths in a weighted graph with positive or negative edge weights, but no negative cycles.

If you are working with a sparse graph that is guaranteed to have no negative weights, it’s preferable to run [`dijkstra`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html "fn petgraph::algo::dijkstra::dijkstra") several times in parallel.

### <a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.parallel_johnson.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: weighted graph.
- `edge_cost`: closure that returns cost of a particular edge.

### <a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.parallel_johnson.html#returns" class="doc-anchor">§</a>Returns

- `Err`: if graph contains negative cycle.
- `Ok`: `HashMap` of shortest distances.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.parallel_johnson.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::{prelude::*, Graph, Directed};
use petgraph::algo::parallel_johnson;
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


let res = parallel_johnson(&graph, |edge| {
    *edge.weight()
}).unwrap();

let nodes = [a, b, c, d];
for node1 in &nodes {
    for node2 in &nodes {
        assert_eq!(res.get(&(*node1, *node2)), expected_res.get(&(*node1, *node2)));
    }
}
```
