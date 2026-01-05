# Struct Topo Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/traversal.rs.html#318-321" class="src">Source</a>

``` rust
pub struct Topo<N, VM> { /* private fields */ }
```

Expand description

A topological order traversal for a graph.

**Note** that `Topo` only visits nodes that are not part of cycles, i.e. nodes in a true DAG. Use other visitors like [`DfsPostOrder`](https://docs.rs/petgraph/latest/petgraph/visit/struct.DfsPostOrder.html "struct petgraph::visit::DfsPostOrder") or algorithms like [`kosaraju_scc`](https://docs.rs/petgraph/latest/petgraph/algo/scc/kosaraju_scc/fn.kosaraju_scc.html "fn petgraph::algo::scc::kosaraju_scc::kosaraju_scc") to handle graphs with possible cycles.

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#impl-Topo%3CN,+VM%3E" class="anchor">§</a>

### impl\<N, VM\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html" class="struct" title="struct petgraph::visit::Topo">Topo</a>\<N, VM\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>, VM: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<N\>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#method.new" class="fn">new</a>\<G\>(graph: G) -\> Self

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\<NodeId = N, Map = VM\>,

Create a new `Topo`, using the graph’s visitor map, and put all initial nodes in the to visit list.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#method.with_initials" class="fn">with_initials</a>\<G, I\>(graph: G, initials: I) -\> Self

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\<NodeId = N, Map = VM\>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = N\>,

Create a new `Topo` with initial nodes.

Nodes with incoming edges are ignored.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#method.reset" class="fn">reset</a>\<G\>(&mut self, graph: G)

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeIdentifiers.html" class="trait" title="trait petgraph::visit::IntoNodeIdentifiers">IntoNodeIdentifiers</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\<NodeId = N, Map = VM\>,

Clear visited state, and put all initial nodes in the to visit list.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#method.next" class="fn">next</a>\<G\>(&mut self, g: G) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<N\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\<NodeId = N, Map = VM\>,

Return the next node in the current topological order traversal, or `None` if the traversal is at the end.

*Note:* The graph may not have a complete topological order, and the only way to know is to run the whole traversal and make sure it visits every node.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#impl-Clone-for-Topo%3CN,+VM%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, VM: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html" class="struct" title="struct petgraph::visit::Topo">Topo</a>\<N, VM\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html" class="struct" title="struct petgraph::visit::Topo">Topo</a>\<N, VM\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#impl-Default-for-Topo%3CN,+VM%3E" class="anchor">§</a>

### impl\<N, VM\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html" class="struct" title="struct petgraph::visit::Topo">Topo</a>\<N, VM\>

where VM: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#impl-Walker%3CG%3E-for-Topo%3C%3CG+as+GraphBase%3E::NodeId,+%3CG+as+Visitable%3E::Map%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html" class="trait" title="trait petgraph::visit::Walker">Walker</a>\<G\> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html" class="struct" title="struct petgraph::visit::Topo">Topo</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype">Item</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#method.walk_next" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#tymethod.walk_next" class="fn">walk_next</a>(&mut self, context: G) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype" title="type petgraph::visit::Walker::Item">Item</a>\>

Advance to the next item

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#method.iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#method.iter" class="fn">iter</a>(self, context: Context) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.WalkerIter.html" class="struct" title="struct petgraph::visit::WalkerIter">WalkerIter</a>\<Self, Context\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#" class="tooltip" data-notable-ty="WalkerIter&lt;Self, Context&gt;">ⓘ</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Context: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Create an iterator out of the walker and given `context`.

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html#blanket-implementations" class="anchor">§</a>
