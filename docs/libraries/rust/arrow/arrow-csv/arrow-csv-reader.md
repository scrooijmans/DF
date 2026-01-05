Description: CSV Reader

Title: arrow\_csv::reader - Rust

Module reader
-------------

arrow\_csv

Module reader Copy item path
============================

Source

Expand description

CSV Reader

§Basic Usage
------------

This CSV reader allows CSV files to be read into the Arrow memory model. Records are loaded in batches and are then converted from row-based data to columnar data.

Example:

```

let schema = Schema::new(vec![
Field::new("city", DataType::Utf8, false),
Field::new("lat", DataType::Float64, false),
Field::new("lng", DataType::Float64, false),
]);

let file = File::open("test/data/uk_cities.csv").unwrap();

let mut csv = ReaderBuilder::new(Arc::new(schema)).build(file).unwrap();
let batch = csv.next().unwrap().unwrap();
```

§Async Usage
------------

The lower-level `Decoder` can be integrated with various forms of async data streams, and is designed to be agnostic to the various different kinds of async IO primitives found within the Rust ecosystem.

For example, see below for how it can be used with an arbitrary `Stream` of `Bytes`

```
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

```
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

Modules§
--------

records 🔒

Structs§
--------

BufReader

CSV file reader

Decoder

A push-based interface for decoding CSV data from an arbitrary byte stream

Format

The format specification for the CSV file

InferredDataType 🔒

NullRegex 🔒

A wrapper over `Option<Regex>` to check if the value is `NULL`.

ReaderBuilder

CSV file reader builder

Statics§
--------

REGEX\_SET 🔒

Order should match `InferredDataType`

Functions§
----------

build\_boolean\_array 🔒

build\_decimal\_array 🔒

build\_primitive\_array 🔒

build\_timestamp\_array 🔒

build\_timestamp\_array\_impl 🔒

infer\_schema\_from\_files

Infer schema from a list of CSV files by reading through first n records with `max_read_records` controlling the maximum number of records to read.

parse 🔒

Parses a slice of `StringRecords` into a \[RecordBatch\]

parse\_bool 🔒

Type Aliases§
-------------

Bounds 🔒

Reader

CSV file reader using `std::io::BufReader`