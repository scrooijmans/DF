Description: A directed acyclic graph.

Title: Acyclic in petgraph::acyclic - Rust

Docs.rs

- petgraph-0.8.3

- petgraph 0.8.3
- Permalink
- Docs.rs crate page
- MIT OR Apache-2.0

- Links
- Repository
- crates.io
- Source

- Owners
- bluss
- github:petgraph:release-team
- ABorgna

- Dependencies
- - dot-parser ^0.5.1 _normal_ _optional_
- dot-parser-macros ^0.5.1 _normal_ _optional_
- fixedbitset ^0.5.7 _normal_
- hashbrown ^0.15.0 _normal_
- indexmap ^2.5.0 _normal_
- quickcheck ^0.8 _normal_ _optional_
- rayon ^1.5.3 _normal_ _optional_
- serde ^1.0 _normal_ _optional_
- serde_derive ^1.0 _normal_ _optional_
- ahash ^0.7.2 _dev_
- bincode ^1.3.3 _dev_
- defmac ^0.2.1 _dev_
- fxhash ^0.2.1 _dev_
- itertools ^0.12.1 _dev_
- odds ^0.4.0 _dev_
- rand ^0.5.5 _dev_

- Versions

- **79.17%** of the crate is documented

- Platform
- i686-pc-windows-msvc
- i686-unknown-linux-gnu
- x86_64-apple-darwin
- x86_64-pc-windows-msvc
- x86_64-unknown-linux-gnu
- Feature flags

- docs.rs
- About docs.rs
- Badges
- Builds
- Metadata
- Shorthand URLs
- Download
- Rustdoc JSON
- Build queue
- Privacy policy

- Rust
- Rust website
- The Book
- Standard Library API Reference
- Rust by Example
- The Cargo Guide
- Clippy Documentation

## Acyclic

petgraph::acyclic

# Struct Acyclic Copy item path

Source

```
pub struct Acyclic<G: Visitable> { /* private fields */ }
```

Expand description

A directed acyclic graph.

Wrap directed acyclic graphs and expose an API that ensures the invariant is maintained, i.e. no cycles can be created. This uses a topological order that is dynamically updated when edges are added. In the worst case, the runtime may be linear in the number of vertices, but it has been shown to be fast in practice, particularly on sparse graphs (Pierce and Kelly, 2004).

To be modifiable (and hence to be useful), the graphs of generic type `G` should implement the `Build` trait. Good candidates for `G` are thus `crate::graph::DiGraph` and `crate::stable_graph::StableDiGraph`.

### §Algorithm

This implements the PK algorithm for dynamic topological sort described in “A Dynamic Topological Sort Algorithm for Directed Acyclic Graphs” by D. Pierce and P. Kelly, JEA, 2004. It maintains a topological order of the nodes that can be efficiently updated when edges are added. Achieves a good balance between simplicity and performance in practice, see the paper for discussions of the running time.

### §Graph traits

All graph traits are delegated to the inner graph, with the exception of the graph construction trait `Build`. The wrapped graph can thus only be modified through the wrapped API that ensures no cycles are created.

### §Behaviour on cycles

By design, edge additions to this datatype may fail. It is recommended to prefer the dedicated `Acyclic::try_add_edge` and `Acyclic::try_update_edge` methods whenever possible. The `Build::update_edge` methods will panic if it is attempted to add an edge that would create a cycle. The `Build::add_edge` on the other hand method will return `None` if the edge cannot be added (either it already exists on a graph type that does not support it or would create a cycle).

## Implementations§

Source§

### impl<G: Visitable\> Acyclic<G>

Source

#### pub fn get_position<'a>(&'a self, id: G::NodeId) -> TopologicalPosition

where &'a G: NodeIndexable + GraphBase<NodeId = G::NodeId\>,

Get the position of a node in the topological sort.

Panics if the node index is out of bounds.

Source

#### pub fn at_position(&self, pos: TopologicalPosition) -> Option<G::NodeId\>

Get the node at a given position in the topological sort, if it exists.

Source§

### impl<G: Visitable\> Acyclic<G>

Source

#### pub fn new() -> Self

where G: Default,

Create a new empty acyclic graph.

Source

