# Trait ExprPlanner Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/planner.rs.html#120" class="src">Source</a>

``` rust
pub trait ExprPlanner:
    Debug
    + Send
    + Sync {
Show 14 methods    // Provided methods
    fn plan_binary_op(
        &self,
        expr: RawBinaryExpr,
        _schema: &DFSchema,
    ) -> Result<PlannerResult<RawBinaryExpr>, DataFusionError> { ... }
    fn plan_field_access(
        &self,
        expr: RawFieldAccessExpr,
        _schema: &DFSchema,
    ) -> Result<PlannerResult<RawFieldAccessExpr>, DataFusionError> { ... }
    fn plan_array_literal(
        &self,
        exprs: Vec<Expr>,
        _schema: &DFSchema,
    ) -> Result<PlannerResult<Vec<Expr>>, DataFusionError> { ... }
    fn plan_position(
        &self,
        args: Vec<Expr>,
    ) -> Result<PlannerResult<Vec<Expr>>, DataFusionError> { ... }
    fn plan_dictionary_literal(
        &self,
        expr: RawDictionaryExpr,
        _schema: &DFSchema,
    ) -> Result<PlannerResult<RawDictionaryExpr>, DataFusionError> { ... }
    fn plan_extract(
        &self,
        args: Vec<Expr>,
    ) -> Result<PlannerResult<Vec<Expr>>, DataFusionError> { ... }
    fn plan_substring(
        &self,
        args: Vec<Expr>,
    ) -> Result<PlannerResult<Vec<Expr>>, DataFusionError> { ... }
    fn plan_struct_literal(
        &self,
        args: Vec<Expr>,
        _is_named_struct: bool,
    ) -> Result<PlannerResult<Vec<Expr>>, DataFusionError> { ... }
    fn plan_overlay(
        &self,
        args: Vec<Expr>,
    ) -> Result<PlannerResult<Vec<Expr>>, DataFusionError> { ... }
    fn plan_make_map(
        &self,
        args: Vec<Expr>,
    ) -> Result<PlannerResult<Vec<Expr>>, DataFusionError> { ... }
    fn plan_compound_identifier(
        &self,
        _field: &Field,
        _qualifier: Option<&TableReference>,
        _nested_names: &[String],
    ) -> Result<PlannerResult<Vec<Expr>>, DataFusionError> { ... }
    fn plan_any(
        &self,
        expr: RawBinaryExpr,
    ) -> Result<PlannerResult<RawBinaryExpr>, DataFusionError> { ... }
    fn plan_aggregate(
        &self,
        expr: RawAggregateExpr,
    ) -> Result<PlannerResult<RawAggregateExpr>, DataFusionError> { ... }
    fn plan_window(
        &self,
        expr: RawWindowExpr,
    ) -> Result<PlannerResult<RawWindowExpr>, DataFusionError> { ... }
}
```

Expand description

