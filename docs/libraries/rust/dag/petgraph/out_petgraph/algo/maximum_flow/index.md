# Module maximum_flow Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/maximum_flow/mod.rs.html#1-23" class="src">Source</a>

Expand description

Collection of algorithms for the [Maximum Flow Problem](https://en.wikipedia.org/wiki/Maximum_flow_problem).

Both algorithms solve the maximum flow problem and compute the same maximum flow value, although they may differ in how much flow is assigned to each edge in the resulting flow.

[dinics](https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.dinics.html "fn petgraph::algo::maximum_flow::dinics") and [ford_fulkerson](https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.ford_fulkerson.html "fn petgraph::algo::maximum_flow::ford_fulkerson") have different time complexities, and their performance can vary significantly depending on the input graph. In general, [dinics](https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.dinics.html "fn petgraph::algo::maximum_flow::dinics") is faster, especially on dense graphs, graphs with unit capacities, and bipartite graphs. [ford_fulkerson](https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.ford_fulkerson.html "fn petgraph::algo::maximum_flow::ford_fulkerson") may be a better choice when working with small or sparse graphs.

For more information about each algorithm and their detailed time complexity, check their respective documentation.

## Functions<a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.dinics.html" class="fn" title="fn petgraph::algo::maximum_flow::dinics">dinics</a>  
Compute the maximum flow from `source` to `destination` in a directed graph. Implements [Dinic’s (or Dinitz’s) algorithm](https://en.wikipedia.org/wiki/Dinic%27s_algorithm), which builds successive level graphs using breadth-first search and finds blocking flows within them through depth-first searches.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/maximum_flow/fn.ford_fulkerson.html" class="fn" title="fn petgraph::algo::maximum_flow::ford_fulkerson">ford_fulkerson</a>  
[Ford-Fulkerson](https://en.wikipedia.org/wiki/Ford%E2%80%93Fulkerson_algorithm) algorithm in the [Edmonds-Karp](https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm) variation. Computes the [maximum flow](https://en.wikipedia.org/wiki/Maximum_flow_problem) from `source` to `destination` in a weighted directed graph.
