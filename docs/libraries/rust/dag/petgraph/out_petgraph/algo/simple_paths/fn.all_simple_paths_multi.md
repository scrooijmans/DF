# Function all_simple_paths_multi Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/simple_paths.rs.html#207-276" class="src">Source</a>

``` rust
pub fn all_simple_paths_multi<'a, TargetColl, G, S>(
    graph: G,
    from: G::NodeId,
    to: &'a HashSet<G::NodeId, S>,
    min_intermediate_nodes: usize,
    max_intermediate_nodes: Option<usize>,
) -> impl Iterator<Item = TargetColl> + 'awhere
    G: NodeCount + IntoNeighborsDirected + 'a,
    <G as IntoNeighborsDirected>::NeighborsDirected: 'a,
    G::NodeId: Eq + Hash,
    TargetColl: FromIterator<G::NodeId>,
    S: BuildHasher + Default,
```

Expand description

Calculate all simple paths from a source node to any of several target nodes.

This function is a variant of [`all_simple_paths`](https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths.html "fn petgraph::algo::simple_paths::all_simple_paths") that accepts a `HashSet` of target nodes instead of a single one. A path is yielded as soon as it reaches any node in the `to` set.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths_multi.html#performance-considerations" class="doc-anchor">§</a>Performance Considerations

The efficiency of this function hinges on the graph’s structure. It provides significant performance gains on graphs where paths share long initial segments (e.g., trees and DAGs), as the benefit of a single traversal outweighs the `HashSet` lookup overhead.

Conversely, in dense graphs where paths diverge quickly or for targets very close to the source, the lookup overhead could make repeated calls to [`all_simple_paths`](https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths.html "fn petgraph::algo::simple_paths::all_simple_paths") a faster alternative.

**Note**: If security is not a concern, a faster hasher (e.g., `FxBuildHasher`) can be specified to minimize the `HashSet` lookup overhead.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths_multi.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: an input graph.
- `from`: an initial node of desired paths.
- `to`: a `HashSet` of target nodes. A path is yielded as soon as it reaches any node in this set.
- `min_intermediate_nodes`: the minimum number of nodes in the desired paths.
- `max_intermediate_nodes`: the maximum number of nodes in the desired paths (optional).

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths_multi.html#returns" class="doc-anchor">§</a>Returns

Returns an iterator that produces all simple paths from `from` node to any node in the `to` set, which contains at least `min_intermediate_nodes` and at most `max_intermediate_nodes` intermediate nodes, if given, or limited by the graph’s order otherwise.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths_multi.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: for computing the first **k** paths, the running time will be **O(k\|V\| + k\|E\|)**.
- Auxillary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths_multi.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::{algo, prelude::*};
use hashbrown::HashSet;
use std::collections::hash_map::RandomState;

let mut graph = DiGraph::<&str, i32>::new();

let a = graph.add_node("a");
let b = graph.add_node("b");
let c = graph.add_node("c");
let d = graph.add_node("d");
graph.extend_with_edges(&[(a, b, 1), (b, c, 1), (b, d, 1)]);

// Find paths from "a" to either "c" or "d".
let targets = HashSet::from_iter([c, d]);
let mut paths = algo::all_simple_paths_multi::<Vec<_>, _, RandomState>(&graph, a, &targets, 0, None)
    .collect::<Vec<_>>();

paths.sort_by_key(|p| p.clone());
let expected_paths = vec![
    vec![a, b, c],
    vec![a, b, d],
];

assert_eq!(paths, expected_paths);
```
