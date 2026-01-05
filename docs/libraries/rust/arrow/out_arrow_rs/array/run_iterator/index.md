# Module run_iterator Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/lib.rs.html#257" class="src">Source</a>

Expand description

Idiomatic iterator for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray")

## Structs<a href="https://docs.rs/arrow/latest/arrow/array/run_iterator/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/run_iterator/struct.RunArrayIter.html" class="struct" title="struct arrow::array::run_iterator::RunArrayIter">RunArrayIter</a>  
The [`RunArrayIter`](https://docs.rs/arrow/latest/arrow/array/run_iterator/struct.RunArrayIter.html "struct arrow::array::run_iterator::RunArrayIter") provides an idiomatic way to iterate over the run array. It returns Some(T) if there is a value or None if the value is null.
