# Struct JsonOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#2637-2643" class="src">Source</a>

``` rust
pub struct JsonOptions {
    pub compression: CompressionTypeVariant,
    pub schema_infer_max_rec: Option<usize>,
}
```

Expand description

Options controlling JSON format

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#structfield.compression" class="anchor field">§</a>`compression: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/parsers/enum.CompressionTypeVariant.html" class="enum" title="enum datafusion::common::parsers::CompressionTypeVariant"><code>CompressionTypeVariant</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#structfield.schema_infer_max_rec" class="anchor field">§</a>`schema_infer_max_rec: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#impl-Clone-for-JsonOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#impl-ConfigField-for-JsonOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#impl-Debug-for-JsonOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#impl-Default-for-JsonOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#impl-PartialEq-for-JsonOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#impl-TryFrom%3C%26JsonOptions%3E-for-JsonWriterOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/json_writer/struct.JsonWriterOptions.html" class="struct" title="struct datafusion::common::file_options::json_writer::JsonWriterOptions">JsonWriterOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(value: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/json_writer/struct.JsonWriterOptions.html" class="struct" title="struct datafusion::common::file_options::json_writer::JsonWriterOptions">JsonWriterOptions</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#impl-StructuralPartialEq-for-JsonOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html#blanket-implementations" class="anchor">§</a>
