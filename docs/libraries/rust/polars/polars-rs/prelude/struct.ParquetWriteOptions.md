# Struct ParquetWriteOptions Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/parquet/write/options.rs.html#15" class="src">Source</a>

``` rust
pub struct ParquetWriteOptions {
    pub compression: ParquetCompression,
    pub statistics: StatisticsOptions,
    pub row_group_size: Option<usize>,
    pub data_page_size: Option<usize>,
    pub key_value_metadata: Option<KeyValueMetadata>,
    pub field_overwrites: Vec<ParquetFieldOverwrites>,
}
```

Available on **crate feature `polars-io`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#structfield.compression" class="anchor field">§</a>`compression: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetCompression.html" class="enum" title="enum polars::prelude::ParquetCompression"><code>ParquetCompression</code></a>

Data page compression

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#structfield.statistics" class="anchor field">§</a>`statistics: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.StatisticsOptions.html" class="struct" title="struct polars::prelude::StatisticsOptions"><code>StatisticsOptions</code></a>

Compute and write column statistics.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#structfield.row_group_size" class="anchor field">§</a>`row_group_size: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

If `None` will be all written to a single row group.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#structfield.data_page_size" class="anchor field">§</a>`data_page_size: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

if `None` will be 1024^2 bytes

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#structfield.key_value_metadata" class="anchor field">§</a>`key_value_metadata: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.KeyValueMetadata.html" class="enum" title="enum polars::prelude::KeyValueMetadata"><code>KeyValueMetadata</code></a>`>`

Custom file-level key value metadata

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#structfield.field_overwrites" class="anchor field">§</a>`field_overwrites: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetFieldOverwrites.html" class="struct" title="struct polars::prelude::ParquetFieldOverwrites"><code>ParquetFieldOverwrites</code></a>`>`

Per-field overwrites for writing properties.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#impl-ParquetWriteOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.to_writer" class="fn">to_writer</a>\<F\>(&self, f: F) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html" class="struct" title="struct polars::prelude::ParquetWriter">ParquetWriter</a>\<F\>

where F: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#impl-Clone-for-ParquetWriteOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#impl-Debug-for-ParquetWriteOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#impl-Default-for-ParquetWriteOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#impl-Deserialize%3C&#39;de%3E-for-ParquetWriteOptions" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#impl-Hash-for-ParquetWriteOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#impl-PartialEq-for-ParquetWriteOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#impl-Serialize-for-ParquetWriteOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#impl-WriteDataFrameToFile-for-ParquetWriteOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/shared/trait.WriteDataFrameToFile.html" class="trait" title="trait polars_io::shared::WriteDataFrameToFile">WriteDataFrameToFile</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#method.write_df_to_file" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/shared/trait.WriteDataFrameToFile.html#tymethod.write_df_to_file" class="fn">write_df_to_file</a>( &self, df: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, addr: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'\_\>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#impl-Eq-for-ParquetWriteOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#impl-StructuralPartialEq-for-ParquetWriteOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html#blanket-implementations" class="anchor">§</a>
