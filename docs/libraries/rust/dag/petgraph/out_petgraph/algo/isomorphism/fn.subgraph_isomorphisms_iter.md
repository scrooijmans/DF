# Function subgraph_isomorphisms_iter Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/isomorphism.rs.html#959-990" class="src">Source</a>

``` rust
pub fn subgraph_isomorphisms_iter<'a, G0, G1, NM, EM>(
    g0: &'a G0,
    g1: &'a G1,
    node_match: &'a mut NM,
    edge_match: &'a mut EM,
) -> Option<impl Iterator<Item = Vec<usize>> + 'a>where
    G0: 'a + NodeCompactIndexable + EdgeCount + DataMap + GetAdjacencyMatrix + GraphProp + IntoEdgesDirected,
    G1: 'a + NodeCompactIndexable + EdgeCount + DataMap + GetAdjacencyMatrix + GraphProp<EdgeType = G0::EdgeType> + IntoEdgesDirected,
    NM: 'a + FnMut(&G0::NodeWeight, &G1::NodeWeight) -> bool,
    EM: 'a + FnMut(&G0::EdgeWeight, &G1::EdgeWeight) -> bool,
```

Expand description

Using the VF2 algorithm, examine both syntactic and semantic graph isomorphism (graph structure and matching node and edge weights) and, if `g0` is isomorphic to a subgraph of `g1`, return the mappings between them.

The graphs should not be [multigraphs](https://en.wikipedia.org/wiki/Multigraph).
