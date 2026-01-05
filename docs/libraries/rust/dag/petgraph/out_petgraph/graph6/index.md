# Module graph6 Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph6/mod.rs.html#1-7" class="src">Source</a>

Expand description

Traits related to [graph6 format](https://users.cecs.anu.edu.au/~bdm/data/formats.txt) for undirected graphs.

## Traits<a href="https://docs.rs/petgraph/latest/petgraph/graph6/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.FromGraph6.html" class="trait" title="trait petgraph::graph6::FromGraph6">FromGraph6</a>  
A graph that can be converted from graph6 format string.

<a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html" class="trait" title="trait petgraph::graph6::ToGraph6">ToGraph6</a>  
A graph that can be converted to graph6 format string.

## Functions<a href="https://docs.rs/petgraph/latest/petgraph/graph6/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph6/fn.from_graph6_representation.html" class="fn" title="fn petgraph::graph6::from_graph6_representation">from_graph6_representation</a>  
Converts a graph6 format string into data can be used to construct an undirected graph. Returns a tuple containing the graph order and its edges.

<a href="https://docs.rs/petgraph/latest/petgraph/graph6/fn.get_graph6_representation.html" class="fn" title="fn petgraph::graph6::get_graph6_representation">get_graph6_representation</a>  
Converts a graph that implements GetAdjacencyMatrix and IntoNodeIdentifers into a graph6 format string.
