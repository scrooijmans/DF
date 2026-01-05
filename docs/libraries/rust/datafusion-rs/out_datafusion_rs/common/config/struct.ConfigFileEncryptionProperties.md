# Struct ConfigFileEncryptionProperties Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#2141" class="src">Source</a>

``` rust
pub struct ConfigFileEncryptionProperties {
    pub encrypt_footer: bool,
    pub footer_key_as_hex: String,
    pub footer_key_metadata_as_hex: String,
    pub column_encryption_properties: HashMap<String, ColumnEncryptionProperties>,
    pub aad_prefix_as_hex: String,
    pub store_aad_prefix: bool,
}
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#structfield.encrypt_footer" class="anchor field">§</a>`encrypt_footer: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Should the parquet footer be encrypted default is true

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#structfield.footer_key_as_hex" class="anchor field">§</a>`footer_key_as_hex: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Key to use for the parquet footer encoded in hex format

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#structfield.footer_key_metadata_as_hex" class="anchor field">§</a>`footer_key_metadata_as_hex: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Metadata information for footer key

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#structfield.column_encryption_properties" class="anchor field">§</a>`column_encryption_properties: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ColumnEncryptionProperties.html" class="struct" title="struct datafusion::config::ColumnEncryptionProperties"><code>ColumnEncryptionProperties</code></a>`>`

HashMap of column names –\> (key in hex format, metadata)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#structfield.aad_prefix_as_hex" class="anchor field">§</a>`aad_prefix_as_hex: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

AAD prefix string uniquely identifies the file and prevents file swapping

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#structfield.store_aad_prefix" class="anchor field">§</a>`store_aad_prefix: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

If true, store the AAD prefix in the file default is false

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#impl-Clone-for-ConfigFileEncryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#impl-ConfigField-for-ConfigFileEncryptionProperties" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#impl-Debug-for-ConfigFileEncryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#impl-Default-for-ConfigFileEncryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#impl-From%3C%26FileEncryptionProperties%3E-for-ConfigFileEncryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileEncryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileEncryptionProperties">FileEncryptionProperties</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(f: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileEncryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileEncryptionProperties">FileEncryptionProperties</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#impl-From%3CConfigFileEncryptionProperties%3E-for-FileEncryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileEncryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileEncryptionProperties">FileEncryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileEncryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileEncryptionProperties">FileEncryptionProperties</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#impl-PartialEq-for-ConfigFileEncryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#impl-StructuralPartialEq-for-ConfigFileEncryptionProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.ConfigFileEncryptionProperties.html#blanket-implementations" class="anchor">§</a>
