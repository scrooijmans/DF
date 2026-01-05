# Function projection_schema Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/plan.rs.html#2195" class="src">Source</a>

``` rust
pub fn projection_schema(
    input: &LogicalPlan,
    exprs: &[Expr],
) -> Result<Arc<DFSchema>, DataFusionError>
```

Expand description

Computes the schema of the result produced by applying a projection to the input logical plan.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/fn.projection_schema.html#arguments" class="doc-anchor">§</a>Arguments

- `input`: A reference to the input `LogicalPlan` for which the projection schema will be computed.
- `exprs`: A slice of `Expr` expressions representing the projection operation to apply.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/fn.projection_schema.html#returns" class="doc-anchor">§</a>Returns

A `Result` containing an `Arc<DFSchema>` representing the schema of the result produced by the projection operation. If the schema computation is successful, the `Result` will contain the schema; otherwise, it will contain an error.
