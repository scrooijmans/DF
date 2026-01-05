# Function greedy_matching Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/matching.rs.html#211-219" class="src">Source</a>

``` rust
pub fn greedy_matching<G>(graph: G) -> Matching<G>where
    G: Visitable + IntoNodeIdentifiers + NodeIndexable + IntoNeighbors,
    G::NodeId: Eq + Hash,
    G::EdgeId: Eq + Hash,
```

Expand description

Compute a [*matching*](https://en.wikipedia.org/wiki/Matching_(graph_theory)) using a greedy heuristic.

The input graph is treated as if undirected. The underlying heuristic is unspecified, but is guaranteed to be bounded by **O(\|V\| + \|E\|)**. No guarantees about the output are given other than that it is a valid matching.

If you require a maximum matching, use [`maximum_matching`](https://docs.rs/petgraph/latest/petgraph/algo/matching/fn.maximum_matching.html) function instead.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/fn.greedy_matching.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: an undirected graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/fn.greedy_matching.html#returns" class="doc-anchor">§</a>Returns

- [`Matching`](https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html "struct petgraph::algo::matching::Matching") calculated using greedy heuristic.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/fn.greedy_matching.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**.
- Auxiliary space: **O(\|V\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.
