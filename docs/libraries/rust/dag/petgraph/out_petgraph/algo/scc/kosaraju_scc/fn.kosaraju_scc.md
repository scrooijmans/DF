# Function kosaraju_scc Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/scc/kosaraju_scc.rs.html#96-135" class="src">Source</a>

``` rust
pub fn kosaraju_scc<G>(g: G) -> Vec<Vec<G::NodeId>>where
    G: IntoNeighborsDirected + Visitable + IntoNodeIdentifiers,
```

Expand description

Compute the *strongly connected components* using [Kosaraju’s algorithm](https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm).

This implementation is iterative and does two passes over the nodes.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/kosaraju_scc/fn.kosaraju_scc.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: a directed or undirected graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/kosaraju_scc/fn.kosaraju_scc.html#returns" class="doc-anchor">§</a>Returns

Return a vector where each element is a strongly connected component (scc). The order of node ids within each scc is arbitrary, but the order of the sccs is their postorder (reverse topological sort).

For an undirected graph, the sccs are simply the connected components.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/kosaraju_scc/fn.kosaraju_scc.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/kosaraju_scc/fn.kosaraju_scc.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::Graph;
use petgraph::algo::kosaraju_scc;
use petgraph::prelude::*;

let mut graph: Graph<i32, (), Directed> = Graph::new();
let a = graph.add_node(1);
let b = graph.add_node(2);
let c = graph.add_node(3);
let d = graph.add_node(4);
let e = graph.add_node(5);
let f = graph.add_node(6);

graph.extend_with_edges(&[
    (a, b), (b, c), (c, a),  // First SCC: a -> b -> c -> a
    (d, e), (e, f), (f, d),  // Second SCC: d -> e -> f -> d
    (c, d),                  // Connection between SCCs
]);

// Graph structure:
// a ---> b       e ---> f
// ↑     ↓       ↑      |
// └---- c --->  d <----┘

let sccs = kosaraju_scc(&graph);
assert_eq!(sccs.len(), 2); // Two strongly connected components

// Each SCC contains 3 nodes
assert_eq!(sccs[0].len(), 3);
assert_eq!(sccs[1].len(), 3);
```

For a simple directed acyclic graph (DAG):

``` rust
use petgraph::Graph;
use petgraph::algo::kosaraju_scc;
use petgraph::prelude::*;

let mut dag: Graph<&str, (), Directed> = Graph::new();
let a = dag.add_node("A");
let b = dag.add_node("B");
let c = dag.add_node("C");

dag.extend_with_edges(&[(a, b), (b, c)]);
// A -> B -> C

let sccs = kosaraju_scc(&dag);
assert_eq!(sccs.len(), 3); // Each node is its own SCC

// Each SCC contains exactly one node
for scc in &sccs {
    assert_eq!(scc.len(), 1);
}
```
