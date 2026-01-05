# Function dag_to_toposorted_adjacency_list Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/tred.rs.html#71-94" class="src">Source</a>

``` rust
pub fn dag_to_toposorted_adjacency_list<G, Ix: IndexType>(
    g: G,
    toposort: &[G::NodeId],
) -> (UnweightedList<Ix>, Vec<Ix>)where
    G: GraphBase + IntoNeighborsDirected + NodeCompactIndexable + NodeCount,
    G::NodeId: IndexType,
```

Expand description

Creates a representation of the same graph respecting topological order for use in `tred::dag_transitive_reduction_closure`.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/fn.dag_to_toposorted_adjacency_list.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: a directed acyclic graph.
- `toposort`: a topological order on the node indices of `g` (for example obtained from [`toposort`](https://docs.rs/petgraph/latest/petgraph/algo/fn.toposort.html "fn petgraph::algo::toposort")).

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/fn.dag_to_toposorted_adjacency_list.html#returns" class="doc-anchor">§</a>Returns

Returns a tuple of:

- [`UnweightedList`](https://docs.rs/petgraph/latest/petgraph/adj/type.UnweightedList.html "type petgraph::adj::UnweightedList") `res` graph.
- `Vec`: reciprocal of the topological sort `revmap`.

`res` is the same graph as `g` with the following differences:

- Node and edge weights are stripped,
- Node indices are replaced by the corresponding rank in `toposort`,
- Iterating on the neighbors of a node respects topological order.

`revmap` is handy to get back to map indices in `g` to indices in `res`.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/fn.dag_to_toposorted_adjacency_list.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**.
- Auxiliary space: **O(\|V\| + \|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/fn.dag_to_toposorted_adjacency_list.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::prelude::*;
use petgraph::graph::DefaultIx;
use petgraph::visit::IntoNeighbors;
use petgraph::algo::tred::dag_to_toposorted_adjacency_list;

let mut g = Graph::<&str, (), Directed, DefaultIx>::new();
let second = g.add_node("second child");
let top = g.add_node("top");
let first = g.add_node("first child");
g.extend_with_edges(&[(top, second), (top, first), (first, second)]);

let toposort = vec![top, first, second];

let (res, revmap) = dag_to_toposorted_adjacency_list(&g, &toposort);

// let's compute the children of top in topological order
let children: Vec<NodeIndex> = res
    .neighbors(revmap[top.index()])
    .map(|ix: NodeIndex| toposort[ix.index()])
    .collect();
assert_eq!(children, vec![first, second])
```
