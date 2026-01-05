# Function parallel_page_rank Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/page_rank.rs.html#134-195" class="src">Source</a>

``` rust
pub fn parallel_page_rank<G, D>(
    graph: G,
    damping_factor: D,
    nb_iter: usize,
    tol: Option<D>,
) -> Vec<D>where
    G: NodeCount + IntoEdges + NodeIndexable + Sync,
    D: UnitMeasure + Copy + Send + Sync,
```

Expand description

Parallel Page Rank algorithm.

See [`page_rank`](https://docs.rs/petgraph/latest/petgraph/algo/page_rank/fn.page_rank.html "fn petgraph::algo::page_rank::page_rank").
