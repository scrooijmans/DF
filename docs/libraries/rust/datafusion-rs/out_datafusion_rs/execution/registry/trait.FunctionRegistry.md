# Trait FunctionRegistry Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/registry.rs.html#29" class="src">Source</a>

``` rust
pub trait FunctionRegistry {
Show 15 methods    // Required methods
    fn udfs(&self) -> HashSet<String>;
    fn udf(&self, name: &str) -> Result<Arc<ScalarUDF>, DataFusionError>;
    fn udaf(&self, name: &str) -> Result<Arc<AggregateUDF>, DataFusionError>;
    fn udwf(&self, name: &str) -> Result<Arc<WindowUDF>, DataFusionError>;
    fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>>;

    // Provided methods
    fn udafs(&self) -> HashSet<String> { ... }
    fn udwfs(&self) -> HashSet<String> { ... }
    fn register_udf(
        &mut self,
        _udf: Arc<ScalarUDF>,
    ) -> Result<Option<Arc<ScalarUDF>>, DataFusionError> { ... }
    fn register_udaf(
        &mut self,
        _udaf: Arc<AggregateUDF>,
    ) -> Result<Option<Arc<AggregateUDF>>, DataFusionError> { ... }
    fn register_udwf(
        &mut self,
        _udaf: Arc<WindowUDF>,
    ) -> Result<Option<Arc<WindowUDF>>, DataFusionError> { ... }
    fn deregister_udf(
        &mut self,
        _name: &str,
    ) -> Result<Option<Arc<ScalarUDF>>, DataFusionError> { ... }
    fn deregister_udaf(
        &mut self,
        _name: &str,
    ) -> Result<Option<Arc<AggregateUDF>>, DataFusionError> { ... }
    fn deregister_udwf(
        &mut self,
        _name: &str,
    ) -> Result<Option<Arc<WindowUDF>>, DataFusionError> { ... }
    fn register_function_rewrite(
        &mut self,
        _rewrite: Arc<dyn FunctionRewrite + Sync + Send>,
    ) -> Result<(), DataFusionError> { ... }
    fn register_expr_planner(
        &mut self,
        _expr_planner: Arc<dyn ExprPlanner>,
    ) -> Result<(), DataFusionError> { ... }
}
```

Expand description

A registry knows how to build logical expressions out of user-defined function’ names

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#tymethod.udfs" class="fn">udfs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available scalar user defined functions.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#tymethod.udf" class="fn">udf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a reference to the user defined scalar function (udf) named `name`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#tymethod.udaf" class="fn">udaf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a reference to the user defined aggregate function (udaf) named `name`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#tymethod.udwf" class="fn">udwf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a reference to the user defined window function (udwf) named `name`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#tymethod.expr_planners" class="fn">expr_planners</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>\>

Set of all registered [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner")s

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#method.udafs" class="fn">udafs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available aggregate user defined functions.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#method.udwfs" class="fn">udwfs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available window user defined functions.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#method.register_udf" class="fn">register_udf</a>( &mut self, \_udf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Registers a new [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF"), returning any previously registered implementation.

Returns an error (the default) if the function can not be registered, for example if the registry is read only.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#method.register_udaf" class="fn">register_udaf</a>( &mut self, \_udaf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Registers a new [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF"), returning any previously registered implementation.

Returns an error (the default) if the function can not be registered, for example if the registry is read only.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#method.register_udwf" class="fn">register_udwf</a>( &mut self, \_udaf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Registers a new [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF"), returning any previously registered implementation.

Returns an error (the default) if the function can not be registered, for example if the registry is read only.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#method.deregister_udf" class="fn">deregister_udf</a>( &mut self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Deregisters a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF"), returning the implementation that was deregistered.

Returns an error (the default) if the function can not be deregistered, for example if the registry is read only.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#method.deregister_udaf" class="fn">deregister_udaf</a>( &mut self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Deregisters a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF"), returning the implementation that was deregistered.

Returns an error (the default) if the function can not be deregistered, for example if the registry is read only.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#method.deregister_udwf" class="fn">deregister_udwf</a>( &mut self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Deregisters a [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF"), returning the implementation that was deregistered.

Returns an error (the default) if the function can not be deregistered, for example if the registry is read only.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#method.register_function_rewrite" class="fn">register_function_rewrite</a>( &mut self, \_rewrite: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html" class="trait" title="trait datafusion::logical_expr::expr_rewriter::FunctionRewrite">FunctionRewrite</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Registers a new [`FunctionRewrite`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html "trait datafusion::logical_expr::expr_rewriter::FunctionRewrite") with the registry.

`FunctionRewrite` rules are used to rewrite certain / operators in the logical plan to function calls. For example `a || b` might be written to `array_concat(a, b)`.

This allows the behavior of operators to be customized by the user.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#method.register_expr_planner" class="fn">register_expr_planner</a>( &mut self, \_expr_planner: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Registers a new [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner") with the registry.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#impl-FunctionRegistry-for-SessionContext" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#impl-FunctionRegistry-for-SessionState" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#impl-FunctionRegistry-for-TaskContext" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html#impl-FunctionRegistry-for-MemoryFunctionRegistry" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html" class="struct" title="struct datafusion::execution::registry::MemoryFunctionRegistry">MemoryFunctionRegistry</a>
