# Function interleave_record_batch Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/interleave.rs.html#385-388" class="src">Source</a>

``` rust
pub fn interleave_record_batch(
    record_batches: &[&RecordBatch],
    indices: &[(usize, usize)],
) -> Result<RecordBatch, ArrowError>
```

Expand description

Interleave rows by index from multiple [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") instances and return a new [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch").

This function will call [`interleave`](https://docs.rs/arrow/latest/arrow/compute/fn.interleave.html "fn arrow::compute::interleave") on each array of the [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") instances and assemble a new [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch").

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/interleave/fn.interleave_record_batch.html#example" class="doc-anchor">§</a>Example

``` rust

let schema = Arc::new(Schema::new(vec![
    Field::new("a", DataType::Int32, true),
    Field::new("b", DataType::Utf8, true),
]));

let batch1 = RecordBatch::try_new(
    schema.clone(),
    vec![
        Arc::new(Int32Array::from(vec![0, 1, 2])),
        Arc::new(StringArray::from(vec!["a", "b", "c"])),
    ],
).unwrap();

let batch2 = RecordBatch::try_new(
    schema.clone(),
    vec![
        Arc::new(Int32Array::from(vec![3, 4, 5])),
        Arc::new(StringArray::from(vec!["d", "e", "f"])),
    ],
).unwrap();

let indices = vec![(0, 1), (1, 2), (0, 0), (1, 1)];
let interleaved = interleave_record_batch(&[&batch1, &batch2], &indices).unwrap();

let expected = RecordBatch::try_new(
    schema,
    vec![
        Arc::new(Int32Array::from(vec![1, 5, 0, 4])),
        Arc::new(StringArray::from(vec!["b", "f", "a", "e"])),
    ],
).unwrap();
assert_eq!(interleaved, expected);
```
