# Struct BitChunks Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/util/bit_chunk_iterator.rs.html#211" class="src">Source</a>

``` rust
pub struct BitChunks<'a> { /* private fields */ }
```

Expand description

Iterates over an arbitrarily aligned byte buffer

Yields an iterator of u64, and a remainder. The first byte in the buffer will be the least significant byte in output u64

## Implementations<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#impl-BitChunks%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html" class="struct" title="struct arrow::util::bit_chunk_iterator::BitChunks">BitChunks</a>\<'a\>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#method.new" class="fn">new</a>(buffer: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html" class="struct" title="struct arrow::util::bit_chunk_iterator::BitChunks">BitChunks</a>\<'a\>

Create a new [`BitChunks`](https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html "struct arrow::util::bit_chunk_iterator::BitChunks") from a byte array, and an offset and length in bits

<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#impl-BitChunks%3C&#39;a%3E-1" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html" class="struct" title="struct arrow::util::bit_chunk_iterator::BitChunks">BitChunks</a>\<'a\>

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#method.remainder_len" class="fn">remainder_len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of remaining bits, guaranteed to be between 0 and 63 (inclusive)

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#method.chunk_len" class="fn">chunk_len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of chunks

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#method.remainder_bits" class="fn">remainder_bits</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Returns the bitmask of remaining bits

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunkIterator.html" class="struct" title="struct arrow::util::bit_chunk_iterator::BitChunkIterator">BitChunkIterator</a>\<'a\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#" class="tooltip" data-notable-ty="BitChunkIterator&lt;&#39;a&gt;">ⓘ</a>

Returns an iterator over chunks of 64 bits represented as an u64

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#method.iter_padded" class="fn">iter_padded</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> + 'a

Returns an iterator over chunks of 64 bits, with the remaining bits zero padded to 64-bits

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#impl-Debug-for-BitChunks%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html" class="struct" title="struct arrow::util::bit_chunk_iterator::BitChunks">BitChunks</a>\<'a\>

<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#impl-IntoIterator-for-BitChunks%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html" class="struct" title="struct arrow::util::bit_chunk_iterator::BitChunks">BitChunks</a>\<'a\>

<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

The type of the elements being iterated over.

<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunkIterator.html" class="struct" title="struct arrow::util::bit_chunk_iterator::BitChunkIterator">BitChunkIterator</a>\<'a\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html" class="struct" title="struct arrow::util::bit_chunk_iterator::BitChunks">BitChunks</a>\<'a\> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunks.html#blanket-implementations" class="anchor">§</a>
