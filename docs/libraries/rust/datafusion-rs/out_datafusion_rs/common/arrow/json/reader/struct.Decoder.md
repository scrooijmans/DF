# Struct Decoder Copy item path

<a href="https://docs.rs/arrow-json/56.0.0/x86_64-unknown-linux-gnu/src/arrow_json/reader/mod.rs.html#412" class="src">Source</a>

``` rust
pub struct Decoder { /* private fields */ }
```

Expand description

A low-level interface for reading JSON data from a byte stream

See [`Reader`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Reader.html "struct datafusion::common::arrow::json::Reader") for a higher-level interface for interface with [`BufRead`](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html "trait std::io::BufRead")

The push-based interface facilitates integration with sources that yield arbitrarily delimited bytes ranges, such as [`BufRead`](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html "trait std::io::BufRead"), or a chunked byte stream received from object storage

``` rust
fn read_from_json<R: BufRead>(
    mut reader: R,
    schema: SchemaRef,
) -> Result<impl Iterator<Item = Result<RecordBatch, ArrowError>>, ArrowError> {
    let mut decoder = ReaderBuilder::new(schema).build_decoder()?;
    let mut next = move || {
        loop {
            // Decoder is agnostic that buf doesn't contain whole records
            let buf = reader.fill_buf()?;
            if buf.is_empty() {
                break; // Input exhausted
            }
            let read = buf.len();
            let decoded = decoder.decode(buf)?;

            // Consume the number of bytes read
            reader.consume(decoded);
            if decoded != read {
                break; // Read batch size
            }
        }
        decoder.flush()
    };
    Ok(std::iter::from_fn(move || next().transpose()))
}
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#impl-Decoder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html" class="struct" title="struct datafusion::common::arrow::json::reader::Decoder">Decoder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.decode" class="fn">decode</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Read JSON objects from `buf`, returning the number of bytes read

This method returns once `batch_size` objects have been parsed since the last call to [`Self::flush`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.flush "method datafusion::common::arrow::json::reader::Decoder::flush"), or `buf` is exhausted. Any remaining bytes should be included in the next call to [`Self::decode`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.decode "method datafusion::common::arrow::json::reader::Decoder::decode")

There is no requirement that `buf` contains a whole number of records, facilitating integration with arbitrary byte streams, such as those yielded by [`BufRead`](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html "trait std::io::BufRead")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.serialize" class="fn">serialize</a>\<S\>(&mut self, rows: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[S]</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a>,

Serialize `rows` to this [`Decoder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html "struct datafusion::common::arrow::json::reader::Decoder")