#### pub fn nodes_iter(&self) -> impl Iterator<Item = G::NodeId\> + '\_

Get an iterator over the nodes, ordered by their position.

Source

#### pub fn range<'r>( &'r self, range: impl RangeBounds<TopologicalPosition\> + 'r, ) -> impl Iterator<Item = G::NodeId\> + 'r

Get an iterator over the nodes within the range of positions.

The nodes are ordered by their position in the topological sort.

Source

#### pub fn inner(&self) -> &G

Get the underlying graph.

Source

#### pub fn into_inner(self) -> G

Consume the `Acyclic` wrapper and return the underlying graph.

Source§

### impl<G: Visitable + NodeIndexable\> Acyclic<G>

where for<'a> &'a G: IntoNeighborsDirected + IntoNodeIdentifiers + GraphBase<NodeId = G::NodeId\>,

Source

#### pub fn try_from_graph(graph: G) -> Result<Self, Cycle<G::NodeId\>>

Wrap a graph into an acyclic graph.

The graph types `DiGraph` and `StableDiGraph` also implement `TryFrom`, which can be used instead of this method and have looser type bounds.

Source

#### pub fn try_add_edge( &mut self, a: G::NodeId, b: G::NodeId, weight: G::EdgeWeight, ) -> Result<G::EdgeId, AcyclicEdgeError<G::NodeId\>>

where G: Build, G::NodeId: IndexType,

Add an edge to the graph using `Build::add_edge`.

Returns the id of the added edge, or an `AcyclicEdgeError` if the edge would create a cycle, a self-loop or if the edge addition failed in the underlying graph.

In cases where edge addition using `Build::add_edge` cannot fail in the underlying graph (e.g. when multi-edges are allowed, as in `DiGraph` and `StableDiGraph`), this will return an error if and only if `Self::is_valid_edge` returns `false`.

Note that for some graph types, the semantics of `Build::add_edge` may not coincide with the semantics of the `add_edge` method provided by the graph type.

**Panics** if `a` or `b` are not found.

Source

#### pub fn try_update_edge( &mut self, a: G::NodeId, b: G::NodeId, weight: G::EdgeWeight, ) -> Result<G::EdgeId, AcyclicEdgeError<G::NodeId\>>

where G: Build, G::NodeId: IndexType,

Add or update an edge in a graph using `Build::update_edge`.

Returns the id of the updated edge, or an `AcyclicEdgeError` if the edge would create a cycle or a self-loop. If the edge does not exist, the edge is created.

This will return an error if and only if `Self::is_valid_edge` returns `false`.

**Panics** if `a` or `b` are not found.

Source

#### pub fn is_valid_edge(&self, a: G::NodeId, b: G::NodeId) -> bool

where G::NodeId: IndexType,

Check if an edge would be valid, i.e. adding it would not create a cycle.

**Panics** if `a` or `b` are not found.

Source§

### impl<N, E, Ix: IndexType\> Acyclic<DiGraph<N, E, Ix>>

Source

#### pub fn remove_edge( &mut self, e: <DiGraph<N, E, Ix> as GraphBase\>::EdgeId, ) -> Option<E>

Remove an edge and return its edge weight, or None if it didn’t exist.

Pass through to underlying graph.

Source

#### pub fn remove_node( &mut self, n: <DiGraph<N, E, Ix> as GraphBase\>::NodeId, ) -> Option<N>

Remove a node from the graph if it exists, and return its weight. If it doesn’t exist in the graph, return None.

This updates the order in O(v) runtime and removes the node in the underlying graph.

Source§

### impl<N, E, Ix: IndexType\> Acyclic<StableDiGraph<N, E, Ix>>

Source

#### pub fn remove_edge( &mut self, e: <StableDiGraph<N, E, Ix> as GraphBase\>::EdgeId, ) -> Option<E>

Remove an edge and return its edge weight, or None if it didn’t exist.

Pass through to underlying graph.

Source

#### pub fn remove_node( &mut self, n: <StableDiGraph<N, E, Ix> as GraphBase\>::NodeId, ) -> Option<N>

Remove a node from the graph if it exists, and return its weight. If it doesn’t exist in the graph, return None.

