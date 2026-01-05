# Enum FileScanDsl Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/file_scan/mod.rs.html#43" class="src">Source</a>

``` rust
pub enum FileScanDsl {
    Csv {
        options: CsvReadOptions,
    },
    NDJson {
        options: NDJsonReadOptions,
    },
    Parquet {
        options: ParquetOptions,
    },
    Ipc {
        options: IpcScanOptions,
    },
    Anonymous {
        options: Arc<AnonymousScanOptions>,
        function: Arc<dyn AnonymousScan>,
        file_info: FileInfo,
    },
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.Csv" class="anchor">§</a>

### Csv

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.Csv.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions"><code>CsvReadOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.NDJson" class="anchor">§</a>

### NDJson

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.NDJson.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.NDJsonReadOptions.html" class="struct" title="struct polars::prelude::NDJsonReadOptions"><code>NDJsonReadOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.Parquet" class="anchor">§</a>

### Parquet

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.Parquet.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetOptions.html" class="struct" title="struct polars::prelude::ParquetOptions"><code>ParquetOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.Ipc" class="anchor">§</a>

### Ipc

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.Ipc.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcScanOptions.html" class="struct" title="struct polars::prelude::IpcScanOptions"><code>IpcScanOptions</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.Anonymous" class="anchor">§</a>

### Anonymous

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.Anonymous.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanOptions.html" class="struct" title="struct polars::prelude::AnonymousScanOptions"><code>AnonymousScanOptions</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.Anonymous.field.function" class="anchor field">§</a>`function: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html" class="trait" title="trait polars::prelude::AnonymousScan"><code>AnonymousScan</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#variant.Anonymous.field.file_info" class="anchor field">§</a>`file_info: `<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/schema/struct.FileInfo.html" class="struct" title="struct polars_plan::plans::schema::FileInfo"><code>FileInfo</code></a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#impl-Clone-for-FileScanDsl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html" class="enum" title="enum polars::prelude::FileScanDsl">FileScanDsl</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html" class="enum" title="enum polars::prelude::FileScanDsl">FileScanDsl</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#impl-Debug-for-FileScanDsl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html" class="enum" title="enum polars::prelude::FileScanDsl">FileScanDsl</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#impl-Deserialize%3C&#39;de%3E-for-FileScanDsl" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html" class="enum" title="enum polars::prelude::FileScanDsl">FileScanDsl</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html" class="enum" title="enum polars::prelude::FileScanDsl">FileScanDsl</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#impl-From%3C%26FileScanDsl%3E-for-%26str" class="anchor">§</a>

### impl\<'\_derivative_strum\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&'\_derivative_strum <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html" class="enum" title="enum polars::prelude::FileScanDsl">FileScanDsl</a>\> for &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(x: &'\_derivative_strum <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html" class="enum" title="enum polars::prelude::FileScanDsl">FileScanDsl</a>) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#impl-From%3CFileScanDsl%3E-for-%26str" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html" class="enum" title="enum polars::prelude::FileScanDsl">FileScanDsl</a>\> for &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(x: <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html" class="enum" title="enum polars::prelude::FileScanDsl">FileScanDsl</a>) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#impl-Serialize-for-FileScanDsl" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html" class="enum" title="enum polars::prelude::FileScanDsl">FileScanDsl</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileScanDsl.html#blanket-implementations" class="anchor">§</a>
