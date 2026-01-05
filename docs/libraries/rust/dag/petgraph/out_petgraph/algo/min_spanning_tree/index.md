# Module min_spanning_tree Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/min_spanning_tree.rs.html#1-377" class="src">Source</a>

Expand description

Minimum Spanning Tree algorithms.

## Structs<a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/struct.MinSpanningTree.html" class="struct" title="struct petgraph::algo::min_spanning_tree::MinSpanningTree">MinSpanningTree</a>  
An iterator producing a minimum spanning forest of a graph. It will first iterate all Node elements from original graph, then iterate Edge elements from computed minimum spanning forest.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/struct.MinSpanningTreePrim.html" class="struct" title="struct petgraph::algo::min_spanning_tree::MinSpanningTreePrim">MinSpanningTreePrim</a>  
An iterator producing a minimum spanning tree of a graph. It will first iterate all Node elements from original graph, then iterate Edge elements from computed minimum spanning tree.

## Functions<a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree.html" class="fn" title="fn petgraph::algo::min_spanning_tree::min_spanning_tree">min_spanning_tree</a>  
Compute a *minimum spanning tree* of a graph.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree_prim.html" class="fn" title="fn petgraph::algo::min_spanning_tree::min_spanning_tree_prim">min_spanning_tree_prim</a>  
Compute a *minimum spanning tree* of a graph using Prim’s algorithm.
