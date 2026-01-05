# Module utils Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#75" class="src">Source</a>

Expand description

Expression utilities

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/enum.AggregateOrderSensitivity.html" class="enum" title="enum datafusion::logical_expr::utils::AggregateOrderSensitivity">AggregateOrderSensitivity</a>  
Represents the sensitivity of an aggregate expression to ordering.

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/constant.COUNT_STAR_EXPANSION.html" class="constant" title="constant datafusion::logical_expr::utils::COUNT_STAR_EXPANSION">COUNT_STAR_EXPANSION</a>  
The value to which `COUNT(*)` is expanded to in `COUNT(<constant>)` expressions

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.add_filter.html" class="fn" title="fn datafusion::logical_expr::utils::add_filter">add_filter</a>  
Returns a new [LogicalPlan](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") that filters the output of `plan` with a [LogicalPlan::Filter](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#variant.Filter "variant datafusion::logical_expr::LogicalPlan::Filter") with all `predicates` ANDed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.can_hash.html" class="fn" title="fn datafusion::logical_expr::utils::can_hash">can_hash</a>  
Can this data type be used in hash join equal conditions?? Data types here come from function ‘equal_rows’, if more data types are supported in create_hashes, add those data types here to generate join logical plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.check_all_columns_from_schema.html" class="fn" title="fn datafusion::logical_expr::utils::check_all_columns_from_schema">check_all_columns_from_schema</a>  
Check whether all columns are from the schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.collect_subquery_cols.html" class="fn" title="fn datafusion::logical_expr::utils::collect_subquery_cols">collect_subquery_cols</a>  
Determine the set of [`Column`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html "struct datafusion::prelude::Column")s produced by the subquery.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.columnize_expr.html" class="fn" title="fn datafusion::logical_expr::utils::columnize_expr">columnize_expr</a>  
Convert an expression into Column expression if it’s already provided as input plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.compare_sort_expr.html" class="fn" title="fn datafusion::logical_expr::utils::compare_sort_expr">compare_sort_expr</a>  
Compare the sort expr as PostgreSQL’s common_prefix_cmp(): <https://github.com/postgres/postgres/blob/master/src/backend/optimizer/plan/planner.c>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.conjunction.html" class="fn" title="fn datafusion::logical_expr::utils::conjunction">conjunction</a>  
Combines an array of filter expressions into a single filter expression consisting of the input filter expressions joined with logical AND.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.disjunction.html" class="fn" title="fn datafusion::logical_expr::utils::disjunction">disjunction</a>  
Combines an array of filter expressions into a single filter expression consisting of the input filter expressions joined with logical OR.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.enumerate_grouping_sets.html" class="fn" title="fn datafusion::logical_expr::utils::enumerate_grouping_sets">enumerate_grouping_sets</a>  
Convert multiple grouping expressions into one [`GroupingSet::GroupingSets`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.GroupingSet.html#variant.GroupingSets "variant datafusion::logical_expr::GroupingSet::GroupingSets"),  
if the grouping expression does not contain [`Expr::GroupingSet`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.GroupingSet "variant datafusion::prelude::Expr::GroupingSet") or only has one expression,  
no conversion will be performed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.expand_qualified_wildcard.html" class="fn" title="fn datafusion::logical_expr::utils::expand_qualified_wildcard">expand_qualified_wildcard</a>  
Resolves an `Expr::Wildcard` to a collection of qualified `Expr::Column`’s.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.expand_wildcard.html" class="fn" title="fn datafusion::logical_expr::utils::expand_wildcard">expand_wildcard</a>  
Resolves an `Expr::Wildcard` to a collection of `Expr::Column`’s.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.expr_as_column_expr.html" class="fn" title="fn datafusion::logical_expr::utils::expr_as_column_expr">expr_as_column_expr</a>  
Convert any `Expr` to an `Expr::Column`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.expr_to_columns.html" class="fn" title="fn datafusion::logical_expr::utils::expr_to_columns">expr_to_columns</a>  
Recursively walk an expression tree, collecting the unique set of columns referenced in the expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.exprlist_to_fields.html" class="fn" title="fn datafusion::logical_expr::utils::exprlist_to_fields">exprlist_to_fields</a>  
Create field meta-data from an expression, for use in a result set schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.find_aggregate_exprs.html" class="fn" title="fn datafusion::logical_expr::utils::find_aggregate_exprs">find_aggregate_exprs</a>  
Collect all deeply nested `Expr::AggregateFunction`. They are returned in order of occurrence (depth first), with duplicates omitted.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.find_column_exprs.html" class="fn" title="fn datafusion::logical_expr::utils::find_column_exprs">find_column_exprs</a>  
Collect all deeply nested `Expr::Column`’s. They are returned in order of appearance (depth first), and may contain duplicates.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.find_join_exprs.html" class="fn" title="fn datafusion::logical_expr::utils::find_join_exprs">find_join_exprs</a>  
Looks for correlating expressions: for example, a binary expression with one field from the subquery, and one not in the subquery (closed upon from outer scope)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.find_out_reference_exprs.html" class="fn" title="fn datafusion::logical_expr::utils::find_out_reference_exprs">find_out_reference_exprs</a>  
Collect all deeply nested `Expr::OuterReferenceColumn`. They are returned in order of occurrence (depth first), with duplicates omitted.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.find_valid_equijoin_key_pair.html" class="fn" title="fn datafusion::logical_expr::utils::find_valid_equijoin_key_pair">find_valid_equijoin_key_pair</a>  
Give two sides of the equijoin predicate, return a valid join key pair. If there is no valid join key pair, return None.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.find_window_exprs.html" class="fn" title="fn datafusion::logical_expr::utils::find_window_exprs">find_window_exprs</a>  
Collect all deeply nested `Expr::WindowFunction`. They are returned in order of occurrence (depth first), with duplicates omitted.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.format_state_name.html" class="fn" title="fn datafusion::logical_expr::utils::format_state_name">format_state_name</a>  
Build state name. State is the intermediate state of the aggregate function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.generate_signature_error_msg.html" class="fn" title="fn datafusion::logical_expr::utils::generate_signature_error_msg">generate_signature_error_msg</a>  
Creates a detailed error message for a function with wrong signature.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.generate_sort_key.html" class="fn" title="fn datafusion::logical_expr::utils::generate_sort_key">generate_sort_key</a>  
Generate a sort key for a given window expr’s partition_by and order_by expr

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.group_window_expr_by_sort_keys.html" class="fn" title="fn datafusion::logical_expr::utils::group_window_expr_by_sort_keys">group_window_expr_by_sort_keys</a>  
Group a slice of window expression expr by their order by expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.grouping_set_expr_count.html" class="fn" title="fn datafusion::logical_expr::utils::grouping_set_expr_count">grouping_set_expr_count</a>  
Count the number of distinct exprs in a list of group by expressions. If the first element is a `GroupingSet` expression then it must be the only expr.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.grouping_set_to_exprlist.html" class="fn" title="fn datafusion::logical_expr::utils::grouping_set_to_exprlist">grouping_set_to_exprlist</a>  
Find all distinct exprs in a list of group by expressions. If the first element is a `GroupingSet` expression then it must be the only expr.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.inspect_expr_pre.html" class="fn" title="fn datafusion::logical_expr::utils::inspect_expr_pre">inspect_expr_pre</a>  
Recursively inspect an [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") and all its children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.iter_conjunction.html" class="fn" title="fn datafusion::logical_expr::utils::iter_conjunction">iter_conjunction</a>  
Iterate parts in a conjunctive [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") such as `A AND B AND C` =\> `[A, B, C]`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.iter_conjunction_owned.html" class="fn" title="fn datafusion::logical_expr::utils::iter_conjunction_owned">iter_conjunction_owned</a>  
Iterate parts in a conjunctive [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") such as `A AND B AND C` =\> `[A, B, C]`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.merge_schema.html" class="fn" title="fn datafusion::logical_expr::utils::merge_schema">merge_schema</a>  
merge inputs schema into a single schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.only_or_err.html" class="fn" title="fn datafusion::logical_expr::utils::only_or_err">only_or_err</a>  
Returns the first (and only) element in a slice, or an error

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.split_binary.html" class="fn" title="fn datafusion::logical_expr::utils::split_binary">split_binary</a>  
Splits an binary operator tree [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") such as `A <OP> B <OP> C` =\> `[A, B, C]`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.split_binary_owned.html" class="fn" title="fn datafusion::logical_expr::utils::split_binary_owned">split_binary_owned</a>  
Splits an owned binary operator tree [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") such as `A <OP> B <OP> C` =\> `[A, B, C]`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.split_conjunction.html" class="fn" title="fn datafusion::logical_expr::utils::split_conjunction">split_conjunction</a>  
Splits a conjunctive [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") such as `A AND B AND C` =\> `[A, B, C]`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.split_conjunction_owned.html" class="fn" title="fn datafusion::logical_expr::utils::split_conjunction_owned">split_conjunction_owned</a>  
Splits an owned conjunctive [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") such as `A AND B AND C` =\> `[A, B, C]`
