# Trait ElementIterator Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/data.rs.html#386-408" class="src">Source</a>

``` rust
pub trait ElementIterator<N, E>: Iterator<Item = Element<N, E>> {
    // Provided method
    fn filter_elements<F>(self, f: F) -> FilterElements<Self, F> ⓘ
       where Self: Sized,
             F: FnMut(Element<&mut N, &mut E>) -> bool { ... }
}
```

Expand description

Iterator adaptors for iterators of `Element`.

## Provided Methods<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.ElementIterator.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.ElementIterator.html#method.filter_elements" class="fn">filter_elements</a>\<F\>(self, f: F) -\> <a href="https://docs.rs/petgraph/latest/petgraph/data/struct.FilterElements.html" class="struct" title="struct petgraph::data::FilterElements">FilterElements</a>\<Self, F\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.ElementIterator.html#" class="tooltip" data-notable-ty="FilterElements&lt;Self, F&gt;">ⓘ</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut N</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut E</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,

Create an iterator adaptor that filters graph elements.

The function `f` is called with each element and if its return value is `true` the element is accepted and if `false` it is removed. `f` is called with mutable references to the node and edge weights, so that they can be mutated (but the edge endpoints can not).

This filter adapts the edge source and target indices in the stream so that they are correct after the removals.

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.ElementIterator.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/data/trait.ElementIterator.html#impl-ElementIterator%3CN,+E%3E-for-I" class="anchor">§</a>

### impl\<N, E, I\> <a href="https://docs.rs/petgraph/latest/petgraph/data/trait.ElementIterator.html" class="trait" title="trait petgraph::data::ElementIterator">ElementIterator</a>\<N, E\> for I

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://docs.rs/petgraph/latest/petgraph/data/enum.Element.html" class="enum" title="enum petgraph::data::Element">Element</a>\<N, E\>\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,
