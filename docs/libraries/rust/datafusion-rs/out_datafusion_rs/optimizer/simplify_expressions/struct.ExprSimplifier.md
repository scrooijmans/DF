# Struct ExprSimplifier Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/simplify_expressions/expr_simplifier.rs.html#98" class="src">Source</a>

``` rust
pub struct ExprSimplifier<S> { /* private fields */ }
```

Expand description

This structure handles API for expression simplification

Provides simplification information based on DFSchema and [`ExecutionProps`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html "struct datafusion::execution::context::ExecutionProps"). This is the default implementation used by DataFusion

For example:

``` rust
use arrow::datatypes::{Schema, Field, DataType};
use datafusion_expr::{col, lit};
use datafusion_common::{DataFusionError, ToDFSchema};
use datafusion_expr::execution_props::ExecutionProps;
use datafusion_expr::simplify::SimplifyContext;
use datafusion_optimizer::simplify_expressions::ExprSimplifier;

// Create the schema
let schema = Schema::new(vec![
    Field::new("i", DataType::Int64, false),
  ])
  .to_dfschema_ref().unwrap();

// Create the simplifier
let props = ExecutionProps::new();
let context = SimplifyContext::new(&props)
   .with_schema(schema);
let simplifier = ExprSimplifier::new(context);

// Use the simplifier

// b < 2 or (1 > 3)
let expr = col("b").lt(lit(2)).or(lit(1).gt(lit(3)));

// b < 2
let simplified = simplifier.simplify(expr).unwrap();
assert_eq!(simplified, col("b").lt(lit(2)));
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#impl-ExprSimplifier%3CS%3E" class="anchor">§</a>

### impl\<S\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html" class="struct" title="struct datafusion::optimizer::simplify_expressions::ExprSimplifier">ExprSimplifier</a>\<S\>

where S: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html" class="trait" title="trait datafusion::logical_expr::simplify::SimplifyInfo">SimplifyInfo</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.new" class="fn">new</a>(info: S) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html" class="struct" title="struct datafusion::optimizer::simplify_expressions::ExprSimplifier">ExprSimplifier</a>\<S\>

Create a new `ExprSimplifier` with the given `info` such as an instance of [`SimplifyContext`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html "struct datafusion::logical_expr::simplify::SimplifyContext"). See [`simplify`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.simplify "method datafusion::optimizer::simplify_expressions::ExprSimplifier::simplify") for an example.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.simplify" class="fn">simplify</a>(&self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Simplifies this [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") as much as possible, evaluating constants and applying algebraic simplifications.

The types of the expression must match what operators expect, or else an error may occur trying to evaluate. See [`coerce`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.coerce "method datafusion::optimizer::simplify_expressions::ExprSimplifier::coerce") for a function to help.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#example" class="doc-anchor">§</a>Example:

`b > 2 AND b > 2`

can be written to

`b > 2`

``` rust
use arrow::datatypes::DataType;
use datafusion_expr::{col, lit, Expr};
use datafusion_common::Result;
use datafusion_expr::execution_props::ExecutionProps;
use datafusion_expr::simplify::SimplifyContext;
use datafusion_expr::simplify::SimplifyInfo;
use datafusion_optimizer::simplify_expressions::ExprSimplifier;
use datafusion_common::DFSchema;
use std::sync::Arc;

/// Simple implementation that provides `Simplifier` the information it needs
/// See SimplifyContext for a structure that does this.
#[derive(Default)]
struct Info {
  execution_props: ExecutionProps,
};

impl SimplifyInfo for Info {
  fn is_boolean_type(&self, expr: &Expr) -> Result<bool> {
    Ok(false)
  }
  fn nullable(&self, expr: &Expr) -> Result<bool> {
    Ok(true)
  }
  fn execution_props(&self) -> &ExecutionProps {
    &self.execution_props
  }
  fn get_data_type(&self, expr: &Expr) -> Result<DataType> {
    Ok(DataType::Int32)
  }
}

// Create the simplifier
let simplifier = ExprSimplifier::new(Info::default());

// b < 2
let b_lt_2 = col("b").gt(lit(2));

// (b < 2) OR (b < 2)
let expr = b_lt_2.clone().or(b_lt_2.clone());

