# Module interval_arithmetic Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/lib.rs.html#40" class="src">Source</a>

Expand description

Interval arithmetic library

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/interval_arithmetic/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr_common::interval_arithmetic::Interval">Interval</a>  
The `Interval` type represents a closed interval used for computing reliable bounds for mathematical expressions.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/interval_arithmetic/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr_common::interval_arithmetic::NullableInterval">NullableInterval</a>  
An [Interval](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html "struct datafusion::logical_expr::interval_arithmetic::Interval") that also tracks null status using a boolean interval.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/interval_arithmetic/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/interval_arithmetic/fn.apply_operator.html" class="fn" title="fn datafusion::logical_expr_common::interval_arithmetic::apply_operator">apply_operator</a>  
Applies the given binary operator the `lhs` and `rhs` arguments.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/interval_arithmetic/fn.cardinality_ratio.html" class="fn" title="fn datafusion::logical_expr_common::interval_arithmetic::cardinality_ratio">cardinality_ratio</a>  
This function computes the selectivity of an operation by computing the cardinality ratio of the given input/output intervals. If this can not be calculated for some reason, it returns `1.0` meaning fully selective (no filtering).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/interval_arithmetic/fn.satisfy_greater.html" class="fn" title="fn datafusion::logical_expr_common::interval_arithmetic::satisfy_greater">satisfy_greater</a>  
This function updates the given intervals by enforcing (i.e. propagating) the inequality `left > right` (or the `left >= right` inequality, if `strict` is `true`).
