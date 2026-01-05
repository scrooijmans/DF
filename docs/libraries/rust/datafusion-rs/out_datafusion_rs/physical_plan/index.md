# Module physical_plan Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#817" class="src">Source</a>

Expand description

re-export of [`datafusion_physical_plan`](https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_plan/index.html "mod datafusion_physical_plan") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/index.html" class="mod" title="mod datafusion::physical_plan::aggregates">aggregates</a>  
Aggregates functionalities

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/analyze/index.html" class="mod" title="mod datafusion::physical_plan::analyze">analyze</a>  
Defines the ANALYZE operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/async_func/index.html" class="mod" title="mod datafusion::physical_plan::async_func">async_func</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce/index.html" class="mod" title="mod datafusion::physical_plan::coalesce">coalesce</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_batches/index.html" class="mod" title="mod datafusion::physical_plan::coalesce_batches">coalesce_batches</a>  
[`CoalesceBatchesExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_batches/struct.CoalesceBatchesExec.html "struct datafusion::physical_plan::coalesce_batches::CoalesceBatchesExec") combines small batches into larger batches.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/index.html" class="mod" title="mod datafusion::physical_plan::coalesce_partitions">coalesce_partitions</a>  
Defines the merge plan for executing partitions in parallel and then merging the results into a single partition

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/common/index.html" class="mod" title="mod datafusion::physical_plan::common">common</a>  
Defines common code used in execution plans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/index.html" class="mod" title="mod datafusion::physical_plan::coop">coop</a>  
Utilities for improved cooperative scheduling.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/index.html" class="mod" title="mod datafusion::physical_plan::display">display</a>  
Implementation of physical plan display. See [`crate::displayable`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.displayable.html "fn datafusion::physical_plan::displayable") for examples of how to format

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/empty/index.html" class="mod" title="mod datafusion::physical_plan::empty">empty</a>  
EmptyRelation with produce_one_row=false execution plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/index.html" class="mod" title="mod datafusion::physical_plan::execution_plan">execution_plan</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/explain/index.html" class="mod" title="mod datafusion::physical_plan::explain">explain</a>  
Defines the EXPLAIN operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/index.html" class="mod" title="mod datafusion::physical_plan::filter">filter</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/index.html" class="mod" title="mod datafusion::physical_plan::filter_pushdown">filter_pushdown</a>  
Filter Pushdown Optimization Process

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/index.html" class="mod" title="mod datafusion::physical_plan::joins">joins</a>  
DataFusion Join implementations

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/index.html" class="mod" title="mod datafusion::physical_plan::limit">limit</a>  
Defines the LIMIT plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/index.html" class="mod" title="mod datafusion::physical_plan::memory">memory</a>  
Execution plan for reading in-memory batches of data

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/index.html" class="mod" title="mod datafusion::physical_plan::metrics">metrics</a>  
Metrics for recording information about execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/placeholder_row/index.html" class="mod" title="mod datafusion::physical_plan::placeholder_row">placeholder_row</a>  
EmptyRelation produce_one_row=true execution plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/index.html" class="mod" title="mod datafusion::physical_plan::projection">projection</a>  
Defines the projection execution plan. A projection determines which columns or expressions are returned from a query. The SQL statement `SELECT a, b, a+b FROM t1` is an example of a projection on table `t1` where the expressions `a`, `b`, and `a+b` are the projection expressions. `SELECT` without `FROM` will only evaluate expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/recursive_query/index.html" class="mod" title="mod datafusion::physical_plan::recursive_query">recursive_query</a>  
Defines the recursive query plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/index.html" class="mod" title="mod datafusion::physical_plan::repartition">repartition</a>  
This file implements the [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec") operator, which maps N input partitions to M output partitions based on a partitioning scheme, optionally maintaining the order of the input rows in the output.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/index.html" class="mod" title="mod datafusion::physical_plan::sorts">sorts</a>  
Sort functionalities

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/spill/index.html" class="mod" title="mod datafusion::physical_plan::spill">spill</a>  
Defines the spilling functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/index.html" class="mod" title="mod datafusion::physical_plan::stream">stream</a>  
Stream wrappers for physical operators

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/index.html" class="mod" title="mod datafusion::physical_plan::streaming">streaming</a>  
Generic plans for deferred execution: [`StreamingTableExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/struct.StreamingTableExec.html "struct datafusion::physical_plan::streaming::StreamingTableExec") and [`PartitionStream`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/trait.PartitionStream.html "trait datafusion::physical_plan::streaming::PartitionStream")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/index.html" class="mod" title="mod datafusion::physical_plan::test">test</a>  
Utilities for testing datafusion-physical-plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/tree_node/index.html" class="mod" title="mod datafusion::physical_plan::tree_node">tree_node</a>  
This module provides common traits for visiting or rewriting tree nodes easily.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/udaf/index.html" class="mod" title="mod datafusion::physical_plan::udaf">udaf</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/index.html" class="mod" title="mod datafusion::physical_plan::union">union</a>  
The Union operator combines multiple inputs with the same schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/unnest/index.html" class="mod" title="mod datafusion::physical_plan::unnest">unnest</a>  
Define a plan for unnesting values in columns that contain a list type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/index.html" class="mod" title="mod datafusion::physical_plan::windows">windows</a>  
Physical expressions for window functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/work_table/index.html" class="mod" title="mod datafusion::physical_plan::work_table">work_table</a>  
Defines the work table query plan

