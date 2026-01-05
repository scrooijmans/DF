# Struct Decoder Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#555-579" class="src">Source</a>

``` rust
pub struct Decoder {
    schema: SchemaRef,
    projection: Option<Vec<usize>>,
    batch_size: usize,
    to_skip: usize,
    line_number: usize,
    end: usize,
    record_decoder: RecordDecoder,
    null_regex: NullRegex,
}
```

Expand description

A push-based interface for decoding CSV data from an arbitrary byte stream

See [`Reader`](https://arrow.apache.org/rust/arrow_csv/reader/type.Reader.html "type arrow_csv::reader::Reader") for a higher-level interface for interface with [`Read`](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read")

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

## Fields<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#fields" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#structfield.schema" class="anchor field">Â§</a>`schema: SchemaRef`

Explicit schema for the CSV file

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#structfield.projection" class="anchor field">Â§</a>`projection: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>>`

Optional projection for which columns to load (zero-based column indices)

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#structfield.batch_size" class="anchor field">Â§</a>`batch_size: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Number of records per batch

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#structfield.to_skip" class="anchor field">Â§</a>`to_skip: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Rows to skip

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#structfield.line_number" class="anchor field">Â§</a>`line_number: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Current line number

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#structfield.end" class="anchor field">Â§</a>`end: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

End line number

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#structfield.record_decoder" class="anchor field">Â§</a>`record_decoder: `<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html" class="struct" title="struct arrow_csv::reader::records::RecordDecoder"><code>RecordDecoder</code></a>

A decoder for [`StringRecords`](https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html "struct arrow_csv::reader::records::StringRecords")

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#structfield.null_regex" class="anchor field">Â§</a>`null_regex: `<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html" class="struct" title="struct arrow_csv::reader::NullRegex"><code>NullRegex</code></a>

Check if the string matches this pattern for `NULL`.

## Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#impl-Decoder" class="anchor">Â§</a>

### impl <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html" class="struct" title="struct arrow_csv::reader::Decoder">Decoder</a>

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#method.decode" class="fn">decode</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ArrowError\>

Decode records from `buf` returning the number of bytes read

This method returns once `batch_size` objects have been parsed since the last call to [`Self::flush`](https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#method.flush "method arrow_csv::reader::Decoder::flush"), or `buf` is exhausted. Any remaining bytes should be included in the next call to [`Self::decode`](https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#method.decode "method arrow_csv::reader::Decoder::decode")

There is no requirement that `buf` contains a whole number of records, facilitating integration with arbitrary byte streams, such as that yielded by [`BufRead`](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html "trait std::io::BufRead") or network sources such as object storage

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#method.flush" class="fn">flush</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<RecordBatch\>, ArrowError\>

Flushes the currently buffered data to a \[`RecordBatch`\]

This should only be called after [`Self::decode`](https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#method.decode "method arrow_csv::reader::Decoder::decode") has returned `Ok(0)`, otherwise may return an error if part way through decoding a record

Returns `Ok(None)` if no buffered data

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#method.capacity" class="fn">capacity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of records that can be read before requiring a call to [`Self::flush`](https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#method.flush "method arrow_csv::reader::Decoder::flush")

## Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#trait-implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#impl-Debug-for-Decoder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html" class="struct" title="struct arrow_csv::reader::Decoder">Decoder</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html#blanket-implementations" class="anchor">Â§</a>
