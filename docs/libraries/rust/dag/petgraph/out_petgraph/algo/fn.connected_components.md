# Function connected_components Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#136-152" class="src">Source</a>

``` rust
pub fn connected_components<G>(g: G) -> usizewhere
    G: NodeCompactIndexable + IntoEdgeReferences,
```

Expand description

Return the number of connected components of the graph.

For a directed graph, this is the *weakly* connected components.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.connected_components.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: an input graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.connected_components.html#returns" class="doc-anchor">§</a>Returns

- `usize`: the number of connected components if `g` is undirected or number of *weakly* connected components if `g` is directed.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.connected_components.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: amortized **O(\|E\| + \|V\|log\|V\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.connected_components.html#example" class="doc-anchor">§</a>Example

``` rust
use petgraph::Graph;
use petgraph::algo::connected_components;
use petgraph::prelude::*;

let mut graph : Graph<(),(),Directed>= Graph::new();
let a = graph.add_node(()); // node with no weight
let b = graph.add_node(());
let c = graph.add_node(());
let d = graph.add_node(());
let e = graph.add_node(());
let f = graph.add_node(());
let g = graph.add_node(());
let h = graph.add_node(());

graph.extend_with_edges(&[
    (a, b),
    (b, c),
    (c, d),
    (d, a),
    (e, f),
    (f, g),
    (g, h),
    (h, e)
]);
// a ----> b       e ----> f
// ^       |       ^       |
// |       v       |       v
// d <---- c       h <---- g

assert_eq!(connected_components(&graph),2);
graph.add_edge(b,e,());
assert_eq!(connected_components(&graph),1);
```
