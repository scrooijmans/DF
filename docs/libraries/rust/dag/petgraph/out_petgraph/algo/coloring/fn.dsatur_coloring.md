# Function dsatur_coloring Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/coloring.rs.html#63-108" class="src">Source</a>

``` rust
pub fn dsatur_coloring<G>(graph: G) -> (HashMap<G::NodeId, usize>, usize)where
    G: IntoEdges + IntoNodeIdentifiers + Visitable + NodeIndexable,
    G::NodeId: Eq + Hash,
```

Expand description

[DStatur algorithm](https://en.wikipedia.org/wiki/DSatur) to properly color a non weighted undirected graph.

This is a heuristic. So, it does not necessarily return a minimum coloring. The graph must be undirected. It should not contain loops.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/coloring/fn.dsatur_coloring.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: undirected graph without loops.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/coloring/fn.dsatur_coloring.html#returns" class="doc-anchor">§</a>Returns

Returns a tuple of:

- [`hashbrown::HashMap`](https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap") that associates to each `NodeId` its color.
- `usize`: the number of used colors.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/coloring/fn.dsatur_coloring.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O((\|V\| + \|E\|)log(\|V\|)**.
- Auxiliary space: **O(\|V\| + \|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/coloring/fn.dsatur_coloring.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::{Graph, Undirected};
use petgraph::algo::dsatur_coloring;

let mut graph: Graph<(), (), Undirected> = Graph::new_undirected();
let a = graph.add_node(());
let b = graph.add_node(());
let c = graph.add_node(());
let d = graph.add_node(());
let e = graph.add_node(());
let f = graph.add_node(());

graph.extend_with_edges(&[
    (a, b),
    (b, c),
    (c, d),
    (d, e),
    (e, f),
    (f, a),
]);

// a ----- b ----- c
// |               |
// |               |
// |               |
// f ----- e------ d

let (coloring, nb_colors) = dsatur_coloring(&graph);
assert_eq!(nb_colors, 2);
assert_ne!(coloring[&a], coloring[&b]);
```
