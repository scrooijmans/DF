# Struct CsvOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#2492-2525" class="src">Source</a>

``` rust
pub struct CsvOptions {Show 17 fields
    pub has_header: Option<bool>,
    pub delimiter: u8,
    pub quote: u8,
    pub terminator: Option<u8>,
    pub escape: Option<u8>,
    pub double_quote: Option<bool>,
    pub newlines_in_values: Option<bool>,
    pub compression: CompressionTypeVariant,
    pub schema_infer_max_rec: Option<usize>,
    pub date_format: Option<String>,
    pub datetime_format: Option<String>,
    pub timestamp_format: Option<String>,
    pub timestamp_tz_format: Option<String>,
    pub time_format: Option<String>,
    pub null_value: Option<String>,
    pub null_regex: Option<String>,
    pub comment: Option<u8>,
}
```

Expand description

Options controlling CSV format

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.has_header" class="anchor field">§</a>`has_header: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

Specifies whether there is a CSV header (i.e. the first line consists of is column names). The value `None` indicates that the configuration should be consulted.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.delimiter" class="anchor field">§</a>`delimiter: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.quote" class="anchor field">§</a>`quote: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.terminator" class="anchor field">§</a>`terminator: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.escape" class="anchor field">§</a>`escape: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.double_quote" class="anchor field">§</a>`double_quote: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.newlines_in_values" class="anchor field">§</a>`newlines_in_values: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

Specifies whether newlines in (quoted) values are supported.

Parsing newlines in quoted values may be affected by execution behaviour such as parallel file scanning. Setting this to `true` ensures that newlines in values are parsed successfully, which may reduce performance.

The default behaviour depends on the `datafusion.catalog.newlines_in_values` setting.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.compression" class="anchor field">§</a>`compression: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/parsers/enum.CompressionTypeVariant.html" class="enum" title="enum datafusion::common::parsers::CompressionTypeVariant"><code>CompressionTypeVariant</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.schema_infer_max_rec" class="anchor field">§</a>`schema_infer_max_rec: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.date_format" class="anchor field">§</a>`date_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.datetime_format" class="anchor field">§</a>`datetime_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.timestamp_format" class="anchor field">§</a>`timestamp_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.timestamp_tz_format" class="anchor field">§</a>`timestamp_tz_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.time_format" class="anchor field">§</a>`time_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.null_value" class="anchor field">§</a>`null_value: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.null_regex" class="anchor field">§</a>`null_regex: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#structfield.comment" class="anchor field">§</a>`comment: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#impl-CsvOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.with_compression" class="fn">with_compression</a>( self, compression_type_variant: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/parsers/enum.CompressionTypeVariant.html" class="enum" title="enum datafusion::common::parsers::CompressionTypeVariant">CompressionTypeVariant</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

Set a limit in terms of records to scan to infer the schema

- default to `DEFAULT_SCHEMA_INFER_MAX_RECORD`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.with_schema_infer_max_rec" class="fn">with_schema_infer_max_rec</a>(self, max_rec: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

Set a limit in terms of records to scan to infer the schema

- default to `DEFAULT_SCHEMA_INFER_MAX_RECORD`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.with_has_header" class="fn">with_has_header</a>(self, has_header: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

Set true to indicate that the first line is a header.

- default to true

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.has_header" class="fn">has_header</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Returns true if the first line is a header. If format options does not specify whether there is a header, returns `None` (indicating that the configuration should be consulted).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.with_delimiter" class="fn">with_delimiter</a>(self, delimiter: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

The character separating values within a row.

- default to ‘,’

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.with_quote" class="fn">with_quote</a>(self, quote: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

The quote character in a row.

- default to ‘“’

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.with_terminator" class="fn">with_terminator</a>(self, terminator: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

The character that terminates a row.

- default to None (CRLF)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.with_escape" class="fn">with_escape</a>(self, escape: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

The escape character in a row.

- default is None

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.with_double_quote" class="fn">with_double_quote</a>(self, double_quote: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

Set true to indicate that the CSV quotes should be doubled.

- default to true

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.with_newlines_in_values" class="fn">with_newlines_in_values</a>(self, newlines_in_values: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

Specifies whether newlines in (quoted) values are supported.

Parsing newlines in quoted values may be affected by execution behaviour such as parallel file scanning. Setting this to `true` ensures that newlines in values are parsed successfully, which may reduce performance.

The default behaviour depends on the `datafusion.catalog.newlines_in_values` setting.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.with_file_compression_type" class="fn">with_file_compression_type</a>( self, compression: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/parsers/enum.CompressionTypeVariant.html" class="enum" title="enum datafusion::common::parsers::CompressionTypeVariant">CompressionTypeVariant</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

Set a `CompressionTypeVariant` of CSV

- defaults to `CompressionTypeVariant::UNCOMPRESSED`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.delimiter" class="fn">delimiter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

The delimiter character.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.quote" class="fn">quote</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

The quote character.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.terminator" class="fn">terminator</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

The terminator character.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.escape" class="fn">escape</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

The escape character.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#impl-Clone-for-CsvOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#impl-ConfigField-for-CsvOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#impl-Debug-for-CsvOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#impl-Default-for-CsvOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#impl-PartialEq-for-CsvOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#impl-TryFrom%3C%26CsvOptions%3E-for-CsvWriterOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/csv_writer/struct.CsvWriterOptions.html" class="struct" title="struct datafusion::common::file_options::csv_writer::CsvWriterOptions">CsvWriterOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(value: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/csv_writer/struct.CsvWriterOptions.html" class="struct" title="struct datafusion::common::file_options::csv_writer::CsvWriterOptions">CsvWriterOptions</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#impl-StructuralPartialEq-for-CsvOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html#blanket-implementations" class="anchor">§</a>
