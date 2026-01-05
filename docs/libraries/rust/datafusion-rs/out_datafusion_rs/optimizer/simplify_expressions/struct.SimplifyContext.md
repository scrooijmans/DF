# Struct SimplifyContext Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/simplify.rs.html#53" class="src">Source</a>

``` rust
pub struct SimplifyContext<'a> { /* private fields */ }
```

Expand description

Provides simplification information based on DFSchema and [`ExecutionProps`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html "struct datafusion::execution::context::ExecutionProps"). This is the default implementation used by DataFusion

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#example" class="doc-anchor">§</a>Example

See the `simplify_demo` in the [`expr_api` example](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/expr_api.rs)

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#impl-SimplifyContext%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html" class="struct" title="struct datafusion::logical_expr::simplify::SimplifyContext">SimplifyContext</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#method.new" class="fn">new</a>(props: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html" class="struct" title="struct datafusion::logical_expr::simplify::SimplifyContext">SimplifyContext</a>\<'a\>

Create a new SimplifyContext

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#method.with_schema" class="fn">with_schema</a>(self, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html" class="struct" title="struct datafusion::logical_expr::simplify::SimplifyContext">SimplifyContext</a>\<'a\>

Register a [`DFSchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/type.DFSchemaRef.html "type datafusion::common::DFSchemaRef") with this context

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#impl-Clone-for-SimplifyContext%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html" class="struct" title="struct datafusion::logical_expr::simplify::SimplifyContext">SimplifyContext</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html" class="struct" title="struct datafusion::logical_expr::simplify::SimplifyContext">SimplifyContext</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#impl-Debug-for-SimplifyContext%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html" class="struct" title="struct datafusion::logical_expr::simplify::SimplifyContext">SimplifyContext</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#impl-SimplifyInfo-for-SimplifyContext%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html" class="trait" title="trait datafusion::logical_expr::simplify::SimplifyInfo">SimplifyInfo</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html" class="struct" title="struct datafusion::logical_expr::simplify::SimplifyContext">SimplifyContext</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#method.is_boolean_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html#tymethod.is_boolean_type" class="fn">is_boolean_type</a>(&self, expr: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns true if this Expr has boolean type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#method.nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html#tymethod.nullable" class="fn">nullable</a>(&self, expr: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns true if expr is nullable

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#method.get_data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html#tymethod.get_data_type" class="fn">get_data_type</a>(&self, expr: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns data type of this expr needed for determining optimized int type of a value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#method.execution_props" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html#tymethod.execution_props" class="fn">execution_props</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

Returns details needed for partial expression evaluation

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html#blanket-implementations" class="anchor">§</a>
