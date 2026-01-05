# Struct StableGraph Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/stable_graph/mod.rs.html#70-88" class="src">Source</a>

``` rust
pub struct StableGraph<N, E, Ty = Directed, Ix = DefaultIx> { /* private fields */ }
```

Expand description

`StableGraph<N, E, Ty, Ix>` is a graph datastructure using an adjacency list representation.

The graph **does not invalidate** any unrelated node or edge indices when items are removed.

`StableGraph` is parameterized over:

- Associated data `N` for nodes and `E` for edges, also called *weights*. The associated data can be of arbitrary type.
- Edge type `Ty` that determines whether the graph edges are directed or undirected.
- Index type `Ix`, which determines the maximum size of the graph.

The graph uses **O(\|V\| + \|E\|)** space where V is the set of nodes and E is the set of edges, and allows fast node and edge insert and efficient graph search.

It implements **O(e’)** edge lookup and edge and node removals, where **e’** is some local measure of edge count.

- Nodes and edges are each numbered in an interval from *0* to some number *m*, but *not all* indices in the range are valid, since gaps are formed by deletions.

- You can select graph index integer type after the size of the graph. A smaller size may have better performance.

- Using indices allows mutation while traversing the graph, see `Dfs`.

- The `StableGraph` is a regular rust collection and is `Send` and `Sync` (as long as associated data `N` and `E` are).

- Indices don’t allow as much compile time checking as references.

Depends on crate feature `stable_graph` (default). *Stable Graph is still missing a few methods compared to Graph. You can contribute to help it achieve parity.*

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-StableGraph%3CN,+E%3E" class="anchor">§</a>

### impl\<N, E\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.new" class="fn">new</a>() -\> Self

Create a new `StableGraph` with directed edges.

This is a convenience method. See `StableGraph::with_capacity` or `StableGraph::default` for a constructor that is generic in all the type parameters of `StableGraph`.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.with_capacity" class="fn">with_capacity</a>(nodes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, edges: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Create a new `StableGraph` with estimated capacity.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-StableGraph%3CN,+E,+Ty,+Ix%3E-1" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.capacity" class="fn">capacity</a>(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Return the current node and edge capacity of the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.reverse" class="fn">reverse</a>(&mut self)

Reverse the direction of all edges

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.clear" class="fn">clear</a>(&mut self)

Remove all nodes and edges

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.clear_edges" class="fn">clear_edges</a>(&mut self)

Remove all edges

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_count" class="fn">node_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of nodes (also called vertices) in the graph.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_count" class="fn">edge_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of edges in the graph.

Computes in **O(1)** time.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.reserve_nodes" class="fn">reserve_nodes</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserves capacity for at least additional more nodes to be inserted in the graph. Graph may reserve more space to avoid frequent reallocations.

Panics if the new capacity overflows usize.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.reserve_edges" class="fn">reserve_edges</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserves capacity for at least additional more edges to be inserted in the graph. Graph may reserve more space to avoid frequent reallocations.

Panics if the new capacity overflows usize.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.reserve_exact_nodes" class="fn">reserve_exact_nodes</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserves the minimum capacity for exactly additional more nodes to be inserted in the graph. Does nothing if the capacity is already sufficient.

Prefer reserve_nodes if future insertions are expected.

Panics if the new capacity overflows usize.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.reserve_exact_edges" class="fn">reserve_exact_edges</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserves the minimum capacity for exactly additional more edges to be inserted in the graph. Does nothing if the capacity is already sufficient.

Prefer reserve_edges if future insertions are expected.

Panics if the new capacity overflows usize.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.shrink_to_fit_nodes" class="fn">shrink_to_fit_nodes</a>(&mut self)

Shrinks the capacity of the underlying nodes collection as much as possible.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.shrink_to_fit_edges" class="fn">shrink_to_fit_edges</a>(&mut self)

Shrinks the capacity of the underlying edges collection as much as possible.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of the graph as much as possible.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.is_directed" class="fn">is_directed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether the graph has directed edges or not.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.add_node" class="fn">add_node</a>(&mut self, weight: N) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>

Add a node (also called vertex) with associated data `weight` to the graph.

Computes in **O(1)** time.

Return the index of the new node.

**Panics** if the `StableGraph` is at the maximum number of nodes for its index type.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.try_add_node" class="fn">try_add_node</a>(&mut self, weight: N) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/graph/enum.GraphError.html" class="enum" title="enum petgraph::graph::GraphError">GraphError</a>\>

Add a node (also called vertex) with associated data `weight` to the graph.

Computes in **O(1)** time.

Return the index of the new node.

