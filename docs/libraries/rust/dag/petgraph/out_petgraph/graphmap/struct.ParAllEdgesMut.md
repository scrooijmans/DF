# Struct ParAllEdgesMut Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/graphmap.rs.html#1496-1503" class="src">Source</a>

``` rust
pub struct ParAllEdgesMut<'a, N, E, Ty>where
    N: NodeTrait + Send + Sync,
    E: Send + 'a,{ /* private fields */ }
```

Expand description

A [ParallelIterator](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html "trait rayon::iter::ParallelIterator") over this graph’s edges by mutable reference.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#impl-IndexedParallelIterator-for-ParAllEdgesMut%3C&#39;_,+N,+E,+Ty%3E" class="anchor">§</a>

### impl\<N, E, Ty\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html" class="struct" title="struct petgraph::graphmap::ParAllEdgesMut">ParAllEdgesMut</a>\<'\_, N, E, Ty\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, E: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Available on **crate feature `rayon`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.drive" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#tymethod.drive" class="fn">drive</a>\<C\>(self, consumer: C) -\> C::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/plumbing/trait.Consumer.html#associatedtype.Result" class="associatedtype" title="type rayon::iter::plumbing::Consumer::Result">Result</a>

where C: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/plumbing/trait.Consumer.html" class="trait" title="trait rayon::iter::plumbing::Consumer">Consumer</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>,

Internal method used to define the behavior of this parallel iterator. You should not need to call this directly. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#tymethod.drive)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Produces an exact count of how many items this iterator will produce, presuming no panic occurs. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#tymethod.len)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.with_producer" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#tymethod.with_producer" class="fn">with_producer</a>\<CB\>(self, callback: CB) -\> CB::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/plumbing/trait.ProducerCallback.html#associatedtype.Output" class="associatedtype" title="type rayon::iter::plumbing::ProducerCallback::Output">Output</a>

where CB: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/plumbing/trait.ProducerCallback.html" class="trait" title="trait rayon::iter::plumbing::ProducerCallback">ProducerCallback</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>,

Internal method used to define the behavior of this parallel iterator. You should not need to call this directly. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#tymethod.with_producer)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.by_exponential_blocks" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.by_exponential_blocks" class="fn">by_exponential_blocks</a>(self) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/blocks/struct.ExponentialBlocks.html" class="struct" title="struct rayon::iter::blocks::ExponentialBlocks">ExponentialBlocks</a>\<Self\>

Divides an iterator into sequential blocks of exponentially-increasing size. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.by_exponential_blocks)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.by_uniform_blocks" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.by_uniform_blocks" class="fn">by_uniform_blocks</a>(self, block_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/blocks/struct.UniformBlocks.html" class="struct" title="struct rayon::iter::blocks::UniformBlocks">UniformBlocks</a>\<Self\>

Divides an iterator into sequential blocks of the given size. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.by_uniform_blocks)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.collect_into_vec" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.collect_into_vec" class="fn">collect_into_vec</a>(self, target: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>)

Collects the results of the iterator into the specified vector. The vector is always cleared before execution begins. If possible, reusing the vector across calls can lead to better performance since it reuses the same backing buffer. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.collect_into_vec)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.unzip_into_vecs" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.unzip_into_vecs" class="fn">unzip_into_vecs</a>\<A, B\>(self, left: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<A\>, right: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<B\>)

where Self: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(A, B)</a>\>, A: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, B: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Unzips the results of the iterator into the specified vectors. The vectors are always cleared before execution begins. If possible, reusing the vectors across calls can lead to better performance since they reuse the same backing buffer. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.unzip_into_vecs)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.zip" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.zip" class="fn">zip</a>\<Z\>(self, zip_op: Z) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/zip/struct.Zip.html" class="struct" title="struct rayon::iter::zip::Zip">Zip</a>\<Self, \<Z as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>\>

where Z: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>, \<Z as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>,

Iterates over tuples `(A, B)`, where the items `A` are from this iterator and `B` are from the iterator given as argument. Like the `zip` method on ordinary iterators, if the two iterators are of unequal length, you only get the items they have in common. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.zip)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.zip_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.zip_eq" class="fn">zip_eq</a>\<Z\>(self, zip_op: Z) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/zip_eq/struct.ZipEq.html" class="struct" title="struct rayon::iter::zip_eq::ZipEq">ZipEq</a>\<Self, \<Z as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>\>

