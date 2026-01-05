# Module dominators Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/dominators.rs.html#1-347" class="src">Source</a>

Expand description

Compute dominators of a control-flow graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/index.html#the-dominance-relation" class="doc-anchor">§</a>The Dominance Relation

In a directed graph with a root node **R**, a node **A** is said to *dominate* a node **B** iff every path from **R** to **B** contains **A**.

The node **A** is said to *strictly dominate* the node **B** iff **A** dominates **B** and **A ≠ B**.

The node **A** is said to be the *immediate dominator* of a node **B** iff it strictly dominates **B** and there does not exist any node **C** where **A** dominates **C** and **C** dominates **B**.

## Structs<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.DominatedByIter.html" class="struct" title="struct petgraph::algo::dominators::DominatedByIter">DominatedByIter</a>  
Iterator for nodes dominated by a given node.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html" class="struct" title="struct petgraph::algo::dominators::Dominators">Dominators</a>  
The dominance relation for some graph and root.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.DominatorsIter.html" class="struct" title="struct petgraph::algo::dominators::DominatorsIter">DominatorsIter</a>  
Iterator for a node’s dominators.

## Functions<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/fn.simple_fast.html" class="fn" title="fn petgraph::algo::dominators::simple_fast">simple_fast</a>  
This is an implementation of the engineered [“Simple, Fast Dominance Algorithm”](http://www.hipersoft.rice.edu/grads/publications/dom14.pdf) discovered by Cooper et al.
