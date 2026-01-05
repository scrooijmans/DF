# Struct TableParquetOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#1868" class="src">Source</a>

``` rust
pub struct TableParquetOptions {
    pub global: ParquetOptions,
    pub column_specific_options: HashMap<String, ParquetColumnOptions>,
    pub key_value_metadata: HashMap<String, Option<String>>,
    pub crypto: ParquetEncryptionOptions,
}
```

Expand description

Options that control how Parquet files are read, including global options that apply to all columns and optional column-specific overrides

Closely tied to [`ParquetWriterOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/parquet_writer/struct.ParquetWriterOptions.html "struct datafusion::common::file_options::parquet_writer::ParquetWriterOptions"). Properties not included in [`TableParquetOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html "struct datafusion::config::TableParquetOptions") may not be configurable at the external API (e.g. sorting_columns).

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#structfield.global" class="anchor field">§</a>`global: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions"><code>ParquetOptions</code></a>

Global Parquet options that propagates to all columns.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#structfield.column_specific_options" class="anchor field">§</a>`column_specific_options: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html" class="struct" title="struct datafusion::config::ParquetColumnOptions"><code>ParquetColumnOptions</code></a>`>`

Column specific options. Default usage is parquet.XX::column.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#structfield.key_value_metadata" class="anchor field">§</a>`key_value_metadata: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>>`

Additional file-level metadata to include. Inserted into the key_value_metadata for the written [`FileMetaData`](https://docs.rs/parquet/latest/parquet/file/metadata/struct.FileMetaData.html).

Multiple entries are permitted

``` sql
OPTIONS (
   'format.metadata::key1' '',
   'format.metadata::key2' 'value',
   'format.metadata::key3' 'value has spaces',
   'format.metadata::key4' 'value has special chars :: :',
   'format.metadata::key_dupe' 'original will be overwritten',
   'format.metadata::key_dupe' 'final'
)
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#structfield.crypto" class="anchor field">§</a>`crypto: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions"><code>ParquetEncryptionOptions</code></a>

Options for configuring Parquet modular encryption

To use Parquet encryption, you must enable the `parquet_encryption` feature flag, as it is not activated by default. See ConfigFileEncryptionProperties and ConfigFileDecryptionProperties in datafusion/common/src/config.rs These can be set via ‘format.crypto’, for example:

``` sql
OPTIONS (
   'format.crypto.file_encryption.encrypt_footer' 'true',
   'format.crypto.file_encryption.footer_key_as_hex' '30313233343536373839303132333435',  -- b"0123456789012345" */
   'format.crypto.file_encryption.column_key_as_hex::double_field' '31323334353637383930313233343530', -- b"1234567890123450"
   'format.crypto.file_encryption.column_key_as_hex::float_field' '31323334353637383930313233343531', -- b"1234567890123451"
    -- Same for decryption
   'format.crypto.file_decryption.footer_key_as_hex' '30313233343536373839303132333435', -- b"0123456789012345"
   'format.crypto.file_decryption.column_key_as_hex::double_field' '31323334353637383930313233343530', -- b"1234567890123450"
   'format.crypto.file_decryption.column_key_as_hex::float_field' '31323334353637383930313233343531', -- b"1234567890123451"
)
```

See datafusion-cli/tests/sql/encrypted_parquet.sql for a more complete example. Note that keys must be provided as in hex format since these are binary strings.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#impl-TableParquetOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

Return new default TableParquetOptions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.with_skip_arrow_metadata" class="fn">with_skip_arrow_metadata</a>(self, skip: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

Set whether the encoding of the arrow metadata should occur during the writing of parquet.

Default is to encode the arrow schema in the file kv_metadata.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.entries" class="fn">entries</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigEntry.html" class="struct" title="struct datafusion::config::ConfigEntry">ConfigEntry</a>\>

Retrieves all configuration entries from this `TableParquetOptions`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#returns" class="doc-anchor">§</a>Returns

A vector of `ConfigEntry` instances, representing all the configuration options within this

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#impl-TableParquetOptions-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.arrow_schema" class="fn">arrow_schema</a>(&mut self, schema: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>)

Add the arrow schema to the parquet kv_metadata. If already exists, then overwrites.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#impl-Clone-for-TableParquetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#impl-ConfigField-for-TableParquetOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#impl-Debug-for-TableParquetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#impl-Default-for-TableParquetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#impl-PartialEq-for-TableParquetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#impl-TryFrom%3C%26TableParquetOptions%3E-for-ParquetWriterOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/parquet_writer/struct.ParquetWriterOptions.html" class="struct" title="struct datafusion::common::file_options::parquet_writer::ParquetWriterOptions">ParquetWriterOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( parquet_table_options: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/parquet_writer/struct.ParquetWriterOptions.html" class="struct" title="struct datafusion::common::file_options::parquet_writer::ParquetWriterOptions">ParquetWriterOptions</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#impl-StructuralPartialEq-for-TableParquetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html#blanket-implementations" class="anchor">§</a>
