# Struct MemoryFunctionRegistry Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/registry.rs.html#162" class="src">Source</a>

``` rust
pub struct MemoryFunctionRegistry { /* private fields */ }
```

Expand description

A [`FunctionRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html "trait datafusion::execution::FunctionRegistry") that uses in memory [`HashMap`](https://docs.rs/datafusion/50.2.0/datafusion/common/type.HashMap.html "type datafusion::common::HashMap")s

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#impl-MemoryFunctionRegistry" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html" class="struct" title="struct datafusion::execution::registry::MemoryFunctionRegistry">MemoryFunctionRegistry</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html" class="struct" title="struct datafusion::execution::registry::MemoryFunctionRegistry">MemoryFunctionRegistry</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#impl-Debug-for-MemoryFunctionRegistry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html" class="struct" title="struct datafusion::execution::registry::MemoryFunctionRegistry">MemoryFunctionRegistry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#impl-Default-for-MemoryFunctionRegistry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html" class="struct" title="struct datafusion::execution::registry::MemoryFunctionRegistry">MemoryFunctionRegistry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html" class="struct" title="struct datafusion::execution::registry::MemoryFunctionRegistry">MemoryFunctionRegistry</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#impl-FunctionRegistry-for-MemoryFunctionRegistry" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html" class="struct" title="struct datafusion::execution::registry::MemoryFunctionRegistry">MemoryFunctionRegistry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.udfs" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udfs" class="fn">udfs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available scalar user defined functions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.udf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udf" class="fn">udf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a reference to the user defined scalar function (udf) named `name`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.udaf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udaf" class="fn">udaf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a reference to the user defined aggregate function (udaf) named `name`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.udwf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.udwf" class="fn">udwf</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a reference to the user defined window function (udwf) named `name`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.register_udf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udf" class="fn">register_udf</a>( &mut self, udf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Registers a new [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF"), returning any previously registered implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.register_udaf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udaf" class="fn">register_udaf</a>( &mut self, udaf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Registers a new [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF"), returning any previously registered implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udaf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.register_udwf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udwf" class="fn">register_udwf</a>( &mut self, udaf: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Registers a new [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF"), returning any previously registered implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_udwf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.expr_planners" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#tymethod.expr_planners" class="fn">expr_planners</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>\>

Set of all registered [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner")s

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.udafs" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.udafs" class="fn">udafs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available aggregate user defined functions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.udwfs" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.udwfs" class="fn">udwfs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns names of all available window user defined functions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.deregister_udf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udf" class="fn">deregister_udf</a>( &mut self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html" class="struct" title="struct datafusion::logical_expr::ScalarUDF">ScalarUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Deregisters a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF"), returning the implementation that was deregistered. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.deregister_udaf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udaf" class="fn">deregister_udaf</a>( &mut self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Deregisters a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF"), returning the implementation that was deregistered. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udaf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.deregister_udwf" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udwf" class="fn">deregister_udwf</a>( &mut self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Deregisters a [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF"), returning the implementation that was deregistered. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.deregister_udwf)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.register_function_rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_function_rewrite" class="fn">register_function_rewrite</a>( &mut self, \_rewrite: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html" class="trait" title="trait datafusion::logical_expr::expr_rewriter::FunctionRewrite">FunctionRewrite</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Registers a new [`FunctionRewrite`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html "trait datafusion::logical_expr::expr_rewriter::FunctionRewrite") with the registry. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_function_rewrite)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#method.register_expr_planner" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html#method.register_expr_planner" class="fn">register_expr_planner</a>( &mut self, \_expr_planner: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Registers a new [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner") with the registry.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html#blanket-implementations" class="anchor">§</a>
