# Function execute_stream Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#1161-1164" class="src">Source</a>

``` rust
pub fn execute_stream(
    plan: Arc<dyn ExecutionPlan>,
    context: Arc<TaskContext>,
) -> Result<Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>, DataFusionError>
```

Expand description

Execute the [ExecutionPlan](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") and return a single stream of `RecordBatch`es.

See [collect](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.collect.html "fn datafusion::physical_plan::collect") to buffer the `RecordBatch`es in memory.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.execute_stream.html#aborting-execution" class="doc-anchor">§</a>Aborting Execution

Dropping the stream will abort the execution of the query, and free up any allocated resources