where Z: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>, \<Z as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>,

The same as `Zip`, but requires that both iterators have the same length. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.zip_eq)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.interleave" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.interleave" class="fn">interleave</a>\<I\>( self, other: I, ) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/interleave/struct.Interleave.html" class="struct" title="struct rayon::iter::interleave::Interleave">Interleave</a>\<Self, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>\>

where I: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\<Item = Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>,

Interleaves elements of this iterator and the other given iterator. Alternately yields elements from this iterator and the given iterator, until both are exhausted. If one iterator is exhausted before the other, the last elements are provided from the other. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.interleave)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.interleave_shortest" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.interleave_shortest" class="fn">interleave_shortest</a>\<I\>( self, other: I, ) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/interleave_shortest/struct.InterleaveShortest.html" class="struct" title="struct rayon::iter::interleave_shortest::InterleaveShortest">InterleaveShortest</a>\<Self, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>\>

where I: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\<Item = Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>,

Interleaves elements of this iterator and the other given iterator, until one is exhausted. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.interleave_shortest)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.chunks" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.chunks" class="fn">chunks</a>(self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/chunks/struct.Chunks.html" class="struct" title="struct rayon::iter::chunks::Chunks">Chunks</a>\<Self\>

Splits an iterator up into fixed-size chunks. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.chunks)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.fold_chunks" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.fold_chunks" class="fn">fold_chunks</a>\<T, ID, F\>( self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, identity: ID, fold_op: F, ) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/fold_chunks/struct.FoldChunks.html" class="struct" title="struct rayon::iter::fold_chunks::FoldChunks">FoldChunks</a>\<Self, ID, F\>

where ID: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Splits an iterator into fixed-size chunks, performing a sequential [`fold()`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold "trait core::iter::traits::iterator::Iterator") on each chunk. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.fold_chunks)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.fold_chunks_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.fold_chunks_with" class="fn">fold_chunks_with</a>\<T, F\>( self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, init: T, fold_op: F, ) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/fold_chunks_with/struct.FoldChunksWith.html" class="struct" title="struct rayon::iter::fold_chunks_with::FoldChunksWith">FoldChunksWith</a>\<Self, T, F\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

Splits an iterator into fixed-size chunks, performing a sequential [`fold()`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold "trait core::iter::traits::iterator::Iterator") on each chunk. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.fold_chunks_with)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.cmp" class="fn">cmp</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

where I: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\<Item = Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Lexicographically compares the elements of this `ParallelIterator` with those of another. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.cmp)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.partial_cmp" class="fn">partial_cmp</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

where I: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Item">Item</a>\>,

Lexicographically compares the elements of this `ParallelIterator` with those of another. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.partial_cmp)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.eq" class="fn">eq</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<\<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Item">Item</a>\>,

Determines if the elements of this `ParallelIterator` are equal to those of another

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.ne" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.ne" class="fn">ne</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<\<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Item">Item</a>\>,

Determines if the elements of this `ParallelIterator` are unequal to those of another

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.lt" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.lt" class="fn">lt</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Item">Item</a>\>,

Determines if the elements of this `ParallelIterator` are lexicographically less than those of another.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.le" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.le" class="fn">le</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Item">Item</a>\>,

Determines if the elements of this `ParallelIterator` are less than or equal to those of another.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.gt" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.gt" class="fn">gt</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Item">Item</a>\>,

Determines if the elements of this `ParallelIterator` are lexicographically greater than those of another.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.ge" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.ge" class="fn">ge</a>\<I\>(self, other: I) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where I: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>, \<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html" class="trait" title="trait rayon::iter::IndexedParallelIterator">IndexedParallelIterator</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\<I as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Item">Item</a>\>,

Determines if the elements of this `ParallelIterator` are greater than or equal to those of another.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.enumerate" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.enumerate" class="fn">enumerate</a>(self) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/enumerate/struct.Enumerate.html" class="struct" title="struct rayon::iter::enumerate::Enumerate">Enumerate</a>\<Self\>

Yields an index along with each item. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.enumerate)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.step_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.step_by" class="fn">step_by</a>(self, step: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/step_by/struct.StepBy.html" class="struct" title="struct rayon::iter::step_by::StepBy">StepBy</a>\<Self\>

