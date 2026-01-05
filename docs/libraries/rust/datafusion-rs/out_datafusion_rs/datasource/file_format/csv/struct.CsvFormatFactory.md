# Struct CsvFormatFactory Copy item path

<a href="https://docs.rs/datafusion-datasource-csv/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource_csv/file_format.rs.html#67" class="src">Source</a>

``` rust
pub struct CsvFormatFactory {
    pub options: Option<CsvOptions>,
}
```

Expand description

Factory used to create [`CsvFormat`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormat.html "struct datafusion::datasource::file_format::csv::CsvFormat")

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#structfield.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions"><code>CsvOptions</code></a>`>`

the options for csv file read

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#impl-CsvFormatFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvFormatFactory">CsvFormatFactory</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvFormatFactory">CsvFormatFactory</a>

Creates an instance of [`CsvFormatFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html "struct datafusion::datasource::file_format::csv::CsvFormatFactory")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#method.new_with_options" class="fn">new_with_options</a>(options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvFormatFactory">CsvFormatFactory</a>

Creates an instance of [`CsvFormatFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html "struct datafusion::datasource::file_format::csv::CsvFormatFactory") with customized default options

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#impl-Debug-for-CsvFormatFactory" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvFormatFactory">CsvFormatFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#impl-Default-for-CsvFormatFactory" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvFormatFactory">CsvFormatFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvFormatFactory">CsvFormatFactory</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#impl-FileFormatFactory-for-CsvFormatFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html" class="trait" title="trait datafusion::datasource::file_format::FileFormatFactory">FileFormatFactory</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvFormatFactory">CsvFormatFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#method.create" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html#tymethod.create" class="fn">create</a>( &self, state: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, format_options: &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormat.html" class="trait" title="trait datafusion::datasource::file_format::FileFormat">FileFormat</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Initialize a [FileFormat](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormat.html "trait datafusion::datasource::file_format::FileFormat") and configure based on session and command level options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#method.default-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html#tymethod.default" class="fn">default</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormat.html" class="trait" title="trait datafusion::datasource::file_format::FileFormat">FileFormat</a>\>

Initialize a [FileFormat](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormat.html "trait datafusion::datasource::file_format::FileFormat") with all options set to default values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the table source as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#impl-GetExt-for-CsvFormatFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html" class="trait" title="trait datafusion::common::GetExt">GetExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvFormatFactory">CsvFormatFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#method.get_ext" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html#tymethod.get_ext" class="fn">get_ext</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

File extension getter

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html#blanket-implementations" class="anchor">§</a>
