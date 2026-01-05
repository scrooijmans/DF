# Function is_bipartite_undirected Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#557-605" class="src">Source</a>

``` rust
pub fn is_bipartite_undirected<G, N, VM>(g: G, start: N) -> boolwhere
    G: GraphRef + Visitable<NodeId = N, Map = VM> + IntoNeighbors<NodeId = N>,
    N: Copy + PartialEq + Debug,
    VM: VisitMap<N>,
```

Expand description

Return `true` if the graph\* is bipartite.

A graph is bipartite if its nodes can be divided into two disjoint and indepedent sets U and V such that every edge connects U to one in V.

This algorithm implements 2-coloring algorithm based on the BFS algorithm. Always treats the input graph as if undirected.

\* The algorithm checks only the subgraph that is reachable from the `start`.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_bipartite_undirected.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: an input graph.
- `start`: some node of the graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_bipartite_undirected.html#returns" class="doc-anchor">§</a>Returns

- `true`: if the subgraph accessible from the start node is bipartite.
- `false`: if such a subgraph is not bipartite.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_bipartite_undirected.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.
