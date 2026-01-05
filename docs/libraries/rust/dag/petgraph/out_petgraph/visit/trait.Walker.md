# Trait Walker Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/traversal.rs.html#438-454" class="src">Source</a>

``` rust
pub trait Walker<Context> {
    type Item;

    // Required method
    fn walk_next(&mut self, context: Context) -> Option<Self::Item>;

    // Provided method
    fn iter(self, context: Context) -> WalkerIter<Self, Context> ⓘ
       where Self: Sized,
             Context: Clone { ... }
}
```

Expand description

A walker is a traversal state, but where part of the traversal information is supplied manually to each next call.

This for example allows graph traversals that don’t hold a borrow of the graph they are traversing.

## Required Associated Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype">Item</a>

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#tymethod.walk_next" class="fn">walk_next</a>(&mut self, context: Context) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype" title="type petgraph::visit::Walker::Item">Item</a>\>

Advance to the next item

## Provided Methods<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#method.iter" class="fn">iter</a>(self, context: Context) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.WalkerIter.html" class="struct" title="struct petgraph::visit::WalkerIter">WalkerIter</a>\<Self, Context\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#" class="tooltip" data-notable-ty="WalkerIter&lt;Self, Context&gt;">ⓘ</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Context: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Create an iterator out of the walker and given `context`.

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#impl-Walker%3CC%3E-for-%26mut+W" class="anchor">§</a>

### impl\<C, W\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html" class="trait" title="trait petgraph::visit::Walker">Walker</a>\<C\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>

where W: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html" class="trait" title="trait petgraph::visit::Walker">Walker</a>\<C\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype">Item</a> = \<W as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html" class="trait" title="trait petgraph::visit::Walker">Walker</a>\<C\>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype" title="type petgraph::visit::Walker::Item">Item</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#method.walk_next" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#tymethod.walk_next" class="fn">walk_next</a>(&mut self, context: C) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype" title="type petgraph::visit::Walker::Item">Item</a>\>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#impl-Walker%3CG%3E-for-Bfs%3C%3CG+as+GraphBase%3E::NodeId,+%3CG+as+Visitable%3E::Map%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html" class="trait" title="trait petgraph::visit::Walker">Walker</a>\<G\> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html" class="struct" title="struct petgraph::visit::Bfs">Bfs</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item-2" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype">Item</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#impl-Walker%3CG%3E-for-Dfs%3C%3CG+as+GraphBase%3E::NodeId,+%3CG+as+Visitable%3E::Map%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html" class="trait" title="trait petgraph::visit::Walker">Walker</a>\<G\> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html" class="struct" title="struct petgraph::visit::Dfs">Dfs</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item-3" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype">Item</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#impl-Walker%3CG%3E-for-DfsPostOrder%3C%3CG+as+GraphBase%3E::NodeId,+%3CG+as+Visitable%3E::Map%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html" class="trait" title="trait petgraph::visit::Walker">Walker</a>\<G\> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.DfsPostOrder.html" class="struct" title="struct petgraph::visit::DfsPostOrder">DfsPostOrder</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighbors.html" class="trait" title="trait petgraph::visit::IntoNeighbors">IntoNeighbors</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item-4" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype">Item</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#impl-Walker%3CG%3E-for-Topo%3C%3CG+as+GraphBase%3E::NodeId,+%3CG+as+Visitable%3E::Map%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html" class="trait" title="trait petgraph::visit::Walker">Walker</a>\<G\> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Topo.html" class="struct" title="struct petgraph::visit::Topo">Topo</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html#associatedtype.Map" class="associatedtype" title="type petgraph::visit::Visitable::Map">Map</a>\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNeighborsDirected.html" class="trait" title="trait petgraph::visit::IntoNeighborsDirected">IntoNeighborsDirected</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item-5" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Walker.html#associatedtype.Item" class="associatedtype">Item</a> = \<G as <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html" class="trait" title="trait petgraph::visit::GraphBase">GraphBase</a>\>::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>
