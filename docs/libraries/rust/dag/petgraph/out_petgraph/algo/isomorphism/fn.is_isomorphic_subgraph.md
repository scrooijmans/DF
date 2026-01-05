# Function is_isomorphic_subgraph Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/isomorphism.rs.html#894-910" class="src">Source</a>

``` rust
pub fn is_isomorphic_subgraph<G0, G1>(g0: G0, g1: G1) -> boolwhere
    G0: NodeCompactIndexable + EdgeCount + GetAdjacencyMatrix + GraphProp + IntoNeighborsDirected,
    G1: NodeCompactIndexable + EdgeCount + GetAdjacencyMatrix + GraphProp<EdgeType = G0::EdgeType> + IntoNeighborsDirected,
```

Expand description

Return `true` if `g0` is isomorphic to a subgraph of `g1`.

Using the VF2 algorithm, only matching graph syntactically (graph structure).

The graphs should not be [multigraphs](https://en.wikipedia.org/wiki/Multigraph).

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/isomorphism/fn.is_isomorphic_subgraph.html#subgraph-isomorphism" class="doc-anchor">§</a>Subgraph isomorphism

(adapted from [`networkx` documentation](https://networkx.github.io/documentation/stable/reference/algorithms/isomorphism.vf2.html))

Graph theory literature can be ambiguous about the meaning of the above statement, and we seek to clarify it now.

In the VF2 literature, a mapping **M** is said to be a *graph-subgraph isomorphism* iff **M** is an isomorphism between **G2** and a subgraph of **G1**. Thus, to say that **G1** and **G2** are graph-subgraph isomorphic is to say that a subgraph of **G1** is isomorphic to **G2**.

Other literature uses the phrase ‘subgraph isomorphic’ as in ‘**G1** does not have a subgraph isomorphic to **G2**’. Another use is as an in adverb for isomorphic. Thus, to say that **G1** and **G2** are subgraph isomorphic is to say that a subgraph of **G1** is isomorphic to **G2**.

Finally, the term ‘subgraph’ can have multiple meanings. In this context, ‘subgraph’ always means a ‘node-induced subgraph’. Edge-induced subgraph isomorphisms are not directly supported. For subgraphs which are not induced, the term ‘monomorphism’ is preferred over ‘isomorphism’.

**Reference**

- Luigi P. Cordella, Pasquale Foggia, Carlo Sansone, Mario Vento; *A (Sub)Graph Isomorphism Algorithm for Matching Large Graphs*
