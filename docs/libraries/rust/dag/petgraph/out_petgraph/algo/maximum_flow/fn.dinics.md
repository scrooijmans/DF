# Function dinics Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/maximum_flow/dinics.rs.html#73-101" class="src">Source</a>

``` rust
pub fn dinics<G>(
    network: G,
    source: G::NodeId,
    destination: G::NodeId,
) -> (G::EdgeWeight, Vec<G::EdgeWeight>)where
    G: NodeCount + EdgeCount + IntoEdgesDirected + EdgeIndexable + NodeIndexable + Visitable,
    G::EdgeWeight: Sub<Output = G::EdgeWeight> + PositiveMeasure,
```

Expand description

Compute the maximum flow from `source` to `destination` in a directed graph. Implements [Dinic’s (or Dinitz’s) algorithm](https://en.wikipedia.org/wiki/Dinic%27s_algorithm), which builds successive level graphs using breadth-first search and finds blocking flows within them through depth-first searches.

For simplicity, the algorithm requires `N::EdgeWeight` to implement only [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") trait, and not [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), but will panic if it tries to compare two elements that aren’t comparable (i.e., given two edge weights `a` and `b`, where neither `a >= b` nor `a < b`).

See also [`maximum_flow`](https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/index.html) module for other maximum flow algorithms.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.dinics.html#arguments" class="doc-anchor">§</a>Arguments

- `network` — A directed graph with positive edge weights, namely “flow capacities”.
- `source` — The source node where flow originates.
- `destination` — The destination node where flow terminates.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.dinics.html#returns" class="doc-anchor">§</a>Returns

Returns a tuple of two values:

- `N::EdgeWeight`: computed maximum flow;
- `Vec<N::EdgeWeight>`: the flow of each edge. The vector is indexed by the graph’s edge indices.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.dinics.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity:
  - In general: **O(\|V\|²\|E\|)**
  - In networks with only unit capacities: **O(min{\|V\|²ᐟ³, \|E\|¹ᐟ²} \|E\|)**
- Auxiliary space: **O(\|V\| + \|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.dinics.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::dinics;
// Example from CLRS book
let mut graph = Graph::<u8, u8>::new();
let source = graph.add_node(0);
let _ = graph.add_node(1);
let _ = graph.add_node(2);
let _ = graph.add_node(3);
let _ = graph.add_node(4);
let destination = graph.add_node(5);
graph.extend_with_edges(&[
   (0, 1, 16),
   (0, 2, 13),
   (1, 2, 10),
   (1, 3, 12),
   (2, 1, 4),
   (2, 4, 14),
   (3, 2, 9),
   (3, 5, 20),
   (4, 3, 7),
   (4, 5, 4),
]);
let (max_flow, _) = dinics(&graph, source, destination);
assert_eq!(23, max_flow);
```
