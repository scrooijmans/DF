# Function k_shortest_path Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/k_shortest_path.rs.html#84-126" class="src">Source</a>

``` rust
pub fn k_shortest_path<G, F, K>(
    graph: G,
    start: G::NodeId,
    goal: Option<G::NodeId>,
    k: usize,
    edge_cost: F,
) -> HashMap<G::NodeId, K>where
    G: IntoEdges + Visitable + NodeCount + NodeIndexable,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> K,
    K: Measure + Copy,
```

Expand description

k’th shortest path algorithm.

Compute the length of the k-th shortest path from `start` to every reachable node. Edge costs must be non-negative.

If `goal` is not `None`, then the algorithm terminates once the `goal` node’s cost is calculated.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/k_shortest_path/fn.k_shortest_path.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: an input graph.
- `start`: the *start* node.
- `goal`: optional *goal* node.
- `k`: sequence number of the required shortest paths.
- `edge_cost`: closure that should return the cost for a particular edge, which is used to compute path costs. Edge costs must be non-negative.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/k_shortest_path/fn.k_shortest_path.html#returns" class="doc-anchor">§</a>Returns

- `HashMap`: [`hashbrown::HashMap`](https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap") that maps `NodeId` to path cost.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/k_shortest_path/fn.k_shortest_path.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(k\|E\| log(k\|E\|))**.
- Auxiliary space: **O(\|V\| + k\|E\|)**.

where **\|V\|** is the number of nodes, **\|E\|** is the number of edges and **k** is the provided parameter.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/k_shortest_path/fn.k_shortest_path.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::k_shortest_path;
use petgraph::prelude::*;
use hashbrown::HashMap;

let mut graph : Graph<(),(),Directed>= Graph::new();
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
    (h, e)
]);
// a ----> b ----> e ----> f
// ^       |       ^       |
// |       v       |       v
// d <---- c       h <---- g

let expected_res: HashMap<NodeIndex, usize> = [
     (a, 7),
     (b, 4),
     (c, 5),
     (d, 6),
     (e, 5),
     (f, 6),
     (g, 7),
     (h, 8)
    ].iter().cloned().collect();
let res = k_shortest_path(&graph,b,None,2, |_| 1);
assert_eq!(res, expected_res);
// z is not inside res because there is not path from b to z.
```
