# Module string_writer Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/string_writer.rs.html#18-105" class="src">Source</a>

Expand description

String Writer This string writer encapsulates `std::string::String` and implements `std::io::Write` trait, which makes String as a writable object like File.

Example:

``` rust
#[cfg(feature = "csv")]
{
use arrow::array::*;
use arrow::csv;
use arrow::datatypes::*;
use arrow::record_batch::RecordBatch;
use arrow::util::string_writer::StringWriter;
use std::sync::Arc;

let schema = Schema::new(vec![
    Field::new("c1", DataType::Utf8, false),
    Field::new("c2", DataType::Float64, true),
    Field::new("c3", DataType::UInt32, false),
    Field::new("c3", DataType::Boolean, true),
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

let sw = StringWriter::new();
let mut writer = csv::Writer::new(sw);
writer.write(&batch).unwrap();
}
```

## Structs<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html" class="struct" title="struct arrow::util::string_writer::StringWriter">StringWriter</a>  
A writer that allows writing to a `String` like an `std::io::Write` object.
