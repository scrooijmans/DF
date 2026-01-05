# Function with_dynamic_goal Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/dijkstra.rs.html#165-218" class="src">Source</a>

``` rust
pub fn with_dynamic_goal<G, GoalFn, CostFn, K>(
    graph: G,
    start: G::NodeId,
    goal_fn: GoalFn,
    edge_cost: CostFn,
) -> AlgoResult<G::NodeId, K>where
    G: IntoEdges + Visitable,
    G::NodeId: Eq + Hash,
    GoalFn: FnMut(&G::NodeId) -> bool,
    CostFn: FnMut(G::EdgeRef) -> K,
    K: Measure + Copy,
```

Expand description

Dijkstra’s shortest path algorithm with a dynamic goal.

This algorithm is identical to [`dijkstra`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html "fn petgraph::algo::dijkstra::dijkstra"), but allows matching multiple goal nodes, whichever is reached first. A node is considered a goal if `goal_fn` returns `true` for it.

See the [`dijkstra`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html "fn petgraph::algo::dijkstra::dijkstra") function for more details.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.with_dynamic_goal.html#example" class="doc-anchor">§</a>Example

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
    (b, 0),
    (c, 1),
    (d, 2),
    (e, 1),
    (f, 2),
].iter().cloned().collect();
let res = dijkstra::with_dynamic_goal(&graph, b, |&node| node == d || node == f, |_| 1);
assert_eq!(res.scores, expected_res);
assert!(res.goal_node == Some(d) || res.goal_node == Some(f));
```
