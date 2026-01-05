# Function toposort Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#211-266" class="src">Source</a>

``` rust
pub fn toposort<G>(
    g: G,
    space: Option<&mut DfsSpace<G::NodeId, G::Map>>,
) -> Result<Vec<G::NodeId>, Cycle<G::NodeId>>where
    G: IntoNeighborsDirected + IntoNodeIdentifiers + Visitable,
```

Expand description

Perform a topological sort of a directed graph.

`toposort` returns `Err` on graphs with cycles. To handle graphs with cycles, use the one of scc algorithms or [`DfsPostOrder`](https://docs.rs/petgraph/latest/petgraph/visit/struct.DfsPostOrder.html "struct petgraph::visit::DfsPostOrder") instead of this function.

The implementation is iterative.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.toposort.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: an acyclic directed graph.
- `space`: optional [`DfsSpace`](https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html "struct petgraph::algo::DfsSpace"). If `space` is not `None`, it is used instead of creating a new workspace for graph traversal.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.toposort.html#returns" class="doc-anchor">§</a>Returns

- `Ok`: a vector of nodes in topological order: each node is ordered before its successors (if the graph was acyclic).
- `Err`: [`Cycle`](https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html "struct petgraph::algo::Cycle") if the graph was not acyclic. Self loops are also cycles this case.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.toposort.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.
