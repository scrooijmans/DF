# Function cooperative Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/coop.rs.html#321-323" class="src">Source</a>

``` rust
pub fn cooperative<T>(stream: T) -> CooperativeStream<T>where
    T: RecordBatchStream + Unpin + Send + 'static,
```

Expand description

Creates a [`CooperativeStream`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html "struct datafusion::physical_plan::coop::CooperativeStream") wrapper around the given [`RecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html "trait datafusion::execution::RecordBatchStream"). This wrapper collaborates with the Tokio cooperative scheduler by consuming a unit of scheduling budget for each returned record batch.
