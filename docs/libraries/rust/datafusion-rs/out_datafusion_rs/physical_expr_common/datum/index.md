# Module datum Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/lib.rs.html#34" class="src">Source</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/datum/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/datum/fn.apply.html" class="fn" title="fn datafusion::physical_expr_common::datum::apply">apply</a>  
Applies a binary [`Datum`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Datum.html "trait datafusion::common::arrow::array::Datum") kernel `f` to `lhs` and `rhs`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/datum/fn.apply_cmp.html" class="fn" title="fn datafusion::physical_expr_common::datum::apply_cmp">apply_cmp</a>  
Applies a binary [`Datum`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Datum.html "trait datafusion::common::arrow::array::Datum") comparison kernel `f` to `lhs` and `rhs`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/datum/fn.apply_cmp_for_nested.html" class="fn" title="fn datafusion::physical_expr_common::datum::apply_cmp_for_nested">apply_cmp_for_nested</a>  
Applies a binary [`Datum`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Datum.html "trait datafusion::common::arrow::array::Datum") comparison kernel `f` to `lhs` and `rhs` for nested type like List, FixedSizeList, LargeList, Struct, Union, Map, or a dictionary of a nested type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/datum/fn.compare_op_for_nested.html" class="fn" title="fn datafusion::physical_expr_common::datum::compare_op_for_nested">compare_op_for_nested</a>  
Compare on nested type List, Struct, and so on

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/datum/fn.compare_with_eq.html" class="fn" title="fn datafusion::physical_expr_common::datum::compare_with_eq">compare_with_eq</a>  
Compare with eq with either nested or non-nested