This provides a simple way to convert [serde](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/index.html "mod serde")-compatible datastructures into arrow [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch").

Custom conversion logic as described in [arrow_array::builder](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html "mod datafusion::common::arrow::array::builder") will likely outperform this, especially where the schema is known at compile-time, however, this provides a mechanism to get something up and running quickly

It can be used with [`serde_json::Value`](https://docs.rs/serde_json/1.0.143/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html "enum serde_json::value::Value")

``` rust
let json = vec![json!({"float": 2.3}), json!({"float": 5.7})];

let schema = Schema::new(vec![Field::new("float", DataType::Float32, true)]);
let mut decoder = ReaderBuilder::new(Arc::new(schema)).build_decoder().unwrap();

decoder.serialize(&json).unwrap();
let batch = decoder.flush().unwrap().unwrap();
assert_eq!(batch.num_rows(), 2);
assert_eq!(batch.num_columns(), 1);
let values = batch.column(0).as_primitive::<Float32Type>().values();
assert_eq!(values, &[2.3, 5.7])
```

Or with arbitrary [`Serialize`](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html "trait serde::ser::Serialize") types

``` rust
#[derive(Serialize)]
struct MyStruct {
    int32: i32,
    float: f32,
}

let schema = Schema::new(vec![
    Field::new("int32", DataType::Int32, false),
    Field::new("float", DataType::Float32, false),
]);

let rows = vec![
    MyStruct{ int32: 0, float: 3. },
    MyStruct{ int32: 4, float: 67.53 },
];

let mut decoder = ReaderBuilder::new(Arc::new(schema)).build_decoder().unwrap();
decoder.serialize(&rows).unwrap();

let batch = decoder.flush().unwrap().unwrap();

// Expect batch containing two columns
let int32 = batch.column(0).as_primitive::<Int32Type>();
assert_eq!(int32.values(), &[0, 4]);

let float = batch.column(1).as_primitive::<Float32Type>();
assert_eq!(float.values(), &[3., 67.53]);
```

Or even complex nested types

``` rust
#[derive(Serialize)]
struct MyStruct {
    int32: i32,
    list: Vec<f64>,
    nested: Vec<Option<Nested>>,
}

impl MyStruct {
    /// Returns the [`Fields`] for [`MyStruct`]
    fn fields() -> Fields {
        let nested = DataType::Struct(Nested::fields());
        Fields::from([
            Arc::new(Field::new("int32", DataType::Int32, false)),
            Arc::new(Field::new_list(
                "list",
                Field::new("element", DataType::Float64, false),
                false,
            )),
            Arc::new(Field::new_list(
                "nested",
                Field::new("element", nested, true),
                true,
            )),
        ])
    }
}

#[derive(Serialize)]
struct Nested {
    map: BTreeMap<String, Vec<String>>
}

impl Nested {
    /// Returns the [`Fields`] for [`Nested`]
    fn fields() -> Fields {
        let element = Field::new("element", DataType::Utf8, false);
        Fields::from([
            Arc::new(Field::new_map(
                "map",
                "entries",
                Field::new("key", DataType::Utf8, false),
                Field::new_list("value", element, false),
                false, // sorted
                false, // nullable
            ))
        ])
    }
}

let data = vec![
    MyStruct {
        int32: 34,
        list: vec![1., 2., 34.],
        nested: vec![
            None,
            Some(Nested {
                map: vec![
                    ("key1".to_string(), vec!["foo".to_string(), "bar".to_string()]),
                    ("key2".to_string(), vec!["baz".to_string()])
                ].into_iter().collect()
            })
        ]
    },
    MyStruct {
        int32: 56,
        list: vec![],
        nested: vec![]
    },
    MyStruct {
        int32: 24,
        list: vec![-1., 245.],
        nested: vec![None]
    }
];

let schema = Schema::new(MyStruct::fields());
let mut decoder = ReaderBuilder::new(Arc::new(schema)).build_decoder().unwrap();
decoder.serialize(&data).unwrap();
let batch = decoder.flush().unwrap().unwrap();
assert_eq!(batch.num_rows(), 3);
assert_eq!(batch.num_columns(), 3);

// Convert to StructArray to format
let s = StructArray::from(batch);
let options = FormatOptions::default().with_null("null");
let formatter = ArrayFormatter::try_new(&s, &options).unwrap();

assert_eq!(&formatter.value(0).to_string(), "{int32: 34, list: [1.0, 2.0, 34.0], nested: [null, {map: {key1: [foo, bar], key2: [baz]}}]}");
assert_eq!(&formatter.value(1).to_string(), "{int32: 56, list: [], nested: []}");
assert_eq!(&formatter.value(2).to_string(), "{int32: 24, list: [-1.0, 245.0], nested: [null]}");
```

Note: this ignores any batch size setting, and always decodes all rows

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.has_partial_record" class="fn">has_partial_record</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

True if the decoder is currently part way through decoding a record.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The number of unflushed records, including the partially decoded record (if any).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

True if there are no records to flush, i.e. [`Self::len`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.len "method datafusion::common::arrow::json::reader::Decoder::len") is zero.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.flush" class="fn">flush</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Flushes the currently buffered data to a [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

Returns `Ok(None)` if no buffered data, i.e. [`Self::is_empty`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.is_empty "method datafusion::common::arrow::json::reader::Decoder::is_empty") is true.

Note: This will return an error if called part way through decoding a record, i.e. [`Self::has_partial_record`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.has_partial_record "method datafusion::common::arrow::json::reader::Decoder::has_partial_record") is true.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#impl-Debug-for-Decoder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html" class="struct" title="struct datafusion::common::arrow::json::reader::Decoder">Decoder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html#blanket-implementations" class="anchor">§</a>
