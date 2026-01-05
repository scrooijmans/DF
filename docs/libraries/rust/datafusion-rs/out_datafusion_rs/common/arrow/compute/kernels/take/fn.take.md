# Function take Copy item path

<a href="https://docs.rs/arrow-select/56.0.0/x86_64-unknown-linux-gnu/src/arrow_select/take.rs.html#86-90" class="src">Source</a>

``` rust
pub fn take(
    values: &dyn Array,
    indices: &dyn Array,
    options: Option<TakeOptions>,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Take elements by index from [Array](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array"), creating a new [Array](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") from those indexes.

``` text
┌─────────────────┐      ┌─────────┐                              ┌─────────────────┐
│        A        │      │    0    │                              │        A        │
├─────────────────┤      ├─────────┤                              ├─────────────────┤
│        D        │      │    2    │                              │        B        │
├─────────────────┤      ├─────────┤   take(values, indices)      ├─────────────────┤
│        B        │      │    3    │ ─────────────────────────▶   │        C        │
├─────────────────┤      ├─────────┤                              ├─────────────────┤
│        C        │      │    1    │                              │        D        │
├─────────────────┤      └─────────┘                              └─────────────────┘
│        E        │
└─────────────────┘
   values array          indices array                              result
```

For selecting values by index from multiple arrays see [`crate::interleave`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/interleave/index.html "mod datafusion::common::arrow::compute::kernels::interleave")

Note that this kernel, similar to other kernels in this crate, will avoid allocating where not necessary. Consequently the returned array may share buffers with the inputs

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/take/fn.take.html#errors" class="doc-anchor">§</a>Errors

This function errors whenever:

- An index cannot be casted to `usize` (typically 32 bit architectures)
- An index is out of bounds and `options` is set to check bounds.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/take/fn.take.html#safety" class="doc-anchor">§</a>Safety

When `options` is not set to check bounds, taking indexes after `len` will panic.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/take/fn.take.html#see-also" class="doc-anchor">§</a>See also

- [`BatchCoalescer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.BatchCoalescer.html "struct datafusion::common::arrow::compute::BatchCoalescer"): to filter multiple [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") and coalesce the results into a single array.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/take/fn.take.html#examples" class="doc-anchor">§</a>Examples

``` rust
let values = StringArray::from(vec!["zero", "one", "two"]);

// Take items at index 2, and 1:
let indices = UInt32Array::from(vec![2, 1]);
let taken = take(&values, &indices, None).unwrap();
let taken = taken.as_string::<i32>();

assert_eq!(*taken, StringArray::from(vec!["two", "one"]));
```
