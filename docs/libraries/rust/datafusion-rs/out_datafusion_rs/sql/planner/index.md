# Module planner Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/lib.rs.html#47" class="src">Source</a>

Expand description

[`SqlToRel`](https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html "struct datafusion::sql::planner::SqlToRel"): SQL Query Planner (produces [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") from SQL AST)

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.IdentNormalizer.html" class="struct" title="struct datafusion::sql::planner::IdentNormalizer">IdentNormalizer</a>  
Ident Normalizer

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>  
SQL parser options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>  
Struct to store the states used by the Planner. The Planner will leverage the states to resolve CTEs, Views, subqueries and PREPARE statements. The states include Common Table Expression (CTE) provided with WITH clause and Parameter Data Types provided with PREPARE statement and the query schema of the outer query plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html" class="struct" title="struct datafusion::sql::planner::SqlToRel">SqlToRel</a>  
SQL query planner and binder

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/enum.NullOrdering.html" class="enum" title="enum datafusion::sql::planner::NullOrdering">NullOrdering</a>  
Represents the null ordering for sorting expressions.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/trait.ContextProvider.html" class="trait" title="trait datafusion::sql::planner::ContextProvider">ContextProvider</a>  
Provides the `SQL` query planner meta-data about tables and functions referenced in SQL statements, without a direct dependency on the `datafusion` Catalog structures such as [`TableProvider`](https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html)

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/fn.object_name_to_qualifier.html" class="fn" title="fn datafusion::sql::planner::object_name_to_qualifier">object_name_to_qualifier</a>  
Construct a WHERE qualifier suitable for e.g. information_schema filtering from the provided object identifiers (catalog, schema and table names).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/fn.object_name_to_table_reference.html" class="fn" title="fn datafusion::sql::planner::object_name_to_table_reference">object_name_to_table_reference</a>  
Create a [`TableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html "enum datafusion::common::TableReference") after normalizing the specified ObjectName
