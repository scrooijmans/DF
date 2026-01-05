# Struct SessionStateBuilder Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/execution/session_state.rs.html#891-916" class="src">Source</a>

``` rust
pub struct SessionStateBuilder { /* private fields */ }
```

Expand description

A builder to be used for building [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState")’s. Defaults will be used for all values unless explicitly provided.

See example on [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#impl-SessionStateBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html" class="struct" title="struct datafusion::execution::session_state::SessionStateBuilder">SessionStateBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.new" class="fn">new</a>() -\> Self

Returns a new empty [`SessionStateBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html "struct datafusion::execution::session_state::SessionStateBuilder").

See [`Self::with_default_features`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_default_features "method datafusion::execution::session_state::SessionStateBuilder::with_default_features") to install the default set of functions, catalogs, etc.

To create a `SessionStateBuilder` with default features such as functions, please see [`Self::new_with_default_features`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.new_with_default_features "associated function datafusion::execution::session_state::SessionStateBuilder::new_with_default_features").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.new_from_existing" class="fn">new_from_existing</a>(existing: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>) -\> Self

Returns a new [SessionStateBuilder](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html "struct datafusion::execution::session_state::SessionStateBuilder") based on an existing [SessionState](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState").

The session id for the new builder will be unset; all other fields will be cloned from `existing`. If the default catalog exists in existing session state, the new session state will not create default catalog and schema.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_default_features" class="fn">with_default_features</a>(self) -\> Self

Adds defaults for table_factories, file formats, expr_planners and builtin scalar, aggregate and windows functions.

Note overwrites any previously registered items with the same name.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.new_with_default_features" class="fn">new_with_default_features</a>() -\> Self

Returns a new [`SessionStateBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html "struct datafusion::execution::session_state::SessionStateBuilder") with default features.

This is equivalent to calling [`Self::new()`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.new "associated function datafusion::execution::session_state::SessionStateBuilder::new") followed by [`Self::with_default_features()`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_default_features "method datafusion::execution::session_state::SessionStateBuilder::with_default_features").

``` rust
use datafusion::execution::session_state::SessionStateBuilder;

// Create a new SessionState with default features
let session_state = SessionStateBuilder::new_with_default_features()
    .with_session_id("my_session".to_string())
    .build();
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_session_id" class="fn">with_session_id</a>(self, session_id: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set the session id.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_analyzer_rules" class="fn">with_analyzer_rules</a>( self, rules: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>\>, ) -\> Self

Set the [`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")s optimizer plan rules.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_analyzer_rule" class="fn">with_analyzer_rule</a>( self, analyzer_rule: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>, ) -\> Self

Add `analyzer_rule` to the end of the list of [`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")s used to rewrite queries.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_optimizer_rules" class="fn">with_optimizer_rules</a>( self, rules: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>\>, ) -\> Self

Set the [`OptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html "trait datafusion::optimizer::OptimizerRule")s used to optimize plans.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_optimizer_rule" class="fn">with_optimizer_rule</a>( self, optimizer_rule: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>, ) -\> Self

Add `optimizer_rule` to the end of the list of [`OptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html "trait datafusion::optimizer::OptimizerRule")s used to rewrite queries.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_expr_planners" class="fn">with_expr_planners</a>( self, expr_planners: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>\>, ) -\> Self

Set the [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner")s used to customize the behavior of the SQL planner.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_type_planner" class="fn">with_type_planner</a>(self, type_planner: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.TypePlanner.html" class="trait" title="trait datafusion::logical_expr::planner::TypePlanner">TypePlanner</a>\>) -\> Self

Set the [`TypePlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.TypePlanner.html "trait datafusion::logical_expr::planner::TypePlanner") used to customize the behavior of the SQL planner.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_physical_optimizer_rules" class="fn">with_physical_optimizer_rules</a>( self, physical_optimizers: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>\>, ) -\> Self

Set the [`PhysicalOptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html "trait datafusion::physical_optimizer::PhysicalOptimizerRule")s used to optimize plans.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_physical_optimizer_rule" class="fn">with_physical_optimizer_rule</a>( self, physical_optimizer_rule: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>, ) -\> Self

Add `physical_optimizer_rule` to the end of the list of [`PhysicalOptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html "trait datafusion::physical_optimizer::PhysicalOptimizerRule")s used to rewrite queries.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_query_planner" class="fn">with_query_planner</a>( self, query_planner: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.QueryPlanner.html" class="trait" title="trait datafusion::execution::context::QueryPlanner">QueryPlanner</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>, ) -\> Self

Set the [`QueryPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.QueryPlanner.html "trait datafusion::execution::context::QueryPlanner")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_catalog_list" class="fn">with_catalog_list</a>( self, catalog_list: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html" class="trait" title="trait datafusion::catalog::CatalogProviderList">CatalogProviderList</a>\>, ) -\> Self

Set the [`CatalogProviderList`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html "trait datafusion::catalog::CatalogProviderList")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_table_functions" class="fn">with_table_functions</a>( self, table_functions: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html" class="struct" title="struct datafusion::catalog::TableFunction">TableFunction</a>\>\>, ) -\> Self

Set the map of [`TableFunction`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html "struct datafusion::catalog::TableFunction")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_table_function_list" class="fn">with_table_function_list</a>( self, table_functions: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html" class="struct" title="struct datafusion::catalog::TableFunction">TableFunction</a>\>\>, ) -\> Self

Set the list of [`TableFunction`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html "struct datafusion::catalog::TableFunction")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_scalar_functions" class="fn">with_scalar_functions</a>( self, scalar_functions: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>, ) -\> Self

Set the map of [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_aggregate_functions" class="fn">with_aggregate_functions</a>( self, aggregate_functions: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>, ) -\> Self

Set the map of [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_window_functions" class="fn">with_window_functions</a>( self, window_functions: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>, ) -\> Self

Set the map of [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_serializer_registry" class="fn">with_serializer_registry</a>( self, serializer_registry: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.SerializerRegistry.html" class="trait" title="trait datafusion::execution::registry::SerializerRegistry">SerializerRegistry</a>\>, ) -\> Self

Set the [`SerializerRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.SerializerRegistry.html "trait datafusion::execution::registry::SerializerRegistry")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_file_formats" class="fn">with_file_formats</a>( self, file_formats: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html" class="trait" title="trait datafusion::datasource::file_format::FileFormatFactory">FileFormatFactory</a>\>\>, ) -\> Self

