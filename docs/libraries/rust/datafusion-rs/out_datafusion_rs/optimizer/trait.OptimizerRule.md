# Trait OptimizerRule Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/optimizer.rs.html#72" class="src">Source</a>

``` rust
pub trait OptimizerRule: Debug {
    // Required method
    fn name(&self) -> &str;

    // Provided methods
    fn apply_order(&self) -> Option<ApplyOrder> { ... }
    fn supports_rewrite(&self) -> bool { ... }
    fn rewrite(
        &self,
        _plan: LogicalPlan,
        _config: &dyn OptimizerConfig,
    ) -> Result<Transformed<LogicalPlan>, DataFusionError> { ... }
}
```

Expand description

`OptimizerRule`s transforms one [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") into another which computes the same results, but in a potentially more efficient way. If there are no suitable transformations for the input plan, the optimizer should simply return it unmodified.

To change the semantics of a `LogicalPlan`, see [`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")

Use [`SessionState::add_optimizer_rule`](https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_optimizer_rule) to register additional `OptimizerRule`s.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this optimizer rule

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.apply_order" class="fn">apply_order</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html" class="enum" title="enum datafusion::optimizer::ApplyOrder">ApplyOrder</a>\>

How should the rule be applied by the optimizer? See comments on [`ApplyOrder`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/enum.ApplyOrder.html "enum datafusion::optimizer::ApplyOrder") for details.

If returns `None`, the default, the rule must handle recursion itself

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.supports_rewrite" class="fn">supports_rewrite</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

👎Deprecated since 47.0.0: This method is no longer used

Does this rule support rewriting owned plans (rather than by reference)?

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#method.rewrite" class="fn">rewrite</a>( &self, \_plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, \_config: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html" class="trait" title="trait datafusion::optimizer::OptimizerConfig">OptimizerConfig</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to rewrite `plan` to an optimized form, returning `Transformed::yes` if the plan was rewritten and `Transformed::no` if it was not.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-CommonSubexprEliminate" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/common_subexpr_eliminate/struct.CommonSubexprEliminate.html" class="struct" title="struct datafusion::optimizer::common_subexpr_eliminate::CommonSubexprEliminate">CommonSubexprEliminate</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-DecorrelateLateralJoin" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate_lateral_join/struct.DecorrelateLateralJoin.html" class="struct" title="struct datafusion::optimizer::decorrelate_lateral_join::DecorrelateLateralJoin">DecorrelateLateralJoin</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-DecorrelatePredicateSubquery" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate_predicate_subquery/struct.DecorrelatePredicateSubquery.html" class="struct" title="struct datafusion::optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery">DecorrelatePredicateSubquery</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-EliminateCrossJoin" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_cross_join/struct.EliminateCrossJoin.html" class="struct" title="struct datafusion::optimizer::eliminate_cross_join::EliminateCrossJoin">EliminateCrossJoin</a>

Eliminate cross joins by rewriting them to inner joins when possible.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#example" class="doc-anchor">§</a>Example

The initial plan for this query:

``` sql
select ... from a, b where a.x = b.y and b.xx = 100;
```

Looks like this:

``` text
Filter(a.x = b.y AND b.xx = 100)
 Cross Join
  TableScan a
  TableScan b
```

After the rule is applied, the plan will look like this:

``` text
Filter(b.xx = 100)
  InnerJoin(a.x = b.y)
    TableScan a
    TableScan b
```

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#other-examples" class="doc-anchor">§</a>Other Examples

- ‘select … from a, b where a.x = b.y and b.xx = 100;’
- ‘select … from a, b where (a.x = b.y and b.xx = 100) or (a.x = b.y and b.xx = 200);’
- ’select … from a, b, c where (a.x = b.y and b.xx = 100 and a.z = c.z)
- or (a.x = b.y and b.xx = 200 and a.z=c.z);’
- ‘select … from a, b where a.x \> b.y’

For above queries, the join predicate is available in filters and they are moved to join nodes appropriately

This fix helps to improve the performance of TPCH Q19. issue#78

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-EliminateDuplicatedExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_duplicated_expr/struct.EliminateDuplicatedExpr.html" class="struct" title="struct datafusion::optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr">EliminateDuplicatedExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-EliminateFilter" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_filter/struct.EliminateFilter.html" class="struct" title="struct datafusion::optimizer::eliminate_filter::EliminateFilter">EliminateFilter</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-EliminateGroupByConstant" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_group_by_constant/struct.EliminateGroupByConstant.html" class="struct" title="struct datafusion::optimizer::eliminate_group_by_constant::EliminateGroupByConstant">EliminateGroupByConstant</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-EliminateJoin" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_join/struct.EliminateJoin.html" class="struct" title="struct datafusion::optimizer::eliminate_join::EliminateJoin">EliminateJoin</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-EliminateLimit" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_limit/struct.EliminateLimit.html" class="struct" title="struct datafusion::optimizer::eliminate_limit::EliminateLimit">EliminateLimit</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-EliminateNestedUnion" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_nested_union/struct.EliminateNestedUnion.html" class="struct" title="struct datafusion::optimizer::eliminate_nested_union::EliminateNestedUnion">EliminateNestedUnion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-EliminateOneUnion" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_one_union/struct.EliminateOneUnion.html" class="struct" title="struct datafusion::optimizer::eliminate_one_union::EliminateOneUnion">EliminateOneUnion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-EliminateOuterJoin" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_outer_join/struct.EliminateOuterJoin.html" class="struct" title="struct datafusion::optimizer::eliminate_outer_join::EliminateOuterJoin">EliminateOuterJoin</a>

Attempt to eliminate outer joins.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-ExtractEquijoinPredicate" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/extract_equijoin_predicate/struct.ExtractEquijoinPredicate.html" class="struct" title="struct datafusion::optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate">ExtractEquijoinPredicate</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-FilterNullJoinKeys" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/filter_null_join_keys/struct.FilterNullJoinKeys.html" class="struct" title="struct datafusion::optimizer::filter_null_join_keys::FilterNullJoinKeys">FilterNullJoinKeys</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-OptimizeProjections" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/optimize_projections/struct.OptimizeProjections.html" class="struct" title="struct datafusion::optimizer::optimize_projections::OptimizeProjections">OptimizeProjections</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-PropagateEmptyRelation" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/propagate_empty_relation/struct.PropagateEmptyRelation.html" class="struct" title="struct datafusion::optimizer::propagate_empty_relation::PropagateEmptyRelation">PropagateEmptyRelation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-PushDownFilter" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_filter/struct.PushDownFilter.html" class="struct" title="struct datafusion::optimizer::push_down_filter::PushDownFilter">PushDownFilter</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-PushDownLimit" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/push_down_limit/struct.PushDownLimit.html" class="struct" title="struct datafusion::optimizer::push_down_limit::PushDownLimit">PushDownLimit</a>

Push down Limit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-ReplaceDistinctWithAggregate" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/replace_distinct_aggregate/struct.ReplaceDistinctWithAggregate.html" class="struct" title="struct datafusion::optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate">ReplaceDistinctWithAggregate</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-ScalarSubqueryToJoin" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/scalar_subquery_to_join/struct.ScalarSubqueryToJoin.html" class="struct" title="struct datafusion::optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin">ScalarSubqueryToJoin</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-SimplifyExpressions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.SimplifyExpressions.html" class="struct" title="struct datafusion::optimizer::simplify_expressions::SimplifyExpressions">SimplifyExpressions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html#impl-OptimizerRule-for-SingleDistinctToGroupBy" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html" class="trait" title="trait datafusion::optimizer::OptimizerRule">OptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/single_distinct_to_groupby/struct.SingleDistinctToGroupBy.html" class="struct" title="struct datafusion::optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy">SingleDistinctToGroupBy</a>
