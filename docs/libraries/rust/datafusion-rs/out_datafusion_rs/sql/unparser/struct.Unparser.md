# Struct Unparser Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/unparser/mod.rs.html#57" class="src">Source</a>

``` rust
pub struct Unparser<'a> { /* private fields */ }
```

Expand description

Convert a DataFusion [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") to [`sqlparser::ast::Expr`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html "enum datafusion::logical_expr::sqlparser::ast::Expr")

See [`expr_to_sql`](https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/fn.expr_to_sql.html "fn datafusion::sql::unparser::expr_to_sql") for background. `Unparser` allows greater control of the conversion, but with a more complicated API.

To get more human-readable output, see [`Self::with_pretty`](https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#method.with_pretty "method datafusion::sql::unparser::Unparser::with_pretty")

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#example" class="doc-anchor">§</a>Example

``` rust
use datafusion_expr::{col, lit};
use datafusion_sql::unparser::Unparser;
let expr = col("a").gt(lit(4)); // form an expression `a > 4`
let unparser = Unparser::default();
let sql = unparser.expr_to_sql(&expr).unwrap();// convert to AST
// use the Display impl to convert to SQL text
assert_eq!(sql.to_string(), "(a > 4)");
// now convert to pretty sql
let unparser = unparser.with_pretty(true);
let sql = unparser.expr_to_sql(&expr).unwrap();
assert_eq!(sql.to_string(), "a > 4"); // note lack of parenthesis
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#impl-Unparser%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html" class="struct" title="struct datafusion::sql::unparser::Unparser">Unparser</a>\<'\_\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#method.expr_to_sql" class="fn">expr_to_sql</a>(&self, expr: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#method.scalar_function_to_sql" class="fn">scalar_function_to_sql</a>( &self, func_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#method.sort_to_sql" class="fn">sort_to_sql</a>(&self, sort: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.OrderByExpr.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::OrderByExpr">OrderByExpr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#method.col_to_sql" class="fn">col_to_sql</a>(&self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#impl-Unparser%3C&#39;_%3E-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html" class="struct" title="struct datafusion::sql::unparser::Unparser">Unparser</a>\<'\_\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#method.plan_to_sql" class="fn">plan_to_sql</a>( &self, plan: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#impl-Unparser%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html" class="struct" title="struct datafusion::sql::unparser::Unparser">Unparser</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#method.new" class="fn">new</a>(dialect: &'a dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/trait.Dialect.html" class="trait" title="trait datafusion::sql::unparser::dialect::Dialect">Dialect</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html" class="struct" title="struct datafusion::sql::unparser::Unparser">Unparser</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#method.with_pretty" class="fn">with_pretty</a>(self, pretty: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html" class="struct" title="struct datafusion::sql::unparser::Unparser">Unparser</a>\<'a\>

Create pretty SQL output, better suited for human consumption

See example on the struct level documentation

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#pretty-output" class="doc-anchor">§</a>Pretty Output

By default, `Unparser` generates SQL text that will parse back to the same parsed [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr"), which is useful for creating machine readable expressions to send to other systems. However, the resulting expressions are not always nice to read for humans.

For example

``` sql
((a + 4) > 5)
```

This method removes parenthesis using to the precedence rules of DataFusion. If the output is reparsed, the resulting [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") produces same value as the original in DataFusion, but with a potentially different order of operations.

Note that this setting may create invalid SQL for other SQL query engines with different precedence rules

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#example-1" class="doc-anchor">§</a>Example

``` rust
use datafusion_expr::{col, lit};
use datafusion_sql::unparser::Unparser;
let expr = col("a").gt(lit(4)).and(col("b").lt(lit(5))); // form an expression `a > 4 AND b < 5`
let unparser = Unparser::default().with_pretty(true);
let sql = unparser.expr_to_sql(&expr).unwrap();
assert_eq!(sql.to_string(), "a > 4 AND b < 5"); // note lack of parenthesis
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#method.with_extension_unparsers" class="fn">with_extension_unparsers</a>( self, extension_unparsers: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/extension_unparser/trait.UserDefinedLogicalNodeUnparser.html" class="trait" title="trait datafusion::sql::unparser::extension_unparser::UserDefinedLogicalNodeUnparser">UserDefinedLogicalNodeUnparser</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html" class="struct" title="struct datafusion::sql::unparser::Unparser">Unparser</a>\<'a\>

Add a custom unparser for user defined logical nodes

DataFusion allows user to define custom logical nodes. This method allows to add custom child unparsers for these nodes. Implementation of [`UserDefinedLogicalNodeUnparser`](https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/extension_unparser/trait.UserDefinedLogicalNodeUnparser.html "trait datafusion::sql::unparser::extension_unparser::UserDefinedLogicalNodeUnparser") can be added to the root unparser to handle custom logical nodes.

The child unparsers are called iteratively. There are two methods in [`Unparser`](https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html "struct datafusion::sql::unparser::Unparser") will be called:

- `extension_to_statement`: This method is called when the custom logical node is a custom statement. If multiple child unparsers return a non-None value, the last unparsing result will be returned.
- `extension_to_sql`: This method is called when the custom logical node is part of a statement. If multiple child unparsers are registered for the same custom logical node, all of them will be called in order.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#impl-Default-for-Unparser%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html" class="struct" title="struct datafusion::sql::unparser::Unparser">Unparser</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html" class="struct" title="struct datafusion::sql::unparser::Unparser">Unparser</a>\<'\_\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html#blanket-implementations" class="anchor">§</a>
