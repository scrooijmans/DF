# Module aggregates Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#61" class="src">Source</a>

Expand description

Aggregates functionalities

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/index.html" class="mod" title="mod datafusion::physical_plan::aggregates::group_values">group_values</a>

[`GroupValues`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html "trait datafusion::physical_plan::aggregates::group_values::GroupValues") trait for storing and interning group keys

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/order/index.html" class="mod" title="mod datafusion::physical_plan::aggregates::order">order</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/struct.AggregateExec.html" class="struct" title="struct datafusion::physical_plan::aggregates::AggregateExec">AggregateExec</a>  
Hash aggregate execution plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/struct.PhysicalGroupBy.html" class="struct" title="struct datafusion::physical_plan::aggregates::PhysicalGroupBy">PhysicalGroupBy</a>  
Represents `GROUP BY` clause in the plan (including the more general GROUPING SET) In the case of a simple `GROUP BY a, b` clause, this will contain the expression \[a, b\] and a single group \[false, false\]. In the case of `GROUP BY GROUPING SETS/CUBE/ROLLUP` the planner will expand the expression into multiple groups, using null expressions to align each group. For example, with a group by clause `GROUP BY GROUPING SETS ((a,b),(a),(b))` the planner should create a `PhysicalGroupBy` like

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/enum.AggregateMode.html" class="enum" title="enum datafusion::physical_plan::aggregates::AggregateMode">AggregateMode</a>  
Aggregation modes

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/fn.aggregate_expressions.html" class="fn" title="fn datafusion::physical_plan::aggregates::aggregate_expressions">aggregate_expressions</a>  
Returns physical expressions for arguments to evaluate against a batch.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/fn.concat_slices.html" class="fn" title="fn datafusion::physical_plan::aggregates::concat_slices">concat_slices</a>  
Concatenates the given slices.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/fn.create_accumulators.html" class="fn" title="fn datafusion::physical_plan::aggregates::create_accumulators">create_accumulators</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/fn.evaluate_group_by.html" class="fn" title="fn datafusion::physical_plan::aggregates::evaluate_group_by">evaluate_group_by</a>  
Evaluate a group by expression against a `RecordBatch`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/fn.evaluate_many.html" class="fn" title="fn datafusion::physical_plan::aggregates::evaluate_many">evaluate_many</a>  
Evaluates expressions against a record batch.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/fn.finalize_aggregation.html" class="fn" title="fn datafusion::physical_plan::aggregates::finalize_aggregation">finalize_aggregation</a>  
returns a vector of ArrayRefs, where each entry corresponds to either the final value (mode = Final, FinalPartitioned and Single) or states (mode = Partial)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/fn.get_finer_aggregate_exprs_requirement.html" class="fn" title="fn datafusion::physical_plan::aggregates::get_finer_aggregate_exprs_requirement">get_finer_aggregate_exprs_requirement</a>  
Gets the common requirement that satisfies all the aggregate expressions. When possible, chooses the requirement that is already satisfied by the equivalence properties.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/type.AccumulatorItem.html" class="type" title="type datafusion::physical_plan::aggregates::AccumulatorItem">AccumulatorItem</a>
