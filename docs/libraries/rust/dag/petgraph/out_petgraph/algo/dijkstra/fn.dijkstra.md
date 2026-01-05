# Function dijkstra Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/dijkstra.rs.html#88-101" class="src">Source</a>

``` rust
pub fn dijkstra<G, F, K>(
    graph: G,
    start: G::NodeId,
    goal: Option<G::NodeId>,
    edge_cost: F,
) -> HashMap<G::NodeId, K>where
    G: IntoEdges + Visitable,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> K,
    K: Measure + Copy,
```

Expand description

Dijkstra’s shortest path algorithm.

Compute the length of the shortest path from `start` to every reachable node.

The function `edge_cost` should return the cost for a particular edge, which is used to compute path costs. Edge costs must be non-negative.

If `goal` is not `None`, then the algorithm terminates once the `goal` node’s cost is calculated.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: weighted graph.
- `start`: the start node.
- `goal`: optional *goal* node.
- `edge_cost`: closure that returns cost of a particular edge.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html#returns" class="doc-anchor">§</a>Returns

- `HashMap`: [`hashbrown::HashMap`](https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap") that maps `NodeId` to path cost.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O((\|V\|+\|E\|)log(\|V\|))**.
- Auxiliary space: **O(\|V\|+\|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use hashbrown::HashMap;

let mut graph: Graph<(), (), Directed> = Graph::new();
let a = graph.add_node(()); // node with no weight
let b = graph.add_node(());
let c = graph.add_node(());
let d = graph.add_node(());
let e = graph.add_node(());
let f = graph.add_node(());
let g = graph.add_node(());
let h = graph.add_node(());
// z will be in another connected component
let z = graph.add_node(());

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

let expected_res: HashMap<NodeIndex, usize> = [
    (a, 3),
    (b, 0),
    (c, 1),
    (d, 2),
    (e, 1),
    (f, 2),
    (g, 3),
    (h, 4),
].iter().cloned().collect();
let res = dijkstra(&graph, b, None, |_| 1);
assert_eq!(res, expected_res);
// z is not inside res because there is not path from b to z.
```
