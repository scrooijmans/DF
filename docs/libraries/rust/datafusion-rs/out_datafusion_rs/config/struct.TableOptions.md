# Struct TableOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#1615" class="src">Source</a>

``` rust
pub struct TableOptions {
    pub csv: CsvOptions,
    pub parquet: TableParquetOptions,
    pub json: JsonOptions,
    pub current_format: Option<ConfigFileType>,
    pub extensions: Extensions,
}
```

Expand description

Represents the configuration options available for handling different table formats within a data processing application. This struct encompasses options for various file formats including CSV, Parquet, and JSON, allowing for flexible configuration of parsing and writing behaviors specific to each format. Additionally, it supports extending functionality through custom extensions.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#structfield.csv" class="anchor field">§</a>`csv: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions"><code>CsvOptions</code></a>

Configuration options for CSV file handling. This includes settings like the delimiter, quote character, and whether the first row is considered as headers.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#structfield.parquet" class="anchor field">§</a>`parquet: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions"><code>TableParquetOptions</code></a>

Configuration options for Parquet file handling. This includes settings for compression, encoding, and other Parquet-specific file characteristics.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#structfield.json" class="anchor field">§</a>`json: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions"><code>JsonOptions</code></a>

Configuration options for JSON file handling.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#structfield.current_format" class="anchor field">§</a>`current_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html" class="enum" title="enum datafusion::config::ConfigFileType"><code>ConfigFileType</code></a>`>`

The current file format that the table operations should assume. This option allows for dynamic switching between the supported file types (e.g., CSV, Parquet, JSON).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#structfield.extensions" class="anchor field">§</a>`extensions: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions"><code>Extensions</code></a>

Optional extensions that can be used to extend or customize the behavior of the table options. Extensions can be registered using `Extensions::insert` and might include custom file handling logic, additional configuration parameters, or other enhancements.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#impl-TableOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Constructs a new instance of `TableOptions` with default settings.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#returns" class="doc-anchor">§</a>Returns

A new `TableOptions` instance with default configuration values.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.default_from_session_config" class="fn">default_from_session_config</a>(config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Creates a new `TableOptions` instance initialized with settings from a given session config.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#parameters" class="doc-anchor">§</a>Parameters

- `config`: A reference to the session `ConfigOptions` from which to derive initial settings.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#returns-1" class="doc-anchor">§</a>Returns

A new `TableOptions` instance with settings applied from the session config.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.combine_with_session_config" class="fn">combine_with_session_config</a>( &self, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Updates the current `TableOptions` with settings from a given session config.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#parameters-1" class="doc-anchor">§</a>Parameters

- `config`: A reference to the session `ConfigOptions` whose settings are to be applied.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#returns-2" class="doc-anchor">§</a>Returns

A new `TableOptions` instance with updated settings from the session config.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.set_config_format" class="fn">set_config_format</a>(&mut self, format: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html" class="enum" title="enum datafusion::config::ConfigFileType">ConfigFileType</a>)

Sets the file format for the table.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#parameters-2" class="doc-anchor">§</a>Parameters

- `format`: The file format to use (e.g., CSV, Parquet).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.with_extensions" class="fn">with_extensions</a>(self, extensions: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions">Extensions</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Sets the extensions for this `TableOptions` instance.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#parameters-3" class="doc-anchor">§</a>Parameters

- `extensions`: The `Extensions` instance to set.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#returns-3" class="doc-anchor">§</a>Returns

A new `TableOptions` instance with the specified extensions applied.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Sets a specific configuration option.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#parameters-4" class="doc-anchor">§</a>Parameters

- `key`: The configuration key (e.g., “format.delimiter”).
- `value`: The value to set for the specified key.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#returns-4" class="doc-anchor">§</a>Returns

A result indicating success or failure in setting the configuration option.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.from_string_hash_map" class="fn">from_string_hash_map</a>( settings: &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Initializes a new `TableOptions` from a hash map of string settings.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#parameters-5" class="doc-anchor">§</a>Parameters

- `settings`: A hash map where each key-value pair represents a configuration setting.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#returns-5" class="doc-anchor">§</a>Returns

A result containing the new `TableOptions` instance or an error if any setting could not be applied.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.alter_with_string_hash_map" class="fn">alter_with_string_hash_map</a>( &mut self, settings: &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Modifies the current `TableOptions` instance with settings from a hash map.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#parameters-6" class="doc-anchor">§</a>Parameters

- `settings`: A hash map where each key-value pair represents a configuration setting.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#returns-6" class="doc-anchor">§</a>Returns

A result indicating success or failure in applying the settings.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.entries" class="fn">entries</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigEntry.html" class="struct" title="struct datafusion::config::ConfigEntry">ConfigEntry</a>\>

Retrieves all configuration entries from this `TableOptions`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#returns-7" class="doc-anchor">§</a>Returns

A vector of `ConfigEntry` instances, representing all the configuration options within this `TableOptions`.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#impl-Clone-for-TableOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#impl-ConfigField-for-TableOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, \_key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

Visits configuration settings for the current file format, or all formats if none is selected.

This method adapts the behavior based on whether a file format is currently selected in `current_format`. If a format is selected, it visits only the settings relevant to that format. Otherwise, it visits all available format settings.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.set-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Sets a configuration value for a specific key within `TableOptions`.

This method delegates setting configuration values to the specific file format configurations, based on the current format selected. If no format is selected, it returns an error.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#parameters-7" class="doc-anchor">§</a>Parameters

- `key`: The configuration key specifying which setting to adjust, prefixed with the format (e.g., “format.delimiter”) for CSV format.
- `value`: The value to set for the specified configuration key.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#returns-8" class="doc-anchor">§</a>Returns

A result indicating success or an error if the key is not recognized, if a format is not specified, or if setting the configuration value fails for the specific format.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#impl-Debug-for-TableOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#impl-Default-for-TableOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html#blanket-implementations" class="anchor">§</a>
