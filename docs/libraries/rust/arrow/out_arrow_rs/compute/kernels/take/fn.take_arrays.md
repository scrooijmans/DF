# Function take_arrays Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/take.rs.html#152-156" class="src">Source</a>

``` rust
pub fn take_arrays(
    arrays: &[Arc<dyn Array>],
    indices: &dyn Array,
    options: Option<TakeOptions>,
) -> Result<Vec<Arc<dyn Array>>, ArrowError>
```

Expand description

For each [ArrayRef](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") in the [`Vec<ArrayRef>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec"), take elements by index and create a new [`Vec<ArrayRef>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") from those indices.

``` text
┌────────┬────────┐
│        │        │           ┌────────┐                                ┌────────┬────────┐
│   A    │   1    │           │        │                                │        │        │
├────────┼────────┤           │   0    │                                │   A    │   1    │
│        │        │           ├────────┤                                ├────────┼────────┤
│   D    │   4    │           │        │                                │        │        │
├────────┼────────┤           │   2    │  take_arrays(values,indices)   │   B    │   2    │
│        │        │           ├────────┤                                ├────────┼────────┤
│   B    │   2    │           │        │  ───────────────────────────►  │        │        │
├────────┼────────┤           │   3    │                                │   C    │   3    │
│        │        │           ├────────┤                                ├────────┼────────┤
│   C    │   3    │           │        │                                │        │        │
├────────┼────────┤           │   1    │                                │   D    │   4    │
│        │        │           └────────┘                                └────────┼────────┘
│   E    │   5    │
└────────┴────────┘
   values arrays             indices array                                      result
```

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/fn.take_arrays.html#errors" class="doc-anchor">§</a>Errors

This function errors whenever:

- An index cannot be casted to `usize` (typically 32 bit architectures)
- An index is out of bounds and `options` is set to check bounds.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/fn.take_arrays.html#safety" class="doc-anchor">§</a>Safety

When `options` is not set to check bounds, taking indexes after `len` will panic.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/fn.take_arrays.html#examples" class="doc-anchor">§</a>Examples

``` rust
let string_values = Arc::new(StringArray::from(vec!["zero", "one", "two"]));
let values = Arc::new(UInt32Array::from(vec![0, 1, 2]));

// Take items at index 2, and 1:
let indices = UInt32Array::from(vec![2, 1]);
let taken_arrays = take_arrays(&[string_values, values], &indices, None).unwrap();
let taken_string = taken_arrays[0].as_string::<i32>();
assert_eq!(*taken_string, StringArray::from(vec!["two", "one"]));
let taken_values = taken_arrays[1].as_primitive();
assert_eq!(*taken_values, UInt32Array::from(vec![2, 1]));
```
