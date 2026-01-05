# Trait NodeRef Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/mod.rs.html#260-265" class="src">Source</a>

``` rust
pub trait NodeRef: Copy {
    type NodeId;
    type Weight;

    // Required methods
    fn id(&self) -> Self::NodeId;
    fn weight(&self) -> &Self::Weight;
}
```

Expand description

A node reference.

## Required Associated Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight" class="associatedtype">Weight</a>

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#tymethod.id" class="fn">id</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::NodeRef::NodeId">NodeId</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#tymethod.weight" class="fn">weight</a>(&self) -\> &Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight" class="associatedtype" title="type petgraph::visit::NodeRef::Weight">Weight</a>

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#impl-NodeRef-for-(Id,+())" class="anchor">§</a>

### impl\<Id\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html" class="trait" title="trait petgraph::visit::NodeRef">NodeRef</a> for (Id, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>)

where Id: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId-1" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Id

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight-1" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#method.id" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#tymethod.id" class="fn">id</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::NodeRef::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#method.weight" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#tymethod.weight" class="fn">weight</a>(&self) -\> &Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight" class="associatedtype" title="type petgraph::visit::NodeRef::Weight">Weight</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#impl-NodeRef-for-(Id,+%26W)" class="anchor">§</a>

### impl\<Id, W\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html" class="trait" title="trait petgraph::visit::NodeRef">NodeRef</a> for (Id, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;W</a>)

where Id: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId-2" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Id

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight-2" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = W

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#method.id-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#tymethod.id" class="fn">id</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::NodeRef::NodeId">NodeId</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#method.weight-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#tymethod.weight" class="fn">weight</a>(&self) -\> &Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight" class="associatedtype" title="type petgraph::visit::NodeRef::Weight">Weight</a>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#impl-NodeRef-for-Ix" class="anchor">§</a>

### impl\<Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html" class="trait" title="trait petgraph::visit::NodeRef">NodeRef</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/type.NodeIndex.html" class="type" title="type petgraph::adj::NodeIndex">NodeIndex</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId-3" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Ix

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight-3" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>
