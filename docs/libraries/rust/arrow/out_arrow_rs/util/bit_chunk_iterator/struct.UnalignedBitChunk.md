# Struct UnalignedBitChunk Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/util/bit_chunk_iterator.rs.html#31" class="src">Source</a>

``` rust
pub struct UnalignedBitChunk<'a> { /* private fields */ }
```

Expand description

Iterates over an arbitrarily aligned byte buffer

Yields an iterator of aligned u64, along with the leading and trailing u64 necessary to align the buffer to a 8-byte boundary

This is unlike [`BitChunkIterator`](https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.BitChunkIterator.html "struct arrow::util::bit_chunk_iterator::BitChunkIterator") which only exposes a trailing u64, and consequently has to perform more work for each read

## Implementations<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#impl-UnalignedBitChunk%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html" class="struct" title="struct arrow::util::bit_chunk_iterator::UnalignedBitChunk">UnalignedBitChunk</a>\<'a\>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#method.new" class="fn">new</a>(buffer: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html" class="struct" title="struct arrow::util::bit_chunk_iterator::UnalignedBitChunk">UnalignedBitChunk</a>\<'a\>

Create a from a byte array, and and an offset and length in bits

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#method.lead_padding" class="fn">lead_padding</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of leading padding bits

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#method.trailing_padding" class="fn">trailing_padding</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of trailing padding bits

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#method.prefix" class="fn">prefix</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Returns the prefix, if any

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#method.suffix" class="fn">suffix</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Returns the suffix, if any

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#method.chunks" class="fn">chunks</a>(&self) -\> &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\]

Returns reference to the chunks

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#method.iter" class="fn">iter</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html" class="struct" title="struct core::iter::adapters::chain::Chain">Chain</a>\<<a href="https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html" class="struct" title="struct core::iter::adapters::chain::Chain">Chain</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/struct.IntoIter.html" class="struct" title="struct core::option::IntoIter">IntoIter</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/cloned/struct.Cloned.html" class="struct" title="struct core::iter::adapters::cloned::Cloned">Cloned</a>\<<a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\>\>, <a href="https://doc.rust-lang.org/nightly/core/option/struct.IntoIter.html" class="struct" title="struct core::option::IntoIter">IntoIter</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#" class="tooltip" data-notable-ty="Chain&lt;Chain&lt;IntoIter&lt;u64&gt;, Cloned&lt;Iter&lt;&#39;a, u64&gt;&gt;&gt;, IntoIter&lt;u64&gt;&gt;">ⓘ</a>

Returns an iterator over the chunks

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#method.count_ones" class="fn">count_ones</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Counts the number of ones

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#impl-Debug-for-UnalignedBitChunk%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html" class="struct" title="struct arrow::util::bit_chunk_iterator::UnalignedBitChunk">UnalignedBitChunk</a>\<'a\>

<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html#blanket-implementations" class="anchor">§</a>
