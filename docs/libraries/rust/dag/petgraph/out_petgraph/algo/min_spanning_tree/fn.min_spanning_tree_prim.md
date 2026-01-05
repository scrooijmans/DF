# Function min_spanning_tree_prim Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/min_spanning_tree.rs.html#262-280" class="src">Source</a>

``` rust
pub fn min_spanning_tree_prim<G>(g: G) -> MinSpanningTreePrim<G> ⓘwhere
    G::EdgeWeight: PartialOrd,
    G: IntoNodeReferences + IntoEdgeReferences,
```

Expand description

Compute a *minimum spanning tree* of a graph using Prim’s algorithm.

Graph is treated as if undirected. The computed minimum spanning tree can be wrong if this is not true.

Graph is treated as if connected (has only 1 component). Otherwise, the resulting graph will only contain edges for an arbitrary minimum spanning tree for a single component.

The resulting graph has all the vertices of the input graph (with identical node indices), and **\|V\| - 1** edges if input graph is connected, and \|W\| edges if disconnected, where \|W\| \< \|V\| - 1.

See also: [`min_spanning_tree`](https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree.html) for an implementation using Kruskal’s algorithm and support for minimum spanning forest.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree_prim.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: an undirected graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree_prim.html#returns" class="doc-anchor">§</a>Returns

- [`MinSpanningTreePrim`](https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/struct.MinSpanningTreePrim.html "struct petgraph::algo::min_spanning_tree::MinSpanningTreePrim"): an iterator producing a minimum spanning tree of a graph. Use `from_elements` to create a graph from the resulting iterator.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree_prim.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|E\| log \|V\|)**.
- Auxiliary space: **O(\|V\| + \|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree_prim.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::min_spanning_tree_prim;
use petgraph::data::FromElements;
use petgraph::graph::UnGraph;

let mut g = Graph::new_undirected();
let a = g.add_node(());
let b = g.add_node(());
let c = g.add_node(());
let d = g.add_node(());
let e = g.add_node(());
let f = g.add_node(());
g.extend_with_edges(&[
    (0, 1, 2.0),
    (0, 3, 4.0),
    (1, 2, 1.0),
    (1, 5, 7.0),
    (2, 4, 5.0),
    (4, 5, 1.0),
    (3, 4, 1.0),
]);

// The graph looks like this:
//     2       1
// a ----- b ----- c
// | 4     | 7     |
// d       f       | 5
// | 1     | 1     |
// \------ e ------/

let mst = UnGraph::<_, _>::from_elements(min_spanning_tree_prim(&g));
assert_eq!(g.node_count(), mst.node_count());
assert_eq!(mst.node_count() - 1, mst.edge_count());

// The resulting minimum spanning tree looks like this:
//     2       1
// a ----- b ----- c
// | 4
// d       f
// | 1     | 1
// \------ e

let mut edge_weight_vec = mst.edge_weights().cloned().collect::<Vec<_>>();
edge_weight_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
assert_eq!(edge_weight_vec , vec![1.0, 1.0, 1.0, 2.0, 4.0]);
```
