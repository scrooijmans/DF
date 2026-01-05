# Struct ParserOptions Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/planner.rs.html#45" class="src">Source</a>

``` rust
pub struct ParserOptions {
    pub parse_float_as_decimal: bool,
    pub enable_ident_normalization: bool,
    pub support_varchar_with_length: bool,
    pub enable_options_value_normalization: bool,
    pub collect_spans: bool,
    pub map_string_types_to_utf8view: bool,
    pub default_null_ordering: NullOrdering,
}
```

Expand description

SQL parser options

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#structfield.parse_float_as_decimal" class="anchor field">§</a>`parse_float_as_decimal: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether to parse float as decimal.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#structfield.enable_ident_normalization" class="anchor field">§</a>`enable_ident_normalization: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether to normalize identifiers.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#structfield.support_varchar_with_length" class="anchor field">§</a>`support_varchar_with_length: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether to support varchar with length.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#structfield.enable_options_value_normalization" class="anchor field">§</a>`enable_options_value_normalization: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether to normalize options value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#structfield.collect_spans" class="anchor field">§</a>`collect_spans: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether to collect spans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#structfield.map_string_types_to_utf8view" class="anchor field">§</a>`map_string_types_to_utf8view: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether string types (VARCHAR, CHAR, Text, and String) are mapped to `Utf8View` during SQL planning.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#structfield.default_null_ordering" class="anchor field">§</a>`default_null_ordering: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/enum.NullOrdering.html" class="enum" title="enum datafusion::sql::planner::NullOrdering"><code>NullOrdering</code></a>

Default null ordering for sorting expressions.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#impl-ParserOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Creates a new `ParserOptions` instance with default values.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#examples" class="doc-anchor">§</a>Examples

``` rust
use datafusion_sql::planner::ParserOptions;
let opts = ParserOptions::new();
assert_eq!(opts.parse_float_as_decimal, false);
assert_eq!(opts.enable_ident_normalization, true);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.with_parse_float_as_decimal" class="fn">with_parse_float_as_decimal</a>(self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Sets the `parse_float_as_decimal` option.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
use datafusion_sql::planner::ParserOptions;
let opts = ParserOptions::new().with_parse_float_as_decimal(true);
assert_eq!(opts.parse_float_as_decimal, true);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.with_enable_ident_normalization" class="fn">with_enable_ident_normalization</a>(self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Sets the `enable_ident_normalization` option.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
use datafusion_sql::planner::ParserOptions;
let opts = ParserOptions::new().with_enable_ident_normalization(false);
assert_eq!(opts.enable_ident_normalization, false);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.with_support_varchar_with_length" class="fn">with_support_varchar_with_length</a>(self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Sets the `support_varchar_with_length` option.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.with_map_string_types_to_utf8view" class="fn">with_map_string_types_to_utf8view</a>(self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Sets the `map_string_types_to_utf8view` option.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.with_enable_options_value_normalization" class="fn">with_enable_options_value_normalization</a>( self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Sets the `enable_options_value_normalization` option.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.with_collect_spans" class="fn">with_collect_spans</a>(self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Sets the `collect_spans` option.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.with_default_null_ordering" class="fn">with_default_null_ordering</a>(self, value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/enum.NullOrdering.html" class="enum" title="enum datafusion::sql::planner::NullOrdering">NullOrdering</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Sets the `default_null_ordering` option.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#impl-Clone-for-ParserOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#impl-Debug-for-ParserOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#impl-Default-for-ParserOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#impl-From%3C%26SqlParserOptions%3E-for-ParserOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(options: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#impl-Copy-for-ParserOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html#blanket-implementations" class="anchor">§</a>
