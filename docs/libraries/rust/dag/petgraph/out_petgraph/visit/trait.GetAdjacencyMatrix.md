# Trait GetAdjacencyMatrix Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/mod.rs.html#485-503" class="src">Source</a>

``` rust
pub trait GetAdjacencyMatrix: GraphBase {
    type AdjMatrix;

    // Required methods
    fn adjacency_matrix(&self) -> Self::AdjMatrix;
    fn is_adjacent(
        &self,
        matrix: &Self::AdjMatrix,
        a: Self::NodeId,
        b: Self::NodeId,
    ) -> bool;
}
```

Expand description

Create or access the adjacency matrix of a graph.

The implementor can either create an adjacency matrix, or it can return a placeholder if it has the needed representation internally.

## Required Associated Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a>

The associated adjacency matrix type

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.adjacency_matrix" class="fn">adjacency_matrix</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype" title="type petgraph::visit::GetAdjacencyMatrix::AdjMatrix">AdjMatrix</a>

Create the adjacency matrix

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.is_adjacent" class="fn">is_adjacent</a>( &self, matrix: &Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype" title="type petgraph::visit::GetAdjacencyMatrix::AdjMatrix">AdjMatrix</a>, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, b: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if there is an edge from `a` to `b`, false otherwise.

Computes in O(1) time.

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#impl-GetAdjacencyMatrix-for-%26G" class="anchor">§</a>

### impl\<'a, G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a G</a>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix-1" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype" title="type petgraph::visit::GetAdjacencyMatrix::AdjMatrix">AdjMatrix</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#method.adjacency_matrix" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.adjacency_matrix" class="fn">adjacency_matrix</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype" title="type petgraph::visit::GetAdjacencyMatrix::AdjMatrix">AdjMatrix</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#method.is_adjacent" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.is_adjacent" class="fn">is_adjacent</a>( &self, matrix: &Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype" title="type petgraph::visit::GetAdjacencyMatrix::AdjMatrix">AdjMatrix</a>, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, b: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#impl-GetAdjacencyMatrix-for-Frozen%3C&#39;a,+G%3E" class="anchor">§</a>

### impl\<'a, G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Frozen.html" class="struct" title="struct petgraph::graph::Frozen">Frozen</a>\<'a, G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix-2" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype" title="type petgraph::visit::GetAdjacencyMatrix::AdjMatrix">AdjMatrix</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#impl-GetAdjacencyMatrix-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

The adjacency matrix for **List** is a bitmap that’s computed by `.adjacency_matrix()`.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix-3" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#impl-GetAdjacencyMatrix-for-Reversed%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Reversed.html" class="struct" title="struct petgraph::visit::Reversed">Reversed</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix-4" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype" title="type petgraph::visit::GetAdjacencyMatrix::AdjMatrix">AdjMatrix</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#impl-GetAdjacencyMatrix-for-Acyclic%3CG%3E" class="anchor">§</a>

### impl\<G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/struct.Acyclic.html" class="struct" title="struct petgraph::acyclic::Acyclic">Acyclic</a>\<G\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix-5" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype" title="type petgraph::visit::GetAdjacencyMatrix::AdjMatrix">AdjMatrix</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#impl-GetAdjacencyMatrix-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix-6" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#impl-GetAdjacencyMatrix-for-%26Csr%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for &<a href="https://docs.rs/petgraph/latest/petgraph/csr/struct.Csr.html" class="struct" title="struct petgraph::csr::Csr">Csr</a>\<N, E, Ty, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>,

The adjacency matrix for **Csr** is a bitmap that’s computed by `.adjacency_matrix()`.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix-7" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#impl-GetAdjacencyMatrix-for-Graph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

The adjacency matrix for **Graph** is a bitmap that’s computed by `.adjacency_matrix()`.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix-8" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#impl-GetAdjacencyMatrix-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Available on **crate feature `stable_graph`** only.

The adjacency matrix for **Graph** is a bitmap that’s computed by `.adjacency_matrix()`.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix-9" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#impl-GetAdjacencyMatrix-for-GraphMap%3CN,+E,+Ty,+S%3E" class="anchor">§</a>

### impl\<N, E, Ty, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html" class="struct" title="struct petgraph::graphmap::GraphMap">GraphMap</a>\<N, E, Ty, S\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

The `GraphMap` keeps an adjacency matrix internally.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix-10" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>
