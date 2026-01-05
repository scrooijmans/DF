# Function sort_limit Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/sort.rs.html#156-160" class="src">Source</a>

``` rust
pub fn sort_limit(
    values: &dyn Array,
    options: Option<SortOptions>,
    limit: Option<usize>,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Sort the `ArrayRef` partially.

If `limit` is specified, the resulting array will contain only first `limit` in the sort order. Any data data after the limit will be discarded.

Note: this is an unstable_sort, meaning it may not preserve the order of equal elements.

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.sort_limit.html#example" class="doc-anchor">§</a>Example

``` rust
let array = Int32Array::from(vec![5, 4, 3, 2, 1]);

// Find the the top 2 items
let sorted_array = sort_limit(&array, None, Some(2)).unwrap();
assert_eq!(sorted_array.as_ref(), &Int32Array::from(vec![1, 2]));

// Find the bottom top 2 items
let options = Some(SortOptions {
                 descending: true,
                 ..Default::default()
              });
let sorted_array = sort_limit(&array, options, Some(2)).unwrap();
assert_eq!(sorted_array.as_ref(), &Int32Array::from(vec![5, 4]));
```
