# Module sql Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#826" class="src">Source</a>

Expand description

re-export of [`datafusion_sql`](https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/datafusion_sql/index.html "mod datafusion_sql") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/index.html" class="mod" title="mod datafusion::sql::parser">parser</a>  
[`DFParser`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html "struct datafusion::sql::parser::DFParser"): DataFusion SQL Parser based on [`sqlparser`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/index.html "mod datafusion::logical_expr::sqlparser")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/index.html" class="mod" title="mod datafusion::sql::planner">planner</a>  
[`SqlToRel`](https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html "struct datafusion::sql::planner::SqlToRel"): SQL Query Planner (produces [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") from SQL AST)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/resolve/index.html" class="mod" title="mod datafusion::sql::resolve">resolve</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/index.html" class="mod" title="mod datafusion::sql::sqlparser">sqlparser</a>  
SQL Parser for Rust

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/index.html" class="mod" title="mod datafusion::sql::unparser">unparser</a>  
[`Unparser`](https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/struct.Unparser.html "struct datafusion::sql::unparser::Unparser") for converting `Expr` to SQL text

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/utils/index.html" class="mod" title="mod datafusion::sql::utils">utils</a>  
SQL Utility Functions

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/struct.ResolvedTableReference.html" class="struct" title="struct datafusion::sql::ResolvedTableReference">ResolvedTableReference</a>  
A fully resolved path to a table of the form “catalog.schema.table”

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/enum.TableReference.html" class="enum" title="enum datafusion::sql::TableReference">TableReference</a>  
A multi part identifier (path) to a table that may require further resolution (e.g. `foo.bar`).
