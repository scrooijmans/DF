# Enum ScanSource Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/scan_sources.rs.html#55" class="src">Source</a>

``` rust
pub enum ScanSource {
    Path(PlPath),
    File(Arc<File>),
    Buffer(MemSlice),
}
```

Available on **crate feature `lazy`** only.

Expand description

A single source to scan from

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#variant.Path" class="anchor">§</a>

### Path(<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#variant.File" class="anchor">§</a>

### File(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/fs/struct.File.html" class="struct" title="struct std::fs::File">File</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#variant.Buffer" class="anchor">§</a>

### Buffer(<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>)

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#impl-ScanSource" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html" class="enum" title="enum polars::prelude::ScanSource">ScanSource</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#method.from_sources" class="fn">from_sources</a>(sources: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html" class="enum" title="enum polars::prelude::ScanSource">ScanSource</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#method.into_sources" class="fn">into_sources</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#method.as_scan_source_ref" class="fn">as_scan_source_ref</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html" class="enum" title="enum polars::prelude::ScanSourceRef">ScanSourceRef</a>\<'\_\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#method.run_async" class="fn">run_async</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#method.is_cloud_url" class="fn">is_cloud_url</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#impl-Clone-for-ScanSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html" class="enum" title="enum polars::prelude::ScanSource">ScanSource</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html" class="enum" title="enum polars::prelude::ScanSource">ScanSource</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#impl-Debug-for-ScanSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html" class="enum" title="enum polars::prelude::ScanSource">ScanSource</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSource.html#blanket-implementations" class="anchor">§</a>
