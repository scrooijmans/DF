# Struct SqliteDialect Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/unparser/dialect.rs.html#467" class="src">Source</a>

``` rust
pub struct SqliteDialect {}
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#impl-Dialect-for-SqliteDialect" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html" class="trait" title="trait datafusion::sql::unparser::dialect::Dialect">Dialect</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html" class="struct" title="struct datafusion::sql::unparser::dialect::SqliteDialect">SqliteDialect</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.identifier_quote_style" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#tymethod.identifier_quote_style" class="fn">identifier_quote_style</a>(&self, \_: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>\>

Return the character used to quote identifiers.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.date_field_extract_style" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.date_field_extract_style" class="fn">date_field_extract_style</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/enum.DateFieldExtractStyle.html" class="enum" title="enum datafusion::sql::unparser::dialect::DateFieldExtractStyle">DateFieldExtractStyle</a>

The date field extract style to use: `DateFieldExtractStyle`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.date32_cast_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.date32_cast_dtype" class="fn">date32_cast_dtype</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType">DataType</a>

The SQL type to use for Arrow Date32 unparsing Most dialects use Date, but some, like SQLite require TEXT

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.character_length_style" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.character_length_style" class="fn">character_length_style</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/enum.CharacterLengthStyle.html" class="enum" title="enum datafusion::sql::unparser::dialect::CharacterLengthStyle">CharacterLengthStyle</a>

The character length extraction style to use: `CharacterLengthStyle`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.supports_column_alias_in_table_alias" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.supports_column_alias_in_table_alias" class="fn">supports_column_alias_in_table_alias</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect support specifying column aliases as part of alias table definition? (SELECT col1, col2 from my_table) AS my_table_alias(col1_alias, col2_alias)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.scalar_function_to_sql_overrides" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.scalar_function_to_sql_overrides" class="fn">scalar_function_to_sql_overrides</a>( &self, unparser: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html" class="struct" title="struct datafusion::sql::unparser::Unparser">Unparser</a>\<'\_\>, func_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Allows the dialect to override scalar function unparsing if the dialect has specific rules. Returns None if the default unparsing should be used, or Some(ast::Expr) if there is a custom implementation for the function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.supports_nulls_first_in_sort" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.supports_nulls_first_in_sort" class="fn">supports_nulls_first_in_sort</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect support specifying `NULLS FIRST/LAST` in `ORDER BY` clauses?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.use_timestamp_for_date64" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.use_timestamp_for_date64" class="fn">use_timestamp_for_date64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the dialect use TIMESTAMP to represent Date64 rather than DATETIME? E.g. Trino, Athena and Dremio does not have DATETIME data type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.interval_style" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.interval_style" class="fn">interval_style</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/enum.IntervalStyle.html" class="enum" title="enum datafusion::sql::unparser::dialect::IntervalStyle">IntervalStyle</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.float64_ast_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.float64_ast_dtype" class="fn">float64_ast_dtype</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType">DataType</a>

Does the dialect use DOUBLE PRECISION to represent Float64 rather than DOUBLE? E.g. Postgres uses DOUBLE PRECISION instead of DOUBLE

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.utf8_cast_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.utf8_cast_dtype" class="fn">utf8_cast_dtype</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType">DataType</a>

The SQL type to use for Arrow Utf8 unparsing Most dialects use VARCHAR, but some, like MySQL, require CHAR

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.large_utf8_cast_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.large_utf8_cast_dtype" class="fn">large_utf8_cast_dtype</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType">DataType</a>

The SQL type to use for Arrow LargeUtf8 unparsing Most dialects use TEXT, but some, like MySQL, require CHAR

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.int64_cast_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.int64_cast_dtype" class="fn">int64_cast_dtype</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType">DataType</a>

The SQL type to use for Arrow Int64 unparsing Most dialects use BigInt, but some, like MySQL, require SIGNED

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.int32_cast_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.int32_cast_dtype" class="fn">int32_cast_dtype</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType">DataType</a>

The SQL type to use for Arrow Int32 unparsing Most dialects use Integer, but some, like MySQL, require SIGNED

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.timestamp_cast_dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.timestamp_cast_dtype" class="fn">timestamp_cast_dtype</a>( &self, \_time_unit: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum datafusion::common::arrow::datatypes::TimeUnit">TimeUnit</a>, tz: &<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType">DataType</a>

The SQL type to use for Timestamp unparsing Most dialects use Timestamp, but some, like MySQL, require Datetime Some dialects like Dremio does not support WithTimeZone and requires always Timestamp

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.requires_derived_table_alias" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.requires_derived_table_alias" class="fn">requires_derived_table_alias</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether the dialect requires a table alias for any subquery in the FROM clause This affects behavior when deriving logical plans for Sort, Limit, etc.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.division_operator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.division_operator" class="fn">division_operator</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BinaryOperator.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BinaryOperator">BinaryOperator</a>

The division operator for the dialect Most dialect uses ` BinaryOperator::Divide` (/) But DuckDB dialect uses `BinaryOperator::DuckIntegerDivide` (//)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.window_func_support_window_frame" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.window_func_support_window_frame" class="fn">window_func_support_window_frame</a>( &self, \_func_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_start_bound: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.WindowFrameBound.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::WindowFrameBound">WindowFrameBound</a>, \_end_bound: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.WindowFrameBound.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::WindowFrameBound">WindowFrameBound</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Allows the dialect to choose to omit window frame in unparsing based on function name and window frame bound Returns false if specific function name / window frame bound indicates no window frame is needed in unparsing

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.with_custom_scalar_overrides" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.with_custom_scalar_overrides" class="fn">with_custom_scalar_overrides</a>( self, \_handlers: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html" class="struct" title="struct datafusion::sql::unparser::Unparser">Unparser</a>\<'\_\>, &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>)\>, ) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Extends the dialect’s default rules for unparsing scalar functions. This is useful for supporting application-specific UDFs or custom engine extensions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.full_qualified_col" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.full_qualified_col" class="fn">full_qualified_col</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Allow to unparse a qualified column with a full qualified name (e.g. catalog_name.schema_name.table_name.column_name) Otherwise, the column will be unparsed with only the table name and column name (e.g. table_name.column_name)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.unnest_as_table_factor" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.unnest_as_table_factor" class="fn">unnest_as_table_factor</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Allow to unparse the unnest plan as [ast::TableFactor::UNNEST](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TableFactor.html#variant.UNNEST "variant datafusion::logical_expr::sqlparser::ast::TableFactor::UNNEST"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.unnest_as_table_factor)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#method.col_alias_overrides" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html#method.col_alias_overrides" class="fn">col_alias_overrides</a>( &self, \_alias: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Allows the dialect to override column alias unparsing if the dialect has specific rules. Returns None if the default unparsing should be used, or Some(String) if there is a custom implementation for the alias.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/struct.SqliteDialect.html#blanket-implementations" class="anchor">§</a>
