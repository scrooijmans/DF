# Function lexsort Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/sort.rs.html#892" class="src">Source</a>

``` rust
pub fn lexsort(
    columns: &[SortColumn],
    limit: Option<usize>,
) -> Result<Vec<Arc<dyn Array>>, ArrowError>
```

Expand description

Sort a list of `ArrayRef` using `SortOptions` provided for each array.

Performs an unstable lexicographical sort on values and indices.

Returns an `ArrowError::ComputeError(String)` if any of the array type is either unsupported by `lexsort_to_indices` or `take`.

Example:

``` rust

let sorted_columns = lexsort(&vec![
    SortColumn {
        values: Arc::new(PrimitiveArray::<Int64Type>::from(vec![
            None,
            Some(-2),
            Some(89),
            Some(-64),
            Some(101),
        ])) as ArrayRef,
        options: None,
    },
    SortColumn {
        values: Arc::new(StringArray::from(vec![
            Some("hello"),
            Some("world"),
            Some(","),
            Some("foobar"),
            Some("!"),
        ])) as ArrayRef,
        options: Some(SortOptions {
            descending: true,
            nulls_first: false,
        }),
    },
], None).unwrap();

assert_eq!(sorted_columns[0].as_primitive::<Int64Type>().value(1), -64);
assert!(sorted_columns[0].is_null(0));
```

Note: for multi-column sorts without a limit, using the [row format](https://docs.rs/arrow-row/latest/arrow_row/) may be significantly faster
