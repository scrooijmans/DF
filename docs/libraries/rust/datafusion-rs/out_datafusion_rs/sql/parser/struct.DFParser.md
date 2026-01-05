# Struct DFParser Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/parser.rs.html#291" class="src">Source</a>

``` rust
pub struct DFParser<'a> {
    pub parser: Parser<'a>,
    /* private fields */
}
```

Expand description

DataFusion SQL Parser based on [`sqlparser`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/index.html "mod datafusion::logical_expr::sqlparser")

Parses DataFusion’s SQL dialect, often delegating to [`sqlparser`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/index.html "mod datafusion::logical_expr::sqlparser")’s [`Parser`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html "struct datafusion::logical_expr::sqlparser::parser::Parser").

DataFusion mostly follows existing SQL dialects via `sqlparser`. However, certain statements such as `COPY` and `CREATE EXTERNAL TABLE` have special syntax in DataFusion. See [`Statement`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html "enum datafusion::sql::parser::Statement") for a list of this special syntax

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#structfield.parser" class="anchor field">§</a>`parser: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html" class="struct" title="struct datafusion::logical_expr::sqlparser::parser::Parser"><code>Parser</code></a>`<'a>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#impl-DFParser%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html" class="struct" title="struct datafusion::sql::parser::DFParser">DFParser</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.new" class="fn">new</a>(sql: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html" class="struct" title="struct datafusion::sql::parser::DFParser">DFParser</a>\<'a\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 46.0.0: DFParserBuilder

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.new_with_dialect" class="fn">new_with_dialect</a>( sql: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, dialect: &'a (dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html" class="trait" title="trait datafusion::logical_expr::sqlparser::dialect::Dialect">Dialect</a> + 'static), ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html" class="struct" title="struct datafusion::sql::parser::DFParser">DFParser</a>\<'a\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 46.0.0: DFParserBuilder

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_sql" class="fn">parse_sql</a>(sql: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html" class="struct" title="struct alloc::collections::vec_deque::VecDeque">VecDeque</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse a sql string into one or [`Statement`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html "enum datafusion::sql::parser::Statement")s using the [`GenericDialect`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.GenericDialect.html "struct datafusion::logical_expr::sqlparser::dialect::GenericDialect").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_sql_with_dialect" class="fn">parse_sql_with_dialect</a>( sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, dialect: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html" class="trait" title="trait datafusion::logical_expr::sqlparser::dialect::Dialect">Dialect</a> + 'static), ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html" class="struct" title="struct alloc::collections::vec_deque::VecDeque">VecDeque</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse a SQL string and produce one or more [`Statement`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html "enum datafusion::sql::parser::Statement")s with with the specified dialect.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_sql_into_expr" class="fn">parse_sql_into_expr</a>(sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ExprWithAlias.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ExprWithAlias">ExprWithAlias</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_sql_into_expr_with_dialect" class="fn">parse_sql_into_expr_with_dialect</a>( sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, dialect: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html" class="trait" title="trait datafusion::logical_expr::sqlparser::dialect::Dialect">Dialect</a> + 'static), ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ExprWithAlias.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ExprWithAlias">ExprWithAlias</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_statements" class="fn">parse_statements</a>( &mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html" class="struct" title="struct alloc::collections::vec_deque::VecDeque">VecDeque</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse a sql string into one or [`Statement`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html "enum datafusion::sql::parser::Statement")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_statement" class="fn">parse_statement</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse a new expression

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_expr" class="fn">parse_expr</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ExprWithAlias.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ExprWithAlias">ExprWithAlias</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_into_expr" class="fn">parse_into_expr</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ExprWithAlias.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ExprWithAlias">ExprWithAlias</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parses the entire SQL string into an expression.

In contrast to [`DFParser::parse_expr`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_expr "method datafusion::sql::parser::DFParser::parse_expr"), this function will report an error if the input contains any trailing, unparsed tokens.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_copy" class="fn">parse_copy</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse a SQL `COPY TO` statement

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_option_key" class="fn">parse_option_key</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse the next token as a key name for an option list

Note this is different than [`parse_literal_string`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html#method.parse_literal_string "method datafusion::logical_expr::sqlparser::parser::Parser::parse_literal_string") because it allows keywords as well as other non words

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_option_value" class="fn">parse_option_value</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value">Value</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse the next token as a value for an option list

Note this is different than [`parse_value`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html#method.parse_value "method datafusion::logical_expr::sqlparser::parser::Parser::parse_value") as it allows any word or keyword in this location.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_explain" class="fn">parse_explain</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse a SQL `EXPLAIN`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_explain_format" class="fn">parse_explain_format</a>( &mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_create" class="fn">parse_create</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse a SQL `CREATE` statement handling `CREATE EXTERNAL TABLE`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_order_by_exprs" class="fn">parse_order_by_exprs</a>( &mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.OrderByExpr.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::OrderByExpr">OrderByExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse the ordering clause of a `CREATE EXTERNAL TABLE` SQL statement

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#method.parse_order_by_expr" class="fn">parse_order_by_expr</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.OrderByExpr.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::OrderByExpr">OrderByExpr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse an ORDER BY sub-expression optionally followed by ASC or DESC.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html#blanket-implementations" class="anchor">§</a>
