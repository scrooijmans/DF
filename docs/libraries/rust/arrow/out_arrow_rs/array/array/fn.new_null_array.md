# Function new_null_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/mod.rs.html#874" class="src">Source</a>

``` rust
pub fn new_null_array(data_type: &DataType, length: usize) -> Arc<dyn Array>
```

Expand description

Creates a new array of `data_type` of length `length` filled entirely of `NULL` values

``` rust
use std::sync::Arc;
use arrow_schema::DataType;
use arrow_array::{ArrayRef, Int32Array, new_null_array};

let null_array = new_null_array(&DataType::Int32, 3);
let array: ArrayRef = Arc::new(Int32Array::from(vec![None, None, None]));

assert_eq!(&array, &null_array);
```
