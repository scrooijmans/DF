# Function execute_input_stream Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#1256-1261" class="src">Source</a>

``` rust
pub fn execute_input_stream(
    input: Arc<dyn ExecutionPlan>,
    sink_schema: Arc<Schema>,
    partition: usize,
    context: Arc<TaskContext>,
) -> Result<Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>, DataFusionError>
```

Expand description

Executes an input stream and ensures that the resulting stream adheres to the `not null` constraints specified in the `sink_schema`.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.execute_input_stream.html#arguments" class="doc-anchor">§</a>Arguments

- `input` - An execution plan
- `sink_schema` - The schema to be applied to the output stream
- `partition` - The partition index to be executed
- `context` - The task context

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.execute_input_stream.html#returns" class="doc-anchor">§</a>Returns

- `Result<SendableRecordBatchStream>` - A stream of `RecordBatch`es if successful

This function first executes the given input plan for the specified partition and context. It then checks if there are any columns in the input that might violate the `not null` constraints specified in the `sink_schema`. If there are such columns, it wraps the resulting stream to enforce the `not null` constraints by invoking the [`check_not_null_constraints`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.check_not_null_constraints.html "fn datafusion::physical_plan::execution_plan::check_not_null_constraints") function on each batch of the stream.
