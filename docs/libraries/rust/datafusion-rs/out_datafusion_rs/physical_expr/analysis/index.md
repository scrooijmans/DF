# Module analysis Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/lib.rs.html#29" class="src">Source</a>

Expand description

Interval and selectivity in [`AnalysisContext`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html "struct datafusion::physical_expr::AnalysisContext")

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/analysis/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/analysis/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::analysis::AnalysisContext">AnalysisContext</a>  
The shared context used during the analysis of an expression. Includes the boundaries for all known columns.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/analysis/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::analysis::ExprBoundaries">ExprBoundaries</a>  
Represents the boundaries (e.g. min and max values) of a particular column

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/analysis/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/analysis/fn.analyze.html" class="fn" title="fn datafusion::physical_expr::analysis::analyze">analyze</a>  
Attempts to refine column boundaries and compute a selectivity value.
