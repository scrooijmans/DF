# Struct SessionState Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/execution/session_state.rs.html#127-182" class="src">Source</a>

``` rust
pub struct SessionState { /* private fields */ }
```

Expand description

`SessionState` contains all the necessary state to plan and execute queries, such as configuration, functions, and runtime environment. Please see the documentation on [`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext") for more information.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#example-sessionstate-from-a-sessioncontext" class="doc-anchor">§</a>Example: `SessionState` from a [`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext")

``` rust
use datafusion::prelude::*;
let ctx = SessionContext::new();
let state = ctx.state();
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#example-sessionstate-via-sessionstatebuilder" class="doc-anchor">§</a>Example: `SessionState` via [`SessionStateBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html "struct datafusion::execution::session_state::SessionStateBuilder")

You can also use [`SessionStateBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html "struct datafusion::execution::session_state::SessionStateBuilder") to build a `SessionState` object directly:

``` rust
use datafusion::prelude::*;
    let state = SessionStateBuilder::new()
        .with_config(SessionConfig::new())  
        .with_runtime_env(Arc::new(RuntimeEnv::default()))
        .with_default_features()
        .build();
    Ok(())  
```

Note that there is no `Default` or `new()` for SessionState, to avoid accidentally running queries or other operations without passing through the [`SessionConfig`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig") or [`RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv"). See [`SessionStateBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html "struct datafusion::execution::session_state::SessionStateBuilder") and [`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext").

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#impl-SessionState" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.schema_for_ref" class="fn">schema_for_ref</a>( &self, table_ref: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>\>\>

