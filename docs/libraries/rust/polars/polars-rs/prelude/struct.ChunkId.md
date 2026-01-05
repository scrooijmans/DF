# Struct ChunkId Copy item path

<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/src/polars_utils/index.rs.html#192" class="src">Source</a>

``` rust
pub struct ChunkId<const CHUNK_BITS: u64 = polars_utils::::index::ChunkId::{constant#0}> { /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#impl-ChunkId%3CCHUNK_BITS%3E" class="anchor">§</a>

### impl\<const CHUNK_BITS: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<CHUNK_BITS\>

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#method.null" class="fn">null</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<CHUNK_BITS\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#method.is_null" class="fn">is_null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#method.store" class="fn">store</a>(chunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, row: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<CHUNK_BITS\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#method.extract" class="fn">extract</a>(self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#method.inner_mut" class="fn">inner_mut</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#method.from_inner" class="fn">from_inner</a>(inner: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<CHUNK_BITS\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#method.into_inner" class="fn">into_inner</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#impl-Clone-for-ChunkId%3CCHUNK_BITS%3E" class="anchor">§</a>

### impl\<const CHUNK_BITS: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<CHUNK_BITS\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<CHUNK_BITS\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#impl-Debug-for-ChunkId%3CCHUNK_BITS%3E" class="anchor">§</a>

### impl\<const CHUNK_BITS: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<CHUNK_BITS\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#impl-Copy-for-ChunkId%3CCHUNK_BITS%3E" class="anchor">§</a>

### impl\<const CHUNK_BITS: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<CHUNK_BITS\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html#blanket-implementations" class="anchor">§</a>
