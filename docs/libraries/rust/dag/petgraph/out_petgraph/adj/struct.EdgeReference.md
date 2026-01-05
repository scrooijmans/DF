# Struct EdgeReference Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/adj.rs.html#66-71" class="src">Source</a>

``` rust
pub struct EdgeReference<'a, E, Ix: IndexType> { /* private fields */ }
```

Expand description

A reference to an edge of the graph.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#impl-Clone-for-EdgeReference%3C&#39;_,+E,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'\_, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> Self

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#impl-Debug-for-EdgeReference%3C&#39;a,+E,+Ix%3E" class="anchor">§</a>

### impl\<'a, E: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, Ix: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#impl-EdgeRef-for-EdgeReference%3C&#39;_,+E,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'\_, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#associatedtype.NodeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype">NodeId</a> = Ix

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#associatedtype.EdgeId" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype">EdgeId</a> = <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeIndex.html" class="struct" title="struct petgraph::adj::EdgeIndex">EdgeIndex</a>\<Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#associatedtype.Weight" class="anchor">§</a>

#### type <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype">Weight</a> = E

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.source" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.source" class="fn">source</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::EdgeRef::NodeId">NodeId</a>

The source node of the edge.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.target" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.target" class="fn">target</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::EdgeRef::NodeId">NodeId</a>

The target node of the edge.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.id" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.id" class="fn">id</a>(&self) -\> Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.EdgeId" class="associatedtype" title="type petgraph::visit::EdgeRef::EdgeId">EdgeId</a>

The edge’s identifier.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.weight" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#tymethod.weight" class="fn">weight</a>(&self) -\> &Self::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html#associatedtype.Weight" class="associatedtype" title="type petgraph::visit::EdgeRef::Weight">Weight</a>

A reference to the weight of the edge.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#impl-Ord-for-EdgeReference%3C&#39;a,+E,+Ix%3E" class="anchor">§</a>

### impl\<'a, E: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>, Ix: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'a, E, Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#impl-PartialEq-for-EdgeReference%3C&#39;a,+E,+Ix%3E" class="anchor">§</a>

### impl\<'a, E: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>, Ix: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> + <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'a, E, Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#impl-PartialOrd-for-EdgeReference%3C&#39;a,+E,+Ix%3E" class="anchor">§</a>

### impl\<'a, E: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>, Ix: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> + <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'a, E, Ix\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#impl-Copy-for-EdgeReference%3C&#39;_,+E,+Ix%3E" class="anchor">§</a>

### impl\<E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'\_, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#impl-Eq-for-EdgeReference%3C&#39;a,+E,+Ix%3E" class="anchor">§</a>

### impl\<'a, E: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a>, Ix: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'a, E, Ix\>

<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#impl-StructuralPartialEq-for-EdgeReference%3C&#39;a,+E,+Ix%3E" class="anchor">§</a>

### impl\<'a, E, Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html" class="struct" title="struct petgraph::adj::EdgeReference">EdgeReference</a>\<'a, E, Ix\>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/adj/struct.EdgeReference.html#blanket-implementations" class="anchor">§</a>
