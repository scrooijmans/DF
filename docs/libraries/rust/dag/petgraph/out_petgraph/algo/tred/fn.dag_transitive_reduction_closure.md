# Function dag_transitive_reduction_closure Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/tred.rs.html#120-151" class="src">Source</a>

``` rust
pub fn dag_transitive_reduction_closure<E, Ix: IndexType>(
    g: &List<E, Ix>,
) -> (UnweightedList<Ix>, UnweightedList<Ix>)
```

Expand description

Computes the transitive reduction and closure of a DAG.

The algorithm implemented here comes from [On the calculation of transitive reduction-closure of orders](https://www.sciencedirect.com/science/article/pii/0012365X9390164O) by Habib, Morvan and Rampon.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/fn.dag_transitive_reduction_closure.html#arguments" class="doc-anchor">§</a>Arguments

- `g`: an input graph in a very specific format: an adjacency list such that node indices are a toposort, and the neighbors of all nodes are stored in topological order. To get such a representation, use the function [`dag_to_toposorted_adjacency_list`](https://docs.rs/petgraph/latest/petgraph/algo/tred/fn.dag_to_toposorted_adjacency_list.html "fn petgraph::algo::tred::dag_to_toposorted_adjacency_list").

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/fn.dag_transitive_reduction_closure.html#returns" class="doc-anchor">§</a>Returns

The output is the pair of the transitive reduction and the transitive closure.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/fn.dag_transitive_reduction_closure.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \sum\_{(x, y) \in Er} d(y))** where **d(y)** denotes the outgoing degree of **y** in the transitive closure of **G** and **Er** the edge set of the transitive reduction. This is still **O(\|V\|³)** in the worst case like the naive algorithm but should perform better for some classes of graphs.
- Auxiliary space: **O(\|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.
