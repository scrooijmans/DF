# Module common Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#67" class="src">Source</a>

Expand description

Defines common code used in execution plans

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/common/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/common/fn.build_checked_file_list.html" class="fn" title="fn datafusion::physical_plan::common::build_checked_file_list">build_checked_file_list</a>  
Recursively builds a list of files in a directory with a given extension

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/common/fn.build_file_list.html" class="fn" title="fn datafusion::physical_plan::common::build_file_list">build_file_list</a>  
Recursively builds a list of files in a directory with a given extension

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/common/fn.can_project.html" class="fn" title="fn datafusion::physical_plan::common::can_project">can_project</a>  
Checks if the given projection is valid for the given schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/common/fn.collect.html" class="fn" title="fn datafusion::physical_plan::common::collect">collect</a>  
Create a vector of record batches from a stream

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/common/fn.compute_record_batch_statistics.html" class="fn" title="fn datafusion::physical_plan::common::compute_record_batch_statistics">compute_record_batch_statistics</a>  
Computes the statistics for an in-memory RecordBatch

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/common/fn.spawn_buffered.html" class="fn" title="fn datafusion::physical_plan::common::spawn_buffered">spawn_buffered</a>  
If running in a tokio context spawns the execution of `stream` to a separate task allowing it to execute in parallel with an intermediate buffer of size `buffer`
