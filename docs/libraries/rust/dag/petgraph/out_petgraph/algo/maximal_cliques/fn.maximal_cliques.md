# Function maximal_cliques Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/maximal_cliques.rs.html#115-125" class="src">Source</a>

``` rust
pub fn maximal_cliques<G>(g: G) -> Vec<HashSet<G::NodeId>>where
    G: GetAdjacencyMatrix + IntoNodeIdentifiers + IntoNeighbors,
    G::NodeId: Eq + Hash,
```

Expand description

Find all maximal cliques in an undirected graph using [Bron–Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm) with pivoting. Also works on symmetric directed graphs, see the note below.

A clique is a set of nodes such that every node connects to every other. A maximal clique is a clique that cannot be extended by including one more adjacent vertex. A graph may have multiple maximal cliques.

This method may also be called on directed graphs, but one needs to ensure that if an edge (u, v) exists, then (v, u) also exists.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximal_cliques/fn.maximal_cliques.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: The graph to find maximal cliques in.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximal_cliques/fn.maximal_cliques.html#returns" class="doc-anchor">§</a>Returns

- `Vec<HashSet>`: A vector of [`hashbrown::HashSet`](https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet") making up the maximal cliques in the graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximal_cliques/fn.maximal_cliques.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(3^(\|V\|/3))**
- Auxiliary space: **O(\|V\|² + \|V\|k)**.

where **\|V\|** is the number of nodes and k is the number of maximal cliques in the graph (possibly up to *3^(\|V\|/3)*\* many).

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/maximal_cliques/fn.maximal_cliques.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::algo::maximal_cliques;
use petgraph::graph::UnGraph;

let mut g = UnGraph::<i32, ()>::from_edges([(0, 1), (0, 2), (1, 2), (2, 3)]);
g.add_node(4);
// The example graph:
//
// 0 --- 2 -- 3
//  \   /
//   \ /
//    1       4
//
// maximal cliques: {4}, {2, 3}, {0, 1, 2}
// Output the result
let cliques = maximal_cliques(&g);
println!("{:?}", cliques);
// [
//   {NodeIndex(4)},
//   {NodeIndex(0), NodeIndex(1), NodeIndex(2)},
//   {NodeIndex(2), NodeIndex(3)}
// ]
```
