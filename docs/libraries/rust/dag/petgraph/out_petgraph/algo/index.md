# Module algo Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#1-800" class="src">Source</a>

Expand description

This module contains most of `petgraph`’s algorithms to operate on graphs. Some, very simple search algorithms like depth-first search or breadth-first search are implemented in the [`visit`](https://docs.rs/petgraph/latest/petgraph/visit/index.html "mod petgraph::visit") module.

The `algo` module contains multiple submodules, each implementing a specific algorithm or set of algorithms. Some functions, like [`connected_components`](https://docs.rs/petgraph/latest/petgraph/algo/fn.connected_components.html "fn petgraph::algo::connected_components"), are implemented directly in this module.

It is a goal to gradually migrate the algorithms to be based on graph traits so that they are generally applicable. For now, some of these still require the `Graph` type.

## Re-exports<a href="https://docs.rs/petgraph/latest/petgraph/algo/index.html#reexports" class="anchor">§</a>

`pub use astar::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/astar/fn.astar.html" class="fn" title="fn petgraph::algo::astar::astar"><code>astar</code></a>`;`

`pub use bellman_ford::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/fn.bellman_ford.html" class="fn" title="fn petgraph::algo::bellman_ford::bellman_ford"><code>bellman_ford</code></a>`;`

`pub use bellman_ford::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/fn.find_negative_cycle.html" class="fn" title="fn petgraph::algo::bellman_ford::find_negative_cycle"><code>find_negative_cycle</code></a>`;`

`pub use bridges::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/bridges/fn.bridges.html" class="fn" title="fn petgraph::algo::bridges::bridges"><code>bridges</code></a>`;`

`pub use coloring::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/coloring/fn.dsatur_coloring.html" class="fn" title="fn petgraph::algo::coloring::dsatur_coloring"><code>dsatur_coloring</code></a>`;`

`pub use dijkstra::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.bidirectional_dijkstra.html" class="fn" title="fn petgraph::algo::dijkstra::bidirectional_dijkstra"><code>bidirectional_dijkstra</code></a>`;`

`pub use dijkstra::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html" class="fn" title="fn petgraph::algo::dijkstra::dijkstra"><code>dijkstra</code></a>`;`

`pub use feedback_arc_set::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/feedback_arc_set/fn.greedy_feedback_arc_set.html" class="fn" title="fn petgraph::algo::feedback_arc_set::greedy_feedback_arc_set"><code>greedy_feedback_arc_set</code></a>`;`

`pub use floyd_warshall::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/fn.floyd_warshall.html" class="fn" title="fn petgraph::algo::floyd_warshall::floyd_warshall"><code>floyd_warshall</code></a>`;`

`pub use isomorphism::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/isomorphism/fn.is_isomorphic.html" class="fn" title="fn petgraph::algo::isomorphism::is_isomorphic"><code>is_isomorphic</code></a>`;`

`pub use isomorphism::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/isomorphism/fn.is_isomorphic_matching.html" class="fn" title="fn petgraph::algo::isomorphism::is_isomorphic_matching"><code>is_isomorphic_matching</code></a>`;`

`pub use isomorphism::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/isomorphism/fn.is_isomorphic_subgraph.html" class="fn" title="fn petgraph::algo::isomorphism::is_isomorphic_subgraph"><code>is_isomorphic_subgraph</code></a>`;`

`pub use isomorphism::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/isomorphism/fn.is_isomorphic_subgraph_matching.html" class="fn" title="fn petgraph::algo::isomorphism::is_isomorphic_subgraph_matching"><code>is_isomorphic_subgraph_matching</code></a>`;`

`pub use isomorphism::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/isomorphism/fn.subgraph_isomorphisms_iter.html" class="fn" title="fn petgraph::algo::isomorphism::subgraph_isomorphisms_iter"><code>subgraph_isomorphisms_iter</code></a>`;`

`pub use johnson::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.johnson.html" class="fn" title="fn petgraph::algo::johnson::johnson"><code>johnson</code></a>`;`

`pub use k_shortest_path::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/k_shortest_path/fn.k_shortest_path.html" class="fn" title="fn petgraph::algo::k_shortest_path::k_shortest_path"><code>k_shortest_path</code></a>`;`

`pub use matching::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/fn.greedy_matching.html" class="fn" title="fn petgraph::algo::matching::greedy_matching"><code>greedy_matching</code></a>`;`

`pub use matching::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/fn.maximum_matching.html" class="fn" title="fn petgraph::algo::matching::maximum_matching"><code>maximum_matching</code></a>`;`

`pub use matching::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html" class="struct" title="struct petgraph::algo::matching::Matching"><code>Matching</code></a>`;`

`pub use maximal_cliques::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/maximal_cliques/fn.maximal_cliques.html" class="fn" title="fn petgraph::algo::maximal_cliques::maximal_cliques"><code>maximal_cliques</code></a>`;`

`pub use maximum_flow::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.dinics.html" class="fn" title="fn petgraph::algo::maximum_flow::dinics"><code>dinics</code></a>`;`

`pub use maximum_flow::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.ford_fulkerson.html" class="fn" title="fn petgraph::algo::maximum_flow::ford_fulkerson"><code>ford_fulkerson</code></a>`;`

`pub use min_spanning_tree::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree.html" class="fn" title="fn petgraph::algo::min_spanning_tree::min_spanning_tree"><code>min_spanning_tree</code></a>`;`

`pub use min_spanning_tree::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/fn.min_spanning_tree_prim.html" class="fn" title="fn petgraph::algo::min_spanning_tree::min_spanning_tree_prim"><code>min_spanning_tree_prim</code></a>`;`

`pub use page_rank::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/page_rank/fn.page_rank.html" class="fn" title="fn petgraph::algo::page_rank::page_rank"><code>page_rank</code></a>`;`

`pub use scc::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/kosaraju_scc/fn.scc.html" class="fn" title="fn petgraph::algo::scc::kosaraju_scc::scc"><code>scc</code></a>`;`Deprecated

`pub use scc::kosaraju_scc::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/kosaraju_scc/fn.kosaraju_scc.html" class="fn" title="fn petgraph::algo::scc::kosaraju_scc::kosaraju_scc"><code>kosaraju_scc</code></a>`;`

`pub use scc::tarjan_scc::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/fn.tarjan_scc.html" class="fn" title="fn petgraph::algo::scc::tarjan_scc::tarjan_scc"><code>tarjan_scc</code></a>`;`

`pub use scc::tarjan_scc::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/struct.TarjanScc.html" class="struct" title="struct petgraph::algo::scc::tarjan_scc::TarjanScc"><code>TarjanScc</code></a>`;`

`pub use simple_paths::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths.html" class="fn" title="fn petgraph::algo::simple_paths::all_simple_paths"><code>all_simple_paths</code></a>`;`

`pub use simple_paths::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/fn.all_simple_paths_multi.html" class="fn" title="fn petgraph::algo::simple_paths::all_simple_paths_multi"><code>all_simple_paths_multi</code></a>`;`

`pub use spfa::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/spfa/fn.spfa.html" class="fn" title="fn petgraph::algo::spfa::spfa"><code>spfa</code></a>`;`

`pub use steiner_tree::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/steiner_tree/fn.steiner_tree.html" class="fn" title="fn petgraph::algo::steiner_tree::steiner_tree"><code>steiner_tree</code></a>`;`

`pub use johnson::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.parallel_johnson.html" class="fn" title="fn petgraph::algo::johnson::parallel_johnson"><code>parallel_johnson</code></a>`;`

## Modules<a href="https://docs.rs/petgraph/latest/petgraph/algo/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/articulation_points/index.html" class="mod" title="mod petgraph::algo::articulation_points">articulation_points</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/astar/index.html" class="mod" title="mod petgraph::algo::astar">astar</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/index.html" class="mod" title="mod petgraph::algo::bellman_ford">bellman_ford</a>  
Bellman-Ford algorithms.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/bridges/index.html" class="mod" title="mod petgraph::algo::bridges">bridges</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/coloring/index.html" class="mod" title="mod petgraph::algo::coloring">coloring</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/index.html" class="mod" title="mod petgraph::algo::dijkstra">dijkstra</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/index.html" class="mod" title="mod petgraph::algo::dominators">dominators</a>  
Compute dominators of a control-flow graph.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/feedback_arc_set/index.html" class="mod" title="mod petgraph::algo::feedback_arc_set">feedback_arc_set</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/index.html" class="mod" title="mod petgraph::algo::floyd_warshall">floyd_warshall</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/ford_fulkerson/index.html" class="mod" title="mod petgraph::algo::ford_fulkerson">ford_fulkerson</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/isomorphism/index.html" class="mod" title="mod petgraph::algo::isomorphism">isomorphism</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/index.html" class="mod" title="mod petgraph::algo::johnson">johnson</a>  
Johnson’s algorithm implementation.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/k_shortest_path/index.html" class="mod" title="mod petgraph::algo::k_shortest_path">k_shortest_path</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/index.html" class="mod" title="mod petgraph::algo::matching">matching</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/maximal_cliques/index.html" class="mod" title="mod petgraph::algo::maximal_cliques">maximal_cliques</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/index.html" class="mod" title="mod petgraph::algo::maximum_flow">maximum_flow</a>  
Collection of algorithms for the [Maximum Flow Problem](https://en.wikipedia.org/wiki/Maximum_flow_problem).

<a href="https://docs.rs/petgraph/latest/petgraph/algo/min_spanning_tree/index.html" class="mod" title="mod petgraph::algo::min_spanning_tree">min_spanning_tree</a>  
Minimum Spanning Tree algorithms.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/page_rank/index.html" class="mod" title="mod petgraph::algo::page_rank">page_rank</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/scc/index.html" class="mod" title="mod petgraph::algo::scc">scc</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/simple_paths/index.html" class="mod" title="mod petgraph::algo::simple_paths">simple_paths</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/spfa/index.html" class="mod" title="mod petgraph::algo::spfa">spfa</a>  
Shortest Path Faster Algorithm.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/steiner_tree/index.html" class="mod" title="mod petgraph::algo::steiner_tree">steiner_tree</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/index.html" class="mod" title="mod petgraph::algo::tred">tred</a>  
Compute the transitive reduction and closure of a directed acyclic graph

## Structs<a href="https://docs.rs/petgraph/latest/petgraph/algo/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>  
An algorithm error: a cycle was found in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html" class="struct" title="struct petgraph::algo::DfsSpace">DfsSpace</a>  
Workspace for a graph traversal.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.NegativeCycle.html" class="struct" title="struct petgraph::algo::NegativeCycle">NegativeCycle</a>  
An algorithm error: a cycle of negative weights was found in the graph.

## Traits<a href="https://docs.rs/petgraph/latest/petgraph/algo/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a>  
<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html" class="trait" title="trait petgraph::algo::FloatMeasure">FloatMeasure</a>  
A floating-point measure.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.Measure.html" class="trait" title="trait petgraph::algo::Measure">Measure</a>  
Associated data that can be used for measures (such as length).

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html" class="trait" title="trait petgraph::algo::PositiveMeasure">PositiveMeasure</a>  
Some measure of positive numbers, assuming positive float-pointing numbers

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html" class="trait" title="trait petgraph::algo::UnitMeasure">UnitMeasure</a>  
A floating-point measure that can be computed from `usize` and with a default measure of proximity.

## Functions<a href="https://docs.rs/petgraph/latest/petgraph/algo/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.condensation.html" class="fn" title="fn petgraph::algo::condensation">condensation</a>  
[Graph](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html "struct petgraph::graph::Graph") Condense every strongly connected component into a single node and return the result.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.connected_components.html" class="fn" title="fn petgraph::algo::connected_components">connected_components</a>  
Return the number of connected components of the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.has_path_connecting.html" class="fn" title="fn petgraph::algo::has_path_connecting">has_path_connecting</a>  
Check if there exists a path starting at `from` and reaching `to`.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_bipartite_undirected.html" class="fn" title="fn petgraph::algo::is_bipartite_undirected">is_bipartite_undirected</a>  
Return `true` if the graph\* is bipartite.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_cyclic_directed.html" class="fn" title="fn petgraph::algo::is_cyclic_directed">is_cyclic_directed</a>  
Return `true` if the input directed graph contains a cycle.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.is_cyclic_undirected.html" class="fn" title="fn petgraph::algo::is_cyclic_undirected">is_cyclic_undirected</a>  
Return `true` if the input graph contains a cycle.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/fn.toposort.html" class="fn" title="fn petgraph::algo::toposort">toposort</a>  
Perform a topological sort of a directed graph.
