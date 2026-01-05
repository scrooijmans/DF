# Struct ReversedEdges Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/reversed.rs.html#81-83" class="src">Source</a>

``` rust
pub struct ReversedEdges<I> { /* private fields */ }
```

Expand description

A reversed edges iterator.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#impl-Clone-for-ReversedEdges%3CI%3E" class="anchor">§</a>

### impl\<I: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html" class="struct" title="struct petgraph::visit::ReversedEdges">ReversedEdges</a>\<I\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html" class="struct" title="struct petgraph::visit::ReversedEdges">ReversedEdges</a>\<I\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#" class="tooltip" data-notable-ty="ReversedEdges&lt;I&gt;">ⓘ</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#impl-Debug-for-ReversedEdges%3CI%3E" class="anchor">§</a>

### impl\<I: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html" class="struct" title="struct petgraph::visit::ReversedEdges">ReversedEdges</a>\<I\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#impl-Iterator-for-ReversedEdges%3CI%3E" class="anchor">§</a>

### impl\<I\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html" class="struct" title="struct petgraph::visit::ReversedEdges">ReversedEdges</a>\<I\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>, I::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.EdgeRef.html" class="trait" title="trait petgraph::visit::EdgeRef">EdgeRef</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdgeReference.html" class="struct" title="struct petgraph::visit::ReversedEdgeReference">ReversedEdgeReference</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

The type of the elements being iterated over.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.next" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

Advances the iterator and returns the next value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#tymethod.next)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.size_hint" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.size_hint" class="fn">size_hint</a>(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Returns the bounds on the remaining length of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.size_hint)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.next_chunk" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.next_chunk" class="fn">next_chunk</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>( &mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\[Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\], <a href="https://doc.rust-lang.org/nightly/core/array/iter/struct.IntoIter.html" class="struct" title="struct core::array::iter::IntoIter">IntoIter</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, N\>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

🔬This is a nightly-only experimental API. (`iter_next_chunk`)

Advances the iterator and returns an array containing the next `N` values. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.next_chunk)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#222-224" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.count" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.count" class="fn">count</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Consumes the iterator, counting the number of iterations and returning it. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.count)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#250-252" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.last" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.last" class="fn">last</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Consumes the iterator, returning the last element. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.last)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.advance_by" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.advance_by" class="fn">advance_by</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

🔬This is a nightly-only experimental API. (`iter_advance_by`)

Advances the iterator by `n` elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.advance_by)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#374" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.nth" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.nth" class="fn">nth</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

Returns the `n`th element of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.nth)

1.28.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#424-426" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.step_by" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.step_by" class="fn">step_by</a>(self, step: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/step_by/struct.StepBy.html" class="struct" title="struct core::iter::adapters::step_by::StepBy">StepBy</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator starting at the same point, but stepping by the given amount at each iteration. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.step_by)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#495-498" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.chain" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.chain" class="fn">chain</a>\<U\>(self, other: U) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html" class="struct" title="struct core::iter::adapters::chain::Chain">Chain</a>\<Self, \<U as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, U: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>,

Takes two iterators and creates a new iterator over both in sequence. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.chain)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.zip" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.zip" class="fn">zip</a>\<U\>(self, other: U) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/zip/struct.Zip.html" class="struct" title="struct core::iter::adapters::zip::Zip">Zip</a>\<Self, \<U as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, U: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>,

‘Zips up’ two iterators into a single iterator of pairs. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.zip)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.intersperse" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse" class="fn">intersperse</a>(self, separator: Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.Intersperse.html" class="struct" title="struct core::iter::adapters::intersperse::Intersperse">Intersperse</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

🔬This is a nightly-only experimental API. (`iter_intersperse`)

Creates a new iterator which places a copy of `separator` between adjacent items of the original iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.intersperse_with" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse_with" class="fn">intersperse_with</a>\<G\>(self, separator: G) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.IntersperseWith.html" class="struct" title="struct core::iter::adapters::intersperse::IntersperseWith">IntersperseWith</a>\<Self, G\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, G: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>() -\> Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>,

🔬This is a nightly-only experimental API. (`iter_intersperse`)

Creates a new iterator which places an item generated by `separator` between adjacent items of the original iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse_with)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#773-776" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.map" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map" class="fn">map</a>\<B, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/map/struct.Map.html" class="struct" title="struct core::iter::adapters::map::Map">Map</a>\<Self, F\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#" class="tooltip" data-notable-ty="Map&lt;Self, F&gt;">ⓘ</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> B,

Takes a closure and creates an iterator which calls that closure on each element. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#818-821" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.for_each" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each" class="fn">for_each</a>\<F\>(self, f: F)

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>),

