# Module graphmap Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graphmap.rs.html#1-1550" class="src">Source</a>

Expand description

`GraphMap<N, E, Ty>` is a graph datastructure where node values are mapping keys.

## Structs<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.AllEdges.html" class="struct" title="struct petgraph::graphmap::AllEdges">AllEdges</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.AllEdgesMut.html" class="struct" title="struct petgraph::graphmap::AllEdgesMut">AllEdgesMut</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Edges.html" class="struct" title="struct petgraph::graphmap::Edges">Edges</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.EdgesDirected.html" class="struct" title="struct petgraph::graphmap::EdgesDirected">EdgesDirected</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>  
`GraphMap<N, E, Ty>` is a graph datastructure using an associative array of its node weights `N`.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Neighbors.html" class="struct" title="struct petgraph::graphmap::Neighbors">Neighbors</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.NeighborsDirected.html" class="struct" title="struct petgraph::graphmap::NeighborsDirected">NeighborsDirected</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.NodeIdentifiers.html" class="struct" title="struct petgraph::graphmap::NodeIdentifiers">NodeIdentifiers</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.NodeReferences.html" class="struct" title="struct petgraph::graphmap::NodeReferences">NodeReferences</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Nodes.html" class="struct" title="struct petgraph::graphmap::Nodes">Nodes</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdges.html" class="struct" title="struct petgraph::graphmap::ParAllEdges">ParAllEdges</a>  
A [ParallelIterator](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html "trait rayon::iter::ParallelIterator") over this graph’s edges.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html" class="struct" title="struct petgraph::graphmap::ParAllEdgesMut">ParAllEdgesMut</a>  
A [ParallelIterator](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html "trait rayon::iter::ParallelIterator") over this graph’s edges by mutable reference.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParNodes.html" class="struct" title="struct petgraph::graphmap::ParNodes">ParNodes</a>  
A [ParallelIterator](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html "trait rayon::iter::ParallelIterator") over this graph’s nodes.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>  
A reference that is hashed and compared by its pointer value.

## Traits<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>  
A trait group for `GraphMap`’s node identifier.

## Type Aliases<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/type.DiGraphMap.html" class="type" title="type petgraph::graphmap::DiGraphMap">DiGraphMap</a>  
A `GraphMap` with directed edges.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/type.UnGraphMap.html" class="type" title="type petgraph::graphmap::UnGraphMap">UnGraphMap</a>  
A `GraphMap` with undirected edges.
