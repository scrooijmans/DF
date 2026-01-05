# Module writer Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/writer.rs.html#18-863" class="src">Source</a>

Expand description

CSV Writer

This CSV writer allows Arrow data (in record batches) to be written as CSV files. The writer does not support writing `ListArray` and `StructArray`.

Example:

``` rust

let schema = Schema::new(vec![
    Field::new("c1", DataType::Utf8, false),
    Field::new("c2", DataType::Float64, true),
    Field::new("c3", DataType::UInt32, false),
    Field::new("c4", DataType::Boolean, true),
]);
let c1 = StringArray::from(vec![
    "Lorem ipsum dolor sit amet",
    "consectetur adipiscing elit",
    "sed do eiusmod tempor",
]);
let c2 = PrimitiveArray::<Float64Type>::from(vec![
    Some(123.564532),
    None,
    Some(-556132.25),
]);
let c3 = PrimitiveArray::<UInt32Type>::from(vec![3, 2, 1]);
let c4 = BooleanArray::from(vec![Some(true), Some(false), None]);

let batch = RecordBatch::try_new(
    Arc::new(schema),
    vec![Arc::new(c1), Arc::new(c2), Arc::new(c3), Arc::new(c4)],
)
.unwrap();

let mut output = Vec::with_capacity(1024);

let mut writer = Writer::new(&mut output);
let batches = vec![&batch, &batch];
for batch in batches {
    writer.write(batch).unwrap();
}
```

## Structs<a href="https://arrow.apache.org/rust/arrow_csv/writer/index.html#structs" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html" class="struct" title="struct arrow_csv::writer::Writer">Writer</a>  
A CSV writer

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html" class="struct" title="struct arrow_csv::writer::WriterBuilder">WriterBuilder</a>  
A CSV writer builder

## Constants<a href="https://arrow.apache.org/rust/arrow_csv/writer/index.html#constants" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/constant.DEFAULT_NULL_VALUE.html" class="constant" title="constant arrow_csv::writer::DEFAULT_NULL_VALUE">DEFAULT_NULL_VALUE</a> ðŸ”’
