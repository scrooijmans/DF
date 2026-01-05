# Function simple_fast Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/dominators.rs.html#178-255" class="src">Source</a>

``` rust
pub fn simple_fast<G>(graph: G, root: G::NodeId) -> Dominators<G::NodeId>where
    G: IntoNeighbors + Visitable,
    <G as GraphBase>::NodeId: Eq + Hash,
```

Expand description

This is an implementation of the engineered [“Simple, Fast Dominance Algorithm”](http://www.hipersoft.rice.edu/grads/publications/dom14.pdf) discovered by Cooper et al.

This algorithm is **O(\|V\|²)** where V is the set of nodes, and therefore has slower theoretical running time than the Lengauer-Tarjan algorithm (which is **O(\|E\| log \|V\|)** where E is the set of edges). However, Cooper et al found it to be faster in practice on control flow graphs of up to ~30,000 nodes.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/fn.simple_fast.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: a control-flow graph.
- `root`: the *root* node of the `graph`.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/fn.simple_fast.html#returns" class="doc-anchor">§</a>Returns

- `Dominators`: the dominance relation for given `graph` and `root` represented by [`Dominators`](https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html "struct petgraph::algo::dominators::Dominators").

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/fn.simple_fast.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\|²)**.
- Auxiliary space: **O(\|V\| + \|E\|)**.

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.
