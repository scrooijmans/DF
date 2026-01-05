# Module optimizer Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#792" class="src">Source</a>

Expand description

re-export of [`datafusion_optimizer`](https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/datafusion_optimizer/index.html "mod datafusion_optimizer") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/index.html" class="mod" title="mod datafusion::optimizer::analyzer">analyzer</a>  
[`Analyzer`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html "struct datafusion::optimizer::Analyzer") and [`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/common_subexpr_eliminate/index.html" class="mod" title="mod datafusion::optimizer::common_subexpr_eliminate">common_subexpr_eliminate</a>  
[`CommonSubexprEliminate`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/common_subexpr_eliminate/struct.CommonSubexprEliminate.html "struct datafusion::optimizer::common_subexpr_eliminate::CommonSubexprEliminate") to avoid redundant computation of common sub-expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/index.html" class="mod" title="mod datafusion::optimizer::decorrelate">decorrelate</a>  
[`PullUpCorrelatedExpr`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html "struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr") converts correlated subqueries to `Joins`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate_lateral_join/index.html" class="mod" title="mod datafusion::optimizer::decorrelate_lateral_join">decorrelate_lateral_join</a>  
[`DecorrelateLateralJoin`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate_lateral_join/struct.DecorrelateLateralJoin.html "struct datafusion::optimizer::decorrelate_lateral_join::DecorrelateLateralJoin") decorrelates logical plans produced by lateral joins.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate_predicate_subquery/index.html" class="mod" title="mod datafusion::optimizer::decorrelate_predicate_subquery">decorrelate_predicate_subquery</a>  
[`DecorrelatePredicateSubquery`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate_predicate_subquery/struct.DecorrelatePredicateSubquery.html "struct datafusion::optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery") converts `IN`/`EXISTS` subquery predicates to `SEMI`/`ANTI` joins

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_cross_join/index.html" class="mod" title="mod datafusion::optimizer::eliminate_cross_join">eliminate_cross_join</a>  
[`EliminateCrossJoin`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_cross_join/struct.EliminateCrossJoin.html "struct datafusion::optimizer::eliminate_cross_join::EliminateCrossJoin") converts `CROSS JOIN` to `INNER JOIN` if join predicates are available.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_duplicated_expr/index.html" class="mod" title="mod datafusion::optimizer::eliminate_duplicated_expr">eliminate_duplicated_expr</a>  
[`EliminateDuplicatedExpr`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_duplicated_expr/struct.EliminateDuplicatedExpr.html "struct datafusion::optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr") Removes redundant expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_filter/index.html" class="mod" title="mod datafusion::optimizer::eliminate_filter">eliminate_filter</a>  
[`EliminateFilter`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_filter/struct.EliminateFilter.html "struct datafusion::optimizer::eliminate_filter::EliminateFilter") replaces `where false` or `where null` with an empty relation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_group_by_constant/index.html" class="mod" title="mod datafusion::optimizer::eliminate_group_by_constant">eliminate_group_by_constant</a>  
[`EliminateGroupByConstant`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_group_by_constant/struct.EliminateGroupByConstant.html "struct datafusion::optimizer::eliminate_group_by_constant::EliminateGroupByConstant") removes constant expressions from `GROUP BY` clause

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_join/index.html" class="mod" title="mod datafusion::optimizer::eliminate_join">eliminate_join</a>  
[`EliminateJoin`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_join/struct.EliminateJoin.html "struct datafusion::optimizer::eliminate_join::EliminateJoin") rewrites `INNER JOIN` with `true`/`null`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_limit/index.html" class="mod" title="mod datafusion::optimizer::eliminate_limit">eliminate_limit</a>  
[`EliminateLimit`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_limit/struct.EliminateLimit.html "struct datafusion::optimizer::eliminate_limit::EliminateLimit") eliminates `LIMIT` when possible

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_nested_union/index.html" class="mod" title="mod datafusion::optimizer::eliminate_nested_union">eliminate_nested_union</a>  
[`EliminateNestedUnion`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_nested_union/struct.EliminateNestedUnion.html "struct datafusion::optimizer::eliminate_nested_union::EliminateNestedUnion"): flattens nested `Union` to a single `Union`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_one_union/index.html" class="mod" title="mod datafusion::optimizer::eliminate_one_union">eliminate_one_union</a>  
[`EliminateOneUnion`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_one_union/struct.EliminateOneUnion.html "struct datafusion::optimizer::eliminate_one_union::EliminateOneUnion") eliminates single element `Union`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/index.html" class="mod" title="mod datafusion::optimizer::eliminate_outer_join">eliminate_outer_join</a>  
[`EliminateOuterJoin`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html "struct datafusion::optimizer::eliminate_outer_join::EliminateOuterJoin") converts `LEFT/RIGHT/FULL` joins to `INNER` joins

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/extract_equijoin_predicate/index.html" class="mod" title="mod datafusion::optimizer::extract_equijoin_predicate">extract_equijoin_predicate</a>  
[`ExtractEquijoinPredicate`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/extract_equijoin_predicate/struct.ExtractEquijoinPredicate.html "struct datafusion::optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate") identifies equality join (equijoin) predicates

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/filter_null_join_keys/index.html" class="mod" title="mod datafusion::optimizer::filter_null_join_keys">filter_null_join_keys</a>  
[`FilterNullJoinKeys`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/filter_null_join_keys/struct.FilterNullJoinKeys.html "struct datafusion::optimizer::filter_null_join_keys::FilterNullJoinKeys") adds filters to join inputs when input isn’t nullable

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimize_projections/index.html" class="mod" title="mod datafusion::optimizer::optimize_projections">optimize_projections</a>  
[`OptimizeProjections`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimize_projections/struct.OptimizeProjections.html "struct datafusion::optimizer::optimize_projections::OptimizeProjections") identifies and eliminates unused columns

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimizer/index.html" class="mod" title="mod datafusion::optimizer::optimizer">optimizer</a>  
[`Optimizer`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html "struct datafusion::optimizer::Optimizer") and [`OptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html "trait datafusion::optimizer::OptimizerRule")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/propagate_empty_relation/index.html" class="mod" title="mod datafusion::optimizer::propagate_empty_relation">propagate_empty_relation</a>  
[`PropagateEmptyRelation`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/propagate_empty_relation/struct.PropagateEmptyRelation.html "struct datafusion::optimizer::propagate_empty_relation::PropagateEmptyRelation") eliminates nodes fed by `EmptyRelation`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/index.html" class="mod" title="mod datafusion::optimizer::push_down_filter">push_down_filter</a>  
[`PushDownFilter`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html "struct datafusion::optimizer::push_down_filter::PushDownFilter") applies filters as early as possible

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_limit/index.html" class="mod" title="mod datafusion::optimizer::push_down_limit">push_down_limit</a>  
[`PushDownLimit`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_limit/struct.PushDownLimit.html "struct datafusion::optimizer::push_down_limit::PushDownLimit") pushes `LIMIT` earlier in the query plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/replace_distinct_aggregate/index.html" class="mod" title="mod datafusion::optimizer::replace_distinct_aggregate">replace_distinct_aggregate</a>  
[`ReplaceDistinctWithAggregate`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/replace_distinct_aggregate/struct.ReplaceDistinctWithAggregate.html "struct datafusion::optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate") replaces `DISTINCT ...` with `GROUP BY ...`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/index.html" class="mod" title="mod datafusion::optimizer::scalar_subquery_to_join">scalar_subquery_to_join</a>  
[`ScalarSubqueryToJoin`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html "struct datafusion::optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin") rewriting scalar subquery filters to `JOIN`s

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/index.html" class="mod" title="mod datafusion::optimizer::simplify_expressions">simplify_expressions</a>  
[`SimplifyExpressions`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyExpressions.html "struct datafusion::optimizer::simplify_expressions::SimplifyExpressions") simplifies expressions in the logical plan, [`ExprSimplifier`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html "struct datafusion::optimizer::simplify_expressions::ExprSimplifier") simplifies individual `Expr`s.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/single_distinct_to_groupby/index.html" class="mod" title="mod datafusion::optimizer::single_distinct_to_groupby">single_distinct_to_groupby</a>  
[`SingleDistinctToGroupBy`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/single_distinct_to_groupby/struct.SingleDistinctToGroupBy.html "struct datafusion::optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy") replaces `AGG(DISTINCT ..)` with `AGG(..) GROUP BY ..`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/index.html" class="mod" title="mod datafusion::optimizer::utils">utils</a>  
Utility functions leveraged by the query optimizer rules

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Analyzer.html" class="struct" title="struct datafusion::optimizer::Analyzer">Analyzer</a>  
Rule-based Analyzer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.Optimizer.html" class="struct" title="struct datafusion::optimizer::Optimizer">Optimizer</a>  
A rule-based optimizer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>  
A standalone [`OptimizerConfig`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html "trait datafusion::optimizer::OptimizerConfig") that can be used independently of DataFusion’s config management

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>  
Specifies how recursion for an `OptimizerRule` should be handled.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html" class="trait" title="trait datafusion::optimizer::AnalyzerRule">AnalyzerRule</a>  
[`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")s transform [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s in some way to make the plan valid prior to the rest of the DataFusion optimization process.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html" class="trait" title="trait datafusion::optimizer::OptimizerConfig">OptimizerConfig</a>  
Options to control the DataFusion Optimizer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a>  
`OptimizerRule`s transforms one [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") into another which computes the same results, but in a potentially more efficient way. If there are no suitable transformations for the input plan, the optimizer should simply return it unmodified.
