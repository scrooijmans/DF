# Module filter Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/lib.rs.html#30" class="src">Source</a>

Expand description

Defines filter kernels

## Structs<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/filter/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/filter/struct.FilterBuilder.html" class="struct" title="struct arrow::compute::kernels::filter::FilterBuilder">FilterBuilder</a>  
A builder to construct [`FilterPredicate`](https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html "struct arrow::compute::FilterPredicate")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/filter/struct.FilterPredicate.html" class="struct" title="struct arrow::compute::kernels::filter::FilterPredicate">FilterPredicate</a>  
A filtering predicate that can be applied to an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/filter/struct.SlicesIterator.html" class="struct" title="struct arrow::compute::kernels::filter::SlicesIterator">SlicesIterator</a>  
An iterator of `(usize, usize)` each representing an interval `[start, end)` whose slots of a bitmap [Buffer](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") are true.

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/filter/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/filter/fn.filter.html" class="fn" title="fn arrow::compute::kernels::filter::filter">filter</a>  
Returns a filtered `values` [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") where the corresponding elements of `predicate` are `true`.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/filter/fn.filter_record_batch.html" class="fn" title="fn arrow::compute::kernels::filter::filter_record_batch">filter_record_batch</a>  
Returns a filtered [RecordBatch](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") where the corresponding elements of `predicate` are true.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/filter/fn.prep_null_mask_filter.html" class="fn" title="fn arrow::compute::kernels::filter::prep_null_mask_filter">prep_null_mask_filter</a>  
Remove null values by do a bitmask AND operation with null bits and the boolean bits.
