# Struct Dfs Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/traversal.rs.html#40-45" class="src">Source</a>

``` rust
pub struct Dfs<N, VM> {
    pub stack: Vec<N>,
    pub discovered: VM,
}
```

Expand description

Visit nodes of a graph in a depth-first-search (DFS) emitting nodes in preorder (when they are first discovered).

The traversal starts at a given node and only traverses nodes reachable from it.

`Dfs` is not recursive.

`Dfs` does not itself borrow the graph, and because of this you can run a traversal over a graph while still retaining mutable access to it, if you use it like the following example:

``` rust
use petgraph::Graph;
use petgraph::visit::Dfs;

let mut graph = Graph::<_,()>::new();
let a = graph.add_node(0);

let mut dfs = Dfs::new(&graph, a);
while let Some(nx) = dfs.next(&graph) {
    // we can access `graph` mutably here still
    graph[nx] += 1;
}

assert_eq!(graph[a], 1);
```

**Note:** The algorithm may not behave correctly if nodes are removed during iteration. It may not necessarily visit added nodes or edges.

## Fields<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#structfield.stack" class="anchor field">§</a>`stack: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<N>`

The stack of nodes to visit

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#structfield.discovered" class="anchor field">§</a>`discovered: VM`

The map of discovered nodes

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#impl-Dfs%3CN,+VM%3E" class="anchor">§</a>

### impl\<N, VM\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html" class="struct" title="struct petgraph::visit::Dfs">Dfs</a>\<N, VM\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>, VM: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<N\>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.new" class="fn">new</a>\<G\>(graph: G, start: N) -\> Self

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\<NodeId = N, Map = VM\>,

Create a new **Dfs**, using the graph’s visitor map, and put **start** in the stack of nodes to visit.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.from_parts" class="fn">from_parts</a>(stack: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<N\>, discovered: VM) -\> Self

Create a `Dfs` from a vector and a visit map

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.reset" class="fn">reset</a>\<G\>(&mut self, graph: G)

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\<NodeId = N, Map = VM\>,

Clear the visit state

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.empty" class="fn">empty</a>\<G\>(graph: G) -\> Self

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\<NodeId = N, Map = VM\>,

Create a new **Dfs** using the graph’s visitor map, and no stack.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.move_to" class="fn">move_to</a>(&mut self, start: N)

Keep the discovered map, but clear the visit stack and restart the dfs from a particular node.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.next" class="fn">next</a>\<G\>(&mut self, graph: G) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<N\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a>\<NodeId = N\>,

Return the next node in the dfs, or **None** if the traversal is done.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#impl-Clone-for-Dfs%3CN,+VM%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, VM: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html" class="struct" title="struct petgraph::visit::Dfs">Dfs</a>\<N, VM\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html" class="struct" title="struct petgraph::visit::Dfs">Dfs</a>\<N, VM\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#impl-Debug-for-Dfs%3CN,+VM%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, VM: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html" class="struct" title="struct petgraph::visit::Dfs">Dfs</a>\<N, VM\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#impl-Default-for-Dfs%3CN,+VM%3E" class="anchor">§</a>

### impl\<N, VM\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html" class="struct" title="struct petgraph::visit::Dfs">Dfs</a>\<N, VM\>

where VM: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#impl-Walker%3CG%3E-for-Dfs%3C%3CG+as+GraphBase%3E::NodeId,+%3CG+as+Visitable%3E::Map%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html" class="trait" title="trait petgraph::visit::Walker">Walker</a>\<G\> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html" class="struct" title="struct petgraph::visit::Dfs">Dfs</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype">Item</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.walk_next" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#tymethod.walk_next" class="fn">walk_next</a>(&mut self, context: G) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype" title="type petgraph::visit::Walker::Item">Item</a>\>

Advance to the next item

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#method.iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#method.iter" class="fn">iter</a>(self, context: Context) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.WalkerIter.html" class="struct" title="struct petgraph::visit::WalkerIter">WalkerIter</a>\<Self, Context\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#" class="tooltip" data-notable-ty="WalkerIter&lt;Self, Context&gt;">ⓘ</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Context: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Create an iterator out of the walker and given `context`.

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html#blanket-implementations" class="anchor">§</a>
