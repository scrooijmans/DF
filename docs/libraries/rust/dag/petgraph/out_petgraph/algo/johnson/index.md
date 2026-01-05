# Module johnson Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/johnson.rs.html#1-273" class="src">Source</a>

Expand description

Johnson’s algorithm implementation.

## Re-exports<a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/index.html#reexports" class="anchor">§</a>

`pub use super::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure"><code>BoundedMeasure</code></a>`;`

`pub use super::`<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.NegativeCycle.html" class="struct" title="struct petgraph::algo::NegativeCycle"><code>NegativeCycle</code></a>`;`

## Functions<a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.johnson.html" class="fn" title="fn petgraph::algo::johnson::johnson">johnson</a>  
[Johnson algorithm](https://en.wikipedia.org/wiki/Johnson%27s_algorithm) for all pairs shortest path problem.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/johnson/fn.parallel_johnson.html" class="fn" title="fn petgraph::algo::johnson::parallel_johnson">parallel_johnson</a>  
[Johnson algorithm](https://en.wikipedia.org/wiki/Johnson%27s_algorithm) implementation for all pairs shortest path problem, parallelizing the [`dijkstra`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.dijkstra.html "fn petgraph::algo::dijkstra::dijkstra") calls with `rayon`.
