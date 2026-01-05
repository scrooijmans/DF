# Struct UnifiedScanArgs Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/file_scan/mod.rs.html#233" class="src">Source</a>

``` rust
pub struct UnifiedScanArgs {Show 16 fields
    pub schema: Option<Arc<Schema<DataType>>>,
    pub cloud_options: Option<CloudOptions>,
    pub hive_options: HiveOptions,
    pub rechunk: bool,
    pub cache: bool,
    pub glob: bool,
    pub projection: Option<Arc<[PlSmallStr]>>,
    pub column_mapping: Option<ColumnMapping>,
    pub default_values: Option<DefaultFieldValues>,
    pub row_index: Option<RowIndex>,
    pub pre_slice: Option<Slice>,
    pub cast_columns_policy: CastColumnsPolicy,
    pub missing_columns_policy: MissingColumnsPolicy,
    pub extra_columns_policy: ExtraColumnsPolicy,
    pub include_file_paths: Option<PlSmallStr>,
    pub deletion_files: Option<DeletionFilesList>,
}
```

Available on **crate feature `lazy`** only.

Expand description

Scan arguments shared across different scan types.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema"><code>Schema</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>>>`

User-provided schema of the file. Will be inferred during IR conversion if None.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.cloud_options" class="anchor field">§</a>`cloud_options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions"><code>CloudOptions</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.hive_options" class="anchor field">§</a>`hive_options: `<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.HiveOptions.html" class="struct" title="struct polars_io::options::HiveOptions"><code>HiveOptions</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.rechunk" class="anchor field">§</a>`rechunk: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.cache" class="anchor field">§</a>`cache: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.glob" class="anchor field">§</a>`glob: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.projection" class="anchor field">§</a>`projection: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<[`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>`]>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.column_mapping" class="anchor field">§</a>`column_mapping: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.ColumnMapping.html" class="enum" title="enum polars::prelude::ColumnMapping"><code>ColumnMapping</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.default_values" class="anchor field">§</a>`default_values: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/default_values/enum.DefaultFieldValues.html" class="enum" title="enum polars::prelude::default_values::DefaultFieldValues"><code>DefaultFieldValues</code></a>`>`

Default values for missing columns.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.row_index" class="anchor field">§</a>`row_index: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex"><code>RowIndex</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.pre_slice" class="anchor field">§</a>`pre_slice: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/slice_enum/enum.Slice.html" class="enum" title="enum polars_utils::slice_enum::Slice"><code>Slice</code></a>`>`

Slice applied before predicates

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.cast_columns_policy" class="anchor field">§</a>`cast_columns_policy: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy"><code>CastColumnsPolicy</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.missing_columns_policy" class="anchor field">§</a>`missing_columns_policy: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.MissingColumnsPolicy.html" class="enum" title="enum polars::prelude::MissingColumnsPolicy"><code>MissingColumnsPolicy</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.extra_columns_policy" class="anchor field">§</a>`extra_columns_policy: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ExtraColumnsPolicy.html" class="enum" title="enum polars::prelude::ExtraColumnsPolicy"><code>ExtraColumnsPolicy</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.include_file_paths" class="anchor field">§</a>`include_file_paths: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#structfield.deletion_files" class="anchor field">§</a>`deletion_files: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/deletion/enum.DeletionFilesList.html" class="enum" title="enum polars::prelude::deletion::DeletionFilesList"><code>DeletionFilesList</code></a>`>`

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#impl-Clone-for-UnifiedScanArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#impl-Debug-for-UnifiedScanArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#impl-Default-for-UnifiedScanArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#impl-Deserialize%3C&#39;de%3E-for-UnifiedScanArgs" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#impl-Hash-for-UnifiedScanArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#impl-PartialEq-for-UnifiedScanArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#impl-Serialize-for-UnifiedScanArgs" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#impl-Eq-for-UnifiedScanArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#impl-StructuralPartialEq-for-UnifiedScanArgs" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html" class="struct" title="struct polars::prelude::UnifiedScanArgs">UnifiedScanArgs</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.UnifiedScanArgs.html#blanket-implementations" class="anchor">§</a>
