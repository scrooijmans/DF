# Function greedy_feedback_arc_set Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/feedback_arc_set.rs.html#73-88" class="src">Source</a>

``` rust
pub fn greedy_feedback_arc_set<G>(g: G) -> impl Iterator<Item = G::EdgeRef>where
    G: IntoEdgeReferences + GraphProp<EdgeType = Directed> + NodeCount,
    G::NodeId: GraphIndex,
```

Expand description

Finds a [feedback arc set](https://en.wikipedia.org/wiki/Feedback_arc_set): a set of edges in the given directed graph, which when removed, make the graph acyclic.

Uses a [greedy heuristic algorithm](https://doi.org/10.1016/0020-0190(93)90079-O) to select a small number of edges, but does not necessarily find the minimum feedback arc set.

Does not consider edge/node weights when selecting edges for the feedback arc set.

Loops (edges to and from the same node) are always included in the returned set.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/feedback_arc_set/fn.greedy_feedback_arc_set.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: a directed graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/feedback_arc_set/fn.greedy_feedback_arc_set.html#returns" class="doc-anchor">§</a>Returns

- `impl Iterator`: the iterator of edge references `G::EdgeRef` in the feedback arc set.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/feedback_arc_set/fn.greedy_feedback_arc_set.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**.
- Auxiliary space: **O(\|V\| + \|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/feedback_arc_set/fn.greedy_feedback_arc_set.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::{
    algo::{greedy_feedback_arc_set, is_cyclic_directed},
    graph::EdgeIndex,
    stable_graph::StableGraph,
    visit::EdgeRef,
};

let mut g: StableGraph<(), ()> = StableGraph::from_edges(&[
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 4),
    (4, 5),
    (5, 0),
    (4, 1),
    (1, 3),
]);

assert!(is_cyclic_directed(&g));

let fas: Vec<EdgeIndex> = greedy_feedback_arc_set(&g).map(|e| e.id()).collect();

// Remove edges in feedback arc set from original graph
for edge_id in fas {
    g.remove_edge(edge_id);
}

assert!(!is_cyclic_directed(&g));
```