Return [`GraphError::NodeIxLimit`](https://docs.rs/petgraph/latest/petgraph/graph/enum.GraphError.html#variant.NodeIxLimit "variant petgraph::graph::GraphError::NodeIxLimit") if the `StableGraph` is at the maximum number of nodes for its index.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.remove_node" class="fn">remove_node</a>(&mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<N\>

Remove `a` from the graph if it exists, and return its weight. If it doesn’t exist in the graph, return `None`.

The node index `a` is invalidated, but none other. Edge indices are invalidated as they would be following the removal of each edge with an endpoint in `a`.

Computes in **O(e’)** time, where **e’** is the number of affected edges, including *n* calls to `.remove_edge()` where *n* is the number of edges with an endpoint in `a`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.contains_node" class="fn">contains_node</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.add_edge" class="fn">add_edge</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, weight: E, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>

Add an edge from `a` to `b` to the graph, with its associated data `weight`.

Return the index of the new edge.

Computes in **O(1)** time.

**Panics** if any of the nodes don’t exist.  
**Panics** if the `StableGraph` is at the maximum number of edges for its index type.

**Note:** `StableGraph` allows adding parallel (“duplicate”) edges.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.try_add_edge" class="fn">try_add_edge</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, weight: E, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/graph/enum.GraphError.html" class="enum" title="enum petgraph::graph::GraphError">GraphError</a>\>

Try to add an edge from `a` to `b` to the graph, with its associated data `weight`.

Return the index of the new edge.

Computes in **O(1)** time.

Possible errors:

- [`GraphError::NodeMissed`](https://docs.rs/petgraph/latest/petgraph/graph/enum.GraphError.html#variant.NodeMissed "variant petgraph::graph::GraphError::NodeMissed") - if any of the nodes don’t exist.  
- [`GraphError::EdgeIxLimit`](https://docs.rs/petgraph/latest/petgraph/graph/enum.GraphError.html#variant.EdgeIxLimit "variant petgraph::graph::GraphError::EdgeIxLimit") if the `StableGraph` is at the maximum number of edges for its index type (N/A if usize).

**Note:** `StableGraph` allows adding parallel (“duplicate”) edges.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.update_edge" class="fn">update_edge</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, weight: E, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>

Add or update an edge from `a` to `b`. If the edge already exists, its weight is updated.

Return the index of the affected edge.

Computes in **O(e’)** time, where **e’** is the number of edges connected to `a` (and `b`, if the graph edges are undirected).

**Panics** if any of the nodes don’t exist or the stable graph is at the maximum number of edges for its index (when adding new edge).

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.try_update_edge" class="fn">try_update_edge</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, weight: E, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/graph/enum.GraphError.html" class="enum" title="enum petgraph::graph::GraphError">GraphError</a>\>

Try to add or update an edge from `a` to `b`. If the edge already exists, its weight is updated.

Return the index of the affected edge.

Computes in **O(e’)** time, where **e’** is the number of edges connected to `a` (and `b`, if the graph edges are undirected).

Possible errors:

- [`GraphError::NodeMissed`](https://docs.rs/petgraph/latest/petgraph/graph/enum.GraphError.html#variant.NodeMissed "variant petgraph::graph::GraphError::NodeMissed") - if any of the nodes don’t exist.  
- [`GraphError::EdgeIxLimit`](https://docs.rs/petgraph/latest/petgraph/graph/enum.GraphError.html#variant.EdgeIxLimit "variant petgraph::graph::GraphError::EdgeIxLimit") if the `StableGraph` is at the maximum number of edges for its index type (N/A if usize).

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.remove_edge" class="fn">remove_edge</a>(&mut self, e: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<E\>

Remove an edge and return its edge weight, or `None` if it didn’t exist.

Invalidates the edge index `e` but no other.

Computes in **O(e’)** time, where **e’** is the number of edges connected to the same endpoints as `e`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_weight" class="fn">node_weight</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;N</a>\>

Access the weight for node `a`.

Also available with indexing syntax: `&graph[a]`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_weight_mut" class="fn">node_weight_mut</a>(&mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut N</a>\>

Access the weight for node `a`, mutably.

Also available with indexing syntax: `&mut graph[a]`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_weights" class="fn">node_weights</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;N</a>\>

Return an iterator yielding immutable access to all node weights.

The order in which weights are yielded matches the order of their node indices.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_weights_mut" class="fn">node_weights_mut</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut N</a>\>

Return an iterator yielding mutable access to all node weights.

The order in which weights are yielded matches the order of their node indices.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_indices" class="fn">node_indices</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.NodeIndices.html" class="struct" title="struct petgraph::stable_graph::NodeIndices">NodeIndices</a>\<'\_, N, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#" class="tooltip" data-notable-ty="NodeIndices&lt;&#39;_, N, Ix&gt;">ⓘ</a>

Return an iterator over the node indices of the graph

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_weight" class="fn">edge_weight</a>(&self, e: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>\>

Access the weight for edge `e`.

Also available with indexing syntax: `&graph[e]`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_weight_mut" class="fn">edge_weight_mut</a>(&mut self, e: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>\>

Access the weight for edge `e`, mutably

Also available with indexing syntax: `&mut graph[e]`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_weights" class="fn">edge_weights</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>\>

Return an iterator yielding immutable access to all edge weights.

The order in which weights are yielded matches the order of their edge indices.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_weights_mut" class="fn">edge_weights_mut</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>\>

Return an iterator yielding mutable access to all edge weights.

The order in which weights are yielded matches the order of their edge indices.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_endpoints" class="fn">edge_endpoints</a>( &self, e: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>)\>

Access the source and target nodes for `e`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_indices" class="fn">edge_indices</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.EdgeIndices.html" class="struct" title="struct petgraph::stable_graph::EdgeIndices">EdgeIndices</a>\<'\_, E, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#" class="tooltip" data-notable-ty="EdgeIndices&lt;&#39;_, E, Ix&gt;">ⓘ</a>

Return an iterator over the edge indices of the graph.

Note: the iterator borrows a graph in contrast to the behavior of [`Graph::edge_indices`](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#method.edge_indices "method petgraph::graph::Graph::edge_indices").

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edges_connecting" class="fn">edges_connecting</a>( &self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.EdgesConnecting.html" class="struct" title="struct petgraph::stable_graph::EdgesConnecting">EdgesConnecting</a>\<'\_, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#" class="tooltip" data-notable-ty="EdgesConnecting&lt;&#39;_, E, Ty, Ix&gt;">ⓘ</a>

Return an iterator over all the edges connecting `a` and `b`.

- `Directed`: Outgoing edges from `a`.
- `Undirected`: All edges connected to `a`.

Iterator element type is `EdgeReference<E, Ix>`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.contains_edge" class="fn">contains_edge</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Lookup if there is an edge from `a` to `b`.

Computes in **O(e’)** time, where **e’** is the number of edges connected to `a` (and `b`, if the graph edges are undirected).

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.find_edge" class="fn">find_edge</a>( &self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>\>

Lookup an edge from `a` to `b`.

Computes in **O(e’)** time, where **e’** is the number of edges connected to `a` (and `b`, if the graph edges are undirected).

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.find_edge_undirected" class="fn">find_edge_undirected</a>( &self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>)\>

Lookup an edge between `a` and `b`, in either direction.

If the graph is undirected, then this is equivalent to `.find_edge()`.

Return the edge index and its directionality, with `Outgoing` meaning from `a` to `b` and `Incoming` the reverse, or `None` if the edge does not exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.neighbors" class="fn">neighbors</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Neighbors.html" class="struct" title="struct petgraph::stable_graph::Neighbors">Neighbors</a>\<'\_, E, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#" class="tooltip" data-notable-ty="Neighbors&lt;&#39;_, E, Ix&gt;">ⓘ</a>

Return an iterator of all nodes with an edge starting from `a`.

- `Directed`: Outgoing edges from `a`.
- `Undirected`: All edges connected to `a`.

Produces an empty iterator if the node doesn’t exist.  
Iterator element type is `NodeIndex<Ix>`.

For the iteration order for `Directed` and `Undirected` graphs respectively, please refer to the documentation of [`Graph::neighbors_directed`](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#method.neighbors_directed "method petgraph::graph::Graph::neighbors_directed").

Use [`.neighbors(a).detach()`](https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Neighbors.html#method.detach) to get a neighbor walker that does not borrow from the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.neighbors_directed" class="fn">neighbors_directed</a>( &self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Neighbors.html" class="struct" title="struct petgraph::stable_graph::Neighbors">Neighbors</a>\<'\_, E, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#" class="tooltip" data-notable-ty="Neighbors&lt;&#39;_, E, Ix&gt;">ⓘ</a>

Return an iterator of all neighbors that have an edge between them and `a`, in the specified direction. If the graph’s edges are undirected, this is equivalent to *.neighbors(a)*.

- `Directed`, `Outgoing`: All edges from `a`.
- `Directed`, `Incoming`: All edges to `a`.
- `Undirected`: All edges connected to `a`.

Produces an empty iterator if the node doesn’t exist.  
Iterator element type is `NodeIndex<Ix>`.

For a `Directed` graph, neighbors are listed in reverse order of their addition to the graph, so the most recently added edge’s neighbor is listed first.

For the ordering in case of an `Undirected` graph, please refer to the documentation of [`Graph::neighbors_undirected`](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#method.neighbors_undirected "method petgraph::graph::Graph::neighbors_undirected").

Use [`.neighbors_directed(a, dir).detach()`](https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Neighbors.html#method.detach) to get a neighbor walker that does not borrow from the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.neighbors_undirected" class="fn">neighbors_undirected</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Neighbors.html" class="struct" title="struct petgraph::stable_graph::Neighbors">Neighbors</a>\<'\_, E, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#" class="tooltip" data-notable-ty="Neighbors&lt;&#39;_, E, Ix&gt;">ⓘ</a>

Return an iterator of all neighbors that have an edge between them and `a`, in either direction. If the graph’s edges are undirected, this is equivalent to *.neighbors(a)*.

- `Directed` and `Undirected`: All edges connected to `a`.

Produces an empty iterator if the node doesn’t exist.  
Iterator element type is `NodeIndex<Ix>`.

All outgoing neighbors are listed first followed by all incoming neighbors. The ordering among the outgoing and incoming neighbors respectively is the reverse order of their addition to the graph. That is, the most recently added edge’s neighbor is listed first. Outgoing and incoming in this case refer to the ordering in which the endpoints were listed when adding the edge (`g.add_edge(a, b, w)` or `g.add_edge(b, a, w)`).

Use [`.neighbors_undirected(a).detach()`](https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Neighbors.html#method.detach) to get a neighbor walker that does not borrow from the graph.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edges" class="fn">edges</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Edges.html" class="struct" title="struct petgraph::stable_graph::Edges">Edges</a>\<'\_, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#" class="tooltip" data-notable-ty="Edges&lt;&#39;_, E, Ty, Ix&gt;">ⓘ</a>

Return an iterator of all edges of `a`.

- `Directed`: Outgoing edges from `a`.
- `Undirected`: All edges connected to `a`.

Produces an empty iterator if the node doesn’t exist.  
Iterator element type is `EdgeReference<E, Ix>`.

For a `Directed` graph, edges are listed in reverse order of their addition to the graph, so the most recently added edge is listed first.

For the ordering in case of an `Undirected` graph, please refer to the `Undirected` case in the documentation of [`Graph::edges_directed`](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#method.edges_directed "method petgraph::graph::Graph::edges_directed").

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edges_directed" class="fn">edges_directed</a>( &self, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Edges.html" class="struct" title="struct petgraph::stable_graph::Edges">Edges</a>\<'\_, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#" class="tooltip" data-notable-ty="Edges&lt;&#39;_, E, Ty, Ix&gt;">ⓘ</a>

Return an iterator of all edges of `a`, in the specified direction.

- `Directed`, `Outgoing`: All edges from `a`.
- `Directed`, `Incoming`: All edges to `a`.
- `Undirected`, `Outgoing`: All edges connected to `a`, with `a` being the source of each edge.
- `Undirected`, `Incoming`: All edges connected to `a`, with `a` being the target of each edge.

Produces an empty iterator if the node `a` doesn’t exist.  
Iterator element type is `EdgeReference<E, Ix>`.

For a `Directed` graph, edges are listed in reverse order of their addition to the graph, so the most recently added edge is listed first.

For an `Undirected` graph, the outgoing edges are listed first, then all incoming edges. The ordering among the outgoing and incoming edges respectively is the reverse order of their addition to the graph, similar to the `Directed` case. Outgoing and incoming in this case refer to the ordering in which the endpoints were listed when adding the edge (`g.add_edge(a, b, w)` or `g.add_edge(b, a, w)`).

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.externals" class="fn">externals</a>(&self, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Externals.html" class="struct" title="struct petgraph::stable_graph::Externals">Externals</a>\<'\_, N, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#" class="tooltip" data-notable-ty="Externals&lt;&#39;_, N, Ty, Ix&gt;">ⓘ</a>

Return an iterator over either the nodes without edges to them (`Incoming`) or from them (`Outgoing`).

An *internal* node has both incoming and outgoing edges. The nodes in `.externals(Incoming)` are the source nodes and `.externals(Outgoing)` are the sinks of the graph.

For a graph with undirected edges, both the sinks and the sources are just the nodes without edges.

The whole iteration computes in **O(\|V\|)** time where V is the set of nodes.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.into_nodes_edges_iters" class="fn">into_nodes_edges_iters</a>( self, ) -\> (impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphNode.html" class="struct" title="struct petgraph::stable_graph::StableGraphNode">StableGraphNode</a>\<N, Ix\>\>, impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraphEdge.html" class="struct" title="struct petgraph::stable_graph::StableGraphEdge">StableGraphEdge</a>\<E, Ix\>\>)

Convert the `StableGraph` into iterators of Nodes and Edges

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.index_twice_mut" class="fn">index_twice_mut</a>\<T, U\>( &mut self, i: T, j: U, ) -\> (&mut \<Self as <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<T\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype" title="type core::ops::index::Index::Output">Output</a>, &mut \<Self as <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<U\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype" title="type core::ops::index::Index::Output">Output</a>)

where Self: <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html" class="trait" title="trait core::ops::index::IndexMut">IndexMut</a>\<T\> + <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html" class="trait" title="trait core::ops::index::IndexMut">IndexMut</a>\<U\>, T: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.GraphIndex.html" class="trait" title="trait petgraph::graph::GraphIndex">GraphIndex</a>, U: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.GraphIndex.html" class="trait" title="trait petgraph::graph::GraphIndex">GraphIndex</a>,

Index the `StableGraph` by two indices, any combination of node or edge indices is fine.

**Panics** if the indices are equal or if they are not found.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.retain_nodes" class="fn">retain_nodes</a>\<F\>(&mut self, visit: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Frozen.html" class="struct" title="struct petgraph::graph::Frozen">Frozen</a>\<'\_, Self\>, <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Keep all nodes that return `true` from the `visit` closure, remove the others.

`visit` is provided a proxy reference to the graph, so that the graph can be walked and associated data modified.

The order nodes are visited is not specified.

The node indices of the removed nodes are invalidated, but none other. Edge indices are invalidated as they would be following the removal of each edge with an endpoint in a removed node.

Computes in **O(n + e’)** time, where **n** is the number of node indices and **e’** is the number of affected edges, including *n* calls to `.remove_edge()` where *n* is the number of edges with an endpoint in a removed node.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.retain_edges" class="fn">retain_edges</a>\<F\>(&mut self, visit: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Frozen.html" class="struct" title="struct petgraph::graph::Frozen">Frozen</a>\<'\_, Self\>, <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Keep all edges that return `true` from the `visit` closure, remove the others.

`visit` is provided a proxy reference to the graph, so that the graph can be walked and associated data modified.

The order edges are visited is not specified.

The edge indices of the removed edes are invalidated, but none other.

Computes in **O(e’‘)** time, **e’** is the number of affected edges, including the calls to `.remove_edge()` for each removed edge.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.from_edges" class="fn">from_edges</a>\<I\>(iterable: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, I::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>: <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\>, \<I::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a> as <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\>\>::<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::IntoWeightedEdge::NodeId">NodeId</a>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>\>, N: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Create a new `StableGraph` from an iterable of edges.

Node weights `N` are set to default values. Edge weights `E` may either be specified in the list, or they are filled with default values.

Nodes are inserted automatically to match the edges.

``` rust
use petgraph::stable_graph::StableGraph;

let gr = StableGraph::<(), i32>::from_edges(&[
    (0, 1), (0, 2), (0, 3),
    (1, 2), (1, 3),
    (2, 3),
]);
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.map" class="fn">map</a>\<'a, F, G, N2, E2\>( &'a self, node_map: F, edge_map: G, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N2, E2, Ty, Ix\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a N</a>) -\> N2, G: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a E</a>) -\> E2,

Create a new `StableGraph` by mapping node and edge weights to new values.

The resulting graph has the same structure and the same graph indices as `self`.

If you want a consuming version of this function, see [`map_owned`](https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.map_owned).

``` rust
use petgraph::stable_graph::StableGraph;

// Create an undirected graph with city names as node data and their distances as edge data.
let mut g = StableGraph::<String, u32>::new();

let bie = g.add_node("Bielefeld".to_owned());
let del = g.add_node("New Delhi".to_owned());
let mex = g.add_node("Mexico City".to_owned());
let syd = g.add_node("Sydney".to_owned());

// Add distances in kilometers as edge data.
g.extend_with_edges(&[
    (bie, del, 6_000),
    (bie, mex, 10_000),
    (bie, syd, 16_000),
    (del, mex, 14_000),
    (del, syd, 12_000),
    (mex, syd, 15_000),
]);

// We might now want to change up the distances to be in miles instead and to be strings.
// We can do this using the `map` method, which takes two closures for the node and edge data,
// respectively, and returns a new graph with the transformed data.
let g_miles: StableGraph<String, i32> = g.map(
    |_, city| city.to_owned(),
    |_, &distance| (distance as f64 * 0.621371).round() as i32,
);

for &edge_weight in g_miles.edge_weights() {
    assert!(edge_weight < 10_000);
}
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.map_owned" class="fn">map_owned</a>\<F, G, N2, E2\>( self, node_map: F, edge_map: G, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N2, E2, Ty, Ix\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, N) -\> N2, G: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>, E) -\> E2,

Create a new `StableGraph` by mapping node and edge weights to new values, consuming the current graph.

The resulting graph has the same structure and the same graph indices as `self`.

If you want a non-consuming version of this function, see [`map`](https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.map).

``` rust
use petgraph::stable_graph::StableGraph;

// Create an undirected graph with city names as node data and their distances as edge data.
let mut g = StableGraph::<String, u32>::new();

let bie = g.add_node("Bielefeld".to_owned());
let del = g.add_node("New Delhi".to_owned());
let mex = g.add_node("Mexico City".to_owned());
let syd = g.add_node("Sydney".to_owned());

// Add distances in kilometers as edge data.
g.extend_with_edges(&[
    (bie, del, 6_000),
    (bie, mex, 10_000),
    (bie, syd, 16_000),
    (del, mex, 14_000),
    (del, syd, 12_000),
    (mex, syd, 15_000),
]);

// We might now want to change up the distances to be in miles instead and to be strings.
// We can do this using the `map` method, which takes two closures for the node and edge data,
// respectively, and returns a new graph with the transformed data.
let g_miles: StableGraph<String, i32> = g.map_owned(
    |_, city| city,
    |_, distance| (distance as f64 * 0.621371).round() as i32,
);

for &edge_weight in g_miles.edge_weights() {
    assert!(edge_weight < 10_000);
}
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.filter_map" class="fn">filter_map</a>\<'a, F, G, N2, E2\>( &'a self, node_map: F, edge_map: G, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N2, E2, Ty, Ix\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a N</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<N2\>, G: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a E</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<E2\>,

Create a new `StableGraph` by mapping nodes and edges. A node or edge may be mapped to `None` to exclude it from the resulting graph.

Nodes are mapped first with the `node_map` closure, then `edge_map` is called for the edges that have not had any endpoint removed.

The resulting graph has the structure of a subgraph of the original graph. Nodes and edges that are not removed maintain their old node or edge indices.

If you want a consuming version of this function, see [`filter_map_owned`](https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.filter_map_owned).

``` rust
use petgraph::stable_graph::StableGraph;

// Create a graph with integer node weights
let mut g = StableGraph::<u32, ()>::new();
let a = g.add_node(0);
let b = g.add_node(2);
let c = g.add_node(5);
let d = g.add_node(7);
let e = g.add_node(4);
g.extend_with_edges(&[(a, b, ()), (a, c, ()), (b, d, ()), (c, d, ()), (d, e, ())]);

// Filter the graph such that only nodes with weight greater than 2 are kept.
let g_filtered = g.filter_map(
    |_, &node_weight| if node_weight > 2 { Some(node_weight) } else { None },
    |_, &edge_weight| Some(edge_weight),
);

assert_eq!(g_filtered.node_count(), 3);
// Node and edge indices are preserved.
assert_eq!(g_filtered.node_weight(c), Some(&5));
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.filter_map_owned" class="fn">filter_map_owned</a>\<F, G, N2, E2\>( self, node_map: F, edge_map: G, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N2, E2, Ty, Ix\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, N) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<N2\>, G: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>, E) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<E2\>,

Create a new `StableGraph` by mapping nodes and edges, consuming the current graph. A node or edge may be mapped to `None` to exclude it from the resulting graph.

Nodes are mapped first with the `node_map` closure, then `edge_map` is called for the edges that have not had any endpoint removed.

The resulting graph has the structure of a subgraph of the original graph. Nodes and edges that are not removed maintain their old node or edge indices.

If you want a non-consuming version of this function, see [`filter_map`](https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.filter_map).

``` rust
use petgraph::stable_graph::StableGraph;

// Create a graph with integer node weights
let mut g = StableGraph::<u32, ()>::new();
let a = g.add_node(0);
let b = g.add_node(2);
let c = g.add_node(5);
let d = g.add_node(7);
let e = g.add_node(4);
g.extend_with_edges(&[(a, b, ()), (a, c, ()), (b, d, ()), (c, d, ()), (d, e, ())]);

// Filter the graph such that only nodes with weight greater than 2 are kept.
let g_filtered = g.filter_map_owned(
    |_, node_weight| if node_weight > 2 { Some(node_weight) } else { None },
    |_, edge_weight| Some(edge_weight),
);

assert_eq!(g_filtered.node_count(), 3);
// Node and edge indices are preserved.
assert_eq!(g_filtered.node_weight(c), Some(&5));
```

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.extend_with_edges" class="fn">extend_with_edges</a>\<I\>(&mut self, iterable: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, I::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>: <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\>, \<I::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a> as <a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html" class="trait" title="trait petgraph::IntoWeightedEdge">IntoWeightedEdge</a>\<E\>\>::<a href="https://docs.rs/petgraph/latest/petgraph/trait.IntoWeightedEdge.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::IntoWeightedEdge::NodeId">NodeId</a>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>\>, N: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Extend the graph from an iterable of edges.

Node weights `N` are set to default values. Edge weights `E` may either be specified in the list, or they are filled with default values.

Nodes are inserted automatically to match the edges.

``` rust
use petgraph::stable_graph::StableGraph;

let mut g = StableGraph::<(), i32>::new();
let a = g.add_node(());
let b = g.add_node(());
let c = g.add_node(());
let d = g.add_node(());

g.extend_with_edges(&[
  (a, b, 7),
  (a, c, 8),
  (a, d, 9),
 (b, c, 10),
]);
```

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Arbitrary-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Arbitrary.html" class="trait" title="trait quickcheck::arbitrary::Arbitrary">Arbitrary</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where N: <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Arbitrary.html" class="trait" title="trait quickcheck::arbitrary::Arbitrary">Arbitrary</a>, E: <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Arbitrary.html" class="trait" title="trait quickcheck::arbitrary::Arbitrary">Arbitrary</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Available on **crate feature `stable_graph`** only.

`Arbitrary` for `StableGraph` creates a graph by selecting a node count and a probability for each possible edge to exist.

The result will be simple graph or digraph, with possible self loops, no parallel edges.

The exact properties of the produced graph is subject to change.

Requires crate features `"quickcheck"` and `"stable_graph"`

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.arbitrary" class="anchor">§</a>

#### fn <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Arbitrary.html#tymethod.arbitrary" class="fn">arbitrary</a>\<G: <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Gen.html" class="trait" title="trait quickcheck::arbitrary::Gen">Gen</a>\>(g: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut G</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.shrink" class="anchor">§</a>

#### fn <a href="https://docs.rs/quickcheck/0.8.5/x86_64-unknown-linux-gnu/quickcheck/arbitrary/trait.Arbitrary.html#method.shrink" class="fn">shrink</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = Self\>\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Build-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html" class="trait" title="trait petgraph::data::Build">Build</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Available on **crate feature `stable_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.add_node-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#tymethod.add_node" class="fn">add_node</a>(&mut self, weight: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.add_edge-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#method.add_edge" class="fn">add_edge</a>( &mut self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, b: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, weight: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>\>

Add a new edge. If parallel edges (duplicate) are not allowed and the edge already exists, return `None`. [Read more](https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#method.add_edge)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.update_edge-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#tymethod.update_edge" class="fn">update_edge</a>( &mut self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, b: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, weight: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>, ) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>

Add or update the edge from `a` to `b`. Return the id of the affected edge. [Read more](https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#tymethod.update_edge)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Clone-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, E: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, Ix: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

The resulting cloned graph has the same graph indices as `self`.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> Self

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, rhs: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Create-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html" class="trait" title="trait petgraph::data::Create">Create</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Available on **crate feature `stable_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.with_capacity-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Create.html#tymethod.with_capacity" class="fn">with_capacity</a>(nodes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, edges: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Data-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.NodeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype">NodeWeight</a> = N

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.EdgeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype">EdgeWeight</a> = E

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-DataMap-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMap.html" class="trait" title="trait petgraph::data::DataMap">DataMap</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Available on **crate feature `stable_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_weight-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMap.html#tymethod.node_weight" class="fn">node_weight</a>(&self, id: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_weight-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMap.html#tymethod.edge_weight" class="fn">edge_weight</a>(&self, id: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-DataMapMut-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Available on **crate feature `stable_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_weight_mut-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#tymethod.node_weight_mut" class="fn">node_weight_mut</a>(&mut self, id: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_weight_mut-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#tymethod.edge_weight_mut" class="fn">edge_weight_mut</a>(&mut self, id: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Debug-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where N: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, E: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Default-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Create a new empty `StableGraph`.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Deserialize%3C&#39;de%3E-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<'de, N, E, Ty, Ix\> <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a> + <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\>, N: <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\>, E: <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\>,

Requires crate feature `"serde-1"`

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>(deserializer: D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, D::<a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-EdgeCount-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeCount.html" class="trait" title="trait petgraph::visit::EdgeCount">EdgeCount</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_count-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeCount.html#tymethod.edge_count" class="fn">edge_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of edges in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-EdgeIndexable-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html" class="trait" title="trait petgraph::visit::EdgeIndexable">EdgeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.edge_bound" class="fn">edge_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return an upper bound of the edge indices in the graph (suitable for the size of a bitmap).

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.to_index-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.to_index" class="fn">to_index</a>(&self, ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert `a` to an integer index.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.from_index-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeIndexable.html#tymethod.from_index" class="fn">from_index</a>(&self, ix: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::GraphBase::EdgeId">EdgeId</a>

Convert `i` to an edge index. `i` must be a valid value in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-From%3CGraph%3CN,+E,+Ty,+Ix%3E%3E-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>\> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Convert a `Graph` into a `StableGraph`

Computes in **O(\|V\| + \|E\|)** time where V is the set of nodes and E is the set of edges.

The resulting graph has the same node and edge indices as the original graph.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(g: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-From%3CStableGraph%3CN,+E,+Ty,+Ix%3E%3E-for-Graph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>\> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html" class="struct" title="struct petgraph::graph::Graph">Graph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Convert a `StableGraph` into a `Graph`

Computes in **O(\|V\| + \|E\|)** time where V is the set of nodes and E is the set of edges.

This translates the stable graph into a graph with node and edge indices in a compact interval without holes (like `Graph`s always are).

Only if the stable graph had no vacancies after deletions (if node bound was equal to node count, and the same for edges), would the resulting graph have the same node and edge indices as the input.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(graph: <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-FromElements-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html" class="trait" title="trait petgraph::data::FromElements">FromElements</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Available on **crate feature `stable_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.from_elements" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.FromElements.html#method.from_elements" class="fn">from_elements</a>\<I\>(iterable: I) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>, Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>\>\>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-FromGraph6-for-StableGraph%3C(),+(),+Undirected,+Ix%3E" class="anchor">§</a>

### impl\<Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.FromGraph6.html" class="trait" title="trait petgraph::graph6::FromGraph6">FromGraph6</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>, Ix\>

Available on **crate feature `stable_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.from_graph6_string" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.FromGraph6.html#tymethod.from_graph6_string" class="fn">from_graph6_string</a>(graph6_string: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-GetAdjacencyMatrix-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Available on **crate feature `stable_graph`** only.

The adjacency matrix for **Graph** is a bitmap that’s computed by `.adjacency_matrix()`.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.AdjMatrix" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

The associated adjacency matrix type

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.adjacency_matrix" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.adjacency_matrix" class="fn">adjacency_matrix</a>(&self) -\> <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

Create the adjacency matrix

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.is_adjacent" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.is_adjacent" class="fn">is_adjacent</a>( &self, matrix: &<a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>, a: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if there is an edge from `a` to `b`, false otherwise. [Read more](https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.is_adjacent)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-GraphBase-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.NodeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>

node identifier

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.EdgeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>

edge identifier

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-GraphProp-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.EdgeType" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#associatedtype.EdgeType" class="associatedtype">EdgeType</a> = Ty

The kind of edges in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.is_directed-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#method.is_directed" class="fn">is_directed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Index%3CEdgeIndex%3CIx%3E%3E-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>\> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Index the `StableGraph` by `EdgeIndex` to access edge weights.

**Panics** if the edge doesn’t exist.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = E

The returned type after indexing.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.index-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>(&self, index: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Index%3CNodeIndex%3CIx%3E%3E-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>\> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Index the `StableGraph` by `NodeIndex` to access node weights.

**Panics** if the node doesn’t exist.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = N

The returned type after indexing.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.index" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>(&self, index: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;N</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-IndexMut%3CEdgeIndex%3CIx%3E%3E-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html" class="trait" title="trait core::ops::index::IndexMut">IndexMut</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>\> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Index the `StableGraph` by `EdgeIndex` to access edge weights.

**Panics** if the edge doesn’t exist.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.index_mut-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut" class="fn">index_mut</a>(&mut self, index: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.EdgeIndex.html" class="struct" title="struct petgraph::graph::EdgeIndex">EdgeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-IndexMut%3CNodeIndex%3CIx%3E%3E-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html" class="trait" title="trait core::ops::index::IndexMut">IndexMut</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>\> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

Index the `StableGraph` by `NodeIndex` to access node weights.

**Panics** if the node doesn’t exist.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.index_mut" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut" class="fn">index_mut</a>(&mut self, index: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut N</a>

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-IntoEdgeReferences-for-%26StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<'a, N: 'a, E: 'a, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edge_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#tymethod.edge_references" class="fn">edge_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeReferences">EdgeReferences</a>

Create an iterator over all edges in the graph, in indexed order.

Iterator element type is `EdgeReference<E, Ix>`.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.EdgeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype">EdgeRef</a> = <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.EdgeReference.html" class="struct" title="struct petgraph::stable_graph::EdgeReference">EdgeReference</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.EdgeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype">EdgeReferences</a> = <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.EdgeReferences.html" class="struct" title="struct petgraph::stable_graph::EdgeReferences">EdgeReferences</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-IntoEdges-for-%26StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html" class="trait" title="trait petgraph::visit::IntoEdges">IntoEdges</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.Edges" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype">Edges</a> = <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Edges.html" class="struct" title="struct petgraph::stable_graph::Edges">Edges</a>\<'a, E, Ty, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edges-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#tymethod.edges" class="fn">edges</a>(self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype" title="type petgraph::visit::IntoEdges::Edges">Edges</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-IntoEdgesDirected-for-%26StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html" class="trait" title="trait petgraph::visit::IntoEdgesDirected">IntoEdgesDirected</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.EdgesDirected" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#associatedtype.EdgesDirected" class="associatedtype">EdgesDirected</a> = <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Edges.html" class="struct" title="struct petgraph::stable_graph::Edges">Edges</a>\<'a, E, Ty, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.edges_directed-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#tymethod.edges_directed" class="fn">edges_directed</a>(self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, dir: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgesDirected.html#associatedtype.EdgesDirected" class="associatedtype" title="type petgraph::visit::IntoEdgesDirected::EdgesDirected">EdgesDirected</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-IntoNeighbors-for-%26StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E: 'a, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.Neighbors" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype">Neighbors</a> = <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Neighbors.html" class="struct" title="struct petgraph::stable_graph::Neighbors">Neighbors</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.neighbors-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#tymethod.neighbors" class="fn">neighbors</a>(self, n: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype" title="type petgraph::visit::IntoNeighbors::Neighbors">Neighbors</a>

Return an iterator of the neighbors of node `a`.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-IntoNeighborsDirected-for-%26StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E: 'a, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.NeighborsDirected" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#associatedtype.NeighborsDirected" class="associatedtype">NeighborsDirected</a> = <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.Neighbors.html" class="struct" title="struct petgraph::stable_graph::Neighbors">Neighbors</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.neighbors_directed-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#tymethod.neighbors_directed" class="fn">neighbors_directed</a>( self, n: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, d: <a href="https://docs.rs/petgraph/latest/petgraph/enum.Direction.html" class="enum" title="enum petgraph::Direction">Direction</a>, ) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html#associatedtype.NeighborsDirected" class="associatedtype" title="type petgraph::visit::IntoNeighborsDirected::NeighborsDirected">NeighborsDirected</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-IntoNodeIdentifiers-for-%26StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E: 'a, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.NodeIdentifiers" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype">NodeIdentifiers</a> = <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.NodeIndices.html" class="struct" title="struct petgraph::stable_graph::NodeIndices">NodeIndices</a>\<'a, N, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_identifiers" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#tymethod.node_identifiers" class="fn">node_identifiers</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype" title="type petgraph::visit::IntoNodeIdentifiers::NodeIdentifiers">NodeIdentifiers</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-IntoNodeReferences-for-%26StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<'a, N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.NodeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeRef" class="associatedtype">NodeRef</a> = (<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a N</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.NodeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype">NodeReferences</a> = <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.NodeReferences.html" class="struct" title="struct petgraph::stable_graph::NodeReferences">NodeReferences</a>\<'a, N, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#tymethod.node_references" class="fn">node_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype" title="type petgraph::visit::IntoNodeReferences::NodeReferences">NodeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-NodeCount-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html" class="trait" title="trait petgraph::visit::NodeCount">NodeCount</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_count-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html#tymethod.node_count" class="fn">node_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-NodeIndexable-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.node_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.node_bound" class="fn">node_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return an upper bound of the node indices in the graph

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.to_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.to_index" class="fn">to_index</a>(&self, ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert `a` to an integer index.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.from_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.from_index" class="fn">from_index</a>(&self, ix: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

Convert `i` to a node index. `i` must be a valid value in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Serialize-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a> + <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a>, N: <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a>, E: <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a>,

Requires crate feature `"serde-1"`

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>(&self, serializer: S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<S::<a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde_core::ser::Serializer::Ok">Ok</a>, S::<a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html" class="trait" title="trait serde_core::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-ToGraph6-for-StableGraph%3CN,+E,+Undirected,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html" class="trait" title="trait petgraph::graph6::ToGraph6">ToGraph6</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Undirected.html" class="enum" title="enum petgraph::Undirected">Undirected</a>, Ix\>

Available on **crate feature `stable_graph`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.graph6_string" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph6/trait.ToGraph6.html#tymethod.graph6_string" class="fn">graph6_string</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-TryFrom%3CStableGraph%3CN,+E,+Directed,+Ix%3E%3E-for-Acyclic%3CStableGraph%3CN,+E,+Directed,+Ix%3E%3E" class="anchor">§</a>

### impl\<N, E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>, Ix\>\> for <a href="https://docs.rs/petgraph/latest/petgraph/acyclic/struct.Acyclic.html" class="struct" title="struct petgraph::acyclic::Acyclic">Acyclic</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/type.StableDiGraph.html" class="type" title="type petgraph::stable_graph::StableDiGraph">StableDiGraph</a>\<N, E, Ix\>\>

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.Cycle.html" class="struct" title="struct petgraph::algo::Cycle">Cycle</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>\>

The type returned in the event of a conversion error.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(graph: <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/type.StableDiGraph.html" class="type" title="type petgraph::stable_graph::StableDiGraph">StableDiGraph</a>\<N, E, Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, Self::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#impl-Visitable-for-StableGraph%3CN,+E,+Ty,+Ix%3E" class="anchor">§</a>

### impl\<N, E, Ty, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html" class="struct" title="struct petgraph::stable_graph::StableGraph">StableGraph</a>\<N, E, Ty, Ix\>

where Ty: <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#associatedtype.Map" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype">Map</a> = <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

The associated map type

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.visit_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.visit_map" class="fn">visit_map</a>(&self) -\> <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

Create a new visitor map

<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#method.reset_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.reset_map" class="fn">reset_map</a>(&self, map: &mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>)

Reset the visitor map (and resize to new size of graph if needed)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html#blanket-implementations" class="anchor">§</a>
