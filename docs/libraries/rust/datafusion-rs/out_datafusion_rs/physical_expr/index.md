# Module physical_expr Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#807" class="src">Source</a>

Expand description

re-export of [`datafusion_physical_expr`](https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_expr/index.html "mod datafusion_physical_expr") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/index.html" class="mod" title="mod datafusion::physical_expr::aggregate">aggregate</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/analysis/index.html" class="mod" title="mod datafusion::physical_expr::analysis">analysis</a>

Interval and selectivity in [`AnalysisContext`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html "struct datafusion::physical_expr::AnalysisContext")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/async_scalar_function/index.html" class="mod" title="mod datafusion::physical_expr::async_scalar_function">async_scalar_function</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/binary_map/index.html" class="mod" title="mod datafusion::physical_expr::binary_map">binary_map</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/index.html" class="mod" title="mod datafusion::physical_expr::equivalence">equivalence</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/execution_props/index.html" class="mod" title="mod datafusion::physical_expr::execution_props">execution_props</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/index.html" class="mod" title="mod datafusion::physical_expr::expressions">expressions</a>

Defines physical expressions that can evaluated at runtime during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/index.html" class="mod" title="mod datafusion::physical_expr::intervals">intervals</a>

Interval arithmetic and constraint propagation library

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/planner/index.html" class="mod" title="mod datafusion::physical_expr::planner">planner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/simplifier/index.html" class="mod" title="mod datafusion::physical_expr::simplifier">simplifier</a>

Simplifier for Physical Expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/statistics/index.html" class="mod" title="mod datafusion::physical_expr::statistics">statistics</a>

