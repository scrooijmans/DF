# Module unparser Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/lib.rs.html#56" class="src">Source</a>

Expand description

[`Unparser`](https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html "struct datafusion::sql::unparser::Unparser") for converting `Expr` to SQL text

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/index.html" class="mod" title="mod datafusion::sql::unparser::ast">ast</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/dialect/index.html" class="mod" title="mod datafusion::sql::unparser::dialect">dialect</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/extension_unparser/index.html" class="mod" title="mod datafusion::sql::unparser::extension_unparser">extension_unparser</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html" class="struct" title="struct datafusion::sql::unparser::Unparser">Unparser</a>  
Convert a DataFusion [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") to [`sqlparser::ast::Expr`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html "enum datafusion::logical_expr::sqlparser::ast::Expr")

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/fn.expr_to_sql.html" class="fn" title="fn datafusion::sql::unparser::expr_to_sql">expr_to_sql</a>  
Convert a DataFusion [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") to [`ast::Expr`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html "enum datafusion::logical_expr::sqlparser::ast::Expr")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/fn.plan_to_sql.html" class="fn" title="fn datafusion::sql::unparser::plan_to_sql">plan_to_sql</a>  
Convert a DataFusion [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") to [`ast::Statement`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html "enum datafusion::logical_expr::sqlparser::ast::Statement")
