# Module concat Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/lib.rs.html#28" class="src">Source</a>

Expand description

Defines concat kernel for `ArrayRef`

Example:

``` rust
use arrow_array::{ArrayRef, StringArray};
use arrow_select::concat::concat;

let arr = concat(&[
    &StringArray::from(vec!["hello", "world"]),
    &StringArray::from(vec!["!"]),
]).unwrap();
assert_eq!(arr.len(), 3);
```

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/concat/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/concat/fn.concat.html" class="fn" title="fn arrow::compute::kernels::concat::concat">concat</a>  
Concatenate multiple [Array](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") of the same type into a single [ArrayRef](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef").

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/concat/fn.concat_batches.html" class="fn" title="fn arrow::compute::kernels::concat::concat_batches">concat_batches</a>  
Concatenates `batches` together into a single [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch").
