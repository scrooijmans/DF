# Function snapshot_physical_expr Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/physical_expr.rs.html#539-541" class="src">Source</a>

``` rust
pub fn snapshot_physical_expr(
    expr: Arc<dyn PhysicalExpr>,
) -> Result<Arc<dyn PhysicalExpr>, DataFusionError>
```

Expand description

Take a snapshot of the given `PhysicalExpr` if it is dynamic.

Take a snapshot of this `PhysicalExpr` if it is dynamic. This is used to capture the current state of `PhysicalExpr`s that may contain dynamic references to other operators in order to serialize it over the wire or treat it via downcast matching.

See the documentation of [`PhysicalExpr::snapshot`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.snapshot "method datafusion::physical_expr::PhysicalExpr::snapshot") for more details.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.snapshot_physical_expr.html#returns" class="doc-anchor">§</a>Returns

Returns an `Option<Arc<dyn PhysicalExpr>>` which is the snapshot of the `PhysicalExpr` if it is dynamic. If the `PhysicalExpr` does not have any dynamic references or state, it returns `None`.
