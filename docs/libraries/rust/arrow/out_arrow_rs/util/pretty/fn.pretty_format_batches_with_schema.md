# Function pretty_format_batches_with_schema Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/pretty.rs.html#92-95" class="src">Source</a>

``` rust
pub fn pretty_format_batches_with_schema(
    schema: Arc<Schema>,
    results: &[RecordBatch],
) -> Result<impl Display, ArrowError>
```

Available on **crate feature `prettyprint`** only.

Expand description

Create a visual representation of [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es with a provided schema.

Useful to display empty batches.

## <a href="https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_batches_with_schema.html#example" class="doc-anchor">§</a>Example

``` rust
let schema = Arc::new(Schema::new(vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Utf8, true),
]));
// Note, returned object implements `Display`
let pretty_table = pretty_format_batches_with_schema(schema, &[]).unwrap();
let table_str = format!("Batches:\n{pretty_table}");
assert_eq!(table_str,
r#"Batches:
+---+---+
| a | b |
+---+---+
+---+---+"#);
```