// (b < 2) OR (b < 2) --> (b < 2)
let expr = simplifier.simplify(expr).unwrap();
assert_eq!(expr, b_lt_2);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.simplify_with_cycle_count" class="fn">simplify_with_cycle_count</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 48.0.0: Use `simplify_with_cycle_count_transformed` instead

Like [Self::simplify](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.simplify "method datafusion::optimizer::simplify_expressions::ExprSimplifier::simplify"), simplifies this [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") as much as possible, evaluating constants and applying algebraic simplifications. Additionally returns a `u32` representing the number of simplification cycles performed, which can be useful for testing optimizations.

See [Self::simplify](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.simplify "method datafusion::optimizer::simplify_expressions::ExprSimplifier::simplify") for details and usage examples.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.simplify_with_cycle_count_transformed" class="fn">simplify_with_cycle_count_transformed</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Like [Self::simplify](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.simplify "method datafusion::optimizer::simplify_expressions::ExprSimplifier::simplify"), simplifies this [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") as much as possible, evaluating constants and applying algebraic simplifications. Additionally returns a `u32` representing the number of simplification cycles performed, which can be useful for testing optimizations.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#returns" class="doc-anchor">§</a>Returns

A tuple containing:

- The simplified expression wrapped in a `Transformed<Expr>` indicating if changes were made
- The number of simplification cycles that were performed

