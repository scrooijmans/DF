# Enum ScanSources Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/scan_sources.rs.html#26" class="src">Source</a>

``` rust
pub enum ScanSources {
    Paths(Arc<[PlPath]>),
    Files(Arc<[File]>),
    Buffers(Arc<[MemSlice]>),
}
```

Available on **crate feature `lazy`** only.

Expand description

Set of sources to scan from

This can either be a list of paths to files, opened files or in-memory buffers. Mixing of buffers is not currently possible.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#variant.Paths" class="anchor">§</a>

### Paths(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\]\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#variant.Files" class="anchor">§</a>

### Files(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/fs/struct.File.html" class="struct" title="struct std::fs::File">File</a>\]\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#variant.Buffers" class="anchor">§</a>

### Buffers(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>\]\>)

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#impl-ScanSources" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.expand_paths" class="fn">expand_paths</a>( &self, scan_args: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.expand_paths_with_hive_update" class="fn">expand_paths_with_hive_update</a>( &self, scan_args: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

This will update `scan_args.hive_options.enabled` to `true` if the existing value is `None` and the paths are expanded from a single directory. Otherwise the existing value is maintained.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanSourceIter.html" class="struct" title="struct polars::prelude::ScanSourceIter">ScanSourceIter</a>\<'\_\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#" class="tooltip" data-notable-ty="ScanSourceIter&lt;&#39;_&gt;">ⓘ</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.is_paths" class="fn">is_paths</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Are the sources all paths?

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.as_paths" class="fn">as_paths</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\]\>

Try cast the scan sources to [`ScanSources::Paths`](https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#variant.Paths "variant polars::prelude::ScanSources::Paths")

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.into_paths" class="fn">into_paths</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\]\>\>

Try cast the scan sources to [`ScanSources::Paths`](https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#variant.Paths "variant polars::prelude::ScanSources::Paths") with a clone

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.first_path" class="fn">first_path</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'\_\>\>

Try get the first path in the scan sources

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.is_cloud_url" class="fn">is_cloud_url</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Is the first path a cloud URL?

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.first" class="fn">first</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html" class="enum" title="enum polars::prelude::ScanSourceRef">ScanSourceRef</a>\<'\_\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.first_or_empty_expand_err" class="fn">first_or_empty_expand_err</a>( &self, failed_message: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, sources_before_expansion: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>, glob: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, hint: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html" class="enum" title="enum polars::prelude::ScanSourceRef">ScanSourceRef</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.id" class="fn">id</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

Turn the [`ScanSources`](https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html "enum polars::prelude::ScanSources") into some kind of identifier

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.get" class="fn">get</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html" class="enum" title="enum polars::prelude::ScanSourceRef">ScanSourceRef</a>\<'\_\>\>

Get the scan source at specific address

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.at" class="fn">at</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSourceRef.html" class="enum" title="enum polars::prelude::ScanSourceRef">ScanSourceRef</a>\<'\_\>

Get the scan source at specific address

##### <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#panics" class="doc-anchor">§</a>Panics

If the `idx` is out of range.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#impl-Clone-for-ScanSources" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#impl-Debug-for-ScanSources" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#impl-Default-for-ScanSources" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#impl-Deserialize%3C&#39;de%3E-for-ScanSources" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#impl-Hash-for-ScanSources" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#impl-PartialEq-for-ScanSources" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#impl-Serialize-for-ScanSources" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#impl-Eq-for-ScanSources" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html#blanket-implementations" class="anchor">§</a>
