# Struct ConfigOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#967" class="src">Source</a>

``` rust
#[non_exhaustive]pub struct ConfigOptions {
    pub catalog: CatalogOptions,
    pub execution: ExecutionOptions,
    pub optimizer: OptimizerOptions,
    pub sql_parser: SqlParserOptions,
    pub explain: ExplainOptions,
    pub extensions: Extensions,
    pub format: FormatOptions,
}
```

Expand description

Configuration options struct, able to store both built-in configuration and custom options

## Fields (Non-exhaustive)<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#fields" class="anchor">§</a>

This struct is marked as non-exhaustive

Non-exhaustive structs could have additional fields added in future. Therefore, non-exhaustive structs cannot be constructed in external crates using the traditional `Struct { .. }` syntax; cannot be matched against without a wildcard `..`; and struct update syntax will not work.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#structfield.catalog" class="anchor field">§</a>`catalog: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html" class="struct" title="struct datafusion::config::CatalogOptions"><code>CatalogOptions</code></a>

Catalog options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#structfield.execution" class="anchor field">§</a>`execution: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html" class="struct" title="struct datafusion::config::ExecutionOptions"><code>ExecutionOptions</code></a>

Execution options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#structfield.optimizer" class="anchor field">§</a>`optimizer: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.OptimizerOptions.html" class="struct" title="struct datafusion::config::OptimizerOptions"><code>OptimizerOptions</code></a>

Optimizer options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#structfield.sql_parser" class="anchor field">§</a>`sql_parser: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions"><code>SqlParserOptions</code></a>

SQL parser options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#structfield.explain" class="anchor field">§</a>`explain: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExplainOptions.html" class="struct" title="struct datafusion::config::ExplainOptions"><code>ExplainOptions</code></a>

Explain options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#structfield.extensions" class="anchor field">§</a>`extensions: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions"><code>Extensions</code></a>

Optional extensions registered using [`Extensions::insert`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#method.insert "method datafusion::config::Extensions::insert")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#structfield.format" class="anchor field">§</a>`format: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions"><code>FormatOptions</code></a>

Formatting options when printing batches

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#impl-ConfigOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

Creates a new [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions") with default values

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.with_extensions" class="fn">with_extensions</a>(self, extensions: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions">Extensions</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

Set extensions to provided value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Set a configuration option

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.from_env" class="fn">from_env</a>() -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create new [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions"), taking values from environment variables where possible.

For example, to configure `datafusion.execution.batch_size` ([`ExecutionOptions::batch_size`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.batch_size "field datafusion::config::ExecutionOptions::batch_size")) you would set the `DATAFUSION_EXECUTION_BATCH_SIZE` environment variable.

The name of the environment variable is the option’s key, transformed to uppercase and with periods replaced with underscores.

Values are parsed according to the [same rules used in casts from Utf8](https://docs.rs/arrow/latest/arrow/compute/kernels/cast/fn.cast.html).

If the value in the environment variable cannot be cast to the type of the configuration option, the default value will be used instead and a warning emitted. Environment variables are read when this method is called, and are not re-read later.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.from_string_hash_map" class="fn">from_string_hash_map</a>( settings: &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create new ConfigOptions struct, taking values from a string hash map.

Only the built-in configurations will be extracted from the hash map and other key value pairs will be ignored.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.entries" class="fn">entries</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigEntry.html" class="struct" title="struct datafusion::config::ConfigEntry">ConfigEntry</a>\>

Returns the [`ConfigEntry`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigEntry.html "struct datafusion::config::ConfigEntry") stored within this [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.generate_config_markdown" class="fn">generate_config_markdown</a>() -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Generate documentation that can be included in the user guide

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#impl-Clone-for-ConfigOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#impl-ConfigField-for-ConfigOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.set-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, \_key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#impl-Debug-for-ConfigOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#impl-Default-for-ConfigOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#impl-From%3CConfigOptions%3E-for-SessionConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#blanket-implementations" class="anchor">§</a>
