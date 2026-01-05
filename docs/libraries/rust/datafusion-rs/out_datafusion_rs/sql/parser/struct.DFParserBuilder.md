# Struct DFParserBuilder Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/parser.rs.html#333" class="src">Source</a>

``` rust
pub struct DFParserBuilder<'a> { /* private fields */ }
```

Expand description

Builder for [`DFParser`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html "struct datafusion::sql::parser::DFParser")

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html#example-create-and-parse-sql-statements" class="doc-anchor">§</a>Example: Create and Parse SQL statements

``` rust
let mut parser = DFParserBuilder::new("SELECT * FROM foo; SELECT 1 + 2")
  .build()?;
// parse the SQL into DFStatements
let statements = parser.parse_statements()?;
assert_eq!(statements.len(), 2);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html#example-create-and-parse-expression-with-a-different-dialect" class="doc-anchor">§</a>Example: Create and Parse expression with a different dialect

``` rust
let dialect = MySqlDialect{}; // Parse using MySQL dialect
let mut parser = DFParserBuilder::new("1 + 2")
  .with_dialect(&dialect)
  .build()?;
// parse 1+2 into an sqlparser::ast::Expr
let res = parser.parse_expr()?;
assert!(matches!(res.expr, Expr::BinaryOp {..}));
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html#impl-DFParserBuilder%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html" class="struct" title="struct datafusion::sql::parser::DFParserBuilder">DFParserBuilder</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html#method.new" class="fn">new</a>(sql: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html" class="struct" title="struct datafusion::sql::parser::DFParserBuilder">DFParserBuilder</a>\<'a\>

Create a new parser builder for the specified tokens using the [`GenericDialect`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.GenericDialect.html "struct datafusion::logical_expr::sqlparser::dialect::GenericDialect").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html#method.with_dialect" class="fn">with_dialect</a>( self, dialect: &'a (dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html" class="trait" title="trait datafusion::logical_expr::sqlparser::dialect::Dialect">Dialect</a> + 'static), ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html" class="struct" title="struct datafusion::sql::parser::DFParserBuilder">DFParserBuilder</a>\<'a\>

Adjust the parser builder’s dialect. Defaults to [`GenericDialect`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/struct.GenericDialect.html "struct datafusion::logical_expr::sqlparser::dialect::GenericDialect")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html#method.with_recursion_limit" class="fn">with_recursion_limit</a>(self, recursion_limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html" class="struct" title="struct datafusion::sql::parser::DFParserBuilder">DFParserBuilder</a>\<'a\>

Adjust the recursion limit of sql parsing. Defaults to 50

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html" class="struct" title="struct datafusion::sql::parser::DFParser">DFParser</a>\<'a\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html#blanket-implementations" class="anchor">§</a>
