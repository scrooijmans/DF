# Function find_negative_cycle Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/bellman_ford.rs.html#170-233" class="src">Source</a>

``` rust
pub fn find_negative_cycle<G>(g: G, source: G::NodeId) -> Option<Vec<G::NodeId>>where
    G: NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable + Visitable,
    G::EdgeWeight: FloatMeasure,
```

Expand description

Find the path of a negative cycle reachable from node `source`.

Using the [find_negative_cycle](https://blogs.asarkar.com/assets/docs/algorithms-curated/Negative-Weight%20Cycle%20Algorithms%20-%20Huang.pdf); will search the graph for negative cycles using [Bellman–Ford algorithm](https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm). If no negative cycle is found the function will return `None`.

If a negative cycle is found from source, return one vec with a path of `NodeId`s.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/fn.find_negative_cycle.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: graph.
- `source`: the source node.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/fn.find_negative_cycle.html#returns" class="doc-anchor">§</a>Returns

- `Some(Vec<G::NodeId>)` - the path of the negative cycle (if found).
- `None` - if `g` doesn’t contain negative cycles reachable from `source`.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/fn.find_negative_cycle.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\|\|E\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/fn.find_negative_cycle.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::find_negative_cycle;
use petgraph::prelude::*;

let graph_with_neg_cycle = Graph::<(), f32, Directed>::from_edges(&[
        (0, 1, 1.),
        (0, 2, 1.),
        (0, 3, 1.),
        (1, 3, 1.),
        (2, 1, 1.),
        (3, 2, -3.),
]);

let path = find_negative_cycle(&graph_with_neg_cycle, NodeIndex::new(0));
assert_eq!(
    path,
    Some([NodeIndex::new(1), NodeIndex::new(3), NodeIndex::new(2)].to_vec())
);
```
