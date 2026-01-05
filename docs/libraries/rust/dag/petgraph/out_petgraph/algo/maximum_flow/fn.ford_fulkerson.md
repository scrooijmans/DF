# Function ford_fulkerson Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/maximum_flow/ford_fulkerson.rs.html#164-214" class="src">Source</a>

``` rust
pub fn ford_fulkerson<G>(
    network: G,
    source: G::NodeId,
    destination: G::NodeId,
) -> (G::EdgeWeight, Vec<G::EdgeWeight>)where
    G: NodeCount + EdgeCount + IntoEdgesDirected + EdgeIndexable + NodeIndexable + DataMap + Visitable,
    G::EdgeWeight: Sub<Output = G::EdgeWeight> + PositiveMeasure,
```

Expand description

[Ford-Fulkerson](https://en.wikipedia.org/wiki/Ford%E2%80%93Fulkerson_algorithm) algorithm in the [Edmonds-Karp](https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm) variation. Computes the [maximum flow](https://en.wikipedia.org/wiki/Maximum_flow_problem) from `source` to `destination` in a weighted directed graph.

See also [`maximum_flow`](https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/index.html) module for other maximum flow algorithms.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.ford_fulkerson.html#arguments" class="doc-anchor">§</a>Arguments

- `network`: a wieghted directed graph.
- `source`: a stream *source* node.
- `destination`: a stream *sink* node.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.ford_fulkerson.html#returns" class="doc-anchor">§</a>Returns

Returns a tuple of two values:

- `N::EdgeWeight`: computed maximum flow;
- `Vec<N::EdgeWeight>`: the flow of each edge. The vector is indexed by the graph’s edge indices.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.ford_fulkerson.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\|\|E\|²)**.
- Auxiliary space: **O(\|V\| + \|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.ford_fulkerson.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::ford_fulkerson;
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
let (max_flow, _) = ford_fulkerson(&graph, source, destination);
assert_eq!(23, max_flow);
```
