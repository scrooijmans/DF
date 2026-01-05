# Function all_simple_paths Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/simple_paths.rs.html#70-142" class="src">Source</a>

``` rust
pub fn all_simple_paths<TargetColl, G, S>(
    graph: G,
    from: G::NodeId,
    to: G::NodeId,
    min_intermediate_nodes: usize,
    max_intermediate_nodes: Option<usize>,
) -> impl Iterator<Item = TargetColl>where
    G: NodeCount + IntoNeighborsDirected,
    G::NodeId: Eq + Hash,
    TargetColl: FromIterator<G::NodeId>,
    S: BuildHasher + Default,
```

Expand description

Calculate all simple paths with specified constraints from node `from` to node `to`.

A simple path is a path without repeating nodes. The number of simple paths between a given pair of vertices can grow exponentially, reaching `O(|V|!)` on complete graphs with `|V|` vertices.

So if you have a large enough graph, be prepared to wait on the results for years. Or consider extracting only part of the simple paths using the adapter [`Iterator::take`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take "method core::iter::traits::iterator::Iterator::take"). Also note, that this algorithm does not check that a path exists between `from` and `to`. This may lead to very long running times and it may be worth it to check if a path exists before running this algorithm on large graphs.

This algorithm is adapted from [NetworkX](https://networkx.github.io/documentation/stable/reference/algorithms/generated/networkx.algorithms.simple_paths.all_simple_paths.html).

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: an input graph.
- `from`: an initial node of desired paths.
- `to`: a target node of desired paths.
- `min_intermediate_nodes`: the minimum number of nodes in the desired paths.
- `max_intermediate_nodes`: the maximum number of nodes in the desired paths (optional).

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths.html#returns" class="doc-anchor">§</a>Returns

Returns an iterator that produces all simple paths from `from` node to `to`, which contains at least `min_intermediate_nodes` and at most `max_intermediate_nodes` intermediate nodes, if given, or limited by the graph’s order otherwise.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: for computing the first **k** paths, the running time will be **O(k\|V\| + k\|E\|)**.
- Auxillary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths.html#example" class="doc-anchor">§</a>Example

``` rust
use std::collections::hash_map::RandomState;
use petgraph::{algo, prelude::*};

let mut graph = DiGraph::<&str, i32>::new();

let a = graph.add_node("a");
let b = graph.add_node("b");
let c = graph.add_node("c");
let d = graph.add_node("d");

graph.extend_with_edges(&[(a, b, 1), (b, c, 1), (c, d, 1), (a, b, 1), (b, d, 1)]);

let paths = algo::all_simple_paths::<Vec<_>, _, RandomState>(&graph, a, d, 0, None)
  .collect::<Vec<_>>();

assert_eq!(paths.len(), 4);


// Take only 2 paths.
let paths = algo::all_simple_paths::<Vec<_>, _, RandomState>(&graph, a, d, 0, None)
  .take(2)
  .collect::<Vec<_>>();

assert_eq!(paths.len(), 2);
```
