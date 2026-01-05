# Trait VisitMap Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/mod.rs.html#399-412" class="src">Source</a>

``` rust
pub trait VisitMap<N> {
    // Required methods
    fn visit(&mut self, a: N) -> bool;
    fn is_visited(&self, a: &N) -> bool;
    fn unvisit(&mut self, _a: N) -> bool;
}
```

Expand description

A mapping for storing the visited status for NodeId `N`.

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.visit" class="fn">visit</a>(&mut self, a: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Mark `a` as visited.

Return **true** if this is the first visit, false otherwise.

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.is_visited" class="fn">is_visited</a>(&self, a: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;N</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return whether `a` has been visited before.

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.unvisit" class="fn">unvisit</a>(&mut self, \_a: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Mark `a` as unvisited.

Return **true** if this vertex was marked as visited at the time of unsetting it, false otherwise.

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#impl-VisitMap%3CIx%3E-for-FixedBitSet" class="anchor">§</a>

### impl\<Ix\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<Ix\> for <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

where Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.visit" class="fn">visit</a>(&mut self, x: Ix) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#method.is_visited" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.is_visited" class="fn">is_visited</a>(&self, x: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Ix</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#method.unvisit" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.unvisit" class="fn">unvisit</a>(&mut self, x: Ix) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#impl-VisitMap%3CN%3E-for-HashSet%3CN,+S%3E" class="anchor">§</a>

### impl\<N, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<N\> for <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<N, S\>

where N: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

Available on **crate feature `std`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#method.visit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.visit" class="fn">visit</a>(&mut self, x: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#method.is_visited-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.is_visited" class="fn">is_visited</a>(&self, x: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;N</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#method.unvisit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.unvisit" class="fn">unvisit</a>(&mut self, x: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#impl-VisitMap%3CN%3E-for-HashSet%3CN,+S%3E-1" class="anchor">§</a>

### impl\<N, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<N\> for <a href="https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<N, S\>

where N: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#method.visit-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.visit" class="fn">visit</a>(&mut self, x: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#method.is_visited-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.is_visited" class="fn">is_visited</a>(&self, x: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;N</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#method.unvisit-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#tymethod.unvisit" class="fn">unvisit</a>(&mut self, x: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html#implementors" class="anchor">§</a>