## Macros<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/macro.handle_state.html" class="macro" title="macro datafusion::physical_plan::handle_state">handle_state</a>  
The `handle_state` macro is designed to process the result of a state-changing operation. It operates on a `StatefulStreamResult` by matching its variants and executing corresponding actions. This macro is used to streamline code that deals with state transitions, reducing boilerplate and improving readability.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/macro.internal_err.html" class="macro" title="macro datafusion::physical_plan::internal_err">internal_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.ColumnStatistics.html" class="struct" title="struct datafusion::physical_plan::ColumnStatistics">ColumnStatistics</a>  
Statistics for a column within a relation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.DefaultDisplay.html" class="struct" title="struct datafusion::physical_plan::DefaultDisplay">DefaultDisplay</a>  
A new type wrapper to display `T` implementing`DisplayAs` using the `Default` mode

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html" class="struct" title="struct datafusion::physical_plan::EmptyRecordBatchStream">EmptyRecordBatchStream</a>  
`EmptyRecordBatchStream` can be used to create a [`RecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html "trait datafusion::execution::RecordBatchStream") that will produce no results

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html" class="struct" title="struct datafusion::physical_plan::Metric">Metric</a>  
Something that tracks a value of interest (metric) of a DataFusion [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") execution.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>  
Stores certain, often expensive to compute, plan properties used in query optimization.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html" class="struct" title="struct datafusion::physical_plan::SpillManager">SpillManager</a>  
The `SpillManager` is responsible for the following tasks:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html" class="struct" title="struct datafusion::physical_plan::Statistics">Statistics</a>  
Statistics for a relation Fields are optional and can be inexact because the sources sometimes provide approximate estimates for performance reasons and the transformations output are not always predictable.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html" class="struct" title="struct datafusion::physical_plan::TopK">TopK</a>  
Global TopK

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.VerboseDisplay.html" class="struct" title="struct datafusion::physical_plan::VerboseDisplay">VerboseDisplay</a>  
A new type wrapper to display `T` implementing `DisplayAs` using the `Verbose` mode

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.WorkTable.html" class="struct" title="struct datafusion::physical_plan::WorkTable">WorkTable</a>  
The name is from PostgreSQL’s terminology. See <https://wiki.postgresql.org/wiki/CTEReadme#How_Recursion_Works> This table serves as a mirror or buffer between each iteration of a recursive query.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.ColumnarValue.html" class="enum" title="enum datafusion::physical_plan::ColumnarValue">ColumnarValue</a>  
The result of evaluating an expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>  
Options for controlling how each [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") should format itself

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.Distribution.html" class="enum" title="enum datafusion::physical_plan::Distribution">Distribution</a>  
How data is distributed amongst partitions. See [`Partitioning`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html "enum datafusion::physical_expr::Partitioning") for more details.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html" class="enum" title="enum datafusion::physical_plan::InputOrderMode">InputOrderMode</a>  
Specifies how the input to an aggregation or window operator is ordered relative to their `GROUP BY` or `PARTITION BY` expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.Partitioning.html" class="enum" title="enum datafusion::physical_plan::Partitioning">Partitioning</a>  
Output partitioning supported by [`ExecutionPlan`](https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html)s.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html" class="trait" title="trait datafusion::physical_plan::Accumulator">Accumulator</a>  
Tracks an aggregate function’s state.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a>  
Trait for types which could have additional details when formatted in `Verbose` mode

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>  
Represent nodes in the DataFusion Physical Plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlanProperties">ExecutionPlanProperties</a>  
Extension trait provides an easy API to fetch various properties of [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") objects based on [`ExecutionPlan::properties`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties "method datafusion::physical_plan::ExecutionPlan::properties").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlanVisitor">ExecutionPlanVisitor</a>  
Trait that implements the [Visitor pattern](https://en.wikipedia.org/wiki/Visitor_pattern) for a depth first walk of `ExecutionPlan` nodes. `pre_visit` is called before any children are visited, and then `post_visit` is called after all children have been visited.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_plan::PhysicalExpr">PhysicalExpr</a>  
[`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr")s represent expressions such as `A + 1` or `CAST(c1 AS int)`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html" class="trait" title="trait datafusion::physical_plan::RecordBatchStream">RecordBatchStream</a>  
Trait for types that stream [RecordBatch](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.WindowExpr.html" class="trait" title="trait datafusion::physical_plan::WindowExpr">WindowExpr</a>  
Common trait for [window function](https://en.wikipedia.org/wiki/Window_function_(SQL)) implementations

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.accept.html" class="fn" title="fn datafusion::physical_plan::accept">accept</a>  
Visit all children of this plan, according to the order defined on `ExecutionPlanVisitor`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.collect.html" class="fn" title="fn datafusion::physical_plan::collect">collect</a>  
Execute the [ExecutionPlan](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") and collect the results in memory

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.collect_partitioned.html" class="fn" title="fn datafusion::physical_plan::collect_partitioned">collect_partitioned</a>  
Execute the [ExecutionPlan](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") and collect the results in memory

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.displayable.html" class="fn" title="fn datafusion::physical_plan::displayable">displayable</a>  
Return a [`DisplayableExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html "struct datafusion::physical_plan::display::DisplayableExecutionPlan") wrapper around an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") which can be displayed in various easier to understand ways.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.execute_input_stream.html" class="fn" title="fn datafusion::physical_plan::execute_input_stream">execute_input_stream</a>  
Executes an input stream and ensures that the resulting stream adheres to the `not null` constraints specified in the `sink_schema`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.execute_stream.html" class="fn" title="fn datafusion::physical_plan::execute_stream">execute_stream</a>  
Execute the [ExecutionPlan](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") and return a single stream of `RecordBatch`es.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.execute_stream_partitioned.html" class="fn" title="fn datafusion::physical_plan::execute_stream_partitioned">execute_stream_partitioned</a>  
Execute the [ExecutionPlan](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") and return a vec with one stream per output partition

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.get_plan_string.html" class="fn" title="fn datafusion::physical_plan::get_plan_string">get_plan_string</a>  
Utility function yielding a string representation of the given [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.project_schema.html" class="fn" title="fn datafusion::physical_plan::project_schema">project_schema</a>  
Applies an optional projection to a [`SchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html "type datafusion::common::arrow::datatypes::SchemaRef"), returning the projected schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.visit_execution_plan.html" class="fn" title="fn datafusion::physical_plan::visit_execution_plan">visit_execution_plan</a>  
Recursively calls `pre_visit` and `post_visit` for this node and all of its children, as described on [`ExecutionPlanVisitor`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html "trait datafusion::physical_plan::ExecutionPlanVisitor")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.with_new_children_if_necessary.html" class="fn" title="fn datafusion::physical_plan::with_new_children_if_necessary">with_new_children_if_necessary</a>  
Returns a copy of this plan if we change any child according to the pointer comparison. The size of `children` must be equal to the size of `ExecutionPlan::children()`.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/type.SendableRecordBatchStream.html" class="type" title="type datafusion::physical_plan::SendableRecordBatchStream">SendableRecordBatchStream</a>  
Trait for a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es that can be passed between threads