Retrieve the [`SchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html "trait datafusion::catalog::SchemaProvider") for a specific [`TableReference`](https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html "enum datafusion::common::TableReference"), if it exists.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.add_analyzer_rule" class="fn">add_analyzer_rule</a>( &mut self, analyzer_rule: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>, ) -\> &Self

Add `analyzer_rule` to the end of the list of [`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")s used to rewrite queries.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.set_function_factory" class="fn">set_function_factory</a>( &mut self, function_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html" class="trait" title="trait datafusion::execution::context::FunctionFactory">FunctionFactory</a>\>, )

Registers a [`FunctionFactory`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html "trait datafusion::execution::context::FunctionFactory") to handle `CREATE FUNCTION` statements

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.function_factory" class="fn">function_factory</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html" class="trait" title="trait datafusion::execution::context::FunctionFactory">FunctionFactory</a>\>\>

Get the function factory

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.table_factories" class="fn">table_factories</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a>\>\>

Get the table factories

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.table_factories_mut" class="fn">table_factories_mut</a>( &mut self, ) -\> &mut <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a>\>\>

Get the table factories

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.sql_to_statement" class="fn">sql_to_statement</a>(&self, sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, dialect: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>\>

Parse an SQL string into an DataFusion specific AST [`Statement`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html "enum datafusion::sql::parser::Statement"). See [`SessionContext::sql`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.sql "method datafusion::execution::context::SessionContext::sql") for running queries.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.sql_to_expr" class="fn">sql_to_expr</a>(&self, sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, dialect: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">SQLExpr</a>\>

parse a sql string into a sqlparser-rs AST [`SQLExpr`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html "enum datafusion::logical_expr::sqlparser::ast::Expr").

See [`Self::create_logical_expr`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.create_logical_expr "method datafusion::execution::session_state::SessionState::create_logical_expr") for parsing sql to [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.sql_to_expr_with_alias" class="fn">sql_to_expr_with_alias</a>( &self, sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, dialect: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ExprWithAlias.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ExprWithAlias">SQLExprWithAlias</a>\>

parse a sql string into a sqlparser-rs AST [`SQLExprWithAlias`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ExprWithAlias.html "struct datafusion::logical_expr::sqlparser::ast::ExprWithAlias").

See [`Self::create_logical_expr`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.create_logical_expr "method datafusion::execution::session_state::SessionState::create_logical_expr") for parsing sql to [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.resolve_table_references" class="fn">resolve_table_references</a>( &self, statement: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>\>

Resolve all table references in the SQL statement. Does not include CTE references.

See [`datafusion_sql::resolve::resolve_table_references`](https://docs.rs/datafusion/50.2.0/datafusion/sql/resolve/fn.resolve_table_references.html "fn datafusion::sql::resolve::resolve_table_references") for more information.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.statement_to_plan" class="fn">statement_to_plan</a>( &self, statement: <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>

Convert an AST Statement into a LogicalPlan

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.create_logical_plan" class="fn">create_logical_plan</a>(&self, sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>

Creates a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") from the provided SQL string. This interface will plan any SQL DataFusion supports, including DML like `CREATE TABLE`, and `COPY` (which can write to local files.

See [`SessionContext::sql`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.sql "method datafusion::execution::context::SessionContext::sql") and [`SessionContext::sql_with_options`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.sql_with_options "method datafusion::execution::context::SessionContext::sql_with_options") for a higher-level interface that handles DDL and verification of allowed statements.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.create_logical_expr" class="fn">create_logical_expr</a>( &self, sql: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, df_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Creates a datafusion style AST [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") from a SQL string.

See example on [SessionContext::parse_sql_expr](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.parse_sql_expr "method datafusion::execution::context::SessionContext::parse_sql_expr")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.analyzer" class="fn">analyzer</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::Analyzer">Analyzer</a>

Returns the [`Analyzer`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html "struct datafusion::optimizer::Analyzer") for this session

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.optimizer" class="fn">optimizer</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>

Returns the [`Optimizer`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html "struct datafusion::optimizer::Optimizer") for this session

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.expr_planners" class="fn">expr_planners</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>\]

Returns the [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner")s for this session

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.query_planner" class="fn">query_planner</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.QueryPlanner.html" class="trait" title="trait datafusion::execution::context::QueryPlanner">QueryPlanner</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>

Returns the [`QueryPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.QueryPlanner.html "trait datafusion::execution::context::QueryPlanner") for this session

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.optimize" class="fn">optimize</a>(&self, plan: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>

Optimizes the logical plan by applying optimizer rules.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.create_physical_plan" class="fn">create_physical_plan</a>( &self, logical_plan: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Creates a physical [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") plan from a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan").

Note: this first calls [`Self::optimize`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.optimize "method datafusion::execution::session_state::SessionState::optimize") on the provided plan.

This function will error for [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s such as catalog DDL like `CREATE TABLE`, which do not have corresponding physical plans and must be handled by another layer, typically [`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.create_physical_expr" class="fn">create_physical_expr</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, df_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Create a [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") from an [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") after applying type coercion, and function rewrites.

Note: The expression is not [simplified](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/index.html "mod datafusion::optimizer::simplify_expressions") or otherwise optimized: `a = 1 + 2` will not be simplified to `a = 3` as this is a more involved process. See the [expr_api](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/expr_api.rs) example for how to simplify expressions.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#see-also" class="doc-anchor">§</a>See Also:

- [`SessionContext::create_physical_expr`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.create_physical_expr "method datafusion::execution::context::SessionContext::create_physical_expr") for a higher-level API
- [`create_physical_expr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_physical_expr.html "fn datafusion::physical_expr::create_physical_expr") for a lower-level API

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.session_id" class="fn">session_id</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return the session ID

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.runtime_env" class="fn">runtime_env</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>

Return the runtime env

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.execution_props" class="fn">execution_props</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

Return the execution properties

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.execution_props_mut" class="fn">execution_props_mut</a>(&mut self) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

Return mutable execution properties

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.config" class="fn">config</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Return the [`SessionConfig`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.config_mut" class="fn">config_mut</a>(&mut self) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Return the mutable [`SessionConfig`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.optimizers" class="fn">optimizers</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>\]

Return the logical optimizers

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.physical_optimizers" class="fn">physical_optimizers</a>( &self, ) -\> &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>\]

Return the physical optimizers

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.config_options" class="fn">config_options</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>\>

return the configuration options

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.mark_start_execution" class="fn">mark_start_execution</a>(&mut self)

Mark the start of the execution

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.table_options" class="fn">table_options</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Return the table options

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.default_table_options" class="fn">default_table_options</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

return the TableOptions options with its extensions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.table_options_mut" class="fn">table_options_mut</a>(&mut self) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Returns a mutable reference to [`TableOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html "struct datafusion::config::TableOptions")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.register_table_options_extension" class="fn">register_table_options_extension</a>\<T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html" class="trait" title="trait datafusion::config::ConfigExtension">ConfigExtension</a>\>( &mut self, extension: T, )

Registers a [`ConfigExtension`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html "trait datafusion::config::ConfigExtension") as a table option extension that can be referenced from SQL statements executed against this context.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.register_file_format" class="fn">register_file_format</a>( &mut self, file_format: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html" class="trait" title="trait datafusion::datasource::file_format::FileFormatFactory">FileFormatFactory</a>\>, overwrite: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Adds or updates a [FileFormatFactory](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html "trait datafusion::datasource::file_format::FileFormatFactory") which can be used with COPY TO or CREATE EXTERNAL TABLE statements for reading and writing files of custom formats.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.get_file_format_factory" class="fn">get_file_format_factory</a>( &self, ext: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html" class="trait" title="trait datafusion::datasource::file_format::FileFormatFactory">FileFormatFactory</a>\>\>

Retrieves a [FileFormatFactory](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html "trait datafusion::datasource::file_format::FileFormatFactory") based on file extension which has been registered via SessionContext::register_file_format. Extensions are not case sensitive.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.task_ctx" class="fn">task_ctx</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>

Get a new TaskContext to run in this session

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.catalog_list" class="fn">catalog_list</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html" class="trait" title="trait datafusion::catalog::CatalogProviderList">CatalogProviderList</a>\>

Return catalog list

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.scalar_functions" class="fn">scalar_functions</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>

Return reference to scalar_functions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.aggregate_functions" class="fn">aggregate_functions</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>

Return reference to aggregate_functions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.window_functions" class="fn">window_functions</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>

Return reference to window functions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.table_functions" class="fn">table_functions</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html" class="struct" title="struct datafusion::catalog::TableFunction">TableFunction</a>\>\>

Return reference to table_functions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.serializer_registry" class="fn">serializer_registry</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.SerializerRegistry.html" class="trait" title="trait datafusion::execution::registry::SerializerRegistry">SerializerRegistry</a>\>

Return [SerializerRegistry](https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.SerializerRegistry.html "trait datafusion::execution::registry::SerializerRegistry") for extensions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.version" class="fn">version</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return version of the cargo package that produced this query

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.register_udtf" class="fn">register_udtf</a>(&mut self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, fun: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html" class="trait" title="trait datafusion::catalog::TableFunctionImpl">TableFunctionImpl</a>\>)

Register a user defined table function

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.deregister_udtf" class="fn">deregister_udtf</a>( &mut self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html" class="trait" title="trait datafusion::catalog::TableFunctionImpl">TableFunctionImpl</a>\>\>\>

Deregister a user defined table function

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#impl-Clone-for-SessionState" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#impl-Debug-for-SessionState" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Prefer having short fields at the top and long vector fields near the end Group fields by

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#impl-From%3C%26SessionState%3E-for-TaskContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>

Create a new task context instance from SessionState

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(state: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#impl-From%3CSessionState%3E-for-SessionContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#impl-From%3CSessionState%3E-for-SessionStateBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html" class="struct" title="struct datafusion::execution::session_state::SessionStateBuilder">SessionStateBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#impl-FunctionRegistry-for-SessionState" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.udfs" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udfs" class="fn">udfs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available scalar user defined functions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.udf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udf" class="fn">udf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>

Returns a reference to the user defined scalar function (udf) named `name`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.udaf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udaf" class="fn">udaf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>

Returns a reference to the user defined aggregate function (udaf) named `name`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.udwf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udwf" class="fn">udwf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>

Returns a reference to the user defined window function (udwf) named `name`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.register_udf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udf" class="fn">register_udf</a>( &mut self, udf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>\>

Registers a new [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF"), returning any previously registered implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.register_udaf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udaf" class="fn">register_udaf</a>( &mut self, udaf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>\>

Registers a new [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF"), returning any previously registered implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udaf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.register_udwf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udwf" class="fn">register_udwf</a>( &mut self, udwf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>\>

Registers a new [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF"), returning any previously registered implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udwf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.deregister_udf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udf" class="fn">deregister_udf</a>(&mut self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>\>

Deregisters a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF"), returning the implementation that was deregistered. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.deregister_udaf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udaf" class="fn">deregister_udaf</a>(&mut self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>\>

Deregisters a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF"), returning the implementation that was deregistered. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udaf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.deregister_udwf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udwf" class="fn">deregister_udwf</a>(&mut self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>\>

Deregisters a [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF"), returning the implementation that was deregistered. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udwf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.register_function_rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_function_rewrite" class="fn">register_function_rewrite</a>( &mut self, rewrite: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html" class="trait" title="trait datafusion::logical_expr::expr_rewriter::FunctionRewrite">FunctionRewrite</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Registers a new [`FunctionRewrite`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html "trait datafusion::logical_expr::expr_rewriter::FunctionRewrite") with the registry. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_function_rewrite)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.expr_planners-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.expr_planners" class="fn">expr_planners</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>\>

Set of all registered [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner")s

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.register_expr_planner" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_expr_planner" class="fn">register_expr_planner</a>( &mut self, expr_planner: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Registers a new [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner") with the registry.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.udafs" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.udafs" class="fn">udafs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available aggregate user defined functions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.udwfs" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.udwfs" class="fn">udwfs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available window user defined functions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#impl-OptimizerConfig-for-SessionState" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html" class="trait" title="trait datafusion::optimizer::OptimizerConfig">OptimizerConfig</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.query_execution_start_time" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#tymethod.query_execution_start_time" class="fn">query_execution_start_time</a>(&self) -\> <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc">Utc</a>\>

Return the time at which the query execution started. This time is used as the value for now()

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.alias_generator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#tymethod.alias_generator" class="fn">alias_generator</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html" class="struct" title="struct datafusion::common::alias::AliasGenerator">AliasGenerator</a>\>

Return alias generator used to generate unique aliases for subqueries

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.options" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#tymethod.options" class="fn">options</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.function_registry" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#method.function_registry" class="fn">function_registry</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#impl-Session-for-SessionState" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.session_id-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.session_id" class="fn">session_id</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return the session ID

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.config-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.config" class="fn">config</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Return the [`SessionConfig`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.create_physical_plan-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.create_physical_plan" class="fn">create_physical_plan</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, logical_plan: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Creates a physical [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") plan from a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.create_physical_plan)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.create_physical_expr-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.create_physical_expr" class="fn">create_physical_expr</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, df_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Create a [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") from an [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") after applying type coercion, and function rewrites. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.create_physical_expr)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.scalar_functions-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.scalar_functions" class="fn">scalar_functions</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>

Return reference to scalar_functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.aggregate_functions-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.aggregate_functions" class="fn">aggregate_functions</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>

Return reference to aggregate_functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.window_functions-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.window_functions" class="fn">window_functions</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>

Return reference to window functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.runtime_env-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.runtime_env" class="fn">runtime_env</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>

Return the runtime env

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.execution_props-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.execution_props" class="fn">execution_props</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

Return the execution properties

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.table_options-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.table_options" class="fn">table_options</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Return the table options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.table_options_mut-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.table_options_mut" class="fn">table_options_mut</a>(&mut self) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Returns a mutable reference to [`TableOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html "struct datafusion::config::TableOptions")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.task_ctx-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.task_ctx" class="fn">task_ctx</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>

Get a new TaskContext to run in this session

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.config_options-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#method.config_options" class="fn">config_options</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

return the [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#method.default_table_options-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#method.default_table_options" class="fn">default_table_options</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

return the TableOptions options with its extensions

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html#blanket-implementations" class="anchor">§</a>
