# Struct SpillManager Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/spill/spill_manager.rs.html#41" class="src">Source</a>

``` rust
pub struct SpillManager { /* private fields */ }
```

Expand description

The `SpillManager` is responsible for the following tasks:

- Reading and writing `RecordBatch`es to raw files based on the provided configurations.
- Updating the associated metrics.

Note: The caller (external operators such as `SortExec`) is responsible for interpreting the spilled files. For example, all records within the same spill file are ordered according to a specific order.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#impl-SpillManager" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html" class="struct" title="struct datafusion::physical_plan::SpillManager">SpillManager</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#method.new" class="fn">new</a>( env: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>, metrics: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.SpillMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::SpillMetrics">SpillMetrics</a>, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html" class="struct" title="struct datafusion::physical_plan::SpillManager">SpillManager</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#method.with_batch_read_buffer_capacity" class="fn">with_batch_read_buffer_capacity</a>( self, batch_read_buffer_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html" class="struct" title="struct datafusion::physical_plan::SpillManager">SpillManager</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#method.with_compression_type" class="fn">with_compression_type</a>( self, spill_compression: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.SpillCompression.html" class="enum" title="enum datafusion::config::SpillCompression">SpillCompression</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html" class="struct" title="struct datafusion::physical_plan::SpillManager">SpillManager</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#method.create_in_progress_file" class="fn">create_in_progress_file</a>( &self, request_msg: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<InProgressSpillFile, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Creates a temporary file for in-progress operations, returning an error message if file creation fails. The file can be used to append batches incrementally and then finish the file when done.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#method.spill_record_batch_and_finish" class="fn">spill_record_batch_and_finish</a>( &self, batches: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\], request_msg: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html" class="struct" title="struct datafusion::execution::disk_manager::RefCountedTempFile">RefCountedTempFile</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Spill input `batches` into a single file in a atomic operation. If it is intended to incrementally write in-memory batches into the same spill file, use [`Self::create_in_progress_file`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#method.create_in_progress_file "method datafusion::physical_plan::SpillManager::create_in_progress_file") instead. None is returned if no batches are spilled.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#errors" class="doc-anchor">§</a>Errors

- Returns an error if spilling would exceed the disk usage limit configured by `max_temp_directory_size` in `DiskManager`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#method.read_spill_as_stream" class="fn">read_spill_as_stream</a>( &self, spill_file_path: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html" class="struct" title="struct datafusion::execution::disk_manager::RefCountedTempFile">RefCountedTempFile</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Reads a spill file as a stream. The file must be created by the current `SpillManager`. This method will generate output in FIFO order: the batch appended first will be read first.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#impl-Clone-for-SpillManager" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html" class="struct" title="struct datafusion::physical_plan::SpillManager">SpillManager</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html" class="struct" title="struct datafusion::physical_plan::SpillManager">SpillManager</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#impl-Debug-for-SpillManager" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html" class="struct" title="struct datafusion::physical_plan::SpillManager">SpillManager</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.SpillManager.html#blanket-implementations" class="anchor">§</a>
