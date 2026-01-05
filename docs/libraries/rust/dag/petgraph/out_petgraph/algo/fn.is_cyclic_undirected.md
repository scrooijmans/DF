# Function is_cyclic_undirected Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#170-185" class="src">Source</a>

``` rust
pub fn is_cyclic_undirected<G>(g: G) -> boolwhere
    G: NodeIndexable + IntoEdgeReferences,
```

Expand description

Return `true` if the input graph contains a cycle.

Always treats the input graph as if undirected.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_cyclic_undirected.html#arguments" class="doc-anchor">§</a>Arguments:

`g`: an input graph that always treated as undirected.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_cyclic_undirected.html#returns" class="doc-anchor">§</a>Returns

`true`: if the input graph contains a cycle. `false`: otherwise.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_cyclic_undirected.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: amortized **O(\|E\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.