Creates an iterator that steps by the given amount [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.step_by)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.skip" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.skip" class="fn">skip</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/skip/struct.Skip.html" class="struct" title="struct rayon::iter::skip::Skip">Skip</a>\<Self\>

Creates an iterator that skips the first `n` elements. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.skip)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.take" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.take" class="fn">take</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/take/struct.Take.html" class="struct" title="struct rayon::iter::take::Take">Take</a>\<Self\>

Creates an iterator that yields the first `n` elements. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.take)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.position_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.position_any" class="fn">position_any</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for **some** item in the parallel iterator that matches the given predicate, and returns its index. Like `ParallelIterator::find_any`, the parallel search will not necessarily find the **first** match, and once a match is found we’ll attempt to stop processing any more. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.position_any)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.position_first" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.position_first" class="fn">position_first</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for the sequentially **first** item in the parallel iterator that matches the given predicate, and returns its index. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.position_first)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.position_last" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.position_last" class="fn">position_last</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for the sequentially **last** item in the parallel iterator that matches the given predicate, and returns its index. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.position_last)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.positions" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.positions" class="fn">positions</a>\<P\>(self, predicate: P) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/positions/struct.Positions.html" class="struct" title="struct rayon::iter::positions::Positions">Positions</a>\<Self, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for items in the parallel iterator that match the given predicate, and returns their indices. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.positions)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.rev" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.rev" class="fn">rev</a>(self) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/rev/struct.Rev.html" class="struct" title="struct rayon::iter::rev::Rev">Rev</a>\<Self\>

Produces a new iterator with the elements of this iterator in reverse order. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.rev)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.with_min_len" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.with_min_len" class="fn">with_min_len</a>(self, min: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/len/struct.MinLen.html" class="struct" title="struct rayon::iter::len::MinLen">MinLen</a>\<Self\>

Sets the minimum length of iterators desired to process in each rayon job. Rayon will not split any smaller than this length, but of course an iterator could already be smaller to begin with. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.with_min_len)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.with_max_len" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.with_max_len" class="fn">with_max_len</a>(self, max: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/len/struct.MaxLen.html" class="struct" title="struct rayon::iter::len::MaxLen">MaxLen</a>\<Self\>

Sets the maximum length of iterators desired to process in each rayon job. Rayon will try to split at least below this length, unless that would put it below the length from `with_min_len()`. For example, given min=10 and max=15, a length of 16 will not be split any further. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IndexedParallelIterator.html#method.with_max_len)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#impl-ParallelIterator-for-ParAllEdgesMut%3C&#39;a,+N,+E,+Ty%3E" class="anchor">§</a>

### impl\<'a, N, E, Ty\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a> for <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html" class="struct" title="struct petgraph::graphmap::ParAllEdgesMut">ParAllEdgesMut</a>\<'a, N, E, Ty\>

where N: <a href="https://docs.rs/petgraph/latest/petgraph/graphmap/trait.NodeTrait.html" class="trait" title="trait petgraph::graphmap::NodeTrait">NodeTrait</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>, E: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Available on **crate feature `rayon`** only.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype">Item</a> = (N, N, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a mut E</a>)

The type of item that this parallel iterator produces. For example, if you use the [`for_each`](https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.for_each) method, this is the type of item that your closure will be invoked with.

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.drive_unindexed" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#tymethod.drive_unindexed" class="fn">drive_unindexed</a>\<C\>(self, c: C) -\> C::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/plumbing/trait.Consumer.html#associatedtype.Result" class="associatedtype" title="type rayon::iter::plumbing::Consumer::Result">Result</a>

where C: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/plumbing/trait.UnindexedConsumer.html" class="trait" title="trait rayon::iter::plumbing::UnindexedConsumer">UnindexedConsumer</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>,

Internal method used to define the behavior of this parallel iterator. You should not need to call this directly. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#tymethod.drive_unindexed)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.opt_len" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.opt_len" class="fn">opt_len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Internal method used to define the behavior of this parallel iterator. You should not need to call this directly. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.opt_len)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.for_each" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each" class="fn">for_each</a>\<OP\>(self, op: OP)

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Executes `OP` on each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.for_each_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each_with" class="fn">for_each_with</a>\<OP, T\>(self, init: T, op: OP)

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Executes `OP` on the given `init` value with each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each_with)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.for_each_init" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each_init" class="fn">for_each_init</a>\<OP, INIT, T\>(self, init: INIT, op: OP)

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, INIT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Executes `OP` on a value returned by `init` with each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each_init)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.try_for_each" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each" class="fn">try_for_each</a>\<OP, R\>(self, op: OP) -\> R

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: Try\<Output = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Executes a fallible `OP` on each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.try_for_each_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each_with" class="fn">try_for_each_with</a>\<OP, T, R\>(self, init: T, op: OP) -\> R

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, R: Try\<Output = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Executes a fallible `OP` on the given `init` value with each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each_with)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.try_for_each_init" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each_init" class="fn">try_for_each_init</a>\<OP, INIT, T, R\>(self, init: INIT, op: OP) -\> R

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, INIT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: Try\<Output = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Executes a fallible `OP` on a value returned by `init` with each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each_init)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.count" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.count" class="fn">count</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Counts the number of items in this parallel iterator. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.count)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.map" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map" class="fn">map</a>\<F, R\>(self, map_op: F) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/map/struct.Map.html" class="struct" title="struct rayon::iter::map::Map">Map</a>\<Self, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `map_op` to each item of this iterator, producing a new iterator with the results. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.map_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map_with" class="fn">map_with</a>\<F, T, R\>(self, init: T, map_op: F) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/map_with/struct.MapWith.html" class="struct" title="struct rayon::iter::map_with::MapWith">MapWith</a>\<Self, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `map_op` to the given `init` value with each item of this iterator, producing a new iterator with the results. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map_with)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.map_init" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map_init" class="fn">map_init</a>\<F, INIT, T, R\>( self, init: INIT, map_op: F, ) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/map_with/struct.MapInit.html" class="struct" title="struct rayon::iter::map_with::MapInit">MapInit</a>\<Self, INIT, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, INIT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `map_op` to a value returned by `init` with each item of this iterator, producing a new iterator with the results. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map_init)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.cloned" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.cloned" class="fn">cloned</a>\<'a, T\>(self) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/cloned/struct.Cloned.html" class="struct" title="struct rayon::iter::cloned::Cloned">Cloned</a>\<Self\>

