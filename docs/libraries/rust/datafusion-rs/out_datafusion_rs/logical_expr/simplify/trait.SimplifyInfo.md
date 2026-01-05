# Trait SimplifyInfo Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/simplify.rs.html#31" class="src">Source</a>

``` rust
pub trait SimplifyInfo {
    // Required methods
    fn is_boolean_type(&self, expr: &Expr) -> Result<bool, DataFusionError>;
    fn nullable(&self, expr: &Expr) -> Result<bool, DataFusionError>;
    fn execution_props(&self) -> &ExecutionProps;
    fn get_data_type(&self, expr: &Expr) -> Result<DataType, DataFusionError>;
}
```

Expand description

Provides the information necessary to apply algebraic simplification to an [Expr](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr"). See [SimplifyContext](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html "struct datafusion::logical_expr::simplify::SimplifyContext") for one concrete implementation.

This trait exists so that other systems can plug schema information in without having to create `DFSchema` objects. If you have a [`DFSchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/type.DFSchemaRef.html "type datafusion::common::DFSchemaRef") you can use [`SimplifyContext`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html "struct datafusion::logical_expr::simplify::SimplifyContext")

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html#tymethod.is_boolean_type" class="fn">is_boolean_type</a>(&self, expr: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns true if this Expr has boolean type

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html#tymethod.nullable" class="fn">nullable</a>(&self, expr: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns true of this expr is nullable (could possibly be NULL)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html#tymethod.execution_props" class="fn">execution_props</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

Returns details needed for partial expression evaluation

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html#tymethod.get_data_type" class="fn">get_data_type</a>(&self, expr: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns data type of this expr needed for determining optimized int type of a value

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html#impl-SimplifyInfo-for-SimplifyContext%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html" class="trait" title="trait datafusion::logical_expr::simplify::SimplifyInfo">SimplifyInfo</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html" class="struct" title="struct datafusion::logical_expr::simplify::SimplifyContext">SimplifyContext</a>\<'\_\>