Calls a closure on each element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#893-896" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.filter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter" class="fn">filter</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/filter/struct.Filter.html" class="struct" title="struct core::iter::adapters::filter::Filter">Filter</a>\<Self, P\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Creates an iterator which uses a closure to determine if an element should be yielded. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#938-941" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.filter_map" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter_map" class="fn">filter_map</a>\<B, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/filter_map/struct.FilterMap.html" class="struct" title="struct core::iter::adapters::filter_map::FilterMap">FilterMap</a>\<Self, F\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>,

Creates an iterator that both filters and maps. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter_map)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#985-987" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.enumerate" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.enumerate" class="fn">enumerate</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/enumerate/struct.Enumerate.html" class="struct" title="struct core::iter::adapters::enumerate::Enumerate">Enumerate</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator which gives the current iteration count as well as the next value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.enumerate)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1056-1058" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.peekable" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.peekable" class="fn">peekable</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html" class="struct" title="struct core::iter::adapters::peekable::Peekable">Peekable</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator which can use the [`peek`](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html#method.peek "method core::iter::adapters::peekable::Peekable::peek") and [`peek_mut`](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html#method.peek_mut "method core::iter::adapters::peekable::Peekable::peek_mut") methods to look at the next element of the iterator without consuming it. See their documentation for more information. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.peekable)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1121-1124" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.skip_while" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip_while" class="fn">skip_while</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/skip_while/struct.SkipWhile.html" class="struct" title="struct core::iter::adapters::skip_while::SkipWhile">SkipWhile</a>\<Self, P\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Creates an iterator that [`skip`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip "method core::iter::traits::iterator::Iterator::skip")s elements based on a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip_while)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1199-1202" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.take_while" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take_while" class="fn">take_while</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/take_while/struct.TakeWhile.html" class="struct" title="struct core::iter::adapters::take_while::TakeWhile">TakeWhile</a>\<Self, P\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Creates an iterator that yields elements based on a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take_while)

1.57.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1287-1290" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.map_while" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_while" class="fn">map_while</a>\<B, P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/map_while/struct.MapWhile.html" class="struct" title="struct core::iter::adapters::map_while::MapWhile">MapWhile</a>\<Self, P\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>,

Creates an iterator that both yields elements based on a predicate and maps. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_while)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1316-1318" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.skip" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip" class="fn">skip</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/skip/struct.Skip.html" class="struct" title="struct core::iter::adapters::skip::Skip">Skip</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator that skips the first `n` elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1388-1390" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.take" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take" class="fn">take</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/take/struct.Take.html" class="struct" title="struct core::iter::adapters::take::Take">Take</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator that yields the first `n` elements, or fewer if the underlying iterator ends sooner. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1435-1438" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.scan" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.scan" class="fn">scan</a>\<St, B, F\>(self, initial_state: St, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/scan/struct.Scan.html" class="struct" title="struct core::iter::adapters::scan::Scan">Scan</a>\<Self, St, F\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut St</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>,

An iterator adapter which, like [`fold`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold "method core::iter::traits::iterator::Iterator::fold"), holds internal state, but unlike [`fold`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold "method core::iter::traits::iterator::Iterator::fold"), produces a new iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.scan)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1473-1477" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.flat_map" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flat_map" class="fn">flat_map</a>\<U, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.FlatMap.html" class="struct" title="struct core::iter::adapters::flatten::FlatMap">FlatMap</a>\<Self, U, F\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, U: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> U,

Creates an iterator that works like map, but flattens nested structure. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flat_map)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.map_windows" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_windows" class="fn">map_windows</a>\<F, R, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/map_windows/struct.MapWindows.html" class="struct" title="struct core::iter::adapters::map_windows::MapWindows">MapWindows</a>\<Self, F, N\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&\[Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]) -\> R,

🔬This is a nightly-only experimental API. (`iter_map_windows`)

Calls the given function `f` for each contiguous window of size `N` over `self` and returns an iterator over the outputs of `f`. Like [`slice::windows()`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.windows "method slice::windows"), the windows during mapping overlap as well. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_windows)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1775-1777" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.fuse" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fuse" class="fn">fuse</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/fuse/struct.Fuse.html" class="struct" title="struct core::iter::adapters::fuse::Fuse">Fuse</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an iterator which ends after the first [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"). [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fuse)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1859-1862" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.inspect" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.inspect" class="fn">inspect</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/inspect/struct.Inspect.html" class="struct" title="struct core::iter::adapters::inspect::Inspect">Inspect</a>\<Self, F\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>),

Does something with each element of an iterator, passing the value on. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.inspect)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1896-1898" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.by_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.by_ref" class="fn">by_ref</a>(&mut self) -\> &mut Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates a “by reference” adapter for this instance of `Iterator`. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.by_ref)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2015-2017" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.collect" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect" class="fn">collect</a>\<B\>(self) -\> B

