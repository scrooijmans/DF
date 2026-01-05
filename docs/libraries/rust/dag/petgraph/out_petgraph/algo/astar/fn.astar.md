# Function astar Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/astar.rs.html#79-161" class="src">Source</a>

``` rust
pub fn astar<G, F, H, K, IsGoal>(
    graph: G,
    start: G::NodeId,
    is_goal: IsGoal,
    edge_cost: F,
    estimate_cost: H,
) -> Option<(K, Vec<G::NodeId>)>where
    G: IntoEdges + Visitable,
    IsGoal: FnMut(G::NodeId) -> bool,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> K,
    H: FnMut(G::NodeId) -> K,
    K: Measure + Copy,
```

Expand description

A\* shortest path algorithm.

Computes the shortest path from `start` to `finish`, including the total path cost.

`finish` is implicitly given via the `is_goal` callback, which should return `true` if the given node is the finish node.

The function `edge_cost` should return the cost for a particular edge. Edge costs must be non-negative.

The function `estimate_cost` should return the estimated cost to the finish for a particular node. For the algorithm to find the actual shortest path, it should be admissible, meaning that it should never overestimate the actual cost to get to the nearest goal node. Estimate costs must also be non-negative.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/astar/fn.astar.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: weighted graph.
- `start`: the start node.
- `is_goal`: the callback defines the goal node.
- `edge_cost`: closure that returns cost of a particular edge.
- `estimate_cost`: closure that returns the estimated cost to the finish for particular node.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/astar/fn.astar.html#returns" class="doc-anchor">§</a>Returns

- `Some(K, Vec<G::NodeId>)` - the total cost and path from start to finish, if one was found.
- `None` - if such a path was not found.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/astar/fn.astar.html#complexity" class="doc-anchor">§</a>Complexity

The time complexity largely depends on the heuristic used. Feel free to contribute and provide the exact time complexity :)

With a trivial heuristic, the algorithm will behave like [`crate::algo::dijkstra`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html "fn petgraph::algo::dijkstra::dijkstra").

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/astar/fn.astar.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::astar;

let mut g = Graph::new();
let a = g.add_node((0., 0.));
let b = g.add_node((2., 0.));
let c = g.add_node((1., 1.));
let d = g.add_node((0., 2.));
let e = g.add_node((3., 3.));
let f = g.add_node((4., 2.));
g.extend_with_edges(&[
    (a, b, 2),
    (a, d, 4),
    (b, c, 1),
    (b, f, 7),
    (c, e, 5),
    (e, f, 1),
    (d, e, 1),
]);

// Graph represented with the weight of each edge
// Edges with '*' are part of the optimal path.
//
//     2       1
// a ----- b ----- c
// | 4*    | 7     |
// d       f       | 5
// | 1*    | 1*    |
// \------ e ------/

let path = astar(&g, a, |finish| finish == f, |e| *e.weight(), |_| 0);
assert_eq!(path, Some((6, vec![a, d, e, f])));
```
