# Struct MatrixGraph Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/matrix_graph.rs.html#243-258" class="src">Source</a>

``` rust
pub struct MatrixGraph<N, E, S = RandomState, Ty = Directed, Null: Nullable<Wrapped = E> = Option<E>, Ix = u16> { /* private fields */ }
```

Expand description

`MatrixGraph<N, E, Ty, Null>` is a graph datastructure using an adjacency matrix representation.

`MatrixGraph` is parameterized over:

- Associated data `N` for nodes and `E` for edges, called *weights*. The associated data can be of arbitrary type.
- Edge type `Ty` that determines whether the graph edges are directed or undirected.
- Nullable type `Null`, which denotes the edges’ presence (defaults to `Option<E>`). You may specify [`NotZero<E>`](https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html) if you want to use a sentinel value (such as 0) to mark the absence of an edge.
- Index type `Ix` that sets the maximum size for the graph (defaults to `DefaultIx`).

The graph uses **O(\|V^2\|)** space, with fast edge insertion & amortized node insertion, as well as efficient graph search and graph algorithms on dense graphs.

This graph is backed by a flattened 2D array. For undirected graphs, only the lower triangular matrix is stored. Since the backing array stores edge weights, it is recommended to box large edge weights.

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.with_capacity" class="fn">with_capacity</a>(node_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

where S: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Create a new `MatrixGraph` with estimated capacity for nodes.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.with_capacity_and_hasher" class="fn">with_capacity_and_hasher</a>(node_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, hasher: S) -\> Self

Create a new `MatrixGraph` with estimated capacity for nodes and a provided hasher.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.clear" class="fn">clear</a>(&mut self)

Remove all nodes and edges.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.node_count" class="fn">node_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of nodes (also called vertices) in the graph.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.edge_count" class="fn">edge_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of edges in the graph.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.is_directed" class="fn">is_directed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return whether the graph has directed edges or not.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.add_node" class="fn">add_node</a>(&mut self, weight: N) -\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>

Add a node (also called vertex) with associated data `weight` to the graph.

Computes in **O(1)** time.

Return the index of the new node.

**Panics** if the MatrixGraph is at the maximum number of nodes for its index type.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.try_add_node" class="fn">try_add_node</a>(&mut self, weight: N) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>\>

Try to add a node (also called vertex) with associated data `weight` to the graph.

Computes in **O(1)** time.

Return the index of the new node.

Possible errors:

- [`MatrixError::NodeIxLimit`](https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#variant.NodeIxLimit "variant petgraph::matrix_graph::MatrixError::NodeIxLimit") if the `MatrixGraph` is at the maximum number of nodes for its index type.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.remove_node" class="fn">remove_node</a>(&mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> N

Remove `a` from the graph.

Computes in **O(V)** time, due to the removal of edges with other nodes.

**Panics** if the node `a` does not exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.update_edge" class="fn">update_edge</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, weight: E, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<E\>

Update the edge from `a` to `b` to the graph, with its associated data `weight`.

Return the previous data, if any.

Computes in **O(1)** time, best case. Computes in **O(\|V\|^2)** time, worst case (matrix needs to be re-allocated).

**Panics** if any of the nodes don’t exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.try_update_edge" class="fn">try_update_edge</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, weight: E, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<E\>, <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>\>

Try to update the edge from `a` to `b`, with its associated data `weight`.

Return the previous data, if any.

Computes in **O(1)** time, best case. Computes in **O(\|V\|^2)** time, worst case (matrix needs to be re-allocated).

Possible errors:

- [`MatrixError::NodeMissed`](https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#variant.NodeMissed "variant petgraph::matrix_graph::MatrixError::NodeMissed") if any of the nodes don’t exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.add_edge" class="fn">add_edge</a>(&mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, weight: E)

Add an edge from `a` to `b` to the graph, with its associated data `weight`.

Computes in **O(1)** time, best case. Computes in **O(\|V\|^2)** time, worst case (matrix needs to be re-allocated).

**Panics** if any of the nodes don’t exist. **Panics** if an edge already exists from `a` to `b`.

**Note:** `MatrixGraph` does not allow adding parallel (“duplicate”) edges. If you want to avoid this, use [`.update_edge(a, b, weight)`](https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.update_edge) instead.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.add_or_update_edge" class="fn">add_or_update_edge</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, weight: E, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<E\>, <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>\>

Add or update edge from `a` to `b` to the graph, with its associated data `weight`.

Return the previous data, if any.

Computes in **O(1)** time, best case. Computes in **O(\|V\|^2)** time, worst case (matrix needs to be re-allocated).

Possible errors:

- [`MatrixError::NodeMissed`](https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#variant.NodeMissed "variant petgraph::matrix_graph::MatrixError::NodeMissed") if any of the nodes don’t exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.remove_edge" class="fn">remove_edge</a>(&mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> E

Remove the edge from `a` to `b` to the graph.

**Panics** if any of the nodes don’t exist. **Panics** if no edge exists between `a` and `b`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.try_remove_edge" class="fn">try_remove_edge</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<E\>

Try to remove the edge from `a` to `b`.

Return old value if present.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.has_edge" class="fn">has_edge</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return `true` if there is an edge between `a` and `b`.

If any of the nodes don’t exist - returns `false`. **Panics** if any of the nodes don’t exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.node_weight" class="fn">node_weight</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;N</a>

Access the weight for node `a`.

Also available with indexing syntax: `&graph[a]`.

**Panics** if the node doesn’t exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.get_node_weight" class="fn">get_node_weight</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;N</a>\>

Try to access the weight for node `a`.

Return `None` if the node doesn’t exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.node_weight_mut" class="fn">node_weight_mut</a>(&mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut N</a>

Access the weight for node `a`, mutably.

Also available with indexing syntax: `&mut graph[a]`.

**Panics** if the node doesn’t exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.get_node_weight_mut" class="fn">get_node_weight_mut</a>(&mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut N</a>\>

Try to access the weight for node `a`, mutably.

Return `None` if the node doesn’t exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.edge_weight" class="fn">edge_weight</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>

Access the weight for edge `e`.

Also available with indexing syntax: `&graph[e]`.

**Panics** if no edge exists between `a` and `b`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.get_edge_weight" class="fn">get_edge_weight</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>\>

Access the weight for edge from `a` to `b`.

Return `None` if the edge doesn’t exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.edge_weight_mut" class="fn">edge_weight_mut</a>(&mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>

Access the weight for edge `e`, mutably.

Also available with indexing syntax: `&mut graph[e]`.

**Panics** if no edge exists between `a` and `b`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.get_edge_weight_mut" class="fn">get_edge_weight_mut</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>\>

Access the weight for edge from `a` to `b`, mutably.

Return `None` if the edge doesn’t exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.neighbors" class="fn">neighbors</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.Neighbors.html" class="struct" title="struct petgraph::matrix_graph::Neighbors">Neighbors</a>\<'\_, Ty, Null, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#" class="tooltip" data-notable-ty="Neighbors&lt;&#39;_, Ty, Null, Ix&gt;">ⓘ</a>

Return an iterator of all nodes with an edge starting from `a`.

- `Directed`: Outgoing edges from `a`.
- `Undirected`: All edges from or to `a`.

Produces an empty iterator if the node doesn’t exist.  
Iterator element type is [`NodeIndex<Ix>`](https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html).

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.edges" class="fn">edges</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.Edges.html" class="struct" title="struct petgraph::matrix_graph::Edges">Edges</a>\<'\_, Ty, Null, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#" class="tooltip" data-notable-ty="Edges&lt;&#39;_, Ty, Null, Ix&gt;">ⓘ</a>

Return an iterator of all edges of `a`.

- `Directed`: Outgoing edges from `a`.
- `Undirected`: All edges connected to `a`.

Produces an empty iterator if the node doesn’t exist.  
Iterator element type is `(NodeIndex<Ix>, NodeIndex<Ix>, &E)`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.from_edges" class="fn">from_edges</a>\<I\>(iterable: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, I::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>: <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\>, \<I::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a> as <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\>\>::<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::IntoWeightedEdge::NodeId">NodeId</a>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>\>, N: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>, S: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Create a new `MatrixGraph` from an iterable of edges.

Node weights `N` are set to default values. Edge weights `E` may either be specified in the list, or they are filled with default values.

Nodes are inserted automatically to match the edges.

``` rust
use petgraph::matrix_graph::MatrixGraph;

let gr = MatrixGraph::<(), i32>::from_edges(&[
    (0, 1), (0, 2), (0, 3),
    (1, 2), (1, 3),
    (2, 3),
]);
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.extend_with_edges" class="fn">extend_with_edges</a>\<I\>(&mut self, iterable: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, I::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>: <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\>, \<I::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a> as <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\>\>::<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::IntoWeightedEdge::NodeId">NodeId</a>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>\>, N: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Extend the graph from an iterable of edges.

Node weights `N` are set to default values. Edge weights `E` may either be specified in the list, or they are filled with default values.

Nodes are inserted automatically to match the edges.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-MatrixGraph%3CN,+E,+S,+Directed,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>, Null, Ix\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.neighbors_directed" class="fn">neighbors_directed</a>( &self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, d: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.Neighbors.html" class="struct" title="struct petgraph::matrix_graph::Neighbors">Neighbors</a>\<'\_, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>, Null, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#" class="tooltip" data-notable-ty="Neighbors&lt;&#39;_, Directed, Null, Ix&gt;">ⓘ</a>

Return an iterator of all neighbors that have an edge between them and `a`, in the specified direction. If the graph’s edges are undirected, this is equivalent to *.neighbors(a)*.

- `Outgoing`: All edges from `a`.
- `Incoming`: All edges to `a`.

Produces an empty iterator if the node doesn’t exist.  
Iterator element type is [`NodeIndex<Ix>`](https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html).

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.edges_directed" class="fn">edges_directed</a>( &self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, d: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.Edges.html" class="struct" title="struct petgraph::matrix_graph::Edges">Edges</a>\<'\_, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>, Null, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#" class="tooltip" data-notable-ty="Edges&lt;&#39;_, Directed, Null, Ix&gt;">ⓘ</a>

Return an iterator of all edges of `a`, in the specified direction.

- `Outgoing`: All edges from `a`.
- `Incoming`: All edges to `a`.

Produces an empty iterator if the node `a` doesn’t exist.  
Iterator element type is `(NodeIndex<Ix>, NodeIndex<Ix>, &E)`.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-MatrixGraph%3CN,+E,+S%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.new" class="fn">new</a>() -\> Self

Create a new `MatrixGraph` with directed edges.

This is a convenience method. Use `MatrixGraph::with_capacity` or `MatrixGraph::default` for a constructor that is generic in all the type parameters of `MatrixGraph`.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-MatrixGraph%3CN,+E,+S,+Undirected%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.new_undirected" class="fn">new_undirected</a>() -\> Self

Create a new `MatrixGraph` with undirected edges.

This is a convenience method. Use `MatrixGraph::with_capacity` or `MatrixGraph::default` for a constructor that is generic in all the type parameters of `MatrixGraph`.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-Build-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html" class="trait" title="trait petgraph::data::Build">Build</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.add_node-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#tymethod.add_node" class="fn">add_node</a>(&mut self, weight: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.add_edge-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#method.add_edge" class="fn">add_edge</a>( &mut self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, b: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, weight: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>\>

Add a new edge. If parallel edges (duplicate) are not allowed and the edge already exists, return `None`. [Read more](https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#method.add_edge)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.update_edge-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#tymethod.update_edge" class="fn">update_edge</a>( &mut self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, b: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, weight: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>, ) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>

Add or update the edge from `a` to `b`. Return the id of the affected edge. [Read more](https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#tymethod.update_edge)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-Clone-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, E: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, S: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, Ty: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, Null: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-Data-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.NodeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype">NodeWeight</a> = N

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.EdgeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype">EdgeWeight</a> = E

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-Default-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

Create a new empty `MatrixGraph`.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-EdgeCount-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeCount.html" class="trait" title="trait petgraph::visit::EdgeCount">EdgeCount</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.edge_count-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeCount.html#tymethod.edge_count" class="fn">edge_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of edges in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-FromGraph6-for-MatrixGraph%3C(),+(),+S,+Undirected,+Null,+Ix%3E" class="anchor">§</a>

### impl\<Null, Ix, S\> <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.FromGraph6.html" class="trait" title="trait petgraph::graph6::FromGraph6">FromGraph6</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, S, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>, Null, Ix\>

where Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Available on **crate feature `matrix_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.from_graph6_string" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.FromGraph6.html#tymethod.from_graph6_string" class="fn">from_graph6_string</a>(graph6_string: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-GetAdjacencyMatrix-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.AdjMatrix" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

The associated adjacency matrix type

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.adjacency_matrix" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.adjacency_matrix" class="fn">adjacency_matrix</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype" title="type petgraph::visit::GetAdjacencyMatrix::AdjMatrix">AdjMatrix</a>

Create the adjacency matrix

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.is_adjacent" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.is_adjacent" class="fn">is_adjacent</a>( &self, \_: &Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype" title="type petgraph::visit::GetAdjacencyMatrix::AdjMatrix">AdjMatrix</a>, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if there is an edge from `a` to `b`, false otherwise. [Read more](https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.is_adjacent)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-GraphBase-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.NodeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>

node identifier

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.EdgeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = (<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>)

edge identifier

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-GraphProp-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.EdgeType" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#associatedtype.EdgeType" class="associatedtype">EdgeType</a> = Ty

The kind of edges in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.is_directed-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#method.is_directed" class="fn">is_directed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-Index%3C(NodeIndex%3CIx%3E,+NodeIndex%3CIx%3E)%3E-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>)\> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

Index the `MatrixGraph` by `NodeIndex` pair to access edge weights.

Also available with indexing syntax: `&graph[e]`.

**Panics** if no edge exists between `a` and `b`.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = E

The returned type after indexing.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.index-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>(&self, (ax, bx): (<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-Index%3CNodeIndex%3CIx%3E%3E-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>\> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

Index the `MatrixGraph` by `NodeIndex` to access node weights.

**Panics** if the node doesn’t exist.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = N

The returned type after indexing.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.index" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>(&self, ax: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;N</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-IndexMut%3C(NodeIndex%3CIx%3E,+NodeIndex%3CIx%3E)%3E-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html" class="trait" title="trait core::ops::index::IndexMut">IndexMut</a>\<(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>)\> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

Index the `MatrixGraph` by `NodeIndex` pair to access edge weights.

Also available with indexing syntax: `&mut graph[e]`.

**Panics** if no edge exists between `a` and `b`.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.index_mut-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut" class="fn">index_mut</a>(&mut self, (ax, bx): (<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-IndexMut%3CNodeIndex%3CIx%3E%3E-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html" class="trait" title="trait core::ops::index::IndexMut">IndexMut</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>\> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

Index the `MatrixGraph` by `NodeIndex` to access node weights.

**Panics** if the node doesn’t exist.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.index_mut" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut" class="fn">index_mut</a>(&mut self, ax: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut N</a>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-IntoEdgeReferences-for-%26MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E, S, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.EdgeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype">EdgeRef</a> = (<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a E</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.EdgeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype">EdgeReferences</a> = <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.EdgeReferences.html" class="struct" title="struct petgraph::matrix_graph::EdgeReferences">EdgeReferences</a>\<'a, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.edge_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#tymethod.edge_references" class="fn">edge_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeReferences">EdgeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-IntoEdges-for-%26MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html" class="trait" title="trait petgraph::visit::IntoEdges">IntoEdges</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.Edges" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype">Edges</a> = <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.Edges.html" class="struct" title="struct petgraph::matrix_graph::Edges">Edges</a>\<'a, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.edges-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#tymethod.edges" class="fn">edges</a>(self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype" title="type petgraph::visit::IntoEdges::Edges">Edges</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-IntoEdgesDirected-for-%26MatrixGraph%3CN,+E,+S,+Directed,+Null,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html" class="trait" title="trait petgraph::visit::IntoEdgesDirected">IntoEdgesDirected</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.EdgesDirected" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#associatedtype.EdgesDirected" class="associatedtype">EdgesDirected</a> = <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.Edges.html" class="struct" title="struct petgraph::matrix_graph::Edges">Edges</a>\<'a, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.edges_directed-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#tymethod.edges_directed" class="fn">edges_directed</a>(self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#associatedtype.EdgesDirected" class="associatedtype" title="type petgraph::visit::IntoEdgesDirected::EdgesDirected">EdgesDirected</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-IntoNeighbors-for-%26MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E: 'a, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.Neighbors" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype">Neighbors</a> = <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.Neighbors.html" class="struct" title="struct petgraph::matrix_graph::Neighbors">Neighbors</a>\<'a, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.neighbors-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#tymethod.neighbors" class="fn">neighbors</a>(self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype" title="type petgraph::visit::IntoNeighbors::Neighbors">Neighbors</a>

Return an iterator of the neighbors of node `a`.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-IntoNeighborsDirected-for-%26MatrixGraph%3CN,+E,+S,+Directed,+Null,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E: 'a, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.NeighborsDirected" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#associatedtype.NeighborsDirected" class="associatedtype">NeighborsDirected</a> = <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.Neighbors.html" class="struct" title="struct petgraph::matrix_graph::Neighbors">Neighbors</a>\<'a, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.neighbors_directed-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#tymethod.neighbors_directed" class="fn">neighbors_directed</a>( self, a: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>, d: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>, ) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#associatedtype.NeighborsDirected" class="associatedtype" title="type petgraph::visit::IntoNeighborsDirected::NeighborsDirected">NeighborsDirected</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-IntoNodeIdentifiers-for-%26MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E: 'a, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.NodeIdentifiers" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype">NodeIdentifiers</a> = <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NodeIdentifiers.html" class="struct" title="struct petgraph::matrix_graph::NodeIdentifiers">NodeIdentifiers</a>\<'a, Ix, S\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.node_identifiers" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#tymethod.node_identifiers" class="fn">node_identifiers</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype" title="type petgraph::visit::IntoNodeIdentifiers::NodeIdentifiers">NodeIdentifiers</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-IntoNodeReferences-for-%26MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + 'a\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.NodeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeRef" class="associatedtype">NodeRef</a> = (<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a N</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.NodeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype">NodeReferences</a> = <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NodeReferences.html" class="struct" title="struct petgraph::matrix_graph::NodeReferences">NodeReferences</a>\<'a, N, Ix, S\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.node_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#tymethod.node_references" class="fn">node_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype" title="type petgraph::visit::IntoNodeReferences::NodeReferences">NodeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-NodeCount-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html" class="trait" title="trait petgraph::visit::NodeCount">NodeCount</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.node_count-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html#tymethod.node_count" class="fn">node_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-NodeIndexable-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.node_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.node_bound" class="fn">node_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return an upper bound of the node indices in the graph (suitable for the size of a bitmap).

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.to_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.to_index" class="fn">to_index</a>(&self, ix: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/type.NodeIndex.html" class="type" title="type petgraph::matrix_graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert `a` to an integer index.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.from_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.from_index" class="fn">from_index</a>(&self, ix: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

Convert `i` to a node index. `i` must be a valid value in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-ToGraph6-for-MatrixGraph%3CN,+E,+S,+Undirected,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S, Null, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html" class="trait" title="trait petgraph::graph6::ToGraph6">ToGraph6</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>, Null, Ix\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Available on **crate feature `matrix_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.graph6_string" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html#tymethod.graph6_string" class="fn">graph6_string</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#impl-Visitable-for-MatrixGraph%3CN,+E,+S,+Ty,+Null,+Ix%3E" class="anchor">§</a>

### impl\<N, E, S, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Null: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a>\<Wrapped = E\>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html" class="struct" title="struct petgraph::matrix_graph::MatrixGraph">MatrixGraph</a>\<N, E, S, Ty, Null, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#associatedtype.Map" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype">Map</a> = <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

The associated map type

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.visit_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.visit_map" class="fn">visit_map</a>(&self) -\> <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

Create a new visitor map

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#method.reset_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.reset_map" class="fn">reset_map</a>(&self, map: &mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>)

Reset the visitor map (and resize to new size of graph if needed)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html#blanket-implementations" class="anchor">§</a>
