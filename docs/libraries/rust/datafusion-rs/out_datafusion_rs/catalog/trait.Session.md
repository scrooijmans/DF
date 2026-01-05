# Trait Session Copy item path

<a href="https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#69" class="src">Source</a>

``` rust
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

Historically, the `SessionState` struct was passed directly to catalog traits such as [`TableProvider`](https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html), which required a direct dependency on the DataFusion core. The interface required is now defined by this trait. See [\#10782](https://github.com/apache/datafusion/issues/10782) for more details.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#migration-from-sessionstate" class="doc-anchor">§</a>Migration from `SessionState`

Using trait methods is preferred, as the implementation may change in future versions. However, you can downcast a `Session` to a `SessionState` as shown in the example below. If you find yourself needing to do this, please open an issue on the DataFusion repository so we can extend the trait to provide the required information.

``` rust
// Given a `Session` reference, get the concrete `SessionState` reference
// Note: this may stop working in future versions,
fn session_state_from_session(session: &dyn Session) -> Result<&SessionState> {
   session.as_any()
    .downcast_ref::<SessionState>()
    .ok_or_else(|| exec_datafusion_err!("Failed to downcast Session to SessionState"))
}
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.session_id" class="fn">session_id</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return the session ID

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.config" class="fn">config</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Return the [`SessionConfig`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig")

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.create_physical_plan" class="fn">create_physical_plan</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, logical_plan: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Creates a physical [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") plan from a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan").

Note: this will optimize the provided plan first.

This function will error for [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s such as catalog DDL like `CREATE TABLE`, which do not have corresponding physical plans and must be handled by another layer, typically the `SessionContext`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.create_physical_expr" class="fn">create_physical_expr</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, df_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") from an [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") after applying type coercion, and function rewrites.

Note: The expression is not simplified or otherwise optimized: \`a = 1

- 2`will not be simplified to`a = 3\` as this is a more involved process. See the [expr_api](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/expr_api.rs) example for how to simplify expressions.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.scalar_functions" class="fn">scalar_functions</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>

Return reference to scalar_functions

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.aggregate_functions" class="fn">aggregate_functions</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>

Return reference to aggregate_functions

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.window_functions" class="fn">window_functions</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>

Return reference to window functions

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.runtime_env" class="fn">runtime_env</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>

Return the runtime env

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.execution_props" class="fn">execution_props</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

Return the execution properties

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.table_options" class="fn">table_options</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Return the table options

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.table_options_mut" class="fn">table_options_mut</a>(&mut self) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

Returns a mutable reference to [`TableOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html "struct datafusion::config::TableOptions")

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#tymethod.task_ctx" class="fn">task_ctx</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>

Get a new TaskContext to run in this session

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#method.config_options" class="fn">config_options</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

return the [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions")

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#method.default_table_options" class="fn">default_table_options</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>

return the TableOptions options with its extensions

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#impl-From%3C%26dyn+Session%3E-for-TaskContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>

Create a new task context instance from Session

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(state: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>

Converts to this type from the input type.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html#impl-Session-for-SessionState" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>
