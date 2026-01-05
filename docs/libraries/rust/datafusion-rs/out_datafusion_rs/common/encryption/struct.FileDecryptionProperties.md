# Struct FileDecryptionProperties Copy item path

<a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/src/parquet/encryption/decrypt.rs.html#331" class="src">Source</a>

``` rust
pub struct FileDecryptionProperties { /* private fields */ }
```

Expand description

`FileDecryptionProperties` hold keys and AAD data required to decrypt a Parquet file.

When reading Arrow data, the `FileDecryptionProperties` should be included in the [`ArrowReaderOptions`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/arrow/arrow_reader/struct.ArrowReaderOptions.html "struct parquet::arrow::arrow_reader::ArrowReaderOptions") using [`with_file_decryption_properties`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/arrow/arrow_reader/struct.ArrowReaderOptions.html#method.with_file_decryption_properties "method parquet::arrow::arrow_reader::ArrowReaderOptions::with_file_decryption_properties").

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#examples" class="doc-anchor">§</a>Examples

Create `FileDecryptionProperties` for a file encrypted with uniform encryption, where all metadata and data are encrypted with the footer key:

``` rust
let file_encryption_properties = FileDecryptionProperties::builder(b"0123456789012345".into())
    .build()?;
```

Create properties for a file where columns are encrypted with different keys:

``` rust
let file_encryption_properties = FileDecryptionProperties::builder(b"0123456789012345".into())
    .with_column_key("x", b"1234567890123450".into())
    .with_column_key("y", b"1234567890123451".into())
    .build()?;
```

Specify additional authenticated data, used to protect against data replacement. This must match the AAD prefix provided when the file was written, otherwise data decryption will fail.

``` rust
let file_encryption_properties = FileDecryptionProperties::builder(b"0123456789012345".into())
    .with_aad_prefix("example_file".into())
    .build()?;
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#impl-FileDecryptionProperties" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.builder" class="fn">builder</a>(footer_key: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/encryption/decrypt/struct.DecryptionPropertiesBuilder.html" class="struct" title="struct parquet::encryption::decrypt::DecryptionPropertiesBuilder">DecryptionPropertiesBuilder</a>

Returns a new [`FileDecryptionProperties`](https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html "struct datafusion::common::encryption::FileDecryptionProperties") builder that will use the provided key to decrypt footer metadata.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.with_key_retriever" class="fn">with_key_retriever</a>( key_retriever: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/encryption/decrypt/trait.KeyRetriever.html" class="trait" title="trait parquet::encryption::decrypt::KeyRetriever">KeyRetriever</a>\>, ) -\> <a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/encryption/decrypt/struct.DecryptionPropertiesBuilderWithRetriever.html" class="struct" title="struct parquet::encryption::decrypt::DecryptionPropertiesBuilderWithRetriever">DecryptionPropertiesBuilderWithRetriever</a>

Returns a new [`FileDecryptionProperties`](https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html "struct datafusion::common::encryption::FileDecryptionProperties") builder that uses a [`KeyRetriever`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/encryption/decrypt/trait.KeyRetriever.html "trait parquet::encryption::decrypt::KeyRetriever") to get decryption keys based on key metadata.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.aad_prefix" class="fn">aad_prefix</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>

AAD prefix string uniquely identifies the file and prevents file swapping

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.check_plaintext_footer_integrity" class="fn">check_plaintext_footer_integrity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if footer signature verification is enabled for files with plaintext footers.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.footer_key" class="fn">footer_key</a>( &self, key_metadata: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>, <a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/errors/enum.ParquetError.html" class="enum" title="enum parquet::errors::ParquetError">ParquetError</a>\>

Get the encryption key for decrypting a file’s footer, and also column data if uniform encryption is used.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.column_key" class="fn">column_key</a>( &self, column_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, key_metadata: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>, <a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/errors/enum.ParquetError.html" class="enum" title="enum parquet::errors::ParquetError">ParquetError</a>\>

Get the column-specific encryption key for decrypting column data and metadata within a file

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.column_keys" class="fn">column_keys</a>(&self) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>)

Get the column names and associated decryption keys that have been configured. If a key retriever is used rather than explicit decryption keys, the result will be empty. Provided for testing consumer code.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#impl-Clone-for-FileDecryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#impl-Debug-for-FileDecryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#impl-From%3C%26FileDecryptionProperties%3E-for-ConfigFileDecryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileDecryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileDecryptionProperties">ConfigFileDecryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(f: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileDecryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileDecryptionProperties">ConfigFileDecryptionProperties</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#impl-From%3CConfigFileDecryptionProperties%3E-for-FileDecryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileDecryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileDecryptionProperties">ConfigFileDecryptionProperties</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileDecryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileDecryptionProperties">ConfigFileDecryptionProperties</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#impl-PartialEq-for-FileDecryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#impl-StructuralPartialEq-for-FileDecryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html#blanket-implementations" class="anchor">§</a>
