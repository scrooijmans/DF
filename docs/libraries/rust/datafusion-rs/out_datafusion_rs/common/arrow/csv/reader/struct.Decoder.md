# Struct Decoder Copy item path

<a href="https://docs.rs/arrow-csv/56.0.0/x86_64-unknown-linux-gnu/src/arrow_csv/reader/mod.rs.html#555" class="src">Source</a>

``` rust
pub struct Decoder { /* private fields */ }
```

Expand description

A push-based interface for decoding CSV data from an arbitrary byte stream

See [`Reader`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/type.Reader.html "type datafusion::common::arrow::csv::Reader") for a higher-level interface for interface with [`Read`](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read")

The push-based interface facilitates integration with sources that yield arbitrarily delimited bytes ranges, such as [`BufRead`](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html "trait std::io::BufRead"), or a chunked byte stream received from object storage

``` rust
fn read_from_csv<R: BufRead>(
    mut reader: R,
    schema: SchemaRef,
    batch_size: usize,
) -> Result<impl Iterator<Item = Result<RecordBatch, ArrowError>>, ArrowError> {
    let mut decoder = ReaderBuilder::new(schema)
        .with_batch_size(batch_size)
        .build_decoder();

    let mut next = move || {
        loop {
            let buf = reader.fill_buf()?;
            let decoded = decoder.decode(buf)?;
            if decoded == 0 {
                break;
            }

            // Consume the number of bytes read
            reader.consume(decoded);
        }
        decoder.flush()
    };
    Ok(std::iter::from_fn(move || next().transpose()))
}
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#impl-Decoder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html" class="struct" title="struct datafusion::common::arrow::csv::reader::Decoder">Decoder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#method.decode" class="fn">decode</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Decode records from `buf` returning the number of bytes read

This method returns once `batch_size` objects have been parsed since the last call to [`Self::flush`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#method.flush "method datafusion::common::arrow::csv::reader::Decoder::flush"), or `buf` is exhausted. Any remaining bytes should be included in the next call to [`Self::decode`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#method.decode "method datafusion::common::arrow::csv::reader::Decoder::decode")

There is no requirement that `buf` contains a whole number of records, facilitating integration with arbitrary byte streams, such as that yielded by [`BufRead`](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html "trait std::io::BufRead") or network sources such as object storage

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#method.flush" class="fn">flush</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Flushes the currently buffered data to a [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

This should only be called after [`Self::decode`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#method.decode "method datafusion::common::arrow::csv::reader::Decoder::decode") has returned `Ok(0)`, otherwise may return an error if part way through decoding a record

Returns `Ok(None)` if no buffered data

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#method.capacity" class="fn">capacity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of records that can be read before requiring a call to [`Self::flush`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#method.flush "method datafusion::common::arrow::csv::reader::Decoder::flush")

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#impl-Debug-for-Decoder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html" class="struct" title="struct datafusion::common::arrow::csv::reader::Decoder">Decoder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/struct.Decoder.html#blanket-implementations" class="anchor">§</a>
