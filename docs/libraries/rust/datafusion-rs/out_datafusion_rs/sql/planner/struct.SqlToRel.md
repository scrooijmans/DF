# Struct SqlToRel Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/planner.rs.html#398" class="src">Source</a>

``` rust
pub struct SqlToRel<'a, S>where
    S: ContextProvider,{ /* private fields */ }
```

Expand description

SQL query planner and binder

This struct is used to convert a SQL AST into a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan").

You can control the behavior of the planner by providing [`ParserOptions`](https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html "struct datafusion::sql::planner::ParserOptions").

It performs the following tasks:

1.  Name and type resolution (called “binding” in other systems). This phase looks up table and column names using the [`ContextProvider`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html "trait datafusion::logical_expr::planner::ContextProvider").
2.  Mechanical translation of the AST into a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan").

It does not perform type coercion, or perform optimization, which are done by subsequent passes.

Key interfaces are:

- [`Self::sql_statement_to_plan`](https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#method.sql_statement_to_plan "method datafusion::sql::planner::SqlToRel::sql_statement_to_plan"): Convert a statement (e.g. `SELECT ...`) into a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")
- [`Self::sql_to_expr`](https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#method.sql_to_expr "method datafusion::sql::planner::SqlToRel::sql_to_expr"): Convert an expression (e.g. `1 + 2`) into an [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#impl-SqlToRel%3C&#39;_,+S%3E" class="anchor">§</a>

### impl\<S\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html" class="struct" title="struct datafusion::sql::planner::SqlToRel">SqlToRel</a>\<'\_, S\>

where S: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html" class="trait" title="trait datafusion::logical_expr::planner::ContextProvider">ContextProvider</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#method.sql_to_expr_with_alias" class="fn">sql_to_expr_with_alias</a>( &self, sql: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ExprWithAlias.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ExprWithAlias">ExprWithAlias</a>, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, planner_context: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#method.sql_to_expr" class="fn">sql_to_expr</a>( &self, sql: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, planner_context: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Generate a relational expression from a SQL expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#impl-SqlToRel%3C&#39;a,+S%3E" class="anchor">§</a>

### impl\<'a, S\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html" class="struct" title="struct datafusion::sql::planner::SqlToRel">SqlToRel</a>\<'a, S\>

where S: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html" class="trait" title="trait datafusion::logical_expr::planner::ContextProvider">ContextProvider</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#method.new" class="fn">new</a>(context_provider: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a S</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html" class="struct" title="struct datafusion::sql::planner::SqlToRel">SqlToRel</a>\<'a, S\>

Create a new query planner.

The query planner derives the parser options from the context provider.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#method.new_with_options" class="fn">new_with_options</a>( context_provider: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a S</a>, options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.ParserOptions.html" class="struct" title="struct datafusion::sql::planner::ParserOptions">ParserOptions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html" class="struct" title="struct datafusion::sql::planner::SqlToRel">SqlToRel</a>\<'a, S\>

Create a new query planner with the given parser options.

The query planner ignores the parser options from the context provider and uses the given parser options instead.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#method.build_schema" class="fn">build_schema</a>( &self, columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ColumnDef.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ColumnDef">ColumnDef</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#impl-SqlToRel%3C&#39;_,+S%3E-1" class="anchor">§</a>

### impl\<S\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html" class="struct" title="struct datafusion::sql::planner::SqlToRel">SqlToRel</a>\<'\_, S\>

where S: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ContextProvider.html" class="trait" title="trait datafusion::logical_expr::planner::ContextProvider">ContextProvider</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#method.statement_to_plan" class="fn">statement_to_plan</a>( &self, statement: <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Generate a logical plan from an DataFusion SQL statement

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#method.sql_statement_to_plan" class="fn">sql_statement_to_plan</a>( &self, statement: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Generate a logical plan from an SQL statement

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#method.sql_statement_to_plan_with_context" class="fn">sql_statement_to_plan_with_context</a>( &self, statement: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>, planner_context: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.PlannerContext.html" class="struct" title="struct datafusion::sql::planner::PlannerContext">PlannerContext</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Generate a logical plan from an SQL statement

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#method.new_constraint_from_table_constraints" class="fn">new_constraint_from_table_constraints</a>( &self, constraints: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TableConstraint.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TableConstraint">TableConstraint</a>\], df_schema: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Convert each [TableConstraint](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TableConstraint.html "enum datafusion::logical_expr::sqlparser::ast::TableConstraint") to corresponding [Constraint](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Constraint.html "enum datafusion::common::Constraint")

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html#blanket-implementations" class="anchor">§</a>
