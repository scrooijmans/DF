# Module tred Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/tred.rs.html#1-172" class="src">Source</a>

Expand description

Compute the transitive reduction and closure of a directed acyclic graph

### <a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/index.html#transitive-reduction-and-closure" class="doc-anchor">§</a>Transitive reduction and closure

The *transitive closure* of a graph **G = (V, E)** is the graph **Gc = (V, Ec)** such that **(i, j)** belongs to **Ec** if and only if there is a path connecting **i** to **j** in **G**. The *transitive reduction* of **G** is the graph **Gr = (V, Er)** such that **Er** is minimal wrt. inclusion in **E** and the transitive closure of **Gr** is the same as that of **G**. The transitive reduction is well-defined for acyclic graphs only.

## Functions<a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/fn.dag_to_toposorted_adjacency_list.html" class="fn" title="fn petgraph::algo::tred::dag_to_toposorted_adjacency_list">dag_to_toposorted_adjacency_list</a>  
Creates a representation of the same graph respecting topological order for use in `tred::dag_transitive_reduction_closure`.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/tred/fn.dag_transitive_reduction_closure.html" class="fn" title="fn petgraph::algo::tred::dag_transitive_reduction_closure">dag_transitive_reduction_closure</a>  
Computes the transitive reduction and closure of a DAG.
