# Struct BigQueryDialect Copy item path

<a href="https://docs.rs/sqlparser/0.58.0/x86_64-unknown-linux-gnu/src/sqlparser/dialect/bigquery.rs.html#45" class="src">Source</a>

``` rust
pub struct BigQueryDialect;
```

Expand description

A [`Dialect`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html "trait datafusion::logical_expr::sqlparser::dialect::Dialect") for [Google Bigquery](https://cloud.google.com/bigquery/)

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#impl-Debug-for-BigQueryDialect" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html" class="struct" title="struct datafusion::logical_expr::sqlparser::dialect::BigQueryDialect">BigQueryDialect</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#impl-Default-for-BigQueryDialect" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html" class="struct" title="struct datafusion::logical_expr::sqlparser::dialect::BigQueryDialect">BigQueryDialect</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html" class="struct" title="struct datafusion::logical_expr::sqlparser::dialect::BigQueryDialect">BigQueryDialect</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#impl-Dialect-for-BigQueryDialect" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html" class="trait" title="trait datafusion::logical_expr::sqlparser::dialect::Dialect">Dialect</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html" class="struct" title="struct datafusion::logical_expr::sqlparser::dialect::BigQueryDialect">BigQueryDialect</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.is_delimited_identifier_start" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_delimited_identifier_start" class="fn">is_delimited_identifier_start</a>(&self, ch: <a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

See <https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#identifiers>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_column_definition_trailing_commas" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_column_definition_trailing_commas" class="fn">supports_column_definition_trailing_commas</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

See <https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_table_statement>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_triple_quoted_string" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_triple_quoted_string" class="fn">supports_triple_quoted_string</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

See [doc](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_window_function_null_treatment_arg" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_window_function_null_treatment_arg" class="fn">supports_window_function_null_treatment_arg</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

See [doc](https://cloud.google.com/bigquery/docs/reference/standard-sql/navigation_functions#first_value)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_window_clause_named_window_reference" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_window_clause_named_window_reference" class="fn">supports_window_clause_named_window_reference</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

See [doc](https://cloud.google.com/bigquery/docs/reference/standard-sql/window-function-calls#ref_named_window)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_parenthesized_set_variables" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_parenthesized_set_variables" class="fn">supports_parenthesized_set_variables</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

See [doc](https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#set)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_select_expr_star" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_select_expr_star" class="fn">supports_select_expr_star</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

See <https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_expression_star>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_execute_immediate" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_execute_immediate" class="fn">supports_execute_immediate</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

See <https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#execute_immediate>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.parse_statement" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.parse_statement" class="fn">parse_statement</a>( &self, parser: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html" class="struct" title="struct datafusion::logical_expr::sqlparser::parser::Parser">Parser</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/enum.ParserError.html" class="enum" title="enum datafusion::logical_expr::sqlparser::parser::ParserError">ParserError</a>\>\>

Dialect-specific statement parser override [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.parse_statement)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_projection_trailing_commas" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_projection_trailing_commas" class="fn">supports_projection_trailing_commas</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect support trailing commas in the projection list?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.is_identifier_start" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#tymethod.is_identifier_start" class="fn">is_identifier_start</a>(&self, ch: <a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determine if a character is a valid start character for an unquoted identifier

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.is_identifier_part" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#tymethod.is_identifier_part" class="fn">is_identifier_part</a>(&self, ch: <a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determine if a character is a valid unquoted identifier character

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_string_literal_backslash_escape" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_string_literal_backslash_escape" class="fn">supports_string_literal_backslash_escape</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determine if the dialect supports escaping characters via ’' in string literals. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_string_literal_backslash_escape)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_select_wildcard_except" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_select_wildcard_except" class="fn">supports_select_wildcard_except</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports an `EXCEPT` clause following a wildcard in a select list. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_select_wildcard_except)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.require_interval_qualifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.require_interval_qualifier" class="fn">require_interval_qualifier</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether `INTERVAL` expressions require units (called “qualifiers” in the ANSI SQL spec) to be specified, e.g. `INTERVAL 1 DAY` vs `INTERVAL 1`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.require_interval_qualifier)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_struct_literal" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_struct_literal" class="fn">supports_struct_literal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if the dialect supports the STRUCT literal [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_struct_literal)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_timestamp_versioning" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_timestamp_versioning" class="fn">supports_timestamp_versioning</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect supports querying historical table data by specifying which version of the data to query.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_group_by_expr" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_group_by_expr" class="fn">supports_group_by_expr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialects supports `group sets, roll up, or cube` expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.is_column_alias" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_column_alias" class="fn">is_column_alias</a>(&self, kw: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/keywords/enum.Keyword.html" class="enum" title="enum datafusion::logical_expr::sqlparser::dialect::keywords::Keyword">Keyword</a>, \_parser: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html" class="struct" title="struct datafusion::logical_expr::sqlparser::parser::Parser">Parser</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the specified keyword should be parsed as a column identifier. See [keywords::RESERVED_FOR_COLUMN_ALIAS](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/keywords/constant.RESERVED_FOR_COLUMN_ALIAS.html "constant datafusion::logical_expr::sqlparser::dialect::keywords::RESERVED_FOR_COLUMN_ALIAS")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_pipe_operator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_pipe_operator" class="fn">supports_pipe_operator</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if the dialect supports pipe operator. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_pipe_operator)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_create_table_multi_schema_info_sources" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_create_table_multi_schema_info_sources" class="fn">supports_create_table_multi_schema_info_sources</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returne true if the dialect supports specifying multiple options in a `CREATE TABLE` statement for the structure of the new table. For example: `CREATE TABLE t (a INT, b INT) AS SELECT 1 AS b, 2 AS a`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.dialect" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.dialect" class="fn">dialect</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html" class="struct" title="struct core::any::TypeId">TypeId</a>

Determine the [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") of this dialect. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.dialect)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.is_nested_delimited_identifier_start" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_nested_delimited_identifier_start" class="fn">is_nested_delimited_identifier_start</a>(&self, \_ch: <a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determine if a character starts a potential nested quoted identifier. Example: RedShift supports the following quote styles to all mean the same thing: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_nested_delimited_identifier_start)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.peek_nested_delimited_identifier_quotes" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.peek_nested_delimited_identifier_quotes" class="fn">peek_nested_delimited_identifier_quotes</a>( &self, \_chars: <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html" class="struct" title="struct core::iter::adapters::peekable::Peekable">Peekable</a>\<<a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html" class="struct" title="struct core::str::iter::Chars">Chars</a>\<'\_\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>\>)\>

Only applicable whenever [`Self::is_nested_delimited_identifier_start`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_nested_delimited_identifier_start "method sqlparser::dialect::Dialect::is_nested_delimited_identifier_start::is_nested_delimited_identifier_start") returns true If the next sequence of tokens potentially represent a nested identifier, then this method returns a tuple containing the outer quote style, and if present, the inner (nested) quote style. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.peek_nested_delimited_identifier_quotes)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.identifier_quote_style" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.identifier_quote_style" class="fn">identifier_quote_style</a>(&self, \_identifier: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>\>

Return the character used to quote identifiers.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.is_custom_operator_part" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_custom_operator_part" class="fn">is_custom_operator_part</a>(&self, \_ch: <a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Most dialects do not have custom operators. Override this method to provide custom operators.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.ignores_wildcard_escapes" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.ignores_wildcard_escapes" class="fn">ignores_wildcard_escapes</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determine whether the dialect strips the backslash when escaping LIKE wildcards (%, \_). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.ignores_wildcard_escapes)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_unicode_string_literal" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_unicode_string_literal" class="fn">supports_unicode_string_literal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determine if the dialect supports string literals with `U&` prefix. This is used to specify Unicode code points in string literals. For example, in PostgreSQL, the following is a valid string literal: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_unicode_string_literal)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_filter_during_aggregation" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_filter_during_aggregation" class="fn">supports_filter_during_aggregation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect support `FILTER (WHERE expr)` for aggregate queries?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_within_after_array_aggregation" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_within_after_array_aggregation" class="fn">supports_within_after_array_aggregation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports `ARRAY_AGG() [WITHIN GROUP (ORDER BY)]` expressions. Otherwise, the dialect should expect an `ORDER BY` without the `WITHIN GROUP` clause, e.g. [`ANSI`](https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#array-aggregate-function)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_group_by_with_modifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_group_by_with_modifier" class="fn">supports_group_by_with_modifier</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialects supports `GROUP BY` modifiers prefixed by a `WITH` keyword. Example: `GROUP BY value WITH ROLLUP`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_left_associative_joins_without_parens" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_left_associative_joins_without_parens" class="fn">supports_left_associative_joins_without_parens</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Indicates whether the dialect supports left-associative join parsing by default when parentheses are omitted in nested joins. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_left_associative_joins_without_parens)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_outer_join_operator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_outer_join_operator" class="fn">supports_outer_join_operator</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports the `(+)` syntax for OUTER JOIN.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_connect_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_connect_by" class="fn">supports_connect_by</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports CONNECT BY.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_match_recognize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_match_recognize" class="fn">supports_match_recognize</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports the MATCH_RECOGNIZE operation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_in_empty_list" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_in_empty_list" class="fn">supports_in_empty_list</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports `(NOT) IN ()` expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_start_transaction_modifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_start_transaction_modifier" class="fn">supports_start_transaction_modifier</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports `BEGIN {DEFERRED | IMMEDIATE | EXCLUSIVE | TRY | CATCH} [TRANSACTION]` statements

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_end_transaction_modifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_end_transaction_modifier" class="fn">supports_end_transaction_modifier</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports `END {TRY | CATCH}` statements

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_named_fn_args_with_eq_operator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_named_fn_args_with_eq_operator" class="fn">supports_named_fn_args_with_eq_operator</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports named arguments of the form `FUN(a = '1', b = '2')`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_named_fn_args_with_colon_operator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_named_fn_args_with_colon_operator" class="fn">supports_named_fn_args_with_colon_operator</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports named arguments of the form `FUN(a : '1', b : '2')`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_named_fn_args_with_assignment_operator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_named_fn_args_with_assignment_operator" class="fn">supports_named_fn_args_with_assignment_operator</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports named arguments of the form `FUN(a := '1', b := '2')`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_named_fn_args_with_rarrow_operator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_named_fn_args_with_rarrow_operator" class="fn">supports_named_fn_args_with_rarrow_operator</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports named arguments of the form `FUN(a => '1', b => '2')`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_named_fn_args_with_expr_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_named_fn_args_with_expr_name" class="fn">supports_named_fn_args_with_expr_name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if dialect supports argument name as arbitrary expression. e.g. `FUN(LOWER('a'):'1', b:'2')` Such function arguments are represented in the AST by the `FunctionArg::ExprNamed` variant, otherwise use the `FunctionArg::Named` variant (compatible reason).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_numeric_prefix" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_numeric_prefix" class="fn">supports_numeric_prefix</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports identifiers starting with a numeric prefix such as tables named `59901_user_login`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_numeric_literal_underscores" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_numeric_literal_underscores" class="fn">supports_numeric_literal_underscores</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports numbers containing underscores, e.g. `10_000_000`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_dictionary_syntax" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_dictionary_syntax" class="fn">supports_dictionary_syntax</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports defining structs or objects using a syntax like `{'x': 1, 'y': 2, 'z': 3}`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.support_map_literal_syntax" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.support_map_literal_syntax" class="fn">support_map_literal_syntax</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports defining object using the syntax like `Map {1: 10, 2: 20}`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_lambda_functions" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_lambda_functions" class="fn">supports_lambda_functions</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports lambda functions, for example: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_lambda_functions)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_comma_separated_set_assignments" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_comma_separated_set_assignments" class="fn">supports_comma_separated_set_assignments</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports multiple `SET` statements in a single statement. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_comma_separated_set_assignments)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.convert_type_before_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.convert_type_before_value" class="fn">convert_type_before_value</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect has a CONVERT function which accepts a type first and an expression second, e.g. `CONVERT(varchar, 1)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.parse_prefix" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.parse_prefix" class="fn">parse_prefix</a>( &self, \_parser: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html" class="struct" title="struct datafusion::logical_expr::sqlparser::parser::Parser">Parser</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/enum.ParserError.html" class="enum" title="enum datafusion::logical_expr::sqlparser::parser::ParserError">ParserError</a>\>\>

Dialect-specific prefix parser override

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_trailing_commas" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_trailing_commas" class="fn">supports_trailing_commas</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect support trailing commas around the query?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_limit_comma" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_limit_comma" class="fn">supports_limit_comma</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect support parsing `LIMIT 1, 2` as `LIMIT 2 OFFSET 1`?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_from_trailing_commas" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_from_trailing_commas" class="fn">supports_from_trailing_commas</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports trailing commas in the `FROM` clause of a `SELECT` statement. Example: `SELECT 1 FROM T, U, LIMIT 1`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_object_name_double_dot_notation" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_object_name_double_dot_notation" class="fn">supports_object_name_double_dot_notation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports double dot notation for object names [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_object_name_double_dot_notation)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_empty_projections" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_empty_projections" class="fn">supports_empty_projections</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if the dialect supports empty projections in SELECT statements [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_empty_projections)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_from_first_select" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_from_first_select" class="fn">supports_from_first_select</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if the dialect supports “FROM-first” selects. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_from_first_select)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_user_host_grantee" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_user_host_grantee" class="fn">supports_user_host_grantee</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect support MySQL-style `'user'@'host'` grantee syntax?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_match_against" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_match_against" class="fn">supports_match_against</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect support the `MATCH() AGAINST()` syntax?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_select_wildcard_exclude" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_select_wildcard_exclude" class="fn">supports_select_wildcard_exclude</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports an exclude option following a wildcard in the projection section. For example: `SELECT * EXCLUDE col1 FROM tbl`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_select_wildcard_exclude)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_select_exclude" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_select_exclude" class="fn">supports_select_exclude</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports an exclude option as the last item in the projection section, not necessarily after a wildcard. For example: `SELECT *, c1, c2 EXCLUDE c3 FROM tbl` [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_select_exclude)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.parse_infix" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.parse_infix" class="fn">parse_infix</a>( &self, \_parser: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html" class="struct" title="struct datafusion::logical_expr::sqlparser::parser::Parser">Parser</a>\<'\_\>, \_expr: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>, \_precedence: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/enum.ParserError.html" class="enum" title="enum datafusion::logical_expr::sqlparser::parser::ParserError">ParserError</a>\>\>

Dialect-specific infix parser override [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.parse_infix)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.get_next_precedence" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.get_next_precedence" class="fn">get_next_precedence</a>( &self, \_parser: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html" class="struct" title="struct datafusion::logical_expr::sqlparser::parser::Parser">Parser</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/enum.ParserError.html" class="enum" title="enum datafusion::logical_expr::sqlparser::parser::ParserError">ParserError</a>\>\>

Dialect-specific precedence override [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.get_next_precedence)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.get_next_precedence_default" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.get_next_precedence_default" class="fn">get_next_precedence_default</a>( &self, parser: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html" class="struct" title="struct datafusion::logical_expr::sqlparser::parser::Parser">Parser</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/enum.ParserError.html" class="enum" title="enum datafusion::logical_expr::sqlparser::parser::ParserError">ParserError</a>\>

Get the precedence of the next token, looking at the full token stream. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.get_next_precedence_default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.parse_column_option" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.parse_column_option" class="fn">parse_column_option</a>( &self, \_parser: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html" class="struct" title="struct datafusion::logical_expr::sqlparser::parser::Parser">Parser</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ColumnOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ColumnOption">ColumnOption</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/enum.ParserError.html" class="enum" title="enum datafusion::logical_expr::sqlparser::parser::ParserError">ParserError</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/enum.ParserError.html" class="enum" title="enum datafusion::logical_expr::sqlparser::parser::ParserError">ParserError</a>\>

Dialect-specific column option parser override [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.parse_column_option)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.prec_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.prec_value" class="fn">prec_value</a>(&self, prec: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/enum.Precedence.html" class="enum" title="enum datafusion::logical_expr::sqlparser::dialect::Precedence">Precedence</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Decide the lexical Precedence of operators. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.prec_value)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.prec_unknown" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.prec_unknown" class="fn">prec_unknown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Returns the precedence when the precedence is otherwise unknown

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.describe_requires_table_keyword" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.describe_requires_table_keyword" class="fn">describe_requires_table_keyword</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect requires the `TABLE` keyword after `DESCRIBE` [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.describe_requires_table_keyword)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.allow_extract_custom" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.allow_extract_custom" class="fn">allow_extract_custom</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect allows the `EXTRACT` function to words other than [`Keyword`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/keywords/enum.Keyword.html "enum datafusion::logical_expr::sqlparser::dialect::keywords::Keyword").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.allow_extract_single_quotes" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.allow_extract_single_quotes" class="fn">allow_extract_single_quotes</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect allows the `EXTRACT` function to use single quotes in the part being extracted.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_dollar_placeholder" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_dollar_placeholder" class="fn">supports_dollar_placeholder</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect allows dollar placeholders e.g. `SELECT $var` (SQLite)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_create_index_with_clause" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_create_index_with_clause" class="fn">supports_create_index_with_clause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect support with clause in create index statement? e.g. `CREATE INDEX idx ON t WITH (key = value, key2)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_explain_with_utility_options" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_explain_with_utility_options" class="fn">supports_explain_with_utility_options</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_asc_desc_in_column_definition" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_asc_desc_in_column_definition" class="fn">supports_asc_desc_in_column_definition</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_factorial_operator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_factorial_operator" class="fn">supports_factorial_operator</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports `a!` expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_nested_comments" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_nested_comments" class="fn">supports_nested_comments</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports nested comments e.g. `/* /* nested */ */`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_eq_alias_assignment" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_eq_alias_assignment" class="fn">supports_eq_alias_assignment</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect supports treating the equals operator `=` within a `SelectItem` as an alias assignment operator, rather than a boolean expression. For example: the following statements are equivalent for such a dialect: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_eq_alias_assignment)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_try_convert" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_try_convert" class="fn">supports_try_convert</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect supports the `TRY_CONVERT` function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_bang_not_operator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_bang_not_operator" class="fn">supports_bang_not_operator</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports `!a` syntax for boolean `NOT` expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_listen_notify" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_listen_notify" class="fn">supports_listen_notify</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports the `LISTEN`, `UNLISTEN` and `NOTIFY` statements

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_load_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_load_data" class="fn">supports_load_data</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports the `LOAD DATA` statement

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_load_extension" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_load_extension" class="fn">supports_load_extension</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports the `LOAD extension` statement

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_top_before_distinct" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_top_before_distinct" class="fn">supports_top_before_distinct</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect expects the `TOP` option before the `ALL`/`DISTINCT` options in a `SELECT` statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_boolean_literals" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_boolean_literals" class="fn">supports_boolean_literals</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports boolean literals (`true` and `false`). For example, in MSSQL these are treated as identifiers rather than boolean literals.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_show_like_before_in" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_show_like_before_in" class="fn">supports_show_like_before_in</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect supports the `LIKE 'pattern'` option in a `SHOW` statement before the `IN` option

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_comment_on" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_comment_on" class="fn">supports_comment_on</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect supports the `COMMENT` statement

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_create_table_select" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_create_table_select" class="fn">supports_create_table_select</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports the `CREATE TABLE SELECT` statement

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_partiql" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_partiql" class="fn">supports_partiql</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports PartiQL for querying semi-structured data <https://partiql.org/index.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.is_reserved_for_identifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_reserved_for_identifier" class="fn">is_reserved_for_identifier</a>(&self, kw: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/keywords/enum.Keyword.html" class="enum" title="enum datafusion::logical_expr::sqlparser::dialect::keywords::Keyword">Keyword</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the specified keyword is reserved and cannot be used as an identifier without special handling like quoting.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.get_reserved_keywords_for_table_factor" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.get_reserved_keywords_for_table_factor" class="fn">get_reserved_keywords_for_table_factor</a>(&self) -\> &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/keywords/enum.Keyword.html" class="enum" title="enum datafusion::logical_expr::sqlparser::dialect::keywords::Keyword">Keyword</a>\]

Returns reserved keywords when looking to parse a `TableFactor`. See [Self::supports_from_trailing_commas](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_from_trailing_commas "method sqlparser::dialect::Dialect::supports_from_trailing_commas::supports_from_trailing_commas")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.get_reserved_keywords_for_select_item_operator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.get_reserved_keywords_for_select_item_operator" class="fn">get_reserved_keywords_for_select_item_operator</a>(&self) -\> &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/keywords/enum.Keyword.html" class="enum" title="enum datafusion::logical_expr::sqlparser::dialect::keywords::Keyword">Keyword</a>\]

Returns reserved keywords that may prefix a select item expression e.g. `SELECT CONNECT_BY_ROOT name FROM Tbl2` (Snowflake)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.get_reserved_grantees_types" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.get_reserved_grantees_types" class="fn">get_reserved_grantees_types</a>(&self) -\> &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.GranteesType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::GranteesType">GranteesType</a>\]

Returns grantee types that should be treated as identifiers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_table_sample_before_alias" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_table_sample_before_alias" class="fn">supports_table_sample_before_alias</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect supports the `TABLESAMPLE` option before the table alias option. For example: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_table_sample_before_alias)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_insert_set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_insert_set" class="fn">supports_insert_set</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect supports the `INSERT INTO ... SET col1 = 1, ...` syntax. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_insert_set)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_insert_table_function" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_insert_table_function" class="fn">supports_insert_table_function</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect support table function in insertion?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_insert_format" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_insert_format" class="fn">supports_insert_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect support insert formats, e.g. `INSERT INTO ... FORMAT <format>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_set_stmt_without_operator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_set_stmt_without_operator" class="fn">supports_set_stmt_without_operator</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect supports `SET` statements without an explicit assignment operator such as `=`. For example: `SET SHOWPLAN_XML ON`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.is_select_item_alias" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_select_item_alias" class="fn">is_select_item_alias</a>( &self, explicit: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, kw: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/keywords/enum.Keyword.html" class="enum" title="enum datafusion::logical_expr::sqlparser::dialect::keywords::Keyword">Keyword</a>, parser: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html" class="struct" title="struct datafusion::logical_expr::sqlparser::parser::Parser">Parser</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the specified keyword should be parsed as a select item alias. When explicit is true, the keyword is preceded by an `AS` word. Parser is provided to enable looking ahead if needed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.is_table_alias" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_table_alias" class="fn">is_table_alias</a>(&self, kw: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/keywords/enum.Keyword.html" class="enum" title="enum datafusion::logical_expr::sqlparser::dialect::keywords::Keyword">Keyword</a>, \_parser: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html" class="struct" title="struct datafusion::logical_expr::sqlparser::parser::Parser">Parser</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the specified keyword should be parsed as a table identifier. See [keywords::RESERVED_FOR_TABLE_ALIAS](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/keywords/constant.RESERVED_FOR_TABLE_ALIAS.html "constant datafusion::logical_expr::sqlparser::dialect::keywords::RESERVED_FOR_TABLE_ALIAS")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.is_table_factor_alias" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_table_factor_alias" class="fn">is_table_factor_alias</a>( &self, explicit: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, kw: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/keywords/enum.Keyword.html" class="enum" title="enum datafusion::logical_expr::sqlparser::dialect::keywords::Keyword">Keyword</a>, parser: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html" class="struct" title="struct datafusion::logical_expr::sqlparser::parser::Parser">Parser</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the specified keyword should be parsed as a table factor alias. When explicit is true, the keyword is preceded by an `AS` word. Parser is provided to enable looking ahead if needed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_string_escape_constant" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_string_escape_constant" class="fn">supports_string_escape_constant</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect supports the E’…’ syntax for string literals [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_string_escape_constant)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_table_hints" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_table_hints" class="fn">supports_table_hints</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports the table hints in the `FROM` clause.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.requires_single_line_comment_whitespace" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.requires_single_line_comment_whitespace" class="fn">requires_single_line_comment_whitespace</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this dialect requires a whitespace character after `--` to start a single line comment. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.requires_single_line_comment_whitespace)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_array_typedef_with_brackets" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_array_typedef_with_brackets" class="fn">supports_array_typedef_with_brackets</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports array type definition with brackets with an optional size. For example: `CREATE TABLE my_table (arr1 INT[], arr2 INT[3])` `SELECT x::INT[]`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_geometric_types" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_geometric_types" class="fn">supports_geometric_types</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports geometric types. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_geometric_types)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_order_by_all" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_order_by_all" class="fn">supports_order_by_all</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports `ORDER BY ALL`. `ALL` which means all columns of the SELECT clause. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_order_by_all)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_set_names" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_set_names" class="fn">supports_set_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports `SET NAMES <charset_name> [COLLATE <collation_name>]`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_set_names)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_space_separated_column_options" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_space_separated_column_options" class="fn">supports_space_separated_column_options</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_alter_column_type_using" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_alter_column_type_using" class="fn">supports_alter_column_type_using</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports the `USING` clause in an `ALTER COLUMN` statement. Example: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_alter_column_type_using)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.supports_comma_separated_drop_column_list" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.supports_comma_separated_drop_column_list" class="fn">supports_comma_separated_drop_column_list</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect supports `ALTER TABLE tbl DROP COLUMN c1, ..., cn`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#method.is_identifier_generating_function_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_identifier_generating_function_name" class="fn">is_identifier_generating_function_name</a>( &self, \_ident: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident">Ident</a>, \_name_parts: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ObjectNamePart.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ObjectNamePart">ObjectNamePart</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the dialect considers the specified ident as a function that returns an identifier. Typically used to generate identifiers programmatically. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html#method.is_identifier_generating_function_name)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.BigQueryDialect.html#blanket-implementations" class="anchor">§</a>
