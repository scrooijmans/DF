# Function tarjan_scc Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/scc/tarjan_scc.rs.html#267-277" class="src">Source</a>

``` rust
pub fn tarjan_scc<G>(g: G) -> Vec<Vec<G::NodeId>>where
    G: IntoNodeIdentifiers + IntoNeighbors + NodeIndexable,
```

Expand description

Compute the *strongly connected components* using [Tarjan’s algorithm](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm).

This implementation is recursive and does one pass over the nodes. It is based on [A Space-Efficient Algorithm for Finding Strongly Connected Components](https://www.researchgate.net/publication/283024636_A_space-efficient_algorithm_for_finding_strongly_connected_components) by David J. Pierce, to provide a memory-efficient implementation of [Tarjan’s algorithm](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm).

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/fn.tarjan_scc.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: a directed or undirected graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/fn.tarjan_scc.html#returns" class="doc-anchor">§</a>Returns

Return a vector where each element is a strongly connected component (scc). The order of node ids within each scc is arbitrary, but the order of the sccs is their postorder (reverse topological sort).

For an undirected graph, the sccs are simply the connected components.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/fn.tarjan_scc.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/fn.tarjan_scc.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::Graph;
use petgraph::algo::tarjan_scc;
use petgraph::prelude::*;

let mut graph: Graph<char, (), Directed> = Graph::new();
let a = graph.add_node('A');
let b = graph.add_node('B');
let c = graph.add_node('C');
let d = graph.add_node('D');
let e = graph.add_node('E');

graph.extend_with_edges(&[
    (a, b), (b, c), (c, a),  // SCC 1: A -> B -> C -> A
    (b, d), (d, e),          // D and E form individual SCCs
]);

// Graph structure:
// A --> B ---> D ---> E
// ↑    ↑
// └--- C

let sccs = tarjan_scc(&graph);
assert_eq!(sccs.len(), 3); // Three strongly connected components

// Find the sizes of each SCC
let mut scc_sizes: Vec<usize> = sccs.iter().map(|scc| scc.len()).collect();
scc_sizes.sort();
assert_eq!(scc_sizes, vec![1, 1, 3]); // Two single-node SCCs and one 3-node SCC
```

Using `TarjanScc` for reusable computation:

``` rust
use petgraph::Graph;
use petgraph::algo::TarjanScc;
use petgraph::prelude::*;

let mut graph: Graph<u32, (), Directed> = Graph::new();
let n1 = graph.add_node(1);
let n2 = graph.add_node(2);
let n3 = graph.add_node(3);
let n4 = graph.add_node(4);

graph.extend_with_edges(&[
    (n1, n2), (n2, n3), (n3, n1),  // Cycle: 1 -> 2 -> 3 -> 1
    (n2, n4),                       // Branch to isolated node 4
]);

let mut tarjan = TarjanScc::new();
let mut sccs = Vec::new();

tarjan.run(&graph, |scc| {
    sccs.push(scc.to_vec());
});

assert_eq!(sccs.len(), 2); // Two SCCs: {1,2,3} and {4}
```

For an undirected graph (SCCs are just connected components):

``` rust
use petgraph::Graph;
use petgraph::algo::tarjan_scc;
use petgraph::prelude::*;

let mut graph: Graph<i32, (), Undirected> = Graph::new_undirected();
let a = graph.add_node(1);
let b = graph.add_node(2);
let c = graph.add_node(3);
let d = graph.add_node(4);

graph.extend_with_edges(&[
    (a, b), // Component 1: {1, 2}
    (c, d), // Component 2: {3, 4}
]);

let sccs = tarjan_scc(&graph);
assert_eq!(sccs.len(), 2); // Two connected components

for scc in &sccs {
    assert_eq!(scc.len(), 2); // Each component has 2 nodes
}
```