Customize planning of SQL AST expressions to [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_binary_op" class="fn">plan_binary_op</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawBinaryExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawBinaryExpr">RawBinaryExpr</a>, \_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawBinaryExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawBinaryExpr">RawBinaryExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plan the binary operation between two expressions, returns original BinaryExpr if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_field_access" class="fn">plan_field_access</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawFieldAccessExpr">RawFieldAccessExpr</a>, \_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawFieldAccessExpr">RawFieldAccessExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plan the field access expression, such as `foo.bar`

returns original [`RawFieldAccessExpr`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html "struct datafusion::logical_expr::planner::RawFieldAccessExpr") if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_array_literal" class="fn">plan_array_literal</a>( &self, exprs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, \_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plan an array literal, such as `[1, 2, 3]`

Returns original expression arguments if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_position" class="fn">plan_position</a>( &self, args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plan a `POSITION` expression, such as `POSITION(<expr> in <expr>)`

Returns original expression arguments if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_dictionary_literal" class="fn">plan_dictionary_literal</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawDictionaryExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawDictionaryExpr">RawDictionaryExpr</a>, \_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawDictionaryExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawDictionaryExpr">RawDictionaryExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plan a dictionary literal, such as `{ key: value, ...}`

Returns original expression arguments if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_extract" class="fn">plan_extract</a>( &self, args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plan an extract expression, such as`EXTRACT(month FROM foo)`

Returns original expression arguments if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_substring" class="fn">plan_substring</a>( &self, args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plan an substring expression, such as `SUBSTRING(<expr> [FROM <expr>] [FOR <expr>])`

Returns original expression arguments if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_struct_literal" class="fn">plan_struct_literal</a>( &self, args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, \_is_named_struct: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plans a struct literal, such as `{'field1' : expr1, 'field2' : expr2, ...}`

This function takes a vector of expressions and a boolean flag indicating whether the struct uses the optional name

Returns the original input expressions if planning is not possible.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_overlay" class="fn">plan_overlay</a>( &self, args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plans an overlay expression, such as `overlay(str PLACING substr FROM pos [FOR count])`

Returns original expression arguments if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_make_map" class="fn">plan_make_map</a>( &self, args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plans a `make_map` expression, such as `make_map(key1, value1, key2, value2, ...)`

Returns original expression arguments if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_compound_identifier" class="fn">plan_compound_identifier</a>( &self, \_field: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>, \_qualifier: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, \_nested_names: &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plans compound identifier such as `db.schema.table` for non-empty nested names

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#note" class="doc-anchor">§</a>Note:

Currently compound identifier for outer query schema is not supported.

Returns original expression if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_any" class="fn">plan_any</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawBinaryExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawBinaryExpr">RawBinaryExpr</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawBinaryExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawBinaryExpr">RawBinaryExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plans `ANY` expression, such as `expr = ANY(array_expr)`

Returns origin binary expression if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_aggregate" class="fn">plan_aggregate</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawAggregateExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawAggregateExpr">RawAggregateExpr</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawAggregateExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawAggregateExpr">RawAggregateExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plans aggregate functions, such as `COUNT(<expr>)`

Returns original expression arguments if not possible

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#method.plan_window" class="fn">plan_window</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawWindowExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawWindowExpr">RawWindowExpr</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/enum.PlannerResult.html" class="enum" title="enum datafusion::logical_expr::planner::PlannerResult">PlannerResult</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawWindowExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawWindowExpr">RawWindowExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Plans window functions, such as `COUNT(<expr>)`

Returns original expression arguments if not possible

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#impl-ExprPlanner-for-CoreFunctionPlanner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/planner/struct.CoreFunctionPlanner.html" class="struct" title="struct datafusion::functions::core::planner::CoreFunctionPlanner">CoreFunctionPlanner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#impl-ExprPlanner-for-DatetimeFunctionPlanner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/planner/struct.DatetimeFunctionPlanner.html" class="struct" title="struct datafusion::functions::datetime::planner::DatetimeFunctionPlanner">DatetimeFunctionPlanner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#impl-ExprPlanner-for-UserDefinedFunctionPlanner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/planner/struct.UserDefinedFunctionPlanner.html" class="struct" title="struct datafusion::functions::planner::UserDefinedFunctionPlanner">UserDefinedFunctionPlanner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#impl-ExprPlanner-for-UnicodeFunctionPlanner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/planner/struct.UnicodeFunctionPlanner.html" class="struct" title="struct datafusion::functions::unicode::planner::UnicodeFunctionPlanner">UnicodeFunctionPlanner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#impl-ExprPlanner-for-AggregateFunctionPlanner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/planner/struct.AggregateFunctionPlanner.html" class="struct" title="struct datafusion::functions_aggregate::planner::AggregateFunctionPlanner">AggregateFunctionPlanner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#impl-ExprPlanner-for-FieldAccessPlanner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/planner/struct.FieldAccessPlanner.html" class="struct" title="struct datafusion::functions_nested::planner::FieldAccessPlanner">FieldAccessPlanner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#impl-ExprPlanner-for-NestedFunctionPlanner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/planner/struct.NestedFunctionPlanner.html" class="struct" title="struct datafusion::functions_nested::planner::NestedFunctionPlanner">NestedFunctionPlanner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html#impl-ExprPlanner-for-WindowFunctionPlanner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html" class="trait" title="trait datafusion::logical_expr::planner::ExprPlanner">ExprPlanner</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/planner/struct.WindowFunctionPlanner.html" class="struct" title="struct datafusion::functions_window::planner::WindowFunctionPlanner">WindowFunctionPlanner</a>
