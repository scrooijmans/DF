# Struct CatalogOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#198-238" class="src">Source</a>

``` rust
pub struct CatalogOptions {
    pub create_default_catalog_and_schema: bool,
    pub default_catalog: String,
    pub default_schema: String,
    pub information_schema: bool,
    pub location: Option<String>,
    pub format: Option<String>,
    pub has_header: bool,
    pub newlines_in_values: bool,
}
```

Expand description

Options related to catalog and directory scanning

See also: [`SessionConfig`](https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html)

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#structfield.create_default_catalog_and_schema" class="anchor field">§</a>`create_default_catalog_and_schema: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether the default catalog and schema should be created automatically.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#structfield.default_catalog" class="anchor field">§</a>`default_catalog: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The default catalog name - this impacts what SQL queries use if not specified

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#structfield.default_schema" class="anchor field">§</a>`default_schema: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The default schema name - this impacts what SQL queries use if not specified

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#structfield.information_schema" class="anchor field">§</a>`information_schema: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Should DataFusion provide access to `information_schema` virtual tables for displaying schema information

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#structfield.location" class="anchor field">§</a>`location: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Location scanned to load tables for `default` schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#structfield.format" class="anchor field">§</a>`format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Type of `TableProvider` to use when loading `default` schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#structfield.has_header" class="anchor field">§</a>`has_header: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Default value for `format.has_header` for `CREATE EXTERNAL TABLE` if not specified explicitly in the statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#structfield.newlines_in_values" class="anchor field">§</a>`newlines_in_values: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Specifies whether newlines in (quoted) CSV values are supported.

This is the default value for `format.newlines_in_values` for `CREATE EXTERNAL TABLE` if not specified explicitly in the statement.

Parsing newlines in quoted values may be affected by execution behaviour such as parallel file scanning. Setting this to `true` ensures that newlines in values are parsed successfully, which may reduce performance.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#impl-Clone-for-CatalogOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html" class="struct" title="struct datafusion::config::CatalogOptions">CatalogOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html" class="struct" title="struct datafusion::config::CatalogOptions">CatalogOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#impl-ConfigField-for-CatalogOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html" class="struct" title="struct datafusion::config::CatalogOptions">CatalogOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#impl-Debug-for-CatalogOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html" class="struct" title="struct datafusion::config::CatalogOptions">CatalogOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#impl-Default-for-CatalogOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html" class="struct" title="struct datafusion::config::CatalogOptions">CatalogOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html" class="struct" title="struct datafusion::config::CatalogOptions">CatalogOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#impl-PartialEq-for-CatalogOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html" class="struct" title="struct datafusion::config::CatalogOptions">CatalogOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html" class="struct" title="struct datafusion::config::CatalogOptions">CatalogOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#impl-StructuralPartialEq-for-CatalogOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html" class="struct" title="struct datafusion::config::CatalogOptions">CatalogOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html#blanket-implementations" class="anchor">§</a>