Set the map of [`FileFormatFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html "trait datafusion::datasource::file_format::FileFormatFactory")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_config" class="fn">with_config</a>(self, config: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>) -\> Self

Set the [`SessionConfig`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_table_options" class="fn">with_table_options</a>(self, table_options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>) -\> Self

Set the [`TableOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html "struct datafusion::config::TableOptions")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_execution_props" class="fn">with_execution_props</a>(self, execution_props: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>) -\> Self

Set the [`ExecutionProps`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html "struct datafusion::execution::context::ExecutionProps")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_table_factory" class="fn">with_table_factory</a>( self, key: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, table_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a>\>, ) -\> Self

Add a [`TableProviderFactory`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html "trait datafusion::catalog::TableProviderFactory") to the map of factories

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_table_factories" class="fn">with_table_factories</a>( self, table_factories: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a>\>\>, ) -\> Self

Set the map of [`TableProviderFactory`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html "trait datafusion::catalog::TableProviderFactory")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_runtime_env" class="fn">with_runtime_env</a>(self, runtime_env: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>) -\> Self

Set the [`RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_function_factory" class="fn">with_function_factory</a>( self, function_factory: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html" class="trait" title="trait datafusion::execution::context::FunctionFactory">FunctionFactory</a>\>\>, ) -\> Self

Set a [`FunctionFactory`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html "trait datafusion::execution::context::FunctionFactory") to handle `CREATE FUNCTION` statements

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.with_object_store" class="fn">with_object_store</a>( self, url: &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>, object_store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, ) -\> Self

Register an `ObjectStore` to the [`RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv"). See [`RuntimeEnv::register_object_store`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#method.register_object_store "method datafusion::execution::runtime_env::RuntimeEnv::register_object_store") for more details.

Note that this creates a default [`RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv") if there isn’t one passed in already.

``` rust
let url = Url::try_from("file://").unwrap();
let object_store = object_store::local::LocalFileSystem::new();
let state = SessionStateBuilder::new()
    .with_config(SessionConfig::new())  
    .with_object_store(&url, Arc::new(object_store))
    .with_default_features()
    .build();
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>

Builds a [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") with the current configuration.

Note that there is an explicit option for enabling catalog and schema defaults in [SessionConfig::create_default_catalog_and_schema](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.create_default_catalog_and_schema "method datafusion::prelude::SessionConfig::create_default_catalog_and_schema") which if enabled will be built here.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.session_id" class="fn">session_id</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns the current session_id value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.analyzer" class="fn">analyzer</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::Analyzer">Analyzer</a>\>

Returns the current analyzer value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.expr_planners" class="fn">expr_planners</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>\>\>

Returns the current expr_planners value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.type_planner" class="fn">type_planner</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.TypePlanner.html" class="trait" title="trait datafusion::logical_expr::planner::TypePlanner">TypePlanner</a>\>\>

Returns the current type_planner value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.optimizer" class="fn">optimizer</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>\>

Returns the current optimizer value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.physical_optimizers" class="fn">physical_optimizers</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/optimizer/struct.PhysicalOptimizer.html" class="struct" title="struct datafusion::physical_optimizer::optimizer::PhysicalOptimizer">PhysicalOptimizer</a>\>

Returns the current physical_optimizers value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.query_planner" class="fn">query_planner</a>( &mut self, ) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.QueryPlanner.html" class="trait" title="trait datafusion::execution::context::QueryPlanner">QueryPlanner</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>\>

Returns the current query_planner value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.catalog_list" class="fn">catalog_list</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html" class="trait" title="trait datafusion::catalog::CatalogProviderList">CatalogProviderList</a>\>\>

Returns the current catalog_list value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.table_functions" class="fn">table_functions</a>( &mut self, ) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html" class="struct" title="struct datafusion::catalog::TableFunction">TableFunction</a>\>\>\>

Returns the current table_functions value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.scalar_functions" class="fn">scalar_functions</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>\>

Returns the current scalar_functions value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.aggregate_functions" class="fn">aggregate_functions</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>\>

Returns the current aggregate_functions value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.window_functions" class="fn">window_functions</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>\>

Returns the current window_functions value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.serializer_registry" class="fn">serializer_registry</a>( &mut self, ) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.SerializerRegistry.html" class="trait" title="trait datafusion::execution::registry::SerializerRegistry">SerializerRegistry</a>\>\>

Returns the current serializer_registry value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.file_formats" class="fn">file_formats</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html" class="trait" title="trait datafusion::datasource::file_format::FileFormatFactory">FileFormatFactory</a>\>\>\>

Returns the current file_formats value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.config" class="fn">config</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>\>

Returns the current session_config value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.table_options" class="fn">table_options</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>\>

Returns the current table_options value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.execution_props" class="fn">execution_props</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>\>

Returns the current execution_props value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.table_factories" class="fn">table_factories</a>( &mut self, ) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a>\>\>\>

Returns the current table_factories value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.runtime_env" class="fn">runtime_env</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>\>

Returns the current runtime_env value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.function_factory" class="fn">function_factory</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html" class="trait" title="trait datafusion::execution::context::FunctionFactory">FunctionFactory</a>\>\>

Returns the current function_factory value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.analyzer_rules" class="fn">analyzer_rules</a>( &mut self, ) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>\>\>

Returns the current analyzer_rules value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.optimizer_rules" class="fn">optimizer_rules</a>( &mut self, ) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>\>\>

Returns the current optimizer_rules value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.physical_optimizer_rules" class="fn">physical_optimizer_rules</a>( &mut self, ) -\> &mut <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>\>\>

Returns the current physical_optimizer_rules value

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#impl-Debug-for-SessionStateBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html" class="struct" title="struct datafusion::execution::session_state::SessionStateBuilder">SessionStateBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Prefer having short fields at the top and long vector fields near the end Group fields by

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#impl-Default-for-SessionStateBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html" class="struct" title="struct datafusion::execution::session_state::SessionStateBuilder">SessionStateBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#impl-From%3CSessionContext%3E-for-SessionStateBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html" class="struct" title="struct datafusion::execution::session_state::SessionStateBuilder">SessionStateBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(session: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#impl-From%3CSessionState%3E-for-SessionStateBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html" class="struct" title="struct datafusion::execution::session_state::SessionStateBuilder">SessionStateBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html#blanket-implementations" class="anchor">§</a>
