# Trait IndexType Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graph_impl/mod.rs.html#37-41" class="src">Source</a>

``` rust
pub unsafe trait IndexType:
    Copy
    + Default
    + Hash
    + Ord
    + Debug
    + 'static {
    // Required methods
    fn new(x: usize) -> Self;
    fn index(&self) -> usize;
    fn max() -> Self;
}
```

Expand description

Trait for the unsigned integer type used for node and edge indices.

## <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#safety" class="doc-anchor">§</a>Safety

Marked `unsafe` because: the trait must faithfully preserve and convert index values.

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.new" class="fn">new</a>(x: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.index" class="fn">index</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.max" class="fn">max</a>() -\> Self

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#impl-IndexType-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.new" class="fn">new</a>(x: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.index" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.index" class="fn">index</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.max" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#impl-IndexType-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.new-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.new" class="fn">new</a>(x: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.index-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.index" class="fn">index</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.max-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#impl-IndexType-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.new-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.new" class="fn">new</a>(x: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.index-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.index" class="fn">index</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.max-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#impl-IndexType-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.new-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.new" class="fn">new</a>(x: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.index-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.index" class="fn">index</a>(&self) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#method.max-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#tymethod.max" class="fn">max</a>() -\> Self

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html#impl-IndexType-for-NodeIndex%3CIx%3E" class="anchor">§</a>

### impl\<Ix: <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/graph/trait.IndexType.html" class="trait" title="trait petgraph::graph::IndexType">IndexType</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graph/struct.NodeIndex.html" class="struct" title="struct petgraph::graph::NodeIndex">NodeIndex</a>\<Ix\>
