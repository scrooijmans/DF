# Module decorrelate Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#42" class="src">Source</a>

Expand description

[`PullUpCorrelatedExpr`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html "struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr") converts correlated subqueries to `Joins`

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html" class="struct" title="struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr">PullUpCorrelatedExpr</a>  
This struct rewrite the sub query plan by pull up the correlated expressions(contains outer reference columns) from the inner subquery’s ‘Filter’. It adds the inner reference columns to the ‘Projection’ or ‘Aggregate’ of the subquery if they are missing, so that they can be evaluated by the parent operator as the join condition.

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/constant.UN_MATCHED_ROW_INDICATOR.html" class="constant" title="constant datafusion::optimizer::decorrelate::UN_MATCHED_ROW_INDICATOR">UN_MATCHED_ROW_INDICATOR</a>  
Used to indicate the unmatched rows from the inner(subquery) table after the left out Join This is used to handle [the Count bug](https://github.com/apache/datafusion/issues/10553)

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/type.ExprResultMap.html" class="type" title="type datafusion::optimizer::decorrelate::ExprResultMap">ExprResultMap</a>  
Mapping from expr display name to its evaluation result on empty record batch (for example: ‘count(*)’ is ‘ScalarValue(0)’, ‘count(*) + 2’ is ‘ScalarValue(2)’)
