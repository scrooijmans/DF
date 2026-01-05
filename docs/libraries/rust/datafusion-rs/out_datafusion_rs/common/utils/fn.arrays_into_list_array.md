# Function arrays_into_list_array Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/utils/mod.rs.html#468-470" class="src">Source</a>

``` rust
pub fn arrays_into_list_array(
    arr: impl IntoIterator<Item = Arc<dyn Array>>,
) -> Result<GenericListArray<i32>, DataFusionError>
```

Expand description

Wrap arrays into a single element `ListArray`.

Example:

``` rust
use arrow::array::{Int32Array, ListArray, ArrayRef};
use arrow::datatypes::{Int32Type, Field};
use std::sync::Arc;

let arr1 = Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef;
let arr2 = Arc::new(Int32Array::from(vec![4, 5, 6])) as ArrayRef;

let list_arr = datafusion_common::utils::arrays_into_list_array([arr1, arr2]).unwrap();

let expected = ListArray::from_iter_primitive::<Int32Type, _, _>(
   vec![
    Some(vec![Some(1), Some(2), Some(3)]),
    Some(vec![Some(4), Some(5), Some(6)]),
   ]
);

assert_eq!(list_arr, expected);
```
