# Module expr_fn Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#48" class="src">Source</a>

Expand description

Functions for creating logical expressions

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::logical_expr::expr_fn::ExprFuncBuilder">ExprFuncBuilder</a>  
Implementation of [`ExprFunctionExt`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html "trait datafusion::prelude::ExprFunctionExt").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/struct.SimpleAggregateUDF.html" class="struct" title="struct datafusion::logical_expr::expr_fn::SimpleAggregateUDF">SimpleAggregateUDF</a>  
Implements [`AggregateUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html "trait datafusion::logical_expr::AggregateUDFImpl") for functions that have a single signature and return type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/struct.SimpleScalarUDF.html" class="struct" title="struct datafusion::logical_expr::expr_fn::SimpleScalarUDF">SimpleScalarUDF</a>  
Implements [`ScalarUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html "trait datafusion::logical_expr::ScalarUDFImpl") for functions that have a single signature and return type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/struct.SimpleWindowUDF.html" class="struct" title="struct datafusion::logical_expr::expr_fn::SimpleWindowUDF">SimpleWindowUDF</a>  
Implements [`WindowUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html "trait datafusion::logical_expr::WindowUDFImpl") for functions that have a single signature and return type.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/enum.ExprFuncKind.html" class="enum" title="enum datafusion::logical_expr::expr_fn::ExprFuncKind">ExprFuncKind</a>

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/trait.ExprFunctionExt.html" class="trait" title="trait datafusion::logical_expr::expr_fn::ExprFunctionExt">ExprFunctionExt</a>  
Extensions for configuring [`Expr::AggregateFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.AggregateFunction "variant datafusion::prelude::Expr::AggregateFunction") or [`Expr::WindowFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.WindowFunction "variant datafusion::prelude::Expr::WindowFunction")

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.and.html" class="fn" title="fn datafusion::logical_expr::expr_fn::and">and</a>  
Return a new expression with a logical AND

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.binary_expr.html" class="fn" title="fn datafusion::logical_expr::expr_fn::binary_expr">binary_expr</a>  
Return a new expression `left <op> right`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.bitwise_and.html" class="fn" title="fn datafusion::logical_expr::expr_fn::bitwise_and">bitwise_and</a>  
Return a new expression with bitwise AND

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.bitwise_or.html" class="fn" title="fn datafusion::logical_expr::expr_fn::bitwise_or">bitwise_or</a>  
Return a new expression with bitwise OR

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.bitwise_shift_left.html" class="fn" title="fn datafusion::logical_expr::expr_fn::bitwise_shift_left">bitwise_shift_left</a>  
Return a new expression with bitwise SHIFT LEFT

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.bitwise_shift_right.html" class="fn" title="fn datafusion::logical_expr::expr_fn::bitwise_shift_right">bitwise_shift_right</a>  
Return a new expression with bitwise SHIFT RIGHT

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.bitwise_xor.html" class="fn" title="fn datafusion::logical_expr::expr_fn::bitwise_xor">bitwise_xor</a>  
Return a new expression with bitwise XOR

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.case.html" class="fn" title="fn datafusion::logical_expr::expr_fn::case">case</a>  
Create a CASE WHEN statement with literal WHEN expressions for comparison to the base expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.cast.html" class="fn" title="fn datafusion::logical_expr::expr_fn::cast">cast</a>  
Create a cast expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.col.html" class="fn" title="fn datafusion::logical_expr::expr_fn::col">col</a>  
Create a column expression based on a qualified or unqualified column name. Will normalize unquoted identifiers according to SQL rules (identifiers will become lowercase).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.create_udaf.html" class="fn" title="fn datafusion::logical_expr::expr_fn::create_udaf">create_udaf</a>  
Creates a new UDAF with a specific signature, state type and return type. The signature and state type must match the `Accumulator's implementation`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.create_udf.html" class="fn" title="fn datafusion::logical_expr::expr_fn::create_udf">create_udf</a>  
Convenience method to create a new user defined scalar function (UDF) with a specific signature and specific return type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.create_udwf.html" class="fn" title="fn datafusion::logical_expr::expr_fn::create_udwf">create_udwf</a>  
Creates a new UDWF with a specific signature, state type and return type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.cube.html" class="fn" title="fn datafusion::logical_expr::expr_fn::cube">cube</a>  
Create a grouping set for all combination of `exprs`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.exists.html" class="fn" title="fn datafusion::logical_expr::expr_fn::exists">exists</a>  
Create an EXISTS subquery expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.grouping_set.html" class="fn" title="fn datafusion::logical_expr::expr_fn::grouping_set">grouping_set</a>  
Create a grouping set

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.ident.html" class="fn" title="fn datafusion::logical_expr::expr_fn::ident">ident</a>  
Create an unqualified column expression from the provided name, without normalizing the column.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.in_list.html" class="fn" title="fn datafusion::logical_expr::expr_fn::in_list">in_list</a>  
Create an in_list expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.in_subquery.html" class="fn" title="fn datafusion::logical_expr::expr_fn::in_subquery">in_subquery</a>  
Create an IN subquery expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.interval_datetime_lit.html" class="fn" title="fn datafusion::logical_expr::expr_fn::interval_datetime_lit">interval_datetime_lit</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.interval_month_day_nano_lit.html" class="fn" title="fn datafusion::logical_expr::expr_fn::interval_month_day_nano_lit">interval_month_day_nano_lit</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.interval_year_month_lit.html" class="fn" title="fn datafusion::logical_expr::expr_fn::interval_year_month_lit">interval_year_month_lit</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.is_false.html" class="fn" title="fn datafusion::logical_expr::expr_fn::is_false">is_false</a>  
Create is false expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.is_not_false.html" class="fn" title="fn datafusion::logical_expr::expr_fn::is_not_false">is_not_false</a>  
Create is not false expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.is_not_true.html" class="fn" title="fn datafusion::logical_expr::expr_fn::is_not_true">is_not_true</a>  
Create is not true expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.is_not_unknown.html" class="fn" title="fn datafusion::logical_expr::expr_fn::is_not_unknown">is_not_unknown</a>  
Create is not unknown expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.is_null.html" class="fn" title="fn datafusion::logical_expr::expr_fn::is_null">is_null</a>  
Create is null expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.is_true.html" class="fn" title="fn datafusion::logical_expr::expr_fn::is_true">is_true</a>  
Create is true expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.is_unknown.html" class="fn" title="fn datafusion::logical_expr::expr_fn::is_unknown">is_unknown</a>  
Create is unknown expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.not.html" class="fn" title="fn datafusion::logical_expr::expr_fn::not">not</a>  
Return a new expression with a logical NOT

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.not_exists.html" class="fn" title="fn datafusion::logical_expr::expr_fn::not_exists">not_exists</a>  
Create a NOT EXISTS subquery expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.not_in_subquery.html" class="fn" title="fn datafusion::logical_expr::expr_fn::not_in_subquery">not_in_subquery</a>  
Create a NOT IN subquery expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.or.html" class="fn" title="fn datafusion::logical_expr::expr_fn::or">or</a>  
Return a new expression with a logical OR

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.out_ref_col.html" class="fn" title="fn datafusion::logical_expr::expr_fn::out_ref_col">out_ref_col</a>  
Create an out reference column which hold a reference that has been resolved to a field outside of the current plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.placeholder.html" class="fn" title="fn datafusion::logical_expr::expr_fn::placeholder">placeholder</a>  
Create placeholder value that will be filled in (such as `$1`)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.qualified_wildcard.html" class="fn" title="fn datafusion::logical_expr::expr_fn::qualified_wildcard">qualified_wildcard</a>  
Create an ‘t.\*’ [`Expr::Wildcard`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard "variant datafusion::prelude::Expr::Wildcard") expression that matches all columns from a specific table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.qualified_wildcard_with_options.html" class="fn" title="fn datafusion::logical_expr::expr_fn::qualified_wildcard_with_options">qualified_wildcard_with_options</a>  
Create an ‘t.\*’ [`Expr::Wildcard`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard "variant datafusion::prelude::Expr::Wildcard") expression with the wildcard options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.rollup.html" class="fn" title="fn datafusion::logical_expr::expr_fn::rollup">rollup</a>  
Create a grouping set for rollup

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.scalar_subquery.html" class="fn" title="fn datafusion::logical_expr::expr_fn::scalar_subquery">scalar_subquery</a>  
Create a scalar subquery expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.try_cast.html" class="fn" title="fn datafusion::logical_expr::expr_fn::try_cast">try_cast</a>  
Create a try cast expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.unnest.html" class="fn" title="fn datafusion::logical_expr::expr_fn::unnest">unnest</a>  
Create a Unnest expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.when.html" class="fn" title="fn datafusion::logical_expr::expr_fn::when">when</a>  
Create a CASE WHEN statement with boolean WHEN expressions and no base expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.wildcard.html" class="fn" title="fn datafusion::logical_expr::expr_fn::wildcard">wildcard</a>  
Create an ‘\*’ [`Expr::Wildcard`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard "variant datafusion::prelude::Expr::Wildcard") expression that matches all columns

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/fn.wildcard_with_options.html" class="fn" title="fn datafusion::logical_expr::expr_fn::wildcard_with_options">wildcard_with_options</a>  
Create an ‘\*’ [`Expr::Wildcard`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard "variant datafusion::prelude::Expr::Wildcard") expression with the wildcard options