This updates the order in O(v) runtime and removes the node in the underlying graph.

## Trait Implementations§

Source§

### impl<G: Build + Visitable + NodeIndexable\> Build for Acyclic<G>

where for<'a> &'a G: IntoNeighborsDirected + IntoNodeIdentifiers + Visitable<Map = G::Map\> + GraphBase<NodeId = G::NodeId\>, G::NodeId: IndexType,

Source§

#### fn add_node(&mut self, weight: Self::NodeWeight) -> Self::NodeId

Source§

#### fn add_edge( &mut self, a: Self::NodeId, b: Self::NodeId, weight: Self::EdgeWeight, ) -> Option<Self::EdgeId\>

Add a new edge. If parallel edges (duplicate) are not allowed and the edge already exists, return `None`. Read more

Source§

#### fn update_edge( &mut self, a: Self::NodeId, b: Self::NodeId, weight: Self::EdgeWeight, ) -> Self::EdgeId

Add or update the edge from `a` to `b`. Return the id of the affected edge. Read more

Source§

### impl<G: Clone + Visitable\> Clone for Acyclic<G>

where G::NodeId: Clone,

Source§

#### fn clone(&self) -> Acyclic<G>

Returns a duplicate of the value. Read more

1.0.0 · Source§

#### fn clone_from(&mut self, source: &Self)

Performs copy-assignment from `source`. Read more

Source§

### impl<G: Create + Visitable + NodeIndexable\> Create for Acyclic<G>

where for<'a> &'a G: IntoNeighborsDirected + IntoNodeIdentifiers + Visitable<Map = G::Map\> + GraphBase<NodeId = G::NodeId\>, G::NodeId: IndexType,

Source§

#### fn with_capacity(nodes: usize, edges: usize) -> Self

Source§

### impl<G: Visitable + Data\> Data for Acyclic<G>

Source§

#### type NodeWeight = <G as Data\>::NodeWeight

Source§

#### type EdgeWeight = <G as Data\>::EdgeWeight

Source§

### impl<G: Visitable + DataMap\> DataMap for Acyclic<G>

Source§

#### fn node_weight(&self, id: Self::NodeId) -> Option<&Self::NodeWeight\>

Source§

#### fn edge_weight(&self, id: Self::EdgeId) -> Option<&Self::EdgeWeight\>

Source§

### impl<G: Visitable + DataMapMut\> DataMapMut for Acyclic<G>

Source§

#### fn node_weight_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::NodeWeight\>

Source§

#### fn edge_weight_mut(&mut self, id: Self::EdgeId) -> Option<&mut Self::EdgeWeight\>

Source§

### impl<G: Debug + Visitable\> Debug for Acyclic<G>

where G::NodeId: Debug,

Source§

#### fn fmt(&self, f: &mut Formatter<'\_>) -> Result

Formats the value using the given formatter. Read more

Source§

### impl<G: Default + Visitable\> Default for Acyclic<G>

Source§

#### fn default() -> Self

Returns the “default value” for a type. Read more

Source§

### impl<G: Visitable\> Deref for Acyclic<G>

Source§

#### type Target = G

The resulting type after dereferencing.

Source§

#### fn deref(&self) -> &Self::Target

Dereferences the value.

Source§

### impl<G: Visitable + EdgeCount\> EdgeCount for Acyclic<G>

Source§

#### fn edge_count(&self) -> usize

Return the number of edges in the graph.

Source§

### impl<G: Visitable + EdgeIndexable\> EdgeIndexable for Acyclic<G>

Source§

#### fn edge_bound(&self) -> usize

Return an upper bound of the edge indices in the graph (suitable for the size of a bitmap).

Source§

#### fn to_index(&self, a: Self::EdgeId) -> usize

Convert `a` to an integer index.

Source§

#### fn from_index(&self, i: usize) -> Self::EdgeId

Convert `i` to an edge index. `i` must be a valid value in the graph.

Source§

### impl<G: Visitable + GetAdjacencyMatrix\> GetAdjacencyMatrix for Acyclic<G>

Source§

#### type AdjMatrix = <G as GetAdjacencyMatrix\>::AdjMatrix

The associated adjacency matrix type

Source§