Statistics and constraint propagation library

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/tree_node/index.html" class="mod" title="mod datafusion::physical_expr::tree_node">tree_node</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/index.html" class="mod" title="mod datafusion::physical_expr::utils">utils</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/window/index.html" class="mod" title="mod datafusion::physical_expr::window">window</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::AnalysisContext">AnalysisContext</a>  
The shared context used during the analysis of an expression. Includes the boundaries for all known columns.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>  
A structure representing a expression known to be constant in a physical execution plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>  
`EquivalenceProperties` stores information about the output of a plan node that can be used to optimize the plan. Currently, it keeps track of:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries">ExprBoundaries</a>  
Represents the boundaries (e.g. min and max values) of a particular column

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html" class="struct" title="struct datafusion::physical_expr::GroupsAccumulatorAdapter">GroupsAccumulatorAdapter</a>  
An adapter that implements [`GroupsAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html "trait datafusion::logical_expr::GroupsAccumulator") for any [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>  
This object represents a lexicographical ordering and contains a vector of `PhysicalSortExpr` objects.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexRequirement.html" class="struct" title="struct datafusion::physical_expr::LexRequirement">LexRequirement</a>  
This object represents a lexicographical ordering requirement and contains a vector of `PhysicalSortRequirement` objects.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html" class="struct" title="struct datafusion::physical_expr::NullState">NullState</a>  
Track the accumulator null state per row: if any values for that group were null and if any values have been seen at all for that group.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html" class="struct" title="struct datafusion::physical_expr::PhysicalExprSimplifier">PhysicalExprSimplifier</a>  
Simplifies physical expressions by applying various optimizations

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>  
Represents Sort operation for a column in a RecordBatch

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>  
Represents sort requirement associated with a plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ScalarFunctionExpr.html" class="struct" title="struct datafusion::physical_expr::ScalarFunctionExpr">ScalarFunctionExpr</a>  
Physical expression of a scalar function

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>  
Represents whether a constant expression’s value is uniform or varies across partitions. Has two variants:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html" class="enum" title="enum datafusion::physical_expr::Distribution">Distribution</a>  
How data is distributed amongst partitions. See [`Partitioning`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html "enum datafusion::physical_expr::Partitioning") for more details.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.OrderingRequirements.html" class="enum" title="enum datafusion::physical_expr::OrderingRequirements">OrderingRequirements</a>  
Represents a plan’s input ordering requirements. Vector elements represent alternative ordering requirements in the order of preference. The list of alternatives can be either hard or soft, depending on whether the operator can work without an input ordering.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>  
Output partitioning supported by [`ExecutionPlan`](https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html)s.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>  
[`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr")s represent expressions such as `A + 1` or `CAST(c1 AS int)`.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.add_offset_to_expr.html" class="fn" title="fn datafusion::physical_expr::add_offset_to_expr">add_offset_to_expr</a>  
Adds the `offset` value to `Column` indices inside `expr`. This function is generally used during the update of the right table schema in join operations.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.add_offset_to_physical_sort_exprs.html" class="fn" title="fn datafusion::physical_expr::add_offset_to_physical_sort_exprs">add_offset_to_physical_sort_exprs</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.analyze.html" class="fn" title="fn datafusion::physical_expr::analyze">analyze</a>  
Attempts to refine column boundaries and compute a selectivity value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.calculate_union.html" class="fn" title="fn datafusion::physical_expr::calculate_union">calculate_union</a>  
Calculates the union (in the sense of `UnionExec`) `EquivalenceProperties` of the given `EquivalenceProperties` in `eqps` according to the given output `schema` (which need not be the same with those of `lhs` and `rhs` as details such as nullability may be different).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.conjunction.html" class="fn" title="fn datafusion::physical_expr::conjunction">conjunction</a>  
Create a conjunction of the given predicates. If the input is empty, return a literal true. If the input contains a single predicate, return the predicate. Otherwise, return a conjunction of the predicates (e.g. `a AND b AND c`).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.conjunction_opt.html" class="fn" title="fn datafusion::physical_expr::conjunction_opt">conjunction_opt</a>  
Create a conjunction of the given predicates. If the input is empty or the return None. If the input contains a single predicate, return Some(predicate). Otherwise, return a Some(..) of a conjunction of the predicates (e.g. `Some(a AND b AND c)`).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_ordering.html" class="fn" title="fn datafusion::physical_expr::create_ordering">create_ordering</a>  
Converts logical sort expressions to physical sort expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_physical_expr.html" class="fn" title="fn datafusion::physical_expr::create_physical_expr">create_physical_expr</a>  
[PhysicalExpr](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") evaluate DataFusion expressions such as `A + 1`, or `CAST(c1 AS int)`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_physical_exprs.html" class="fn" title="fn datafusion::physical_expr::create_physical_exprs">create_physical_exprs</a>  
Create vector of Physical Expression from a vector of logical expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_physical_sort_expr.html" class="fn" title="fn datafusion::physical_expr::create_physical_sort_expr">create_physical_sort_expr</a>  
Create a physical sort expression from a logical expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_physical_sort_exprs.html" class="fn" title="fn datafusion::physical_expr::create_physical_sort_exprs">create_physical_sort_exprs</a>  
Create vector of physical sort expression from a vector of logical expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.physical_exprs_bag_equal.html" class="fn" title="fn datafusion::physical_expr::physical_exprs_bag_equal">physical_exprs_bag_equal</a>  
Checks whether the given physical expression slices are equal in the sense of bags (multi-sets), disregarding their orderings.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.physical_exprs_contains.html" class="fn" title="fn datafusion::physical_expr::physical_exprs_contains">physical_exprs_contains</a>  
This function is similar to the `contains` method of `Vec`. It finds whether `expr` is among `physical_exprs`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.physical_exprs_equal.html" class="fn" title="fn datafusion::physical_expr::physical_exprs_equal">physical_exprs_equal</a>  
Checks whether the given physical expression slices are equal.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.split_conjunction.html" class="fn" title="fn datafusion::physical_expr::split_conjunction">split_conjunction</a>  
Assume the predicate is in the form of CNF, split the predicate to a Vec of PhysicalExprs.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/type.PhysicalExprRef.html" class="type" title="type datafusion::physical_expr::PhysicalExprRef">PhysicalExprRef</a>  
Shared [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr").
