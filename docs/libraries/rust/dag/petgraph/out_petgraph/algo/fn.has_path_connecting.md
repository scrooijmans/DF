# Function has_path_connecting Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#369-383" class="src">Source</a>

``` rust
pub fn has_path_connecting<G>(
    g: G,
    from: G::NodeId,
    to: G::NodeId,
    space: Option<&mut DfsSpace<G::NodeId, G::Map>>,
) -> boolwhere
    G: IntoNeighbors + Visitable,
```

Expand description

Check if there exists a path starting at `from` and reaching `to`.

If `from` and `to` are equal, this function returns true.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.has_path_connecting.html#arguments" class="doc-anchor">§</a>Arguments:

- `g`: an input graph.
- `from`: the first node of a desired path.
- `to`: the last node of a desired path.
- `space`: optional [`DfsSpace`](https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html "struct petgraph::algo::DfsSpace"). If `space` is not `None`, it is used instead of creating a new workspace for graph traversal.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.has_path_connecting.html#returns" class="doc-anchor">§</a>Returns

- `true`: if there exists a path starting at `from` and reaching `to` or `from` and `to` are equal.
- `false`: otherwise.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.has_path_connecting.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**.
- Auxiliary space: **O(\|V\|)** or **O(1)** if `space` was provided.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.
