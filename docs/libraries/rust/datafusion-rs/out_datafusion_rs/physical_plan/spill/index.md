# Module spill Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#84" class="src">Source</a>

Expand description

Defines the spilling functions

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/spill/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/spill/fn.get_record_batch_memory_size.html" class="fn" title="fn datafusion::physical_plan::spill::get_record_batch_memory_size">get_record_batch_memory_size</a>  
Calculate total used memory of this batch.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/spill/fn.spill_record_batch_by_size.html" class="fn" title="fn datafusion::physical_plan::spill::spill_record_batch_by_size">spill_record_batch_by_size</a>Deprecated  
Spill the `RecordBatch` to disk as smaller batches split by `batch_size_rows`
