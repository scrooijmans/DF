# Function is_cyclic_directed Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#284-295" class="src">Source</a>

``` rust
pub fn is_cyclic_directed<G>(g: G) -> boolwhere
    G: IntoNodeIdentifiers + IntoNeighbors + Visitable,
```

Expand description

Return `true` if the input directed graph contains a cycle.

This implementation is recursive; use [`toposort`](https://docs.rs/petgraph/latest/petgraph/algo/fn.toposort.html "fn petgraph::algo::toposort") if an alternative is needed.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_cyclic_directed.html#arguments" class="doc-anchor">§</a>Arguments:

`g`: a directed graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_cyclic_directed.html#returns" class="doc-anchor">§</a>Returns

`true`: if the input graph contains a cycle. `false`: otherwise.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_cyclic_directed.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.
