# Module simplify Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#62" class="src">Source</a>

Expand description

Structs and traits to provide the information needed for expression simplification.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html" class="struct" title="struct datafusion::logical_expr::simplify::SimplifyContext">SimplifyContext</a>  
Provides simplification information based on DFSchema and [`ExecutionProps`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html "struct datafusion::execution::context::ExecutionProps"). This is the default implementation used by DataFusion

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/enum.ExprSimplifyResult.html" class="enum" title="enum datafusion::logical_expr::simplify::ExprSimplifyResult">ExprSimplifyResult</a>  
Was the expression simplified?

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/trait.SimplifyInfo.html" class="trait" title="trait datafusion::logical_expr::simplify::SimplifyInfo">SimplifyInfo</a>  
Provides the information necessary to apply algebraic simplification to an [Expr](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr"). See [SimplifyContext](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/simplify/struct.SimplifyContext.html "struct datafusion::logical_expr::simplify::SimplifyContext") for one concrete implementation.
