# Function make_comparator Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/ord.rs.html#368-372" class="src">Source</a>

``` rust
pub fn make_comparator(
    left: &dyn Array,
    right: &dyn Array,
    opts: SortOptions,
) -> Result<Box<dyn Fn(usize, usize) -> Ordering + Send + Sync>, ArrowError>
```

Expand description

Returns a comparison function that compares two values at two different positions between the two arrays.

For comparing arrays element-wise, see also the vectorised kernels in [`crate::cmp`](https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/index.html "mod arrow::compute::kernels::cmp").

If `nulls_first` is true `NULL` values will be considered less than any non-null value, otherwise they will be considered greater.

## <a href="https://docs.rs/arrow/latest/arrow/array/fn.make_comparator.html#basic-usage" class="doc-anchor">§</a>Basic Usage

``` rust
let array1 = Int32Array::from(vec![1, 2]);
let array2 = Int32Array::from(vec![3, 4]);

let cmp = make_comparator(&array1, &array2, SortOptions::default()).unwrap();
// 1 (index 0 of array1) is smaller than 4 (index 1 of array2)
assert_eq!(cmp(0, 1), Ordering::Less);

let array1 = Int32Array::from(vec![Some(1), None]);
let array2 = Int32Array::from(vec![None, Some(2)]);
let cmp = make_comparator(&array1, &array2, SortOptions::default()).unwrap();

assert_eq!(cmp(0, 1), Ordering::Less); // Some(1) vs Some(2)
assert_eq!(cmp(1, 1), Ordering::Less); // None vs Some(2)
assert_eq!(cmp(1, 0), Ordering::Equal); // None vs None
assert_eq!(cmp(0, 0), Ordering::Greater); // Some(1) vs None
```

## <a href="https://docs.rs/arrow/latest/arrow/array/fn.make_comparator.html#postgres-compatible-nested-comparison" class="doc-anchor">§</a>Postgres-compatible Nested Comparison

Whilst SQL prescribes ternary logic for nulls, that is comparing a value against a NULL yields a NULL, many systems, including postgres, instead apply a total ordering to comparison of nested nulls. That is nulls within nested types are either greater than any value (postgres), or less than any value (Spark).

In particular

<a href="https://docs.rs/arrow/latest/arrow/array/fn.make_comparator.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
{ a: 1, b: null } == { a: 1, b: null } => true
{ a: 1, b: null } == { a: 1, b: 1 } => false
{ a: 1, b: null } == null => null
null == null => null
```

This could be implemented as below

``` rust
fn eq(a: &dyn Array, b: &dyn Array) -> Result<BooleanArray, ArrowError> {
    if !a.data_type().is_nested() {
        return cmp::eq(&a, &b); // Use faster vectorised kernel
    }

    let cmp = make_comparator(a, b, SortOptions::default())?;
    let len = a.len().min(b.len());
    let values = (0..len).map(|i| cmp(i, i).is_eq()).collect();
    let nulls = NullBuffer::union(a.nulls(), b.nulls());
    Ok(BooleanArray::new(values, nulls))
}
```
