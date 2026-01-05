# Function steiner_tree Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/steiner_tree.rs.html#179-211" class="src">Source</a>

``` rust
pub fn steiner_tree<N, E, Ix>(
    graph: &UnGraph<N, E, Ix>,
    terminals: &[NodeIndex<Ix>],
) -> StableGraph<N, E, Undirected, Ix>where
    N: Default + Clone + Eq + Hash + Debug,
    E: Copy + Eq + Ord + Measure + BoundedMeasure,
    Ix: IndexType,
```

Expand description

[Steiner Tree](https://en.wikipedia.org/wiki/Steiner_tree_problem) algorithm.

Computes the Steiner tree of an undirected connected graph given a set of terminal nodes via [Kou’s algorithm](https://doi.org/10.1007/BF00288961). Implementation details are the same as in the [NetworkX implementation](https://networkx.org/documentation/stable/_modules/networkx/algorithms/approximation/steinertree.html#steiner_tree).

### <a href="https://docs.rs/petgraph/latest/petgraph/algo/steiner_tree/fn.steiner_tree.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: The undirected graph in which to find the Steiner tree.
- `terminals`: A slice of node indices representing the terminals for which the Steiner tree is computed.

### <a href="https://docs.rs/petgraph/latest/petgraph/algo/steiner_tree/fn.steiner_tree.html#returns" class="doc-anchor">§</a>Returns

A `StableGraph` containing the nodes and edges of the Steiner tree.

### <a href="https://docs.rs/petgraph/latest/petgraph/algo/steiner_tree/fn.steiner_tree.html#complexity" class="doc-anchor">§</a>Complexity

Time complexity: **O(\|S\| \|V\|²)**. where **\|V\|** the number of vertices (i.e nodes) and **\|S\|** the number of provided terminals.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/steiner_tree/fn.steiner_tree.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::steiner_tree::steiner_tree;
use petgraph::graph::UnGraph;
let mut graph = UnGraph::<(), i32>::default();
let a = graph.add_node(());
let b = graph.add_node(());
let c = graph.add_node(());
let d = graph.add_node(());
let e = graph.add_node(());
let f = graph.add_node(());
graph.extend_with_edges([
    (a, b, 7),
    (a, f, 6),
    (b, c, 1),
    (b, f, 5),
    (c, d, 1),
    (c, e, 3),
    (d, e, 1),
    (d, f, 4),
    (e, f, 10),
]);
let terminals = vec![a, c, e, f];
let tree = steiner_tree(&graph, &terminals);
assert_eq!(tree.edge_weights().sum::<i32>(), 12);
```
