# Struct ManifestListWriter Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest_list.rs.html#87-93" class="src">Source</a>

``` rust
pub struct ManifestListWriter { /* private fields */ }
```

Expand description

A manifest list writer.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html#impl-ManifestListWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html" class="struct" title="struct iceberg::spec::ManifestListWriter">ManifestListWriter</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html#method.v1" class="fn">v1</a>( output_file: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html" class="struct" title="struct iceberg::io::OutputFile">OutputFile</a>, snapshot_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, parent_snapshot_id: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, ) -\> Self

Construct a v1 [`ManifestListWriter`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html "struct iceberg::spec::ManifestListWriter") that writes to a provided [`OutputFile`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html "struct iceberg::io::OutputFile").

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html#method.v2" class="fn">v2</a>( output_file: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html" class="struct" title="struct iceberg::io::OutputFile">OutputFile</a>, snapshot_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, parent_snapshot_id: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, sequence_number: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, ) -\> Self

Construct a v2 [`ManifestListWriter`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html "struct iceberg::spec::ManifestListWriter") that writes to a provided [`OutputFile`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html "struct iceberg::io::OutputFile").

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html#method.add_manifests" class="fn">add_manifests</a>( &mut self, manifests: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Append manifests to be written.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html#method.close" class="fn">close</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write the manifest list to the output file.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html#impl-Debug-for-ManifestListWriter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html" class="struct" title="struct iceberg::spec::ManifestListWriter">ManifestListWriter</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html#blanket-implementations" class="anchor">§</a>