where B: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Transforms an iterator into a collection. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.collect_into" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect_into" class="fn">collect_into</a>\<E\>(self, collection: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

🔬This is a nightly-only experimental API. (`iter_collect_into`)

Collects all the items from an iterator into a collection. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect_into)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2206-2210" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.partition" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition" class="fn">partition</a>\<B, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(B, B)</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, B: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Consumes an iterator, creating two collections from it. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.is_partitioned" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_partitioned" class="fn">is_partitioned</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

🔬This is a nightly-only experimental API. (`iter_is_partitioned`)

Checks if the elements of this iterator are partitioned according to the given predicate, such that all those that return `true` precede all those that return `false`. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_partitioned)

1.27.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2419-2423" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.try_fold" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_fold" class="fn">try_fold</a>\<B, F, R\>(&mut self, init: B, f: F) -\> R

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(B, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> R, R: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\<Output = B\>,

An iterator method that applies a function as long as it returns successfully, producing a single, final value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_fold)

1.27.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2477-2481" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.try_for_each" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_for_each" class="fn">try_for_each</a>\<F, R\>(&mut self, f: F) -\> R

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> R, R: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\<Output = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>,

An iterator method that applies a fallible function to each item in the iterator, stopping at the first error and returning that error. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_for_each)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2596-2599" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.fold" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold" class="fn">fold</a>\<B, F\>(self, init: B, f: F) -\> B

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(B, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> B,

Folds every element into an accumulator by applying an operation, returning the final result. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold)

1.51.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2633-2636" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.reduce" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.reduce" class="fn">reduce</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>,

Reduces the elements to a single one, by repeatedly applying a reducing operation. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.reduce)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.try_reduce" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_reduce" class="fn">try_reduce</a>\<R\>( &mut self, f: impl <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> R, ) -\> \<\<R as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual" class="associatedtype" title="type core::ops::try_trait::Try::Residual">Residual</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html" class="trait" title="trait core::ops::try_trait::Residual">Residual</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<R as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Output" class="associatedtype" title="type core::ops::try_trait::Try::Output">Output</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html#associatedtype.TryType" class="associatedtype" title="type core::ops::try_trait::Residual::TryType">TryType</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, R: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\<Output = Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, \<R as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual" class="associatedtype" title="type core::ops::try_trait::Try::Residual">Residual</a>: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html" class="trait" title="trait core::ops::try_trait::Residual">Residual</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>\>,

🔬This is a nightly-only experimental API. (`iterator_try_reduce`)

Reduces the elements to a single one by repeatedly applying a reducing operation. If the closure returns a failure, the failure is propagated back to the caller immediately. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_reduce)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2762-2765" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.all" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.all" class="fn">all</a>\<F\>(&mut self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Tests if every element of the iterator matches a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.all)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2815-2818" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.any" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.any" class="fn">any</a>\<F\>(&mut self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Tests if any element of the iterator matches a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.any)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2877-2880" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.find" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find" class="fn">find</a>\<P\>(&mut self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Searches for an element of an iterator that satisfies a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find)

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2908-2911" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.find_map" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find_map" class="fn">find_map</a>\<B, F\>(&mut self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>,

Applies function to the elements of iterator and returns the first non-none result. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find_map)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.try_find" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_find" class="fn">try_find</a>\<R\>( &mut self, f: impl <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> R, ) -\> \<\<R as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual" class="associatedtype" title="type core::ops::try_trait::Try::Residual">Residual</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html" class="trait" title="trait core::ops::try_trait::Residual">Residual</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html#associatedtype.TryType" class="associatedtype" title="type core::ops::try_trait::Residual::TryType">TryType</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, R: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\<Output = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>, \<R as <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html" class="trait" title="trait core::ops::try_trait::Try">Try</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual" class="associatedtype" title="type core::ops::try_trait::Try::Residual">Residual</a>: <a href="https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html" class="trait" title="trait core::ops::try_trait::Residual">Residual</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>\>,

🔬This is a nightly-only experimental API. (`try_find`)

Applies function to the elements of iterator and returns the first true result or the first error. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_find)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3049-3052" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.position" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position" class="fn">position</a>\<P\>(&mut self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Searches for an element in an iterator, returning its index. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position)

