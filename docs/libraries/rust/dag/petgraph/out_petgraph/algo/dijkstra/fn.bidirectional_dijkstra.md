# Function bidirectional_dijkstra Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/dijkstra.rs.html#287-407" class="src">Source</a>

``` rust
pub fn bidirectional_dijkstra<G, F, K>(
    graph: G,
    start: G::NodeId,
    goal: G::NodeId,
    edge_cost: F,
) -> Option<K>where
    G: Visitable + IntoEdgesDirected,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> K,
    K: Measure + Copy,
```

Expand description

Bidirectional Dijkstra’s shortest path algorithm.

Compute the length of the shortest path from `start` to `target`.

Bidirectional Dijkstra has the same time complexity as standard [`Dijkstra`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html "fn petgraph::algo::dijkstra::dijkstra"). However, because it searches simultaneously from both the start and goal nodes, meeting in the middle, it often explores roughly half the nodes that regular [`Dijkstra`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html "fn petgraph::algo::dijkstra::dijkstra") would explore. This is especially the case when the path is long relative to the graph size or when working with sparse graphs.

However, regular [`Dijkstra`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html "fn petgraph::algo::dijkstra::dijkstra") may be preferable when you need the shortest paths from the start node to multiple goals or when the start and goal are relatively close to each other.

The function `edge_cost` should return the cost for a particular edge, which is used to compute path costs. Edge costs must be non-negative.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.bidirectional_dijkstra.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: weighted graph.
- `start`: the start node.
- `goal`: the goal node.
- `edge_cost`: closure that returns the cost of a particular edge.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.bidirectional_dijkstra.html#returns" class="doc-anchor">§</a>Returns

- `Some(K)` - the total cost from start to finish, if one was found.
- `None` - if such a path was not found.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.bidirectional_dijkstra.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O((\|V\|+\|E\|)log(\|V\|))**.
- Auxiliary space: **O(\|V\|+\|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.bidirectional_dijkstra.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::bidirectional_dijkstra;
use petgraph::prelude::*;
use hashbrown::HashMap;

let mut graph: Graph<(), (), Directed> = Graph::new();
let a = graph.add_node(());
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
    (e, f),
    (b, e),
    (f, g),
    (g, h),
    (h, e),
]);
// a ----> b ----> e ----> f
// ^       |       ^       |
// |       v       |       v
// d <---- c       h <---- g

let result = bidirectional_dijkstra(&graph, a, g, |_| 1);
assert_eq!(result, Some(4));
```
