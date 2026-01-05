# Struct FormatOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#898-922" class="src">Source</a>

``` rust
pub struct FormatOptions {
    pub safe: bool,
    pub null: String,
    pub date_format: Option<String>,
    pub datetime_format: Option<String>,
    pub timestamp_format: Option<String>,
    pub timestamp_tz_format: Option<String>,
    pub time_format: Option<String>,
    pub duration_format: String,
    pub types_info: bool,
}
```

Expand description

Options controlling the format of output when printing record batches Copies [`arrow::util::display::FormatOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/util/display/struct.FormatOptions.html "struct datafusion::common::arrow::util::display::FormatOptions")

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#structfield.safe" class="anchor field">§</a>`safe: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

If set to `true` any formatting errors will be written to the output instead of being converted into a [`std::fmt::Error`](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#structfield.null" class="anchor field">§</a>`null: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Format string for nulls

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#structfield.date_format" class="anchor field">§</a>`date_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Date format for date arrays

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#structfield.datetime_format" class="anchor field">§</a>`datetime_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Format for DateTime arrays

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#structfield.timestamp_format" class="anchor field">§</a>`timestamp_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Timestamp format for timestamp arrays

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#structfield.timestamp_tz_format" class="anchor field">§</a>`timestamp_tz_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Timestamp format for timestamp with timezone arrays. When `None`, ISO 8601 format is used.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#structfield.time_format" class="anchor field">§</a>`time_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Time format for time arrays

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#structfield.duration_format" class="anchor field">§</a>`duration_format: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Duration format. Can be either `"pretty"` or `"ISO8601"`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#structfield.types_info" class="anchor field">§</a>`types_info: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Show types in visual representation batches

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#impl-Clone-for-FormatOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions">FormatOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions">FormatOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#impl-ConfigField-for-FormatOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions">FormatOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#impl-Debug-for-FormatOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions">FormatOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#impl-Default-for-FormatOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions">FormatOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions">FormatOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#impl-PartialEq-for-FormatOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions">FormatOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions">FormatOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#impl-TryInto%3CFormatOptions%3C&#39;a%3E%3E-for-%26FormatOptions" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html" class="trait" title="trait core::convert::TryInto">TryInto</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct datafusion::common::arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions">FormatOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#method.try_into" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into" class="fn">try_into</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct datafusion::common::arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#impl-StructuralPartialEq-for-FormatOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions">FormatOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html#blanket-implementations" class="anchor">§</a>
