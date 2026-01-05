# Function infer_field_schema Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/schema_inference.rs.html#106" class="src">Source</a>

``` rust
pub fn infer_field_schema(
    string: &str,
    try_parse_dates: bool,
    decimal_comma: bool,
) -> DataType
```

Available on **crate feature `polars-io`** only.

Expand description

Infer the data type of a record