where T: 'a + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, Self: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>,

Creates an iterator which clones all of its elements. This may be useful when you have an iterator over `&T`, but you need `T`, and that type implements `Clone`. See also [`copied()`](https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.copied). [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.cloned)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.copied" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.copied" class="fn">copied</a>\<'a, T\>(self) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/copied/struct.Copied.html" class="struct" title="struct rayon::iter::copied::Copied">Copied</a>\<Self\>

where T: 'a + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, Self: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>,

Creates an iterator which copies all of its elements. This may be useful when you have an iterator over `&T`, but you need `T`, and that type implements `Copy`. See also [`cloned()`](https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.cloned). [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.copied)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.inspect" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.inspect" class="fn">inspect</a>\<OP\>(self, inspect_op: OP) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/inspect/struct.Inspect.html" class="struct" title="struct rayon::iter::inspect::Inspect">Inspect</a>\<Self, OP\>

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `inspect_op` to a reference to each item of this iterator, producing a new iterator passing through the original items. This is often useful for debugging to see what’s happening in iterator stages. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.inspect)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.update" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.update" class="fn">update</a>\<F\>(self, update_op: F) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/update/struct.Update.html" class="struct" title="struct rayon::iter::update::Update">Update</a>\<Self, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&mut Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Mutates each item of this iterator before yielding it. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.update)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.filter" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.filter" class="fn">filter</a>\<P\>(self, filter_op: P) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/filter/struct.Filter.html" class="struct" title="struct rayon::iter::filter::Filter">Filter</a>\<Self, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `filter_op` to each item of this iterator, producing a new iterator with only the items that gave `true` results. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.filter)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.filter_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.filter_map" class="fn">filter_map</a>\<P, R\>(self, filter_op: P) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/filter_map/struct.FilterMap.html" class="struct" title="struct rayon::iter::filter_map::FilterMap">FilterMap</a>\<Self, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `filter_op` to each item of this iterator to get an `Option`, producing a new iterator with only the items from `Some` results. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.filter_map)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.flat_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flat_map" class="fn">flat_map</a>\<F, PI\>(self, map_op: F) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/flat_map/struct.FlatMap.html" class="struct" title="struct rayon::iter::flat_map::FlatMap">FlatMap</a>\<Self, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> PI + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, PI: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>,

