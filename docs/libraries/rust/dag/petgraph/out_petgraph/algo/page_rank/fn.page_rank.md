# Function page_rank Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/page_rank.rs.html#63-107" class="src">Source</a>

``` rust
pub fn page_rank<G, D>(graph: G, damping_factor: D, nb_iter: usize) -> Vec<D>where
    G: NodeCount + IntoEdges + NodeIndexable,
    D: UnitMeasure + Copy,
```

Expand description

Page Rank algorithm.

Computes the ranks of every node in a graph using the [Page Rank algorithm](https://en.wikipedia.org/wiki/PageRank).

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/page_rank/fn.page_rank.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: a directed graph.
- `damping_factor`: a value in range `0.0 <= damping_factor <= 1.0`.
- `nb_iter`: number of iterations of the main loop.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/page_rank/fn.page_rank.html#returns" class="doc-anchor">§</a>Returns

- A `Vec` mapping each node index to its rank.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/page_rank/fn.page_rank.html#panics" class="doc-anchor">§</a>Panics

The damping factor should be a measure (like `f32` or `f64`) between 0 and 1 (0 and 1 included). Otherwise, it panics.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/page_rank/fn.page_rank.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(n\|V\|²\|E\|)**.
- Auxiliary space: **O(\|V\| + \|E\|)**.

where **n** is the number of iterations, **\|V\|** the number of vertices (i.e nodes) and **\|E\|** the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/page_rank/fn.page_rank.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::page_rank;
let mut g: Graph<(), usize> = Graph::new();
assert_eq!(page_rank(&g, 0.5_f64, 1), vec![]); // empty graphs have no node ranks.
let a = g.add_node(());
let b = g.add_node(());
let c = g.add_node(());
let d = g.add_node(());
let e = g.add_node(());
g.extend_with_edges(&[(0, 1), (0, 3), (1, 2), (1, 3)]);
// With the following dot representation.
//digraph {
//    0 [ label = "()" ]
//    1 [ label = "()" ]
//    2 [ label = "()" ]
//    3 [ label = "()" ]
//    4 [ label = "()" ]
//    0 -> 1 [ label = "0.0" ]
//    0 -> 3 [ label = "0.0" ]
//    1 -> 2 [ label = "0.0" ]
//    1 -> 3 [ label = "0.0" ]
//}
let damping_factor = 0.7_f32;
let number_iterations = 10;
let output_ranks = page_rank(&g, damping_factor, number_iterations);
let expected_ranks = vec![0.14685437, 0.20267677, 0.22389607, 0.27971846, 0.14685437];
assert_eq!(expected_ranks, output_ranks);
```
