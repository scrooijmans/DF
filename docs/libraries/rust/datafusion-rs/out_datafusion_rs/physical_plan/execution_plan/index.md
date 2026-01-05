# Module execution_plan Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#71" class="src">Source</a>

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/expressions/index.html" class="mod" title="mod datafusion::physical_plan::execution_plan::expressions">expressions</a>  
Defines physical expressions that can evaluated at runtime during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/hash_utils/index.html" class="mod" title="mod datafusion::physical_plan::execution_plan::hash_utils">hash_utils</a>  
Functionality used both on logical and physical plans

## Macros<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/macro.internal_err.html" class="macro" title="macro datafusion::physical_plan::execution_plan::internal_err">internal_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/struct.ColumnStatistics.html" class="struct" title="struct datafusion::physical_plan::execution_plan::ColumnStatistics">ColumnStatistics</a>  
Statistics for a column within a relation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/struct.DefaultDisplay.html" class="struct" title="struct datafusion::physical_plan::execution_plan::DefaultDisplay">DefaultDisplay</a>  
A new type wrapper to display `T` implementing`DisplayAs` using the `Default` mode

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/struct.EmptyRecordBatchStream.html" class="struct" title="struct datafusion::physical_plan::execution_plan::EmptyRecordBatchStream">EmptyRecordBatchStream</a>  
`EmptyRecordBatchStream` can be used to create a [`RecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html "trait datafusion::execution::RecordBatchStream") that will produce no results

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/struct.Metric.html" class="struct" title="struct datafusion::physical_plan::execution_plan::Metric">Metric</a>  
Something that tracks a value of interest (metric) of a DataFusion [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") execution.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::execution_plan::PlanProperties">PlanProperties</a>  
Stores certain, often expensive to compute, plan properties used in query optimization.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/struct.Statistics.html" class="struct" title="struct datafusion::physical_plan::execution_plan::Statistics">Statistics</a>  
Statistics for a relation Fields are optional and can be inexact because the sources sometimes provide approximate estimates for performance reasons and the transformations output are not always predictable.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/struct.VerboseDisplay.html" class="struct" title="struct datafusion::physical_plan::execution_plan::VerboseDisplay">VerboseDisplay</a>  
A new type wrapper to display `T` implementing `DisplayAs` using the `Verbose` mode

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>  
Represents whether a stream of data **generated** by an operator is bounded (finite) or unbounded (infinite).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html" class="enum" title="enum datafusion::physical_plan::execution_plan::CardinalityEffect">CardinalityEffect</a>  
Indicates the effect an execution plan operator will have on the cardinality of its input stream

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.ColumnarValue.html" class="enum" title="enum datafusion::physical_plan::execution_plan::ColumnarValue">ColumnarValue</a>  
The result of evaluating an expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::DisplayFormatType">DisplayFormatType</a>  
Options for controlling how each [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") should format itself

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Distribution.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Distribution">Distribution</a>  
How data is distributed amongst partitions. See [`Partitioning`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html "enum datafusion::physical_expr::Partitioning") for more details.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>  
Represents how an operator emits its output records.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EvaluationType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EvaluationType">EvaluationType</a>  
Represents how an operator’s `Stream` implementation generates `RecordBatch`es.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.InputOrderMode.html" class="enum" title="enum datafusion::physical_plan::execution_plan::InputOrderMode">InputOrderMode</a>  
Specifies how the input to an aggregation or window operator is ordered relative to their `GROUP BY` or `PARTITION BY` expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.InvariantLevel.html" class="enum" title="enum datafusion::physical_plan::execution_plan::InvariantLevel">InvariantLevel</a>  
[`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") Invariant Level

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Partitioning.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Partitioning">Partitioning</a>  
Output partitioning supported by [`ExecutionPlan`](https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html)s.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.SchedulingType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::SchedulingType">SchedulingType</a>  
Represents whether an operator’s `Stream` has been implemented to actively cooperate with the Tokio scheduler or not. Please refer to the [`coop`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/index.html "mod datafusion::physical_plan::coop") module for more details.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/trait.Accumulator.html" class="trait" title="trait datafusion::physical_plan::execution_plan::Accumulator">Accumulator</a>  
Tracks an aggregate function’s state.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::execution_plan::DisplayAs">DisplayAs</a>  
Trait for types which could have additional details when formatted in `Verbose` mode

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::execution_plan::ExecutionPlan">ExecutionPlan</a>  
Represent nodes in the DataFusion Physical Plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/trait.ExecutionPlanProperties.html" class="trait" title="trait datafusion::physical_plan::execution_plan::ExecutionPlanProperties">ExecutionPlanProperties</a>  
Extension trait provides an easy API to fetch various properties of [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") objects based on [`ExecutionPlan::properties`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties "method datafusion::physical_plan::ExecutionPlan::properties").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_plan::execution_plan::PhysicalExpr">PhysicalExpr</a>  
[`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr")s represent expressions such as `A + 1` or `CAST(c1 AS int)`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/trait.RecordBatchStream.html" class="trait" title="trait datafusion::physical_plan::execution_plan::RecordBatchStream">RecordBatchStream</a>  
Trait for types that stream [RecordBatch](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/trait.WindowExpr.html" class="trait" title="trait datafusion::physical_plan::execution_plan::WindowExpr">WindowExpr</a>  
Common trait for [window function](https://en.wikipedia.org/wiki/Window_function_(SQL)) implementations

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.check_default_invariants.html" class="fn" title="fn datafusion::physical_plan::execution_plan::check_default_invariants">check_default_invariants</a>  
Checks a set of invariants that apply to all ExecutionPlan implementations. Returns an error if the given node does not conform.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.check_not_null_constraints.html" class="fn" title="fn datafusion::physical_plan::execution_plan::check_not_null_constraints">check_not_null_constraints</a>  
Checks a `RecordBatch` for `not null` constraints on specified columns.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.collect.html" class="fn" title="fn datafusion::physical_plan::execution_plan::collect">collect</a>  
Execute the [ExecutionPlan](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") and collect the results in memory

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.collect_partitioned.html" class="fn" title="fn datafusion::physical_plan::execution_plan::collect_partitioned">collect_partitioned</a>  
Execute the [ExecutionPlan](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") and collect the results in memory

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.displayable.html" class="fn" title="fn datafusion::physical_plan::execution_plan::displayable">displayable</a>  
Return a [`DisplayableExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html "struct datafusion::physical_plan::display::DisplayableExecutionPlan") wrapper around an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") which can be displayed in various easier to understand ways.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.execute_input_stream.html" class="fn" title="fn datafusion::physical_plan::execution_plan::execute_input_stream">execute_input_stream</a>  
Executes an input stream and ensures that the resulting stream adheres to the `not null` constraints specified in the `sink_schema`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.execute_stream.html" class="fn" title="fn datafusion::physical_plan::execution_plan::execute_stream">execute_stream</a>  
Execute the [ExecutionPlan](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") and return a single stream of `RecordBatch`es.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.execute_stream_partitioned.html" class="fn" title="fn datafusion::physical_plan::execution_plan::execute_stream_partitioned">execute_stream_partitioned</a>  
Execute the [ExecutionPlan](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") and return a vec with one stream per output partition

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.get_plan_string.html" class="fn" title="fn datafusion::physical_plan::execution_plan::get_plan_string">get_plan_string</a>  
Utility function yielding a string representation of the given [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.need_data_exchange.html" class="fn" title="fn datafusion::physical_plan::execution_plan::need_data_exchange">need_data_exchange</a>  
Indicate whether a data exchange is needed for the input of `plan`, which will be very helpful especially for the distributed engine to judge whether need to deal with shuffling. Currently, there are 3 kinds of execution plan which needs data exchange 1. RepartitionExec for changing the partition number between two `ExecutionPlan`s 2. CoalescePartitionsExec for collapsing all of the partitions into one without ordering guarantee 3. SortPreservingMergeExec for collapsing all of the sorted partitions into one with ordering guarantee

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.project_schema.html" class="fn" title="fn datafusion::physical_plan::execution_plan::project_schema">project_schema</a>  
Applies an optional projection to a [`SchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html "type datafusion::common::arrow::datatypes::SchemaRef"), returning the projected schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.with_new_children_if_necessary.html" class="fn" title="fn datafusion::physical_plan::execution_plan::with_new_children_if_necessary">with_new_children_if_necessary</a>  
Returns a copy of this plan if we change any child according to the pointer comparison. The size of `children` must be equal to the size of `ExecutionPlan::children()`.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/type.SendableRecordBatchStream.html" class="type" title="type datafusion::physical_plan::execution_plan::SendableRecordBatchStream">SendableRecordBatchStream</a>  
Trait for a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es that can be passed between threads