Applies `map_op` to each item of this iterator to get nested parallel iterators, producing a new parallel iterator that flattens these back into one. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flat_map)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.flat_map_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flat_map_iter" class="fn">flat_map_iter</a>\<F, SI\>(self, map_op: F) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/flat_map_iter/struct.FlatMapIter.html" class="struct" title="struct rayon::iter::flat_map_iter::FlatMapIter">FlatMapIter</a>\<Self, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> SI + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, SI: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, \<SI as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `map_op` to each item of this iterator to get nested serial iterators, producing a new parallel iterator that flattens these back into one. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flat_map_iter)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.flatten" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flatten" class="fn">flatten</a>(self) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/flatten/struct.Flatten.html" class="struct" title="struct rayon::iter::flatten::Flatten">Flatten</a>\<Self\>

where Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>,

An adaptor that flattens parallel-iterable `Item`s into one large iterator. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flatten)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.flatten_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flatten_iter" class="fn">flatten_iter</a>(self) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/flatten_iter/struct.FlattenIter.html" class="struct" title="struct rayon::iter::flatten_iter::FlattenIter">FlattenIter</a>\<Self\>

where Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, \<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

An adaptor that flattens serial-iterable `Item`s into one large iterator. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flatten_iter)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.reduce" class="fn">reduce</a>\<OP, ID\>(self, identity: ID, op: OP) -\> Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, ID: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Reduces the items in the iterator into one item using `op`. The argument `identity` should be a closure that can produce “identity” value which may be inserted into the sequence as needed to create opportunities for parallel execution. So, for example, if you are doing a summation, then `identity()` ought to produce something that represents the zero for your type (but consider just calling `sum()` in that case). [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.reduce)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.reduce_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.reduce_with" class="fn">reduce_with</a>\<OP\>(self, op: OP) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Reduces the items in the iterator into one item using `op`. If the iterator is empty, `None` is returned; otherwise, `Some` is returned. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.reduce_with)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.try_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_reduce" class="fn">try_reduce</a>\<T, OP, ID\>(self, identity: ID, op: OP) -\> Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, T) -\> Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, ID: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: Try\<Output = T\>,

Reduces the items in the iterator into one item using a fallible `op`. The `identity` argument is used the same way as in [`reduce()`](https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.reduce). [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_reduce)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.try_reduce_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_reduce_with" class="fn">try_reduce_with</a>\<T, OP\>(self, op: OP) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, T) -\> Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: Try\<Output = T\>,

Reduces the items in the iterator into one item using a fallible `op`. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_reduce_with)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.fold" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.fold" class="fn">fold</a>\<T, ID, F\>(self, identity: ID, fold_op: F) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/fold/struct.Fold.html" class="struct" title="struct rayon::iter::fold::Fold">Fold</a>\<Self, ID, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, ID: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Parallel fold is similar to sequential fold except that the sequence of items may be subdivided before it is folded. Consider a list of numbers like `22 3 77 89 46`. If you used sequential fold to add them (`fold(0, |a,b| a+b)`, you would wind up first adding 0 + 22, then 22 + 3, then 25 + 77, and so forth. The **parallel fold** works similarly except that it first breaks up your list into sublists, and hence instead of yielding up a single sum at the end, it yields up multiple sums. The number of results is nondeterministic, as is the point where the breaks occur. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.fold)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.fold_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.fold_with" class="fn">fold_with</a>\<F, T\>(self, init: T, fold_op: F) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/fold/struct.FoldWith.html" class="struct" title="struct rayon::iter::fold::FoldWith">FoldWith</a>\<Self, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Applies `fold_op` to the given `init` value with each item of this iterator, finally producing the value for further use. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.fold_with)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.try_fold" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_fold" class="fn">try_fold</a>\<T, R, ID, F\>( self, identity: ID, fold_op: F, ) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/try_fold/struct.TryFold.html" class="struct" title="struct rayon::iter::try_fold::TryFold">TryFold</a>\<Self, R, ID, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, ID: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: Try\<Output = T\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Performs a fallible parallel fold. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_fold)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.try_fold_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_fold_with" class="fn">try_fold_with</a>\<F, T, R\>(self, init: T, fold_op: F) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/try_fold/struct.TryFoldWith.html" class="struct" title="struct rayon::iter::try_fold::TryFoldWith">TryFoldWith</a>\<Self, R, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: Try\<Output = T\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Performs a fallible parallel fold with a cloneable `init` value. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_fold_with)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.sum" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.sum" class="fn">sum</a>\<S\>(self) -\> S

