# Struct SqlParserOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#240-292" class="src">Source</a>

``` rust
pub struct SqlParserOptions {
    pub parse_float_as_decimal: bool,
    pub enable_ident_normalization: bool,
    pub enable_options_value_normalization: bool,
    pub dialect: String,
    pub support_varchar_with_length: bool,
    pub map_string_types_to_utf8view: bool,
    pub collect_spans: bool,
    pub recursion_limit: usize,
    pub default_null_ordering: String,
}
```

Expand description

Options related to SQL parser

See also: [`SessionConfig`](https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html)

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#structfield.parse_float_as_decimal" class="anchor field">§</a>`parse_float_as_decimal: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

When set to true, SQL parser will parse float as decimal type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#structfield.enable_ident_normalization" class="anchor field">§</a>`enable_ident_normalization: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

When set to true, SQL parser will normalize ident (convert ident to lowercase when not quoted)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#structfield.enable_options_value_normalization" class="anchor field">§</a>`enable_options_value_normalization: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

When set to true, SQL parser will normalize options value (convert value to lowercase). Note that this option is ignored and will be removed in the future. All case-insensitive values are normalized automatically.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#structfield.dialect" class="anchor field">§</a>`dialect: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Configure the SQL dialect used by DataFusion’s parser; supported values include: Generic, MySQL, PostgreSQL, Hive, SQLite, Snowflake, Redshift, MsSQL, ClickHouse, BigQuery, Ansi, DuckDB and Databricks.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#structfield.support_varchar_with_length" class="anchor field">§</a>`support_varchar_with_length: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

If true, permit lengths for `VARCHAR` such as `VARCHAR(20)`, but ignore the length. If false, error if a `VARCHAR` with a length is specified. The Arrow type system does not have a notion of maximum string length and thus DataFusion can not enforce such limits.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#structfield.map_string_types_to_utf8view" class="anchor field">§</a>`map_string_types_to_utf8view: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

If true, string types (VARCHAR, CHAR, Text, and String) are mapped to `Utf8View` during SQL planning. If false, they are mapped to `Utf8`. Default is true.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#structfield.collect_spans" class="anchor field">§</a>`collect_spans: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

When set to true, the source locations relative to the original SQL query (i.e. [`Span`](https://docs.rs/sqlparser/latest/sqlparser/tokenizer/struct.Span.html)) will be collected and recorded in the logical plan nodes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#structfield.recursion_limit" class="anchor field">§</a>`recursion_limit: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Specifies the recursion depth limit when parsing complex SQL Queries

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#structfield.default_null_ordering" class="anchor field">§</a>`default_null_ordering: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Specifies the default null ordering for query results. There are 4 options:

- `nulls_max`: Nulls appear last in ascending order.
- `nulls_min`: Nulls appear first in ascending order.
- `nulls_first`: Nulls always be first in any order.
- `nulls_last`: Nulls always be last in any order.

By default, `nulls_max` is used to follow Postgres’s behavior. postgres rule: <https://www.postgresql.org/docs/current/queries-order.html>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#impl-Clone-for-SqlParserOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#impl-ConfigField-for-SqlParserOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#impl-Debug-for-SqlParserOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#impl-Default-for-SqlParserOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#impl-From%3C%26SqlParserOptions%3E-for-ParserOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(options: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#impl-PartialEq-for-SqlParserOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#impl-StructuralPartialEq-for-SqlParserOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html#blanket-implementations" class="anchor">§</a>
