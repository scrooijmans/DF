# Function maximum_matching Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/matching.rs.html#373-500" class="src">Source</a>

``` rust
pub fn maximum_matching<G>(graph: G) -> Matching<G>where
    G: Visitable + NodeIndexable + IntoNodeIdentifiers + IntoEdges,
```

Expand description

Compute the [*maximum matching*](https://en.wikipedia.org/wiki/Matching_(graph_theory)) using [Gabow’s algorithm](https://dl.acm.org/doi/10.1145/321941.321942).

The input graph is treated as if undirected. The algorithm runs in **O(\|V\|³)**. An algorithm with a better time complexity might be used in the future.

**Panics** if `g.node_bound()` is `usize::MAX`.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/fn.maximum_matching.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: an undirected graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/fn.maximum_matching.html#returns" class="doc-anchor">§</a>Returns

- [`Matching`](https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html "struct petgraph::algo::matching::Matching"): computed maximum matching.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/fn.maximum_matching.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\|³)**.
- Auxiliary space: **O(\|V\| + \|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/fn.maximum_matching.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::prelude::*;
use petgraph::algo::maximum_matching;

// The example graph:
//
//    +-- b ---- d ---- f
//   /    |      |
//  a     |      |
//   \    |      |
//    +-- c ---- e
//
// Maximum matching: { (a, b), (c, e), (d, f) }

let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();
let a = graph.add_node(());
let b = graph.add_node(());
let c = graph.add_node(());
let d = graph.add_node(());
let e = graph.add_node(());
let f = graph.add_node(());
graph.extend_with_edges(&[(a, b), (a, c), (b, c), (b, d), (c, e), (d, e), (d, f)]);

let matching = maximum_matching(&graph);
assert!(matching.contains_edge(a, b));
assert!(matching.contains_edge(c, e));
assert_eq!(matching.mate(d), Some(f));
assert_eq!(matching.mate(f), Some(d));
```
