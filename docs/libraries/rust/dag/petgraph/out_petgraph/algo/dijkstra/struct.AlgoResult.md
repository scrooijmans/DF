# Struct AlgoResult Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/dijkstra.rs.html#104-109" class="src">Source</a>

``` rust
pub struct AlgoResult<N, K> {
    pub scores: HashMap<N, K>,
    pub goal_node: Option<N>,
}
```

Expand description

Return value of [`with_dynamic_goal`](https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/fn.with_dynamic_goal.html "fn petgraph::algo::dijkstra::with_dynamic_goal").

## Fields<a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/struct.AlgoResult.html#fields" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/struct.AlgoResult.html#structfield.scores" class="anchor field">§</a>`scores: `<a href="https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap"><code>HashMap</code></a>`<N, K>`

A [`hashbrown::HashMap`](https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap") that maps `NodeId` to path cost.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/struct.AlgoResult.html#structfield.goal_node" class="anchor field">§</a>`goal_node: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<N>`

The goal node that terminated the search, if any was found.

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/struct.AlgoResult.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/dijkstra/struct.AlgoResult.html#blanket-implementations" class="anchor">§</a>
