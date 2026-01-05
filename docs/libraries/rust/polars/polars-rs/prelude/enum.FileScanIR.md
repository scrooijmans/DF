# Enum FileScanIR Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/file_scan/mod.rs.html#73" class="src">Source</a>

``` rust
pub enum FileScanIR {
    Csv {
        options: CsvReadOptions,
    },
    NDJson {
        options: NDJsonReadOptions,
    },
    Parquet {
        options: ParquetOptions,
        metadata: Option<Arc<FileMetadata>>,
    },
    Ipc {
        options: IpcScanOptions,
        metadata: Option<Arc<FileMetadata>>,
    },
    Anonymous {
        options: Arc<AnonymousScanOptions>,
        function: Arc<dyn AnonymousScan>,
    },
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.Csv" class="anchor">§</a>

### Csv

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.Csv.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions"><code>CsvReadOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.NDJson" class="anchor">§</a>

### NDJson

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.NDJson.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.NDJsonReadOptions.html" class="struct" title="struct polars::prelude::NDJsonReadOptions"><code>NDJsonReadOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.Parquet" class="anchor">§</a>

### Parquet

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.Parquet.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetOptions.html" class="struct" title="struct polars::prelude::ParquetOptions"><code>ParquetOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.Parquet.field.metadata" class="anchor field">§</a>`metadata: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html" class="struct" title="struct polars::prelude::FileMetadata"><code>FileMetadata</code></a>`>>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.Ipc" class="anchor">§</a>

### Ipc

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.Ipc.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcScanOptions.html" class="struct" title="struct polars::prelude::IpcScanOptions"><code>IpcScanOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.Ipc.field.metadata" class="anchor field">§</a>`metadata: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/io/ipc/read/file/struct.FileMetadata.html" class="struct" title="struct polars_arrow::io::ipc::read::file::FileMetadata"><code>FileMetadata</code></a>`>>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.Anonymous" class="anchor">§</a>

### Anonymous

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.Anonymous.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanOptions.html" class="struct" title="struct polars::prelude::AnonymousScanOptions"><code>AnonymousScanOptions</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#variant.Anonymous.field.function" class="anchor field">§</a>`function: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html" class="trait" title="trait polars::prelude::AnonymousScan"><code>AnonymousScan</code></a>`>`

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#impl-FileScanIR" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.flags" class="fn">flags</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanFlags.html" class="struct" title="struct polars::prelude::ScanFlags">ScanFlags</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.streamable" class="fn">streamable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#impl-Clone-for-FileScanIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#impl-Debug-for-FileScanIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#impl-Deserialize%3C&#39;de%3E-for-FileScanIR" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#impl-From%3C%26FileScanIR%3E-for-%26str" class="anchor">§</a>

### impl\<'\_derivative_strum\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&'\_derivative_strum <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>\> for &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(x: &'\_derivative_strum <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#impl-From%3CFileScanIR%3E-for-%26str" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>\> for &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(x: <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#impl-Hash-for-FileScanIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#impl-PartialEq-for-FileScanIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#impl-Serialize-for-FileScanIR" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#impl-Eq-for-FileScanIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html" class="enum" title="enum polars::prelude::FileScanIR">FileScanIR</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanIR.html#blanket-implementations" class="anchor">§</a>
