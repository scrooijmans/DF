# Module utils Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/lib.rs.html#43" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/struct.LiteralGuarantee.html" class="struct" title="struct datafusion::physical_expr::utils::LiteralGuarantee">LiteralGuarantee</a>  
Represents a guarantee that must be true for a boolean expression to evaluate to `true`.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/enum.Guarantee.html" class="enum" title="enum datafusion::physical_expr::utils::Guarantee">Guarantee</a>  
What is guaranteed about the values for a [`LiteralGuarantee`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/struct.LiteralGuarantee.html "struct datafusion::physical_expr::utils::LiteralGuarantee")?

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/fn.build_dag.html" class="fn" title="fn datafusion::physical_expr::utils::build_dag">build_dag</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/fn.collect_columns.html" class="fn" title="fn datafusion::physical_expr::utils::collect_columns">collect_columns</a>  
Recursively extract referenced [`Column`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.Column.html "struct datafusion::physical_expr::expressions::Column")s within a [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/fn.conjunction.html" class="fn" title="fn datafusion::physical_expr::utils::conjunction">conjunction</a>  
Create a conjunction of the given predicates. If the input is empty, return a literal true. If the input contains a single predicate, return the predicate. Otherwise, return a conjunction of the predicates (e.g. `a AND b AND c`).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/fn.conjunction_opt.html" class="fn" title="fn datafusion::physical_expr::utils::conjunction_opt">conjunction_opt</a>  
Create a conjunction of the given predicates. If the input is empty or the return None. If the input contains a single predicate, return Some(predicate). Otherwise, return a Some(..) of a conjunction of the predicates (e.g. `Some(a AND b AND c)`).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/fn.convert_to_expr.html" class="fn" title="fn datafusion::physical_expr::utils::convert_to_expr">convert_to_expr</a>  
This function returns all `Arc<dyn PhysicalExpr>`s inside the given `PhysicalSortExpr` sequence.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/fn.get_indices_of_exprs_strict.html" class="fn" title="fn datafusion::physical_expr::utils::get_indices_of_exprs_strict">get_indices_of_exprs_strict</a>  
This function finds the indices of `targets` within `items` using strict equality.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/fn.map_columns_before_projection.html" class="fn" title="fn datafusion::physical_expr::utils::map_columns_before_projection">map_columns_before_projection</a>  
This function maps back requirement after ProjectionExec to the Executor for its input.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/fn.reassign_predicate_columns.html" class="fn" title="fn datafusion::physical_expr::utils::reassign_predicate_columns">reassign_predicate_columns</a>  
Re-assign column indices referenced in predicate according to given schema. This may be helpful when dealing with projections.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/fn.split_conjunction.html" class="fn" title="fn datafusion::physical_expr::utils::split_conjunction">split_conjunction</a>  
Assume the predicate is in the form of CNF, split the predicate to a Vec of PhysicalExprs.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/fn.split_disjunction.html" class="fn" title="fn datafusion::physical_expr::utils::split_disjunction">split_disjunction</a>  
Assume the predicate is in the form of DNF, split the predicate to a Vec of PhysicalExprs.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/type.ExprTreeNode.html" class="type" title="type datafusion::physical_expr::utils::ExprTreeNode">ExprTreeNode</a>
