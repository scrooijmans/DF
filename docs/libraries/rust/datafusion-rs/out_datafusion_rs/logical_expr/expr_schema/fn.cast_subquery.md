# Function cast_subquery Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_schema.rs.html#699" class="src">Source</a>

``` rust
pub fn cast_subquery(
    subquery: Subquery,
    cast_to_type: &DataType,
) -> Result<Subquery, DataFusionError>
```

Expand description

Cast subquery in InSubquery/ScalarSubquery to a given type.

1.  **Projection plan**: If the subquery is a projection (i.e. a SELECT statement with specific columns), it casts the first expression in the projection to the target type and creates a new projection with the casted expression.
2.  **Non-projection plan**: If the subquery isn’t a projection, it adds a projection to the plan with the casted first column.
