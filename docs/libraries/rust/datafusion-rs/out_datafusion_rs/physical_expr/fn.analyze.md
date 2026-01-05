# Function analyze Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/analysis.rs.html#163-167" class="src">Source</a>

``` rust
pub fn analyze(
    expr: &Arc<dyn PhysicalExpr>,
    context: AnalysisContext,
    schema: &Schema,
) -> Result<AnalysisContext, DataFusionError>
```

Expand description

Attempts to refine column boundaries and compute a selectivity value.

The function accepts boundaries of the input columns in the `context` parameter. It then tries to tighten these boundaries based on the provided `expr`. The resulting selectivity value is calculated by comparing the initial and final boundaries. The computation assumes that the data within the column is uniformly distributed and not sorted.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.analyze.html#arguments" class="doc-anchor">§</a>Arguments

- `context` - The context holding input column boundaries.
- `expr` - The expression used to shrink the column boundaries.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.analyze.html#returns" class="doc-anchor">§</a>Returns

- `AnalysisContext` constructed by pruned boundaries and a selectivity value.