#### fn adjacency_matrix(&self) -> Self::AdjMatrix

Create the adjacency matrix

Source§

#### fn is_adjacent( &self, matrix: &Self::AdjMatrix, a: Self::NodeId, b: Self::NodeId, ) -> bool

Return true if there is an edge from `a` to `b`, false otherwise. Read more

Source§

### impl<G: Visitable\> GraphBase for Acyclic<G>

Source§

#### type NodeId = <G as GraphBase\>::NodeId

node identifier

Source§

#### type EdgeId = <G as GraphBase\>::EdgeId

edge identifier

Source§

### impl<G: Visitable + GraphProp\> GraphProp for Acyclic<G>

Source§

#### type EdgeType = <G as GraphProp\>::EdgeType

The kind of edges in the graph.

Source§

#### fn is_directed(&self) -> bool

Source§

### impl<'a, N, E, Ix: IndexType\> IntoEdgeReferences for &'a Acyclic<DiGraph<N, E, Ix>>

Source§

#### type EdgeRef = <&'a Graph<N, E, Directed, Ix> as IntoEdgeReferences\>::EdgeRef

Source§

#### type EdgeReferences = <&'a Graph<N, E, Directed, Ix> as IntoEdgeReferences\>::EdgeReferences

Source§

#### fn edge_references(self) -> Self::EdgeReferences

Source§

### impl<'a, N, E, Ix: IndexType\> IntoEdgeReferences for &'a Acyclic<StableDiGraph<N, E, Ix>>

Source§

#### type EdgeRef = <&'a StableGraph<N, E, Directed, Ix> as IntoEdgeReferences\>::EdgeRef

Source§

#### type EdgeReferences = <&'a StableGraph<N, E, Directed, Ix> as IntoEdgeReferences\>::EdgeReferences

Source§

#### fn edge_references(self) -> Self::EdgeReferences

Source§

### impl<'a, N, E, Ix: IndexType\> IntoEdges for &'a Acyclic<DiGraph<N, E, Ix>>

Source§

#### type Edges = <&'a Graph<N, E, Directed, Ix> as IntoEdges\>::Edges

Source§

#### fn edges(self, a: Self::NodeId) -> Self::Edges

Source§

### impl<'a, N, E, Ix: IndexType\> IntoEdges for &'a Acyclic<StableDiGraph<N, E, Ix>>

Source§

#### type Edges = <&'a StableGraph<N, E, Directed, Ix> as IntoEdges\>::Edges

Source§

#### fn edges(self, a: Self::NodeId) -> Self::Edges

Source§

### impl<'a, N, E, Ix: IndexType\> IntoEdgesDirected for &'a Acyclic<DiGraph<N, E, Ix>>

Source§

#### type EdgesDirected = <&'a Graph<N, E, Directed, Ix> as IntoEdgesDirected\>::EdgesDirected

Source§

#### fn edges_directed(self, a: Self::NodeId, dir: Direction) -> Self::EdgesDirected

Source§

### impl<'a, N, E, Ix: IndexType\> IntoEdgesDirected for &'a Acyclic<StableDiGraph<N, E, Ix>>

Source§

#### type EdgesDirected = <&'a StableGraph<N, E, Directed, Ix> as IntoEdgesDirected\>::EdgesDirected

Source§

#### fn edges_directed(self, a: Self::NodeId, dir: Direction) -> Self::EdgesDirected

Source§

### impl<'a, N, E, Ix: IndexType\> IntoNeighbors for &'a Acyclic<DiGraph<N, E, Ix>>

Source§

#### type Neighbors = <&'a Graph<N, E, Directed, Ix> as IntoNeighbors\>::Neighbors

Source§

#### fn neighbors(self, a: Self::NodeId) -> Self::Neighbors

Return an iterator of the neighbors of node `a`.

Source§

### impl<'a, N, E, Ix: IndexType\> IntoNeighbors for &'a Acyclic<StableDiGraph<N, E, Ix>>

Source§

#### type Neighbors = <&'a StableGraph<N, E, Directed, Ix> as IntoNeighbors\>::Neighbors

Source§

#### fn neighbors(self, a: Self::NodeId) -> Self::Neighbors

Return an iterator of the neighbors of node `a`.

