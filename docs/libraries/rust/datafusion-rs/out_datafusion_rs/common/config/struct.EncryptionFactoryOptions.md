# Struct EncryptionFactoryOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#2460" class="src">Source</a>

``` rust
pub struct EncryptionFactoryOptions {
    pub options: HashMap<String, String>,
}
```

Expand description

Holds implementation-specific options for an encryption factory

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#structfield.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#impl-EncryptionFactoryOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#method.to_extension_options" class="fn">to_extension_options</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html" class="trait" title="trait datafusion::config::ExtensionOptions">ExtensionOptions</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

Convert these encryption factory options to an [`ExtensionOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html "trait datafusion::config::ExtensionOptions") instance.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#impl-Clone-for-EncryptionFactoryOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#impl-ConfigField-for-EncryptionFactoryOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#impl-Debug-for-EncryptionFactoryOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#impl-Default-for-EncryptionFactoryOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#impl-PartialEq-for-EncryptionFactoryOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#impl-StructuralPartialEq-for-EncryptionFactoryOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/struct.EncryptionFactoryOptions.html#blanket-implementations" class="anchor">§</a>