See [Self::simplify](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.simplify "method datafusion::optimizer::simplify_expressions::ExprSimplifier::simplify") for details and usage examples.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.coerce" class="fn">coerce</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply type coercion to an [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") so that it can be evaluated as a [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr").

See the [type coercion module](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/index.html "mod datafusion::logical_expr::type_coercion") documentation for more details on type coercion

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.with_guarantees" class="fn">with_guarantees</a>( self, guarantees: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>)\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html" class="struct" title="struct datafusion::optimizer::simplify_expressions::ExprSimplifier">ExprSimplifier</a>\<S\>

Input guarantees about the values of columns.

The guarantees can simplify expressions. For example, if a column `x` is guaranteed to be `3`, then the expression `x > 1` can be replaced by the literal `true`.

The guarantees are provided as a `Vec<(Expr, NullableInterval)>`, where the [Expr](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") is a column reference and the [NullableInterval](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html "enum datafusion::logical_expr::interval_arithmetic::NullableInterval") is an interval representing the known possible values of that column.

``` rust
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_expr::{col, lit, Expr};
use datafusion_expr::interval_arithmetic::{Interval, NullableInterval};
use datafusion_common::{Result, ScalarValue, ToDFSchema};
use datafusion_expr::execution_props::ExecutionProps;
use datafusion_expr::simplify::SimplifyContext;
use datafusion_optimizer::simplify_expressions::ExprSimplifier;

let schema = Schema::new(vec![
  Field::new("x", DataType::Int64, false),
  Field::new("y", DataType::UInt32, false),
  Field::new("z", DataType::Int64, false),
  ])
  .to_dfschema_ref().unwrap();

// Create the simplifier
let props = ExecutionProps::new();
let context = SimplifyContext::new(&props)
   .with_schema(schema);

// Expression: (x >= 3) AND (y + 2 < 10) AND (z > 5)
let expr_x = col("x").gt_eq(lit(3_i64));
let expr_y = (col("y") + lit(2_u32)).lt(lit(10_u32));
let expr_z = col("z").gt(lit(5_i64));
let expr = expr_x.and(expr_y).and(expr_z.clone());

let guarantees = vec![
   // x ∈ [3, 5]
   (
       col("x"),
       NullableInterval::NotNull {
           values: Interval::make(Some(3_i64), Some(5_i64)).unwrap()
       }
   ),
   // y = 3
   (col("y"), NullableInterval::from(ScalarValue::UInt32(Some(3)))),
];
let simplifier = ExprSimplifier::new(context).with_guarantees(guarantees);
let output = simplifier.simplify(expr).unwrap();
// Expression becomes: true AND true AND (z > 5), which simplifies to
// z > 5.
assert_eq!(output, expr_z);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.with_canonicalize" class="fn">with_canonicalize</a>(self, canonicalize: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html" class="struct" title="struct datafusion::optimizer::simplify_expressions::ExprSimplifier">ExprSimplifier</a>\<S\>

Should `Canonicalizer` be applied before simplification?

If true (the default), the expression will be rewritten to canonical form before simplification. This is useful to ensure that the simplifier can apply all possible simplifications.

Some expressions, such as those in some Joins, can not be canonicalized without changing their meaning. In these cases, canonicalization should be disabled.

``` rust
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_expr::{col, lit, Expr};
use datafusion_expr::interval_arithmetic::{Interval, NullableInterval};
use datafusion_common::{Result, ScalarValue, ToDFSchema};
use datafusion_expr::execution_props::ExecutionProps;
use datafusion_expr::simplify::SimplifyContext;
use datafusion_optimizer::simplify_expressions::ExprSimplifier;

let schema = Schema::new(vec![
  Field::new("a", DataType::Int64, false),
  Field::new("b", DataType::Int64, false),
  Field::new("c", DataType::Int64, false),
  ])
  .to_dfschema_ref().unwrap();

// Create the simplifier
let props = ExecutionProps::new();
let context = SimplifyContext::new(&props)
   .with_schema(schema);
let simplifier = ExprSimplifier::new(context);

// Expression: a = c AND 1 = b
let expr = col("a").eq(col("c")).and(lit(1).eq(col("b")));

// With canonicalization, the expression is rewritten to canonical form
// (though it is no simpler in this case):
let canonical = simplifier.simplify(expr.clone()).unwrap();
// Expression has been rewritten to: (c = a AND b = 1)
assert_eq!(canonical, col("c").eq(col("a")).and(col("b").eq(lit(1))));

// If canonicalization is disabled, the expression is not changed
let non_canonicalized = simplifier
  .with_canonicalize(false)
  .simplify(expr.clone())
  .unwrap();

assert_eq!(non_canonicalized, expr);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.with_max_cycles" class="fn">with_max_cycles</a>(self, max_simplifier_cycles: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html" class="struct" title="struct datafusion::optimizer::simplify_expressions::ExprSimplifier">ExprSimplifier</a>\<S\>

Specifies the maximum number of simplification cycles to run.

The simplifier can perform multiple passes of simplification. This is because the output of one simplification step can allow more optimizations in another simplification step. For example, constant evaluation can allow more expression simplifications, and expression simplifications can allow more constant evaluations.

This method specifies the maximum number of allowed iteration cycles before the simplifier returns an [Expr](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") output. However, it does not always perform the maximum number of cycles. The simplifier will attempt to detect when an [Expr](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") is unchanged by all the simplification passes, and return early. This avoids wasting time on unnecessary [Expr](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") tree traversals.

If no maximum is specified, the value of [DEFAULT_MAX_SIMPLIFIER_CYCLES](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/constant.DEFAULT_MAX_SIMPLIFIER_CYCLES.html "constant datafusion::optimizer::simplify_expressions::DEFAULT_MAX_SIMPLIFIER_CYCLES") is used instead.

``` rust
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_expr::{col, lit, Expr};
use datafusion_common::{Result, ScalarValue, ToDFSchema};
use datafusion_expr::execution_props::ExecutionProps;
use datafusion_expr::simplify::SimplifyContext;
use datafusion_optimizer::simplify_expressions::ExprSimplifier;

let schema = Schema::new(vec![
  Field::new("a", DataType::Int64, false),
  ])
  .to_dfschema_ref().unwrap();

// Create the simplifier
let props = ExecutionProps::new();
let context = SimplifyContext::new(&props)
   .with_schema(schema);
let simplifier = ExprSimplifier::new(context);

// Expression: a IS NOT NULL
let expr = col("a").is_not_null();

// When using default maximum cycles, 2 cycles will be performed.
let (simplified_expr, count) = simplifier.simplify_with_cycle_count_transformed(expr.clone()).unwrap();
assert_eq!(simplified_expr.data, lit(true));
// 2 cycles were executed, but only 1 was needed
assert_eq!(count, 2);

// Only 1 simplification pass is necessary here, so we can set the maximum cycles to 1.
let (simplified_expr, count) = simplifier.with_max_cycles(1).simplify_with_cycle_count_transformed(expr.clone()).unwrap();
// Expression has been rewritten to: (c = a AND b = 1)
assert_eq!(simplified_expr.data, lit(true));
// Only 1 cycle was executed
assert_eq!(count, 1);
```

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#blanket-implementations" class="anchor">§</a>