Source§

### impl<'a, N, E, Ix: IndexType\> IntoNeighborsDirected for &'a Acyclic<DiGraph<N, E, Ix>>

Source§

#### type NeighborsDirected = <&'a Graph<N, E, Directed, Ix> as IntoNeighborsDirected\>::NeighborsDirected

Source§

#### fn neighbors_directed( self, n: Self::NodeId, d: Direction, ) -> Self::NeighborsDirected

Source§

### impl<'a, N, E, Ix: IndexType\> IntoNeighborsDirected for &'a Acyclic<StableDiGraph<N, E, Ix>>

Source§

#### type NeighborsDirected = <&'a StableGraph<N, E, Directed, Ix> as IntoNeighborsDirected\>::NeighborsDirected

Source§

#### fn neighbors_directed( self, n: Self::NodeId, d: Direction, ) -> Self::NeighborsDirected

Source§

### impl<'a, N, E, Ix: IndexType\> IntoNodeIdentifiers for &'a Acyclic<DiGraph<N, E, Ix>>

Source§

#### type NodeIdentifiers = <&'a Graph<N, E, Directed, Ix> as IntoNodeIdentifiers\>::NodeIdentifiers

Source§

#### fn node_identifiers(self) -> Self::NodeIdentifiers

Source§

### impl<'a, N, E, Ix: IndexType\> IntoNodeIdentifiers for &'a Acyclic<StableDiGraph<N, E, Ix>>

Source§

#### type NodeIdentifiers = <&'a StableGraph<N, E, Directed, Ix> as IntoNodeIdentifiers\>::NodeIdentifiers

Source§

#### fn node_identifiers(self) -> Self::NodeIdentifiers

Source§

### impl<'a, N, E, Ix: IndexType\> IntoNodeReferences for &'a Acyclic<DiGraph<N, E, Ix>>

Source§

#### type NodeRef = <&'a Graph<N, E, Directed, Ix> as IntoNodeReferences\>::NodeRef

Source§

#### type NodeReferences = <&'a Graph<N, E, Directed, Ix> as IntoNodeReferences\>::NodeReferences

Source§

#### fn node_references(self) -> Self::NodeReferences

Source§

### impl<'a, N, E, Ix: IndexType\> IntoNodeReferences for &'a Acyclic<StableDiGraph<N, E, Ix>>

Source§

#### type NodeRef = <&'a StableGraph<N, E, Directed, Ix> as IntoNodeReferences\>::NodeRef

Source§

#### type NodeReferences = <&'a StableGraph<N, E, Directed, Ix> as IntoNodeReferences\>::NodeReferences

Source§

#### fn node_references(self) -> Self::NodeReferences

Source§

### impl<G: Visitable + NodeCount\> NodeCount for Acyclic<G>

Source§

#### fn node_count(&self) -> usize

Source§

### impl<G: Visitable + NodeIndexable\> NodeIndexable for Acyclic<G>

Source§

#### fn node_bound(&self) -> usize

Return an upper bound of the node indices in the graph (suitable for the size of a bitmap).

Source§

#### fn to_index(&self, a: Self::NodeId) -> usize

Convert `a` to an integer index.

Source§

#### fn from_index(&self, i: usize) -> Self::NodeId

Convert `i` to a node index. `i` must be a valid value in the graph.

Source§

### impl<N, E, Ix: IndexType\> TryFrom<Graph<N, E, Directed, Ix>> for Acyclic<DiGraph<N, E, Ix>>

Source§

#### type Error = Cycle<NodeIndex<Ix>>

The type returned in the event of a conversion error.

Source§

#### fn try_from(graph: DiGraph<N, E, Ix>) -> Result<Self, Self::Error\>

Performs the conversion.

Source§

### impl<N, E, Ix: IndexType\> TryFrom<StableGraph<N, E, Directed, Ix>> for Acyclic<StableDiGraph<N, E, Ix>>

Source§

#### type Error = Cycle<NodeIndex<Ix>>

The type returned in the event of a conversion error.

Source§

#### fn try_from(graph: StableDiGraph<N, E, Ix>) -> Result<Self, Self::Error\>

Performs the conversion.

Source§

