# Function take_record_batch Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/take.rs.html#964-967" class="src">Source</a>

``` rust
pub fn take_record_batch(
    record_batch: &RecordBatch,
    indices: &dyn Array,
) -> Result<RecordBatch, ArrowError>
```

Expand description

Take rows by index from [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") and returns a new [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") from those indexes.

This function will call [`take`](https://docs.rs/arrow/latest/arrow/compute/fn.take.html "fn arrow::compute::take") on each array of the [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") and assemble a new [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch").

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/fn.take_record_batch.html#example" class="doc-anchor">§</a>Example

``` rust

let schema = Arc::new(Schema::new(vec![
    Field::new("a", DataType::Int32, true),
    Field::new("b", DataType::Utf8, true),
]));
let batch = RecordBatch::try_new(
    schema.clone(),
    vec![
        Arc::new(Int32Array::from_iter_values(0..20)),
        Arc::new(StringArray::from_iter_values(
            (0..20).map(|i| format!("str-{}", i)),
        )),
    ],
)
.unwrap();

let indices = UInt32Array::from(vec![1, 5, 10]);
let taken = take_record_batch(&batch, &indices).unwrap();

let expected = RecordBatch::try_new(
    schema,
    vec![
        Arc::new(Int32Array::from(vec![1, 5, 10])),
        Arc::new(StringArray::from(vec!["str-1", "str-5", "str-10"])),
    ],
)
.unwrap();
assert_eq!(taken, expected);
```
