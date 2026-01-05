# Enum DynByteSourceBuilder Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/byte_source.rs.html#172" class="src">Source</a>

``` rust
pub enum DynByteSourceBuilder {
    Mmap,
    ObjectStore,
}
```

Available on **crate feature `polars-io`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#variant.Mmap" class="anchor">§</a>

### Mmap

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#variant.ObjectStore" class="anchor">§</a>

### ObjectStore

Supports both cloud and local files.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#impl-DynByteSourceBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html" class="enum" title="enum polars::prelude::byte_source::DynByteSourceBuilder">DynByteSourceBuilder</a>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#method.try_build_from_path" class="fn">try_build_from_path</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSource.html" class="enum" title="enum polars::prelude::byte_source::DynByteSource">DynByteSource</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#impl-Clone-for-DynByteSourceBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html" class="enum" title="enum polars::prelude::byte_source::DynByteSourceBuilder">DynByteSourceBuilder</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html" class="enum" title="enum polars::prelude::byte_source::DynByteSourceBuilder">DynByteSourceBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#impl-Debug-for-DynByteSourceBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html" class="enum" title="enum polars::prelude::byte_source::DynByteSourceBuilder">DynByteSourceBuilder</a>

<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/byte_source/enum.DynByteSourceBuilder.html#blanket-implementations" class="anchor">§</a>
