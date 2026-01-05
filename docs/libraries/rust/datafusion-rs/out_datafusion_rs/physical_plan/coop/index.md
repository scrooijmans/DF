# Module coop Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#68" class="src">Source</a>

Expand description

Utilities for improved cooperative scheduling.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/index.html#cooperative-scheduling" class="doc-anchor">§</a>Cooperative scheduling

A single call to `poll_next` on a top-level [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") may potentially perform a lot of work before it returns a `Poll::Pending`. Think for instance of calculating an aggregation over a large dataset. If a `Stream` runs for a long period of time without yielding back to the Tokio executor, it can starve other tasks waiting on that executor to execute them. Additionally, this prevents the query execution from being cancelled.

To ensure that `Stream` implementations yield regularly, operators can insert explicit yield points using the utilities in this module. For most operators this is **not** necessary. The `Stream`s of the built-in DataFusion operators that generate (rather than manipulate) `RecordBatch`es such as `DataSourceExec` and those that eagerly consume `RecordBatch`es (for instance, `RepartitionExec`) contain yield points that will make most query `Stream`s yield periodically.

There are a couple of types of operators that *should* insert yield points:

- New source operators that do not make use of Tokio resources
- Exchange like operators that do not use Tokio’s `Channel` implementation to pass data between tasks

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/index.html#adding-yield-points" class="doc-anchor">§</a>Adding yield points

Yield points can be inserted manually using the facilities provided by the [Tokio coop module](https://docs.rs/tokio/latest/tokio/task/coop/index.html) such as [`tokio::task::coop::consume_budget`](https://docs.rs/tokio/latest/tokio/task/coop/fn.consume_budget.html).

Another option is to use the wrapper `Stream` implementation provided by this module which will consume a unit of task budget every time a `RecordBatch` is produced. Wrapper `Stream`s can be created using the [`cooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/fn.cooperative.html "fn datafusion::physical_plan::coop::cooperative") and [`make_cooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/fn.make_cooperative.html "fn datafusion::physical_plan::coop::make_cooperative") functions.

[`cooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/fn.cooperative.html "fn datafusion::physical_plan::coop::cooperative") is a generic function that takes ownership of the wrapped [`RecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html "trait datafusion::execution::RecordBatchStream"). This function has the benefit of not requiring an additional heap allocation and can avoid dynamic dispatch.

[`make_cooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/fn.make_cooperative.html "fn datafusion::physical_plan::coop::make_cooperative") is a non-generic function that wraps a [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream"). This can be used to wrap dynamically typed, heap allocated [`RecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html "trait datafusion::execution::RecordBatchStream")s.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/index.html#automatic-cooperation" class="doc-anchor">§</a>Automatic cooperation

The `EnsureCooperative` physical optimizer rule, which is included in the default set of optimizer rules, inspects query plans for potential cooperative scheduling issues. It injects the [`CooperativeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeExec.html "struct datafusion::physical_plan::coop::CooperativeExec") wrapper `ExecutionPlan` into the query plan where necessary. This `ExecutionPlan` uses [`make_cooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/fn.make_cooperative.html "fn datafusion::physical_plan::coop::make_cooperative") to wrap the `Stream` of its input.

The optimizer rule currently checks the plan for exchange-like operators and leave operators that report [`SchedulingType::NonCooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.SchedulingType.html#variant.NonCooperative "variant datafusion::physical_plan::execution_plan::SchedulingType::NonCooperative") in their [plan properties](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties "method datafusion::physical_plan::ExecutionPlan::properties").

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeExec.html" class="struct" title="struct datafusion::physical_plan::coop::CooperativeExec">CooperativeExec</a>  
An execution plan decorator that enables cooperative multitasking. It wraps the streams produced by its input execution plan using the [`make_cooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/fn.make_cooperative.html "fn datafusion::physical_plan::coop::make_cooperative") function, which makes the stream participate in Tokio cooperative scheduling.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html" class="struct" title="struct datafusion::physical_plan::coop::CooperativeStream">CooperativeStream</a>  
A stream that passes record batches through unchanged while cooperating with the Tokio runtime. It consumes cooperative scheduling budget for each returned [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch"), allowing other tasks to execute when the budget is exhausted.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/fn.cooperative.html" class="fn" title="fn datafusion::physical_plan::coop::cooperative">cooperative</a>  
Creates a [`CooperativeStream`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html "struct datafusion::physical_plan::coop::CooperativeStream") wrapper around the given [`RecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html "trait datafusion::execution::RecordBatchStream"). This wrapper collaborates with the Tokio cooperative scheduler by consuming a unit of scheduling budget for each returned record batch.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/fn.make_cooperative.html" class="fn" title="fn datafusion::physical_plan::coop::make_cooperative">make_cooperative</a>  
Wraps a `SendableRecordBatchStream` inside a [`CooperativeStream`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html "struct datafusion::physical_plan::coop::CooperativeStream") to enable cooperative multitasking. Since `SendableRecordBatchStream` is a `dyn RecordBatchStream` this requires the use of dynamic method dispatch. When the stream type is statically known, consider use the generic [`cooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/fn.cooperative.html "fn datafusion::physical_plan::coop::cooperative") function to allow static method dispatch.
