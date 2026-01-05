# Module simplify_expressions Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#63" class="src">Source</a>

Expand description

[`SimplifyExpressions`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyExpressions.html "struct datafusion::optimizer::simplify_expressions::SimplifyExpressions") simplifies expressions in the logical plan, [`ExprSimplifier`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html "struct datafusion::optimizer::simplify_expressions::ExprSimplifier") simplifies individual `Expr`s.

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/expr_simplifier/index.html" class="mod" title="mod datafusion::optimizer::simplify_expressions::expr_simplifier">expr_simplifier</a>  
Expression simplification API

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/simplify_exprs/index.html" class="mod" title="mod datafusion::optimizer::simplify_expressions::simplify_exprs">simplify_exprs</a>  
Simplify expressions optimizer rule and implementation

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html" class="struct" title="struct datafusion::optimizer::simplify_expressions::ExprSimplifier">ExprSimplifier</a>  
This structure handles API for expression simplification

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.GuaranteeRewriter.html" class="struct" title="struct datafusion::optimizer::simplify_expressions::GuaranteeRewriter">GuaranteeRewriter</a>  
Rewrite expressions to incorporate guarantees.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyContext.html" class="struct" title="struct datafusion::optimizer::simplify_expressions::SimplifyContext">SimplifyContext</a>  
Provides simplification information based on DFSchema and [`ExecutionProps`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html "struct datafusion::execution::context::ExecutionProps"). This is the default implementation used by DataFusion

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyExpressions.html" class="struct" title="struct datafusion::optimizer::simplify_expressions::SimplifyExpressions">SimplifyExpressions</a>  
Optimizer Pass that simplifies [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s by rewriting [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")\`s evaluating constants and applying algebraic simplifications

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/constant.DEFAULT_MAX_SIMPLIFIER_CYCLES.html" class="constant" title="constant datafusion::optimizer::simplify_expressions::DEFAULT_MAX_SIMPLIFIER_CYCLES">DEFAULT_MAX_SIMPLIFIER_CYCLES</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/constant.THRESHOLD_INLINE_INLIST.html" class="constant" title="constant datafusion::optimizer::simplify_expressions::THRESHOLD_INLINE_INLIST">THRESHOLD_INLINE_INLIST</a>

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/trait.SimplifyInfo.html" class="trait" title="trait datafusion::optimizer::simplify_expressions::SimplifyInfo">SimplifyInfo</a>  
Provides the information necessary to apply algebraic simplification to an [Expr](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr"). See [SimplifyContext](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html "struct datafusion::logical_expr::simplify::SimplifyContext") for one concrete implementation.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/fn.simplify_predicates.html" class="fn" title="fn datafusion::optimizer::simplify_expressions::simplify_predicates">simplify_predicates</a>  
Simplifies a list of predicates by removing redundancies.
