# Function create_physical_expr Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/planner.rs.html#109-113" class="src">Source</a>

``` rust
pub fn create_physical_expr(
    e: &Expr,
    input_dfschema: &DFSchema,
    execution_props: &ExecutionProps,
) -> Result<Arc<dyn PhysicalExpr>, DataFusionError>
```

Expand description

[PhysicalExpr](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") evaluate DataFusion expressions such as `A + 1`, or `CAST(c1 AS int)`.

[PhysicalExpr](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") are the physical counterpart to [Expr](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") used in logical planning, and can be evaluated directly on a [RecordBatch](https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatch.html). They are normally created from [Expr](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") by a [PhysicalPlanner](https://docs.rs/datafusion/latest/datafusion/physical_planner/trait.PhysicalPlanner.html) and can be created directly using [create_physical_expr](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_physical_expr.html "fn datafusion::physical_expr::create_physical_expr").

A Physical expression knows its type, nullability and how to evaluate itself.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_physical_expr.html#example-create-physicalexpr-from-expr" class="doc-anchor">§</a>Example: Create `PhysicalExpr` from `Expr`

``` rust
// For a logical expression `a = 1`, we can create a physical expression
let expr = col("a").eq(lit(1));
// To create a PhysicalExpr we need 1. a schema
let schema = Schema::new(vec![Field::new("a", DataType::Int32, true)]);
let df_schema = DFSchema::try_from(schema).unwrap();
// 2. ExecutionProps
let props = ExecutionProps::new();
// We can now create a PhysicalExpr:
let physical_expr = create_physical_expr(&expr, &df_schema, &props).unwrap();
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_physical_expr.html#example-executing-a-physicalexpr-to-obtain-columnarvalue" class="doc-anchor">§</a>Example: Executing a PhysicalExpr to obtain [ColumnarValue](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html "enum datafusion::logical_expr::ColumnarValue")

``` rust
// Given a PhysicalExpr, for `a = 1` we can evaluate it against a RecordBatch like this:
let physical_expr = create_physical_expr(&expr, &df_schema, &props).unwrap();
// Input of [1,2,3]
let input_batch = RecordBatch::try_from_iter(vec![
  ("a", Arc::new(Int32Array::from(vec![1, 2, 3])) as _)
]).unwrap();
// The result is a ColumnarValue (either an Array or a Scalar)
let result = physical_expr.evaluate(&input_batch).unwrap();
// In this case, a BooleanArray with the result of the comparison
let ColumnarValue::Array(arr) = result else {
 panic!("Expected an array")
};
assert_eq!(arr.as_boolean(), &BooleanArray::from(vec![true, false, false]));
```

Create a physical expression from a logical expression ([Expr](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_physical_expr.html#arguments" class="doc-anchor">§</a>Arguments

- `e` - The logical expression
- `input_dfschema` - The DataFusion schema for the input, used to resolve `Column` references to qualified or unqualified fields by name.
