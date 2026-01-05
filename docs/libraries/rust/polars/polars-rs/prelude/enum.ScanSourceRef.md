# Enum ScanSourceRef Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/scan_sources.rs.html#47" class="src">Source</a>

``` rust
pub enum ScanSourceRef<'a> {
    Path(PlPathRef<'a>),
    File(&'a File),
    Buffer(&'a MemSlice),
}
```

Available on **crate feature `lazy`** only.

Expand description

A reference to a single item in [`ScanSources`](https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html "enum polars::prelude::ScanSources")

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#variant.Path" class="anchor">§</a>

### Path(<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'a\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#variant.File" class="anchor">§</a>

### File(&'a <a href="https://doc.rust-lang.org/nightly/std/fs/struct.File.html" class="struct" title="struct std::fs::File">File</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#variant.Buffer" class="anchor">§</a>

### Buffer(&'a <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>)

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#impl-ScanSourceRef%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html" class="enum" title="enum polars::prelude::ScanSourceRef">ScanSourceRef</a>\<'\_\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.to_include_path_name" class="fn">to_include_path_name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Get the name for `include_paths`

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.into_owned" class="fn">into_owned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html" class="enum" title="enum polars::prelude::ScanSource">ScanSource</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.as_path" class="fn">as_path</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'\_\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.is_cloud_url" class="fn">is_cloud_url</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.to_memslice" class="fn">to_memslice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Turn the scan source into a memory slice

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.to_memslice_async_assume_latest" class="fn">to_memslice_async_assume_latest</a>( &self, run_async: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.to_memslice_async_check_latest" class="fn">to_memslice_async_check_latest</a>( &self, run_async: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.to_memslice_possibly_async" class="fn">to_memslice_possibly_async</a>( &self, run_async: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, cache_entries: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/file_cache/entry/struct.FileCacheEntry.html" class="struct" title="struct polars_io::file_cache::entry::FileCacheEntry">FileCacheEntry</a>\>\>\>, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.to_dyn_byte_source" class="fn">to_dyn_byte_source</a>( &self, builder: &<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html" class="enum" title="enum polars::prelude::byte_source::DynByteSourceBuilder">DynByteSourceBuilder</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.run_async" class="fn">run_async</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#impl-Clone-for-ScanSourceRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html" class="enum" title="enum polars::prelude::ScanSourceRef">ScanSourceRef</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html" class="enum" title="enum polars::prelude::ScanSourceRef">ScanSourceRef</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#impl-Debug-for-ScanSourceRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html" class="enum" title="enum polars::prelude::ScanSourceRef">ScanSourceRef</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#impl-Display-for-ScanSourceRef%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html" class="enum" title="enum polars::prelude::ScanSourceRef">ScanSourceRef</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#impl-Copy-for-ScanSourceRef%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html" class="enum" title="enum polars::prelude::ScanSourceRef">ScanSourceRef</a>\<'a\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html#blanket-implementations" class="anchor">§</a>
