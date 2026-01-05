# Module interleave Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/lib.rs.html#31" class="src">Source</a>

Expand description

Interleave elements from multiple arrays

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/interleave/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/interleave/fn.interleave.html" class="fn" title="fn arrow::compute::kernels::interleave::interleave">interleave</a>  
Takes elements by index from a list of [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), creating a new [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") from those values.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/interleave/fn.interleave_record_batch.html" class="fn" title="fn arrow::compute::kernels::interleave::interleave_record_batch">interleave_record_batch</a>  
Interleave rows by index from multiple [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") instances and return a new [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch").
