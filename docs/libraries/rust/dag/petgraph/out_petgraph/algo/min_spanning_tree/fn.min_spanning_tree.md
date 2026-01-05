# Function min_spanning_tree Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/min_spanning_tree.rs.html#90-117" class="src">Source</a>

``` rust
pub fn min_spanning_tree<G>(g: G) -> MinSpanningTree<G> ⓘwhere
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone + PartialOrd,
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable,
```

Expand description

Compute a *minimum spanning tree* of a graph.

The input graph is treated as if undirected.

Using Kruskal’s algorithm with runtime **O(\|E\| log \|E\|)**. We actually return a minimum spanning forest, i.e. a minimum spanning tree for each connected component of the graph.

The resulting graph has all the vertices of the input graph (with identical node indices), and **\|V\| - c** edges, where **c** is the number of connected components in `g`.

See also: [`min_spanning_tree_prim`](https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree_prim.html) for an implementation using Prim’s algorithm.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: an undirected graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree.html#returns" class="doc-anchor">§</a>Returns

- [`MinSpanningTree`](https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/struct.MinSpanningTree.html "struct petgraph::algo::min_spanning_tree::MinSpanningTree"): an iterator producing a minimum spanning forest of a graph. Use `from_elements` to create a graph from the resulting iterator.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|E\| log \|E\|)**.
- Auxiliary space: **O(\|V\| + \|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::min_spanning_tree;
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

let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&g));
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
