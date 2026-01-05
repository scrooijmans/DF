# Function is_isomorphic Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/isomorphism.rs.html#801-817" class="src">Source</a>

``` rust
pub fn is_isomorphic<G0, G1>(g0: G0, g1: G1) -> boolwhere
    G0: NodeCompactIndexable + EdgeCount + GetAdjacencyMatrix + GraphProp + IntoNeighborsDirected,
    G1: NodeCompactIndexable + EdgeCount + GetAdjacencyMatrix + GraphProp<EdgeType = G0::EdgeType> + IntoNeighborsDirected,
```

Expand description

Return `true` if the graphs `g0` and `g1` are isomorphic.

Using the VF2 algorithm, only matching graph syntactically (graph structure).

The graphs should not be [multigraphs](https://en.wikipedia.org/wiki/Multigraph).

**Reference**

- Luigi P. Cordella, Pasquale Foggia, Carlo Sansone, Mario Vento; *A (Sub)Graph Isomorphism Algorithm for Matching Large Graphs*
