# Function from_graph6_representation Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph6/graph6_decoder.rs.html#32-46" class="src">Source</a>

``` rust
pub fn from_graph6_representation<Ix>(
    graph6_representation: String,
) -> (usize, Vec<(Ix, Ix)>)where
    Ix: IndexType,
```

Expand description

Converts a graph6 format string into data can be used to construct an undirected graph. Returns a tuple containing the graph order and its edges.
