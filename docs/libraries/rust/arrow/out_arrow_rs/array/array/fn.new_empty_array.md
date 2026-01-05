# Function new_empty_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/mod.rs.html#856" class="src">Source</a>

``` rust
pub fn new_empty_array(data_type: &DataType) -> Arc<dyn Array>
```

Expand description

Creates a new empty array

``` rust
use std::sync::Arc;
use arrow_schema::DataType;
use arrow_array::{ArrayRef, Int32Array, new_empty_array};

let empty_array = new_empty_array(&DataType::Int32);
let array: ArrayRef = Arc::new(Int32Array::from(vec![] as Vec<i32>));

assert_eq!(&array, &empty_array);
```
