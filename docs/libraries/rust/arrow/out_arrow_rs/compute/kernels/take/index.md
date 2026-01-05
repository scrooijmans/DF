# Module take Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/lib.rs.html#33" class="src">Source</a>

Expand description

Defines take kernel for [Array](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

## Structs<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html" class="struct" title="struct arrow::compute::kernels::take::TakeOptions">TakeOptions</a>  
Options that define how `take` should behave

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/fn.take.html" class="fn" title="fn arrow::compute::kernels::take::take">take</a>  
Take elements by index from [Array](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), creating a new [Array](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") from those indexes.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/fn.take_arrays.html" class="fn" title="fn arrow::compute::kernels::take::take_arrays">take_arrays</a>  
For each [ArrayRef](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") in the [`Vec<ArrayRef>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec"), take elements by index and create a new [`Vec<ArrayRef>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") from those indices.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/fn.take_record_batch.html" class="fn" title="fn arrow::compute::kernels::take::take_record_batch">take_record_batch</a>  
Take rows by index from [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") and returns a new [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") from those indexes.
