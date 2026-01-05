# Trait ByteSource Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/byte_source.rs.html#15" class="src">Source</a>

``` rust
pub trait ByteSource: Send + Sync {
    // Required methods
    async fn get_size(&self) -> Result<usize, PolarsError>;
    async fn get_range(
        &self,
        range: Range<usize>,
    ) -> Result<MemSlice, PolarsError>;
    async fn get_ranges(
        &self,
        ranges: &mut [Range<usize>],
    ) -> Result<HashMap<usize, MemSlice, RandomState>, PolarsError>;
}
```

Available on **crate feature `polars-io`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#required-methods" class="anchor">§</a>

#### async fn <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#tymethod.get_size" class="fn">get_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### async fn <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#tymethod.get_range" class="fn">get_range</a>(&self, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

##### <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#panics" class="doc-anchor">§</a>Panics

Panics if `range` is not in bounds.

#### async fn <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#tymethod.get_ranges" class="fn">get_ranges</a>( &self, ranges: &mut \[<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Note: This will mutably sort ranges for coalescing.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#impl-ByteSource-for-DynByteSource" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html" class="trait" title="trait polars::prelude::byte_source::ByteSource">ByteSource</a> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#impl-ByteSource-for-MemSliceByteSource" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html" class="trait" title="trait polars::prelude::byte_source::ByteSource">ByteSource</a> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html" class="struct" title="struct polars::prelude::byte_source::MemSliceByteSource">MemSliceByteSource</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#impl-ByteSource-for-ObjectStoreByteSource" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html" class="trait" title="trait polars::prelude::byte_source::ByteSource">ByteSource</a> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.ObjectStoreByteSource.html" class="struct" title="struct polars::prelude::byte_source::ObjectStoreByteSource">ObjectStoreByteSource</a>