where S: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html" class="trait" title="trait core::iter::traits::accum::Sum">Sum</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html" class="trait" title="trait core::iter::traits::accum::Sum">Sum</a>,

Sums up the items in the iterator. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.sum)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.product" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.product" class="fn">product</a>\<P\>(self) -\> P

where P: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html" class="trait" title="trait core::iter::traits::accum::Product">Product</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html" class="trait" title="trait core::iter::traits::accum::Product">Product</a>,

Multiplies all the items in the iterator. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.product)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.min" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min" class="fn">min</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Computes the minimum of all the items in the iterator. If the iterator is empty, `None` is returned; otherwise, `Some(min)` is returned. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.min_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min_by" class="fn">min_by</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>, &Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Computes the minimum of all the items in the iterator with respect to the given comparison function. If the iterator is empty, `None` is returned; otherwise, `Some(min)` is returned. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min_by)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.min_by_key" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min_by_key" class="fn">min_by_key</a>\<K, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, F: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> K,

Computes the item that yields the minimum value for the given function. If the iterator is empty, `None` is returned; otherwise, `Some(item)` is returned. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min_by_key)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.max" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max" class="fn">max</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Computes the maximum of all the items in the iterator. If the iterator is empty, `None` is returned; otherwise, `Some(max)` is returned. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.max_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max_by" class="fn">max_by</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>, &Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Computes the maximum of all the items in the iterator with respect to the given comparison function. If the iterator is empty, `None` is returned; otherwise, `Some(max)` is returned. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max_by)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.max_by_key" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max_by_key" class="fn">max_by_key</a>\<K, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, F: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> K,

Computes the item that yields the maximum value for the given function. If the iterator is empty, `None` is returned; otherwise, `Some(item)` is returned. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max_by_key)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.chain" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.chain" class="fn">chain</a>\<C\>(self, chain: C) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/chain/struct.Chain.html" class="struct" title="struct rayon::iter::chain::Chain">Chain</a>\<Self, \<C as <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>\>

where C: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\<Item = Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>,

Takes two iterators and creates a new iterator over both. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.chain)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.find_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_any" class="fn">find_any</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for **some** item in the parallel iterator that matches the given predicate and returns it. This operation is similar to [`find` on sequential iterators](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find "method core::iter::traits::iterator::Iterator::find") but the item returned may not be the **first** one in the parallel sequence which matches, since we search the entire sequence in parallel. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_any)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.find_first" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_first" class="fn">find_first</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for the sequentially **first** item in the parallel iterator that matches the given predicate and returns it. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_first)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.find_last" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_last" class="fn">find_last</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for the sequentially **last** item in the parallel iterator that matches the given predicate and returns it. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_last)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.find_map_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_any" class="fn">find_map_any</a>\<P, R\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies the given predicate to the items in the parallel iterator and returns **any** non-None result of the map operation. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_any)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.find_map_first" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_first" class="fn">find_map_first</a>\<P, R\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies the given predicate to the items in the parallel iterator and returns the sequentially **first** non-None result of the map operation. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_first)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.find_map_last" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_last" class="fn">find_map_last</a>\<P, R\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies the given predicate to the items in the parallel iterator and returns the sequentially **last** non-None result of the map operation. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_last)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.any" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.any" class="fn">any</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for **some** item in the parallel iterator that matches the given predicate, and if so returns true. Once a match is found, we’ll attempt to stop process the rest of the items. Proving that there’s no match, returning false, does require visiting every item. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.any)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.all" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.all" class="fn">all</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Tests that every item in the parallel iterator matches the given predicate, and if so returns true. If a counter-example is found, we’ll attempt to stop processing more items, then return false. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.all)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.while_some" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.while_some" class="fn">while_some</a>\<T\>(self) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/while_some/struct.WhileSome.html" class="struct" title="struct rayon::iter::while_some::WhileSome">WhileSome</a>\<Self\>

