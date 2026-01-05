# Struct ParquetEncryptionOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#672-691" class="src">Source</a>

``` rust
pub struct ParquetEncryptionOptions {
    pub file_decryption: Option<ConfigFileDecryptionProperties>,
    pub file_encryption: Option<ConfigFileEncryptionProperties>,
    pub factory_id: Option<String>,
    pub factory_options: EncryptionFactoryOptions,
}
```

Expand description

Options for configuring Parquet Modular Encryption

To use Parquet encryption, you must enable the `parquet_encryption` feature flag, as it is not activated by default.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#structfield.file_decryption" class="anchor field">§</a>`file_decryption: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileDecryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileDecryptionProperties"><code>ConfigFileDecryptionProperties</code></a>`>`

Optional file decryption properties

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#structfield.file_encryption" class="anchor field">§</a>`file_encryption: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties"><code>ConfigFileEncryptionProperties</code></a>`>`

Optional file encryption properties

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#structfield.factory_id" class="anchor field">§</a>`factory_id: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Identifier for the encryption factory to use to create file encryption and decryption properties. Encryption factories can be registered in the runtime environment with `RuntimeEnv::register_parquet_encryption_factory`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#structfield.factory_options" class="anchor field">§</a>`factory_options: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions"><code>EncryptionFactoryOptions</code></a>

Any encryption factory specific options

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#impl-ParquetEncryptionOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions">ParquetEncryptionOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#method.configure_factory" class="fn">configure_factory</a>( &mut self, factory_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, config: &impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html" class="trait" title="trait datafusion::config::ExtensionOptions">ExtensionOptions</a>, )

Specify the encryption factory to use for Parquet modular encryption, along with its configuration

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#impl-Clone-for-ParquetEncryptionOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions">ParquetEncryptionOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions">ParquetEncryptionOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#impl-ConfigField-for-ParquetEncryptionOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions">ParquetEncryptionOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#impl-Debug-for-ParquetEncryptionOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions">ParquetEncryptionOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#impl-Default-for-ParquetEncryptionOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions">ParquetEncryptionOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions">ParquetEncryptionOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#impl-PartialEq-for-ParquetEncryptionOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions">ParquetEncryptionOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions">ParquetEncryptionOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#impl-StructuralPartialEq-for-ParquetEncryptionOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions">ParquetEncryptionOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html#blanket-implementations" class="anchor">§</a>
