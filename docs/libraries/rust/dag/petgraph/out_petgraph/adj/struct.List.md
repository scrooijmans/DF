# Struct List Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/adj.rs.html#157-162" class="src">Source</a>

``` rust
pub struct List<E, Ix = DefaultIx>where
    Ix: IndexType,{ /* private fields */ }
```

Expand description

An adjacency list with labeled edges.

Can be interpreted as a directed graph with unweighted nodes.

This is the most simple adjacency list you can imagine. [`Graph`](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html), in contrast, maintains both the list of successors and predecessors for each node, which is a different trade-off.

Allows parallel edges and self-loops.

This data structure is append-only (except for [`clear`](https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.clear)), so indices returned at some point for a given graph will stay valid with this same graph until it is dropped or [`clear`](https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.clear) is called.

Space consumption: **O(\|E\|)**.

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

Creates a new, empty adjacency list.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.with_capacity" class="fn">with_capacity</a>(nodes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

Creates a new, empty adjacency list tailored for `nodes` nodes.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.clear" class="fn">clear</a>(&mut self)

Removes all nodes and edges from the list.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.edge_count" class="fn">edge_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of edges in the list

Computes in **O(\|V\|)** time.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.add_node" class="fn">add_node</a>(&mut self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>

Adds a new node to the list. This allocates a new `Vec` and then should run in amortized **O(1)** time.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.add_node_with_capacity" class="fn">add_node_with_capacity</a>(&mut self, successors: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>

Adds a new node to the list. This allocates a new `Vec` and then should run in amortized **O(1)** time.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.add_node_from_edges" class="fn">add_node_from_edges</a>\<I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = (<a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, E)\>\>( &mut self, edges: I, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>

Adds a new node to the list by giving its list of successors and the corresponding weigths.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.add_edge" class="fn">add_edge</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, weight: E, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndex.html" class="struct" title="struct petgraph::adj::EdgeIndex">EdgeIndex</a>\<Ix\>

Add an edge from `a` to `b` to the graph, with its associated data `weight`.

Return the index of the new edge.

Computes in **O(1)** time.

**Panics** if the source node does not exist.  

**Note:** `List` allows adding parallel (“duplicate”) edges. If you want to avoid this, use [`.update_edge(a, b, weight)`](https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.update_edge) instead.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.edge_endpoints" class="fn">edge_endpoints</a>( &self, e: <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndex.html" class="struct" title="struct petgraph::adj::EdgeIndex">EdgeIndex</a>\<Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>)\>

Accesses the source and target of edge `e`

Computes in **O(1)**

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.edge_indices_from" class="fn">edge_indices_from</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.OutgoingEdgeIndices.html" class="struct" title="struct petgraph::adj::OutgoingEdgeIndices">OutgoingEdgeIndices</a>\<Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#" class="tooltip" data-notable-ty="OutgoingEdgeIndices&lt;Ix&gt;">ⓘ</a>

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.contains_edge" class="fn">contains_edge</a>(&self, a: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Lookups whether there is an edge from `a` to `b`.

Computes in **O(e’)** time, where **e’** is the number of successors of `a`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.find_edge" class="fn">find_edge</a>( &self, a: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndex.html" class="struct" title="struct petgraph::adj::EdgeIndex">EdgeIndex</a>\<Ix\>\>

Lookups whether there is an edge from `a` to `b`.

Computes in **O(e’)** time, where **e’** is the number of successors of `a`.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.node_indices" class="fn">node_indices</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.NodeIndices.html" class="struct" title="struct petgraph::adj::NodeIndices">NodeIndices</a>\<Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#" class="tooltip" data-notable-ty="NodeIndices&lt;Ix&gt;">ⓘ</a>

Returns an iterator over all node indices of the graph.

Consuming the whole iterator take **O(\|V\|)**.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.edge_indices" class="fn">edge_indices</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndices.html" class="struct" title="struct petgraph::adj::EdgeIndices">EdgeIndices</a>\<'\_, E, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#" class="tooltip" data-notable-ty="EdgeIndices&lt;&#39;_, E, Ix&gt;">ⓘ</a>

Returns an iterator over all edge indices of the graph.

