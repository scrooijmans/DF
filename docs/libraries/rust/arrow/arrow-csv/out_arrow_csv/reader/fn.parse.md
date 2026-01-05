# Function parse Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#637-890" class="src">Source</a>

``` rust
fn parse(
    rows: &StringRecords<'_>,
    fields: &Fields,
    metadata: Option<HashMap<String, String>>,
    projection: Option<&Vec<usize>>,
    line_number: usize,
    null_regex: &NullRegex,
) -> Result<RecordBatch, ArrowError>
```

Expand description

Parses a slice of [`StringRecords`](https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html "struct arrow_csv::reader::records::StringRecords") into a \[RecordBatch\]