where Self: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Creates an iterator over the `Some` items of this iterator, halting as soon as any `None` is found. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.while_some)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.panic_fuse" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.panic_fuse" class="fn">panic_fuse</a>(self) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/panic_fuse/struct.PanicFuse.html" class="struct" title="struct rayon::iter::panic_fuse::PanicFuse">PanicFuse</a>\<Self\>

Wraps an iterator with a fuse in case of panics, to halt all threads as soon as possible. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.panic_fuse)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.collect" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.collect" class="fn">collect</a>\<C\>(self) -\> C

where C: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.FromParallelIterator.html" class="trait" title="trait rayon::iter::FromParallelIterator">FromParallelIterator</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>,

Creates a fresh collection containing all the elements produced by this parallel iterator. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.collect)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.unzip" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.unzip" class="fn">unzip</a>\<A, B, FromA, FromB\>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(FromA, FromB)</a>

where Self: <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(A, B)</a>\>, FromA: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<A\>, FromB: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<B\>, A: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, B: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Unzips the items of a parallel iterator into a pair of arbitrary `ParallelExtend` containers. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.unzip)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.partition" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.partition" class="fn">partition</a>\<A, B, P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(A, B)</a>

where A: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>, B: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Partitions the items of a parallel iterator into a pair of arbitrary `ParallelExtend` containers. Items for which the `predicate` returns true go into the first container, and the rest go into the second. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.partition)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.partition_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.partition_map" class="fn">partition_map</a>\<A, B, P, L, R\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(A, B)</a>

where A: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<L\>, B: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<R\>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html" class="enum" title="enum either::Either">Either</a>\<L, R\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, L: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Partitions and maps the items of a parallel iterator into a pair of arbitrary `ParallelExtend` containers. `Either::Left` items go into the first container, and `Either::Right` items go into the second. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.partition_map)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.intersperse" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.intersperse" class="fn">intersperse</a>(self, element: Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/intersperse/struct.Intersperse.html" class="struct" title="struct rayon::iter::intersperse::Intersperse">Intersperse</a>\<Self\>

where Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Intersperses clones of an element between items of this iterator. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.intersperse)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.take_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.take_any" class="fn">take_any</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/take_any/struct.TakeAny.html" class="struct" title="struct rayon::iter::take_any::TakeAny">TakeAny</a>\<Self\>

Creates an iterator that yields `n` elements from *anywhere* in the original iterator. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.take_any)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.skip_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.skip_any" class="fn">skip_any</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/skip_any/struct.SkipAny.html" class="struct" title="struct rayon::iter::skip_any::SkipAny">SkipAny</a>\<Self\>

Creates an iterator that skips `n` elements from *anywhere* in the original iterator. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.skip_any)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.take_any_while" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.take_any_while" class="fn">take_any_while</a>\<P\>(self, predicate: P) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/take_any_while/struct.TakeAnyWhile.html" class="struct" title="struct rayon::iter::take_any_while::TakeAnyWhile">TakeAnyWhile</a>\<Self, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Creates an iterator that takes elements from *anywhere* in the original iterator until the given `predicate` returns `false`. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.take_any_while)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.skip_any_while" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.skip_any_while" class="fn">skip_any_while</a>\<P\>(self, predicate: P) -\> <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/skip_any_while/struct.SkipAnyWhile.html" class="struct" title="struct rayon::iter::skip_any_while::SkipAnyWhile">SkipAnyWhile</a>\<Self, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Creates an iterator that skips elements from *anywhere* in the original iterator until the given `predicate` returns `false`. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.skip_any_while)

<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#method.collect_vec_list" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.collect_vec_list" class="fn">collect_vec_list</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html" class="struct" title="struct alloc::collections::linked_list::LinkedList">LinkedList</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>\>

Collects this iterator into a linked list of vectors. [Read more](https://docs.rs/rayon/1.11.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.collect_vec_list)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/graphmap/struct.ParAllEdgesMut.html#blanket-implementations" class="anchor">§</a>
