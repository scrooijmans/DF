# Session in datafusion::catalog - Rust

```
pub trait Session: Send + Sync {
Show 15 methods    // Required methods
    fn session_id(&self) -> &str;
    fn config(&self) -> &SessionConfig;
    fn create_physical_plan<'life0, 'life1, 'async_trait>(
        &'life0 self,
        logical_plan: &'life1 LogicalPlan,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn ExecutionPlan>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             Self: 'async_trait;
    fn create_physical_expr(
        &self,
        expr: Expr,
        df_schema: &DFSchema,
    ) -> Result<Arc<dyn PhysicalExpr>, DataFusionError>;
    fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>;
    fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>;
    fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>;
    fn runtime_env(&self) -> &Arc<RuntimeEnv>;
    fn execution_props(&self) -> &ExecutionProps;
    fn as_any(&self) -> &(dyn Any + 'static);
    fn table_options(&self) -> &TableOptions;
    fn table_options_mut(&mut self) -> &mut TableOptions;
    fn task_ctx(&self) -> Arc<TaskContext>;

    // Provided methods
    fn config_options(&self) -> &ConfigOptions { ... }
    fn default_table_options(&self) -> TableOptions { ... }
}
```

Expand description

Interface for accessing [`SessionState`](https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html) from the catalog and data source.

This trait provides access to the information needed to plan and execute queries, such as configuration, functions, and runtime environment. See the documentation on [`SessionState`](https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html) for more information.

Historically, the `SessionState` struct was passed directly to catalog traits such as [`TableProvider`](https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html), which required a direct dependency on the DataFusion core. The interface required is now defined by this trait. See [#10782](https://github.com/apache/datafusion/issues/10782) for more details.

## [§](#migration-from-sessionstate)Migration from `SessionState`

Using trait methods is preferred, as the implementation may change in future versions. However, you can downcast a `Session` to a `SessionState` as shown in the example below. If you find yourself needing to do this, please open an issue on the DataFusion repository so we can extend the trait to provide the required information.

```
// Given a `Session` reference, get the concrete `SessionState` reference
// Note: this may stop working in future versions,
fn session_state_from_session(session: &dyn Session) -> Result<&SessionState> {
   session.as_any()
    .downcast_ref::<SessionState>()
    .ok_or_else(|| exec_datafusion_err!("Failed to downcast Session to SessionState"))
}
```

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#71)

Return the session ID

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#74)

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#88-91)

Creates a physical [`ExecutionPlan`](../physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") plan from a [`LogicalPlan`](../logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan").

Note: this will optimize the provided plan first.

This function will error for [`LogicalPlan`](../logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s such as catalog DDL like `CREATE TABLE`, which do not have corresponding physical plans and must be handled by another layer, typically the `SessionContext`.

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#101-105)

Create a [`PhysicalExpr`](../physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") from an [`Expr`](../prelude/enum.Expr.html "enum datafusion::prelude::Expr") after applying type coercion, and function rewrites.

Note: The expression is not simplified or otherwise optimized: \`a = 1

- 2`will not be simplified to`a = 3\` as this is a more involved process. See the [expr_api](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/expr_api.rs) example for how to simplify expressions.

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#108)

Return reference to scalar_functions

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#111)

Return reference to aggregate_functions

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#114)

Return reference to window functions

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#117)

Return the runtime env

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#120)

Return the execution properties

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#122)

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#125)

Return the table options

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#134)

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#137)

Get a new TaskContext to run in this session

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#77)

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#128)

return the TableOptions options with its extensions

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#141)
[§](#impl-From%3C%26dyn+Session%3E-for-TaskContext)

Create a new task context instance from Session

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#142)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/datafusion/execution/session_state.rs.html#215-274)
[§](#impl-Session-for-SessionState)
