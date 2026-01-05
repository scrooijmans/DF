# Function get_graph6_representation Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph6/graph6_encoder.rs.html#37-49" class="src">Source</a>

``` rust
pub fn get_graph6_representation<G>(graph: G) -> Stringwhere
    G: GetAdjacencyMatrix + IntoNodeIdentifiers,
```

Expand description

Converts a graph that implements GetAdjacencyMatrix and IntoNodeIdentifers into a graph6 format string.
