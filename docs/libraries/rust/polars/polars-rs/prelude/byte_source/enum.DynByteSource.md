# Enum DynByteSource Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/byte_source.rs.html#107" class="src">Source</a>

``` rust
pub enum DynByteSource {
    MemSlice(MemSliceByteSource),
    Cloud(ObjectStoreByteSource),
}
```

Available on **crate feature `polars-io`** only.

Expand description

Dynamic dispatch to async functions.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#variant.MemSlice" class="anchor">§</a>

### MemSlice(<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html" class="struct" title="struct polars::prelude::byte_source::MemSliceByteSource">MemSliceByteSource</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#variant.Cloud" class="anchor">§</a>

### Cloud(<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.ObjectStoreByteSource.html" class="struct" title="struct polars::prelude::byte_source::ObjectStoreByteSource">ObjectStoreByteSource</a>)

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#impl-DynByteSource" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#method.variant_name" class="fn">variant_name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#impl-ByteSource-for-DynByteSource" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html" class="trait" title="trait polars::prelude::byte_source::ByteSource">ByteSource</a> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#method.get_size" class="anchor">§</a>

#### async fn <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#tymethod.get_size" class="fn">get_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#method.get_range" class="anchor">§</a>

#### async fn <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#tymethod.get_range" class="fn">get_range</a>(&self, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Panics [Read more](https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#tymethod.get_range)

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#method.get_ranges" class="anchor">§</a>

#### async fn <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/trait.ByteSource.html#tymethod.get_ranges" class="fn">get_ranges</a>( &self, ranges: &mut \[<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Note: This will mutably sort ranges for coalescing.

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#impl-Default-for-DynByteSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#impl-From%3CMemSlice%3E-for-DynByteSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#impl-From%3CMemSliceByteSource%3E-for-DynByteSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html" class="struct" title="struct polars::prelude::byte_source::MemSliceByteSource">MemSliceByteSource</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.MemSliceByteSource.html" class="struct" title="struct polars::prelude::byte_source::MemSliceByteSource">MemSliceByteSource</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#impl-From%3CObjectStoreByteSource%3E-for-DynByteSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.ObjectStoreByteSource.html" class="struct" title="struct polars::prelude::byte_source::ObjectStoreByteSource">ObjectStoreByteSource</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/struct.ObjectStoreByteSource.html" class="struct" title="struct polars::prelude::byte_source::ObjectStoreByteSource">ObjectStoreByteSource</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html#blanket-implementations" class="anchor">§</a>
