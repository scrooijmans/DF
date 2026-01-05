# Struct Ptr Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graphmap.rs.html#1030" class="src">Source</a>

``` rust
pub struct Ptr<'b, T: 'b>(pub &'b T);
```

Expand description

A reference that is hashed and compared by its pointer value.

`Ptr` is used for certain configurations of `GraphMap`, in particular in the combination where the node type for `GraphMap` is something of type for example `Ptr(&Cell<T>)`, with the `Cell<T>` being `TypedArena` allocated.

## Tuple Fields<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#fields" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#structfield.0" class="anchor field">§</a>`0: `<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive"><code>&amp;'b T</code></a>

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#impl-Clone-for-Ptr%3C&#39;_,+T%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'\_, T\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> Self

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#impl-Debug-for-Ptr%3C&#39;_,+T%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'\_, T\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#impl-Deref-for-Ptr%3C&#39;_,+T%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'\_, T\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype">Target</a> = T

The resulting type after dereferencing.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.deref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref" class="fn">deref</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>

Dereferences the value.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#impl-Hash-for-Ptr%3C&#39;_,+T%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'\_, T\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, st: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#impl-Ord-for-Ptr%3C&#39;b,+T%3E" class="anchor">§</a>

### impl\<'b, T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'b, T\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'b, T\>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

Ptr is ordered by pointer value, i.e. an arbitrary but stable and total order.

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#impl-PartialEq-for-Ptr%3C&#39;b,+T%3E" class="anchor">§</a>

### impl\<'b, T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'b, T\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'b, T\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Ptr compares by pointer equality, i.e if they point to the same value

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#impl-PartialOrd-for-Ptr%3C&#39;b,+T%3E" class="anchor">§</a>

### impl\<'b, T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'b, T\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'b, T\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#impl-Copy-for-Ptr%3C&#39;_,+T%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'\_, T\>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#impl-Eq-for-Ptr%3C&#39;_,+T%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html" class="struct" title="struct petgraph::graphmap::Ptr">Ptr</a>\<'\_, T\>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.Ptr.html#blanket-implementations" class="anchor">§</a>