Consuming the whole iterator take **O(\|V\| + \|E\|)**.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-Build-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html" class="trait" title="trait petgraph::data::Build">Build</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.add_node-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#tymethod.add_node" class="fn">add_node</a>(&mut self, \_weight: <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>

Adds a new node to the list. This allocates a new `Vec` and then should run in amortized **O(1)** time.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.add_edge-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#method.add_edge" class="fn">add_edge</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, weight: E, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndex.html" class="struct" title="struct petgraph::adj::EdgeIndex">EdgeIndex</a>\<Ix\>\>

Add an edge from `a` to `b` to the graph, with its associated data `weight`.

Return the index of the new edge.

Computes in **O(1)** time.

**Panics** if the source node does not exist.  

**Note:** `List` allows adding parallel (“duplicate”) edges. If you want to avoid this, use [`.update_edge(a, b, weight)`](https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.update_edge) instead.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.update_edge" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.Build.html#tymethod.update_edge" class="fn">update_edge</a>( &mut self, a: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, weight: E, ) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndex.html" class="struct" title="struct petgraph::adj::EdgeIndex">EdgeIndex</a>\<Ix\>

Updates or adds an edge from `a` to `b` to the graph, with its associated data `weight`.

Return the index of the new edge.

Computes in **O(e’)** time, where **e’** is the number of successors of `a`.

**Panics** if the source node does not exist.  

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-Clone-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, Ix\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-Data-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html" class="trait" title="trait petgraph::visit::Data">Data</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.NodeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype">NodeWeight</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.EdgeWeight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype">EdgeWeight</a> = E

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-DataMap-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMap.html" class="trait" title="trait petgraph::data::DataMap">DataMap</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.edge_weight" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMap.html#tymethod.edge_weight" class="fn">edge_weight</a>(&self, e: <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndex.html" class="struct" title="struct petgraph::adj::EdgeIndex">EdgeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;E</a>\>

Accesses the weight of edge `e`

Computes in **O(1)**

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.node_weight" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMap.html#tymethod.node_weight" class="fn">node_weight</a>(&self, n: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-DataMapMut-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html" class="trait" title="trait petgraph::data::DataMapMut">DataMapMut</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.edge_weight_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#tymethod.edge_weight_mut" class="fn">edge_weight_mut</a>(&mut self, e: <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndex.html" class="struct" title="struct petgraph::adj::EdgeIndex">EdgeIndex</a>\<Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>\>

Accesses the weight of edge `e`

Computes in **O(1)**

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.node_weight_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.DataMapMut.html#tymethod.node_weight_mut" class="fn">node_weight_mut</a>(&mut self, n: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-Debug-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

where E: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-Default-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>, Ix\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-EdgeCount-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeCount.html" class="trait" title="trait petgraph::visit::EdgeCount">EdgeCount</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.edge_count-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeCount.html#tymethod.edge_count" class="fn">edge_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of edges in the list

Computes in **O(\|V\|)** time.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-GetAdjacencyMatrix-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html" class="trait" title="trait petgraph::visit::GetAdjacencyMatrix">GetAdjacencyMatrix</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

The adjacency matrix for **List** is a bitmap that’s computed by `.adjacency_matrix()`.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.AdjMatrix" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#associatedtype.AdjMatrix" class="associatedtype">AdjMatrix</a> = <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

The associated adjacency matrix type

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.adjacency_matrix" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.adjacency_matrix" class="fn">adjacency_matrix</a>(&self) -\> <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

Create the adjacency matrix

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.is_adjacent" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.is_adjacent" class="fn">is_adjacent</a>( &self, matrix: &<a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>, a: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, b: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if there is an edge from `a` to `b`, false otherwise. [Read more](https://docs.rs/petgraph/latest/petgraph/visit/trait.GetAdjacencyMatrix.html#tymethod.is_adjacent)

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-GraphBase-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.NodeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Ix

node identifier

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.EdgeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndex.html" class="struct" title="struct petgraph::adj::EdgeIndex">EdgeIndex</a>\<Ix\>

edge identifier

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-GraphProp-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.EdgeType" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#associatedtype.EdgeType" class="associatedtype">EdgeType</a> = <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>

The kind of edges in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.is_directed" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html#method.is_directed" class="fn">is_directed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-IntoEdgeReferences-for-%26List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<'a, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>, E\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.EdgeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype">EdgeRef</a> = <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.EdgeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype">EdgeReferences</a> = <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReferences.html" class="struct" title="struct petgraph::adj::EdgeReferences">EdgeReferences</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.edge_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#tymethod.edge_references" class="fn">edge_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeReferences" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeReferences">EdgeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-IntoEdges-for-%26List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<'a, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>, E\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html" class="trait" title="trait petgraph::visit::IntoEdges">IntoEdges</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.Edges" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype">Edges</a> = <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.OutgoingEdgeReferences.html" class="struct" title="struct petgraph::adj::OutgoingEdgeReferences">OutgoingEdgeReferences</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.edges" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#tymethod.edges" class="fn">edges</a>(self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdges.html#associatedtype.Edges" class="associatedtype" title="type petgraph::visit::IntoEdges::Edges">Edges</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-IntoNeighbors-for-%26List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<'a, E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a> for &'a <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.neighbors" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#tymethod.neighbors" class="fn">neighbors</a>(self, a: <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype" title="type petgraph::visit::IntoNeighbors::Neighbors">Neighbors</a>

Returns an iterator of all nodes with an edge starting from `a`. Panics if `a` is out of bounds. Use [`List::edge_indices_from`](https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.edge_indices_from "method petgraph::adj::List::edge_indices_from") instead if you do not want to borrow the adjacency list while iterating.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.Neighbors" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html#associatedtype.Neighbors" class="associatedtype">Neighbors</a> = <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.Neighbors.html" class="struct" title="struct petgraph::adj::Neighbors">Neighbors</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-IntoNodeIdentifiers-for-%26List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a> for &<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.NodeIdentifiers" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#associatedtype.NodeIdentifiers" class="associatedtype">NodeIdentifiers</a> = <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.NodeIndices.html" class="struct" title="struct petgraph::adj::NodeIndices">NodeIndices</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.node_identifiers" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html#tymethod.node_identifiers" class="fn">node_identifiers</a>(self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.NodeIndices.html" class="struct" title="struct petgraph::adj::NodeIndices">NodeIndices</a>\<Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#" class="tooltip" data-notable-ty="NodeIndices&lt;Ix&gt;">ⓘ</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-IntoNodeReferences-for-%26List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>, E\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> for &<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.NodeRef" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeRef" class="associatedtype">NodeRef</a> = Ix

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.NodeReferences" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype">NodeReferences</a> = <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.NodeIndices.html" class="struct" title="struct petgraph::adj::NodeIndices">NodeIndices</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.node_references" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#tymethod.node_references" class="fn">node_references</a>(self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeReferences" class="associatedtype" title="type petgraph::visit::IntoNodeReferences::NodeReferences">NodeReferences</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-NodeCount-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html" class="trait" title="trait petgraph::visit::NodeCount">NodeCount</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.node_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html#tymethod.node_count" class="fn">node_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of nodes in the list

Computes in **O(1)** time.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-NodeIndexable-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.node_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.node_bound" class="fn">node_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return an upper bound of the node indices in the graph (suitable for the size of a bitmap).

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.to_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.to_index" class="fn">to_index</a>(&self, a: Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert `a` to an integer index.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.from_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html#tymethod.from_index" class="fn">from_index</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

Convert `i` to a node index. `i` must be a valid value in the graph.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-Visitable-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#associatedtype.Map" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype">Map</a> = <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

The associated map type

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.visit_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.visit_map" class="fn">visit_map</a>(&self) -\> <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

Create a new visitor map

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#method.reset_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#tymethod.reset_map" class="fn">reset_map</a>(&self, map: &mut Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>)

Reset the visitor map (and resize to new size of graph if needed)

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#impl-NodeCompactIndexable-for-List%3CE,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCompactIndexable.html" class="trait" title="trait petgraph::visit::NodeCompactIndexable">NodeCompactIndexable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html" class="struct" title="struct petgraph::adj::List">List</a>\<E, Ix\>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.List.html#blanket-implementations" class="anchor">§</a>
