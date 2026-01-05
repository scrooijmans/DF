# Function is_isomorphic_matching Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/isomorphism.rs.html#827-856" class="src">Source</a>

``` rust
pub fn is_isomorphic_matching<G0, G1, NM, EM>(
    g0: G0,
    g1: G1,
    node_match: NM,
    edge_match: EM,
) -> boolwhere
    G0: NodeCompactIndexable + EdgeCount + DataMap + GetAdjacencyMatrix + GraphProp + IntoEdgesDirected,
    G1: NodeCompactIndexable + EdgeCount + DataMap + GetAdjacencyMatrix + GraphProp<EdgeType = G0::EdgeType> + IntoEdgesDirected,
    NM: FnMut(&G0::NodeWeight, &G1::NodeWeight) -> bool,
    EM: FnMut(&G0::EdgeWeight, &G1::EdgeWeight) -> bool,
```

Expand description

Return `true` if the graphs `g0` and `g1` are isomorphic.

Using the VF2 algorithm, examining both syntactic and semantic graph isomorphism (graph structure and matching node and edge weights).

The graphs should not be [multigraphs](https://en.wikipedia.org/wiki/Multigraph).
