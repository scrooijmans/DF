# Function make_cooperative Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/coop.rs.html#333" class="src">Source</a>

``` rust
pub fn make_cooperative(
    stream: Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>,
) -> Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>
```

Expand description

Wraps a `SendableRecordBatchStream` inside a [`CooperativeStream`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html "struct datafusion::physical_plan::coop::CooperativeStream") to enable cooperative multitasking. Since `SendableRecordBatchStream` is a `dyn RecordBatchStream` this requires the use of dynamic method dispatch. When the stream type is statically known, consider use the generic [`cooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/fn.cooperative.html "fn datafusion::physical_plan::coop::cooperative") function to allow static method dispatch.
