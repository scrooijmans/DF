# Struct Bfs Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/traversal.rs.html#256-261" class="src">Source</a>

``` rust
pub struct Bfs<N, VM> {
    pub stack: VecDeque<N>,
    pub discovered: VM,
}
```

Expand description

A breadth first search (BFS) of a graph.

The traversal starts at a given node and only traverses nodes reachable from it.

`Bfs` is not recursive.

`Bfs` does not itself borrow the graph, and because of this you can run a traversal over a graph while still retaining mutable access to it, if you use it like the following example:

``` rust
use petgraph::Graph;
use petgraph::visit::Bfs;

let mut graph = Graph::<_,()>::new();
let a = graph.add_node(0);

let mut bfs = Bfs::new(&graph, a);
while let Some(nx) = bfs.next(&graph) {
    // we can access `graph` mutably here still
    graph[nx] += 1;
}

assert_eq!(graph[a], 1);
```

**Note:** The algorithm may not behave correctly if nodes are removed during iteration. It may not necessarily visit added nodes or edges.

## Fields<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#structfield.stack" class="anchor field">§</a>`stack: `<a href="https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html" class="struct" title="struct alloc::collections::vec_deque::VecDeque"><code>VecDeque</code></a>`<N>`

The queue of nodes to visit

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#structfield.discovered" class="anchor field">§</a>`discovered: VM`

The map of discovered nodes

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#impl-Bfs%3CN,+VM%3E" class="anchor">§</a>

### impl\<N, VM\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html" class="struct" title="struct petgraph::visit::Bfs">Bfs</a>\<N, VM\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>, VM: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<N\>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#method.new" class="fn">new</a>\<G\>(graph: G, start: N) -\> Self

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\<NodeId = N, Map = VM\>,

Create a new **Bfs**, using the graph’s visitor map, and put **start** in the stack of nodes to visit.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#method.next" class="fn">next</a>\<G\>(&mut self, graph: G) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<N\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a>\<NodeId = N\>,

Return the next node in the bfs, or **None** if the traversal is done.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#impl-Clone-for-Bfs%3CN,+VM%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, VM: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html" class="struct" title="struct petgraph::visit::Bfs">Bfs</a>\<N, VM\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html" class="struct" title="struct petgraph::visit::Bfs">Bfs</a>\<N, VM\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#impl-Default-for-Bfs%3CN,+VM%3E" class="anchor">§</a>

### impl\<N, VM\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html" class="struct" title="struct petgraph::visit::Bfs">Bfs</a>\<N, VM\>

where VM: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#impl-Walker%3CG%3E-for-Bfs%3C%3CG+as+GraphBase%3E::NodeId,+%3CG+as+Visitable%3E::Map%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html" class="trait" title="trait petgraph::visit::Walker">Walker</a>\<G\> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html" class="struct" title="struct petgraph::visit::Bfs">Bfs</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype">Item</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#method.walk_next" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#tymethod.walk_next" class="fn">walk_next</a>(&mut self, context: G) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype" title="type petgraph::visit::Walker::Item">Item</a>\>

Advance to the next item

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#method.iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#method.iter" class="fn">iter</a>(self, context: Context) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.WalkerIter.html" class="struct" title="struct petgraph::visit::WalkerIter">WalkerIter</a>\<Self, Context\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#" class="tooltip" data-notable-ty="WalkerIter&lt;Self, Context&gt;">ⓘ</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Context: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Create an iterator out of the walker and given `context`.

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html#blanket-implementations" class="anchor">§</a>
