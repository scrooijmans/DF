# Struct SessionStateDefaults Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/execution/session_state_defaults.rs.html#46" class="src">Source</a>

``` rust
pub struct SessionStateDefaults {}
```

Expand description

Defaults that are used as part of creating a SessionState such as table providers, file formats, registering of builtin functions, etc.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#impl-SessionStateDefaults" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html" class="struct" title="struct datafusion::execution::SessionStateDefaults">SessionStateDefaults</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.default_table_factories" class="fn">default_table_factories</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a>\>\>

returns a map of the default [`TableProviderFactory`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html "trait datafusion::catalog::TableProviderFactory")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.default_catalog" class="fn">default_catalog</a>( config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, table_factories: &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a>\>\>, runtime: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProvider">MemoryCatalogProvider</a>

returns the default MemoryCatalogProvider

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.default_expr_planners" class="fn">default_expr_planners</a>() -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>\>

returns the list of default [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.default_scalar_functions" class="fn">default_scalar_functions</a>() -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>

returns the list of default \[\`ScalarUDF’\]’s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.default_aggregate_functions" class="fn">default_aggregate_functions</a>() -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>

returns the list of default \[\`AggregateUDF’\]’s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.default_window_functions" class="fn">default_window_functions</a>() -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>

returns the list of default \[\`WindowUDF’\]’s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.default_table_functions" class="fn">default_table_functions</a>() -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html" class="struct" title="struct datafusion::catalog::TableFunction">TableFunction</a>\>\>

returns the list of default [`TableFunction`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html "struct datafusion::catalog::TableFunction")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.default_file_formats" class="fn">default_file_formats</a>() -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html" class="trait" title="trait datafusion::datasource::file_format::FileFormatFactory">FileFormatFactory</a>\>\>

returns the list of default \[\`FileFormatFactory’\]’s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.register_builtin_functions" class="fn">register_builtin_functions</a>(state: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>)

registers all builtin functions - scalar, array and aggregate

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.register_scalar_functions" class="fn">register_scalar_functions</a>(state: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>)

registers all the builtin scalar functions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.register_array_functions" class="fn">register_array_functions</a>(state: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>)

registers all the builtin array functions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.register_aggregate_functions" class="fn">register_aggregate_functions</a>(state: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>)

registers all the builtin aggregate functions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.register_default_schema" class="fn">register_default_schema</a>( config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, table_factories: &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a>\>\>, runtime: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>, default_catalog: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProvider">MemoryCatalogProvider</a>, )

registers the default schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#method.register_default_file_formats" class="fn">register_default_file_formats</a>(state: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>)

registers the default [`FileFormatFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormatFactory.html "trait datafusion::datasource::file_format::FileFormatFactory")s

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html#blanket-implementations" class="anchor">§</a>
