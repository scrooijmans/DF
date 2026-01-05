# Function filter Copy item path

<a href="https://docs.rs/arrow-select/56.0.0/x86_64-unknown-linux-gnu/src/arrow_select/filter.rs.html#143" class="src">Source</a>

``` rust
pub fn filter(
    values: &dyn Array,
    predicate: &BooleanArray,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Returns a filtered `values` [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") where the corresponding elements of `predicate` are `true`.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.filter.html#see-also" class="doc-anchor">§</a>See also

- [`FilterBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.FilterBuilder.html "struct datafusion::common::arrow::compute::FilterBuilder") for more control over the filtering process.
- [`filter_record_batch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.filter_record_batch.html "fn datafusion::common::arrow::compute::filter_record_batch") to filter a [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")
- [`BatchCoalescer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.BatchCoalescer.html "struct datafusion::common::arrow::compute::BatchCoalescer"): to filter multiple [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") and coalesce the results into a single array.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.filter.html#example" class="doc-anchor">§</a>Example

``` rust
let array = Int32Array::from(vec![5, 6, 7, 8, 9]);
let filter_array = BooleanArray::from(vec![true, false, false, true, false]);
let c = filter(&array, &filter_array).unwrap();
let c = c.as_any().downcast_ref::<Int32Array>().unwrap();
assert_eq!(c, &Int32Array::from(vec![5, 8]));
```
