# Module reader Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#18-2810" class="src">Source</a>

Expand description

CSV Reader

## <a href="https://arrow.apache.org/rust/arrow_csv/reader/index.html#basic-usage" class="doc-anchor">Â§</a>Basic Usage

This CSV reader allows CSV files to be read into the Arrow memory model. Records are loaded in batches and are then converted from row-based data to columnar data.

Example:

``` rust

let schema = Schema::new(vec![
    Field::new("city", DataType::Utf8, false),
    Field::new("lat", DataType::Float64, false),
    Field::new("lng", DataType::Float64, false),
]);

let file = File::open("test/data/uk_cities.csv").unwrap();

let mut csv = ReaderBuilder::new(Arc::new(schema)).build(file).unwrap();
let batch = csv.next().unwrap().unwrap();
```

## <a href="https://arrow.apache.org/rust/arrow_csv/reader/index.html#async-usage" class="doc-anchor">Â§</a>Async Usage

The lower-level [`Decoder`](https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html "struct arrow_csv::reader::Decoder") can be integrated with various forms of async data streams, and is designed to be agnostic to the various different kinds of async IO primitives found within the Rust ecosystem.

For example, see below for how it can be used with an arbitrary `Stream` of `Bytes`

``` rust
fn decode_stream<S: Stream<Item = Bytes> + Unpin>(
    mut decoder: Decoder,
    mut input: S,
) -> impl Stream<Item = Result<RecordBatch, ArrowError>> {
    let mut buffered = Bytes::new();
    futures::stream::poll_fn(move |cx| {
        loop {
            if buffered.is_empty() {
                if let Some(b) = ready!(input.poll_next_unpin(cx)) {
                    buffered = b;
                }
                // Note: don't break on `None` as the decoder needs
                // to be called with an empty array to delimit the
                // final record
            }
            let decoded = match decoder.decode(buffered.as_ref()) {
                Ok(0) => break,
                Ok(decoded) => decoded,
                Err(e) => return Poll::Ready(Some(Err(e))),
            };
            buffered.advance(decoded);
        }

        Poll::Ready(decoder.flush().transpose())
    })
}
```

In a similar vein, it can also be used with tokio-based IO primitives

``` rust
fn decode_stream<R: AsyncBufRead + Unpin>(
    mut decoder: Decoder,
    mut reader: R,
) -> impl Stream<Item = Result<RecordBatch, ArrowError>> {
    futures::stream::poll_fn(move |cx| {
        loop {
            let b = match ready!(Pin::new(&mut reader).poll_fill_buf(cx)) {
                Ok(b) => b,
                Err(e) => return Poll::Ready(Some(Err(e.into()))),
            };
            let decoded = match decoder.decode(b) {
                // Note: the decoder needs to be called with an empty
                // array to delimit the final record
                Ok(0) => break,
                Ok(decoded) => decoded,
                Err(e) => return Poll::Ready(Some(Err(e))),
            };
            Pin::new(&mut reader).consume(decoded);
        }

        Poll::Ready(decoder.flush().transpose())
    })
}
```

## Modules<a href="https://arrow.apache.org/rust/arrow_csv/reader/index.html#modules" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/index.html" class="mod" title="mod arrow_csv::reader::records">records</a> ðŸ”’

## Structs<a href="https://arrow.apache.org/rust/arrow_csv/reader/index.html#structs" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.BufReader.html" class="struct" title="struct arrow_csv::reader::BufReader">BufReader</a>  
CSV file reader

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html" class="struct" title="struct arrow_csv::reader::Decoder">Decoder</a>  
A push-based interface for decoding CSV data from an arbitrary byte stream

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html" class="struct" title="struct arrow_csv::reader::Format">Format</a>  
The format specification for the CSV file

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html" class="struct" title="struct arrow_csv::reader::InferredDataType">InferredDataType</a> ðŸ”’  
<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html" class="struct" title="struct arrow_csv::reader::NullRegex">NullRegex</a> ðŸ”’  
A wrapper over `Option<Regex>` to check if the value is `NULL`.

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html" class="struct" title="struct arrow_csv::reader::ReaderBuilder">ReaderBuilder</a>  
CSV file reader builder

## Statics<a href="https://arrow.apache.org/rust/arrow_csv/reader/index.html#statics" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/static.REGEX_SET.html" class="static" title="static arrow_csv::reader::REGEX_SET">REGEX_SET</a> ðŸ”’  
Order should match [`InferredDataType`](https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html "struct arrow_csv::reader::InferredDataType")

## Functions<a href="https://arrow.apache.org/rust/arrow_csv/reader/index.html#functions" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/fn.build_boolean_array.html" class="fn" title="fn arrow_csv::reader::build_boolean_array">build_boolean_array</a> ðŸ”’

<a href="https://arrow.apache.org/rust/arrow_csv/reader/fn.build_decimal_array.html" class="fn" title="fn arrow_csv::reader::build_decimal_array">build_decimal_array</a> ðŸ”’

<a href="https://arrow.apache.org/rust/arrow_csv/reader/fn.build_primitive_array.html" class="fn" title="fn arrow_csv::reader::build_primitive_array">build_primitive_array</a> ðŸ”’

<a href="https://arrow.apache.org/rust/arrow_csv/reader/fn.build_timestamp_array.html" class="fn" title="fn arrow_csv::reader::build_timestamp_array">build_timestamp_array</a> ðŸ”’

<a href="https://arrow.apache.org/rust/arrow_csv/reader/fn.build_timestamp_array_impl.html" class="fn" title="fn arrow_csv::reader::build_timestamp_array_impl">build_timestamp_array_impl</a> ðŸ”’

<a href="https://arrow.apache.org/rust/arrow_csv/reader/fn.infer_schema_from_files.html" class="fn" title="fn arrow_csv::reader::infer_schema_from_files">infer_schema_from_files</a>

Infer schema from a list of CSV files by reading through first n records with `max_read_records` controlling the maximum number of records to read.

<a href="https://arrow.apache.org/rust/arrow_csv/reader/fn.parse.html" class="fn" title="fn arrow_csv::reader::parse">parse</a> ðŸ”’

Parses a slice of [`StringRecords`](https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html "struct arrow_csv::reader::records::StringRecords") into a \[RecordBatch\]

<a href="https://arrow.apache.org/rust/arrow_csv/reader/fn.parse_bool.html" class="fn" title="fn arrow_csv::reader::parse_bool">parse_bool</a> ðŸ”’

## Type Aliases<a href="https://arrow.apache.org/rust/arrow_csv/reader/index.html#types" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Bounds.html" class="type" title="type arrow_csv::reader::Bounds">Bounds</a> ðŸ”’  
<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Reader.html" class="type" title="type arrow_csv::reader::Reader">Reader</a>  
CSV file reader using [`std::io::BufReader`](https://doc.rust-lang.org/nightly/std/io/buffered/bufreader/struct.BufReader.html "struct std::io::buffered::bufreader::BufReader")
