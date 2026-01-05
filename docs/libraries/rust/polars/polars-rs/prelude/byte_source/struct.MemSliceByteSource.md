# Struct MemSliceByteSource Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/byte_source.rs.html#28" class="src">Source</a>

``` rust
pub struct MemSliceByteSource(pub MemSlice);
```

Available on **crate feature `polars-io`** only.

Expand description

Byte source backed by a `MemSlice`, which can potentially be memory-mapped.

## Tuple Fields<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html#structfield.0" class="anchor field">§</a>`0: `<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice"><code>MemSlice</code></a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html#impl-ByteSource-for-MemSliceByteSource" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html" class="trait" title="trait polars::prelude::byte_source::ByteSource">ByteSource</a> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html" class="struct" title="struct polars::prelude::byte_source::MemSliceByteSource">MemSliceByteSource</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html#method.get_size" class="anchor">§</a>

#### async fn <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#tymethod.get_size" class="fn">get_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html#method.get_range" class="anchor">§</a>

#### async fn <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#tymethod.get_range" class="fn">get_range</a>(&self, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Panics [Read more](https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#tymethod.get_range)

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html#method.get_ranges" class="anchor">§</a>

#### async fn <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#tymethod.get_ranges" class="fn">get_ranges</a>( &self, ranges: &mut \[<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Note: This will mutably sort ranges for coalescing.

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html#impl-From%3CMemSliceByteSource%3E-for-DynByteSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html" class="struct" title="struct polars::prelude::byte_source::MemSliceByteSource">MemSliceByteSource</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html" class="struct" title="struct polars::prelude::byte_source::MemSliceByteSource">MemSliceByteSource</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html#blanket-implementations" class="anchor">§</a>