### impl<G: Visitable\> Visitable for Acyclic<G>

Source§

#### type Map = <G as Visitable\>::Map

The associated map type

Source§

#### fn visit_map(&self) -> Self::Map

Create a new visitor map

Source§

#### fn reset_map(&self, map: &mut Self::Map)

Reset the visitor map (and resize to new size of graph if needed)

Source§

### impl<G: Visitable + NodeCompactIndexable\> NodeCompactIndexable for Acyclic<G>

## Auto Trait Implementations§

§

### impl<G> !Freeze for Acyclic<G>

§

### impl<G> !RefUnwindSafe for Acyclic<G>

§

### impl<G> Send for Acyclic<G>

where G: Send, <G as GraphBase\>::NodeId: Send,

§

### impl<G> !Sync for Acyclic<G>

§

### impl<G> Unpin for Acyclic<G>

where G: Unpin,

§

### impl<G> UnwindSafe for Acyclic<G>

where G: UnwindSafe, <G as GraphBase\>::NodeId: RefUnwindSafe,

## Blanket Implementations§

Source§

### impl<T> Any for T

where T: 'static + ?Sized,

Source§

#### fn type_id(&self) -> TypeId

Gets the `TypeId` of `self`. Read more

Source§

### impl<T> Borrow<T> for T

where T: ?Sized,

Source§

#### fn borrow(&self) -> &T

Immutably borrows from an owned value. Read more

Source§

### impl<T> BorrowMut<T> for T

where T: ?Sized,

Source§

#### fn borrow_mut(&mut self) -> &mut T

Mutably borrows from an owned value. Read more

Source§

### impl<T> CloneToUninit for T

where T: Clone,

Source§

#### unsafe fn clone_to_uninit(&self, dest: \*mut u8)

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. Read more

Source§

### impl<T> From<T> for T

Source§

#### fn from(t: T) -> T

Returns the argument unchanged.

Source§

### impl<T, U> Into<U> for T

where U: From<T>,

Source§

#### fn into(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

Source§

### impl<T> IntoEither for T

Source§

#### fn into_either(self, into_left: bool) -> Either<Self, Self>

Converts `self` into a `Left` variant of `Either<Self, Self>` if `into_left` is `true`. Converts `self` into a `Right` variant of `Either<Self, Self>` otherwise. Read more

Source§

#### fn into_either_with<F>(self, into_left: F) -> Either<Self, Self>

where F: FnOnce(&Self) -> bool,

Converts `self` into a `Left` variant of `Either<Self, Self>` if `into_left(&self)` returns `true`. Converts `self` into a `Right` variant of `Either<Self, Self>` otherwise. Read more

Source§

### impl<T> Pointable for T

Source§

#### const ALIGN: usize

The alignment of pointer.

Source§

#### type Init = T

The type for initializers.

Source§

#### unsafe fn init(init: <T as Pointable\>::Init) -> usize

Initializes a with the given initializer. Read more

Source§

#### unsafe fn deref<'a>(ptr: usize) -> &'a T

Dereferences the given pointer. Read more

Source§

#### unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T

Mutably dereferences the given pointer. Read more

Source§

#### unsafe fn drop(ptr: usize)

Drops the object pointed to by the given pointer. Read more

Source§

### impl<P, T> Receiver for P

where P: Deref<Target = T> + ?Sized, T: ?Sized,

Source§

#### type Target = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

Source§

### impl<T> ToOwned for T

where T: Clone,

Source§

#### type Owned = T

The resulting type after obtaining ownership.

Source§

#### fn to_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning. Read more

Source§

#### fn clone_into(&self, target: &mut T)

Uses borrowed data to replace owned data, usually by cloning. Read more

Source§

### impl<T, U> TryFrom<U> for T

where U: Into<T>,

Source§

#### type Error = Infallible

The type returned in the event of a conversion error.

Source§

#### fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error\>

Performs the conversion.

Source§

### impl<T, U> TryInto<U> for T

where U: TryFrom<T>,

Source§

#### type Error = <U as TryFrom<T>>::Error

The type returned in the event of a conversion error.

Source§

#### fn try_into(self) -> Result<U, <U as TryFrom<T>>::Error\>

Performs the conversion.
