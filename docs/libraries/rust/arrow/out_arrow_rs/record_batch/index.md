# Module record_batch Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/lib.rs.html#403" class="src">Source</a>

Expand description

Contains the `RecordBatch` type and associated traits

## Structs<a href="https://docs.rs/arrow/latest/arrow/record_batch/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatch.html" class="struct" title="struct arrow::record_batch::RecordBatch">RecordBatch</a>  
A two-dimensional batch of column-oriented data with a defined [schema](https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html "struct arrow::datatypes::Schema").

<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchIterator.html" class="struct" title="struct arrow::record_batch::RecordBatchIterator">RecordBatchIterator</a>  
Generic implementation of [RecordBatchReader](https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html "trait arrow::array::RecordBatchReader") that wraps an iterator.

<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html" class="struct" title="struct arrow::record_batch::RecordBatchOptions">RecordBatchOptions</a>  
Options that control the behaviour used when creating a [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch").

## Traits<a href="https://docs.rs/arrow/latest/arrow/record_batch/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/record_batch/trait.RecordBatchReader.html" class="trait" title="trait arrow::record_batch::RecordBatchReader">RecordBatchReader</a>  
Trait for types that can read `RecordBatch`’s.

<a href="https://docs.rs/arrow/latest/arrow/record_batch/trait.RecordBatchWriter.html" class="trait" title="trait arrow::record_batch::RecordBatchWriter">RecordBatchWriter</a>  
Trait for types that can write `RecordBatch`’s.