1.6.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3221-3224" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.max_by_key" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by_key" class="fn">max_by_key</a>\<B, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where B: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> B,

Returns the element that gives the maximum value from the specified function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by_key)

1.15.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3254-3257" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.max_by" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by" class="fn">max_by</a>\<F\>(self, compare: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, &Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Returns the element that gives the maximum value with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by)

1.6.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3281-3284" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.min_by_key" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by_key" class="fn">min_by_key</a>\<B, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where B: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> B,

Returns the element that gives the minimum value from the specified function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by_key)

1.15.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3314-3317" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.min_by" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by" class="fn">min_by</a>\<F\>(self, compare: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, &Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Returns the element that gives the minimum value with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3387-3391" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.unzip" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.unzip" class="fn">unzip</a>\<A, B, FromA, FromB\>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(FromA, FromB)</a>

where FromA: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<A\>, FromB: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<B\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(A, B)</a>\>,

Converts an iterator of pairs into a pair of containers. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.unzip)

1.36.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3418-3421" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.copied" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.copied" class="fn">copied</a>\<'a, T\>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html" class="struct" title="struct core::iter::adapters::copied::Copied">Copied</a>\<Self\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + 'a, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>,

Creates an iterator which copies all of its elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.copied)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3466-3469" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.cloned" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cloned" class="fn">cloned</a>\<'a, T\>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/cloned/struct.Cloned.html" class="struct" title="struct core::iter::adapters::cloned::Cloned">Cloned</a>\<Self\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + 'a, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>,

Creates an iterator which [`clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone "method core::clone::Clone::clone")s all of its elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cloned)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3497-3499" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.cycle" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cycle" class="fn">cycle</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/cycle/struct.Cycle.html" class="struct" title="struct core::iter::adapters::cycle::Cycle">Cycle</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Repeats an iterator endlessly. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cycle)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.array_chunks" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.array_chunks" class="fn">array_chunks</a>\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/array_chunks/struct.ArrayChunks.html" class="struct" title="struct core::iter::adapters::array_chunks::ArrayChunks">ArrayChunks</a>\<Self, N\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

🔬This is a nightly-only experimental API. (`iter_array_chunks`)

Returns an iterator over `N` elements of the iterator at a time. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.array_chunks)

1.11.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3576-3579" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.sum" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.sum" class="fn">sum</a>\<S\>(self) -\> S

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, S: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html" class="trait" title="trait core::iter::traits::accum::Sum">Sum</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>,

Sums the elements of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.sum)

1.11.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3608-3611" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.product" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.product" class="fn">product</a>\<P\>(self) -\> P

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, P: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html" class="trait" title="trait core::iter::traits::accum::Product">Product</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>,

Iterates over the entire iterator, multiplying all the elements [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.product)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.cmp_by" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp_by" class="fn">cmp_by</a>\<I, F\>(self, other: I, cmp: F) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

🔬This is a nightly-only experimental API. (`iter_order_by`)

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp_by)

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3712-3716" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp" class="fn">partial_cmp</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another. The comparison works like short-circuit evaluation, returning a result without comparing the remaining elements. As soon as an order can be determined, the evaluation stops and a result is returned. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.partial_cmp_by" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp_by" class="fn">partial_cmp_by</a>\<I, F\>(self, other: I, partial_cmp: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>,

🔬This is a nightly-only experimental API. (`iter_order_by`)

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp_by)

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3781-3785" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq" class="fn">eq</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.eq_by" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq_by" class="fn">eq_by</a>\<I, F\>(self, other: I, eq: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

🔬This is a nightly-only experimental API. (`iter_order_by`)

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are equal to those of another with respect to the specified equality function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq_by)

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3833-3837" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ne" class="fn">ne</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are not equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ne)

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3854-3858" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.lt" class="fn">lt</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") less than those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.lt)

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3875-3879" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.le" class="fn">le</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") less or equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.le)

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3896-3900" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.gt" class="fn">gt</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") greater than those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.gt)

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3917-3921" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ge" class="fn">ge</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") greater than or equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ge)

1.82.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3972-3975" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.is_sorted_by" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by" class="fn">is_sorted_by</a>\<F\>(self, compare: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>, &Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Checks if the elements of this iterator are sorted using the given comparator function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by)

1.82.0 · <a href="https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#4016-4020" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#method.is_sorted_by_key" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by_key" class="fn">is_sorted_by_key</a>\<F, K\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>) -\> K, K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>,

Checks if the elements of this iterator are sorted using the given key extraction function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by_key)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.ReversedEdges.html#blanket-implementations" class="anchor">§</a>
