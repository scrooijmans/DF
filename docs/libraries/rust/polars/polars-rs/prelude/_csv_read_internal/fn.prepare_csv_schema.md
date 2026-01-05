# Function prepare_csv_schema Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/reader.rs.html#200-203" class="src">Source</a>

``` rust
pub fn prepare_csv_schema(
    schema: &mut Arc<Schema<DataType>>,
    fields_to_cast: &mut Vec<Field>,
) -> Result<(), PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Splits datatypes that cannot be natively read into a `fields_to_cast` for post-read casting.
