# Module projection Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#80" class="src">Source</a>

Expand description

Defines the projection execution plan. A projection determines which columns or expressions are returned from a query. The SQL statement `SELECT a, b, a+b FROM t1` is an example of a projection on table `t1` where the expressions `a`, `b`, and `a+b` are the projection expressions. `SELECT` without `FROM` will only evaluate expressions.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.JoinData.html" class="struct" title="struct datafusion::physical_plan::projection::JoinData">JoinData</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html" class="struct" title="struct datafusion::physical_plan::projection::ProjectionExec">ProjectionExec</a>  
[`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") for a projection

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExpr.html" class="struct" title="struct datafusion::physical_plan::projection::ProjectionExpr">ProjectionExpr</a>  
A projection expression that is created by [`ProjectionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html "struct datafusion::physical_plan::projection::ProjectionExec")

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/trait.EmbeddedProjection.html" class="trait" title="trait datafusion::physical_plan::projection::EmbeddedProjection">EmbeddedProjection</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.all_alias_free_columns.html" class="fn" title="fn datafusion::physical_plan::projection::all_alias_free_columns">all_alias_free_columns</a>  
Given the expression set of a projection, checks if the projection causes any renaming or constructs a non-`Column` physical expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.all_columns.html" class="fn" title="fn datafusion::physical_plan::projection::all_columns">all_columns</a>  
Returns `true` if all the expressions in the argument are `Column`s.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.join_allows_pushdown.html" class="fn" title="fn datafusion::physical_plan::projection::join_allows_pushdown">join_allows_pushdown</a>  
Checks three conditions for pushing a projection down through a join:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.join_table_borders.html" class="fn" title="fn datafusion::physical_plan::projection::join_table_borders">join_table_borders</a>  
Returns the last index before encountering a column coming from the right table when traveling through the projection from left to right, and the last index before encountering a column coming from the left table when traveling through the projection from right to left. If there is no column in the projection coming from the left side, it returns (-1, …), if there is no column in the projection coming from the right side, it returns (…, projection length).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.make_with_child.html" class="fn" title="fn datafusion::physical_plan::projection::make_with_child">make_with_child</a>  
Creates a new [`ProjectionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html "struct datafusion::physical_plan::projection::ProjectionExec") instance with the given child plan and projected expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.new_join_children.html" class="fn" title="fn datafusion::physical_plan::projection::new_join_children">new_join_children</a>  
If pushing down the projection over this join’s children seems possible, this function constructs the new [`ProjectionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html "struct datafusion::physical_plan::projection::ProjectionExec")s that will come on top of the original children of the join.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.new_projections_for_columns.html" class="fn" title="fn datafusion::physical_plan::projection::new_projections_for_columns">new_projections_for_columns</a>  
Updates a source provider’s projected columns according to the given projection operator’s expressions. To use this function safely, one must ensure that all expressions are `Column` expressions without aliases.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.physical_to_column_exprs.html" class="fn" title="fn datafusion::physical_plan::projection::physical_to_column_exprs">physical_to_column_exprs</a>  
Downcasts all the expressions in `exprs` to `Column`s. If any of the given expressions is not a `Column`, returns `None`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.remove_unnecessary_projections.html" class="fn" title="fn datafusion::physical_plan::projection::remove_unnecessary_projections">remove_unnecessary_projections</a>  
This function checks if `plan` is a [`ProjectionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html "struct datafusion::physical_plan::projection::ProjectionExec"), and inspects its input(s) to test whether it can push `plan` under its input(s). This function will operate on the entire tree and may ultimately remove `plan` entirely by leveraging source providers with built-in projection capabilities.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.try_embed_projection.html" class="fn" title="fn datafusion::physical_plan::projection::try_embed_projection">try_embed_projection</a>  
Some projection can’t be pushed down left input or right input of hash join because filter or on need may need some columns that won’t be used in later. By embed those projection to hash join, we can reduce the cost of build_batch_from_indices in hash join (build_batch_from_indices need to can compute::take() for each column) and avoid unnecessary output creation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.try_pushdown_through_join.html" class="fn" title="fn datafusion::physical_plan::projection::try_pushdown_through_join">try_pushdown_through_join</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.update_expr.html" class="fn" title="fn datafusion::physical_plan::projection::update_expr">update_expr</a>  
The function operates in two modes:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.update_join_filter.html" class="fn" title="fn datafusion::physical_plan::projection::update_join_filter">update_join_filter</a>  
Tries to update the column indices of a [`JoinFilter`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/struct.JoinFilter.html "struct datafusion::physical_plan::joins::utils::JoinFilter") as if the input of the join was replaced by a projection.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.update_join_on.html" class="fn" title="fn datafusion::physical_plan::projection::update_join_on">update_join_on</a>  
Tries to update the equi-join `Column`’s of a join as if the input of the join was replaced by a projection.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.update_ordering.html" class="fn" title="fn datafusion::physical_plan::projection::update_ordering">update_ordering</a>  
Updates the given lexicographic ordering according to given projected expressions using the [`update_expr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.update_expr.html "fn datafusion::physical_plan::projection::update_expr") function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.update_ordering_requirement.html" class="fn" title="fn datafusion::physical_plan::projection::update_ordering_requirement">update_ordering_requirement</a>  
Updates the given lexicographic requirement according to given projected expressions using the [`update_expr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/fn.update_expr.html "fn datafusion::physical_plan::projection::update_expr") function.
