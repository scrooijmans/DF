# Function sort Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/sort.rs.html#57" class="src">Source</a>

``` rust
pub fn sort(
    values: &dyn Array,
    options: Option<SortOptions>,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Sort the `ArrayRef` using `SortOptions`.

Performs a sort on values and indices. Nulls are ordered according to the `nulls_first` flag in `options`. Floats are sorted using IEEE 754 totalOrder

Returns an `ArrowError::ComputeError(String)` if the array type is either unsupported by `sort_to_indices` or `take`.

Note: this is an unstable_sort, meaning it may not preserve the order of equal elements.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/fn.sort.html#example" class="doc-anchor">§</a>Example

``` rust
let array = Int32Array::from(vec![5, 4, 3, 2, 1]);
let sorted_array = sort(&array, None).unwrap();
assert_eq!(sorted_array.as_ref(), &Int32Array::from(vec![1, 2, 3, 4, 5]));
```
