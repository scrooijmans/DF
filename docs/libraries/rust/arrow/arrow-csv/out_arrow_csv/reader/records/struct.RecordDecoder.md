# Struct RecordDecoder Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/records.rs.html#29-65" class="src">Source</a>

``` rust
pub struct RecordDecoder {
    delimiter: Reader,
    num_columns: usize,
    line_number: usize,
    offsets: Vec<usize>,
    offsets_len: usize,
    current_field: usize,
    num_rows: usize,
    data: Vec<u8>,
    data_len: usize,
    truncated_rows: bool,
}
```

Expand description

[`RecordDecoder`](https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html "struct arrow_csv::reader::records::RecordDecoder") provides a push-based interface to decoder [`StringRecords`](https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html "struct arrow_csv::reader::records::StringRecords")

## Fields<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#fields" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#structfield.delimiter" class="anchor field">Â§</a>`delimiter: Reader`<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#structfield.num_columns" class="anchor field">Â§</a>`num_columns: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

The expected number of fields per row

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#structfield.line_number" class="anchor field">Â§</a>`line_number: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

The current line number

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#structfield.offsets" class="anchor field">Â§</a>`offsets: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Offsets delimiting field start positions

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#structfield.offsets_len" class="anchor field">Â§</a>`offsets_len: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

The current offset into `self.offsets`

We track this independently of Vec to avoid re-zeroing memory

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#structfield.current_field" class="anchor field">Â§</a>`current_field: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

The number of fields read for the current record

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#structfield.num_rows" class="anchor field">Â§</a>`num_rows: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

The number of rows buffered

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#structfield.data" class="anchor field">Â§</a>`data: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`

Decoded field data

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#structfield.data_len" class="anchor field">Â§</a>`data_len: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Offsets into data

We track this independently of Vec to avoid re-zeroing memory

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#structfield.truncated_rows" class="anchor field">Â§</a>`truncated_rows: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether rows with less than expected columns are considered valid

Default value is false When enabled fills in missing columns with null

## Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#impl-RecordDecoder" class="anchor">Â§</a>

### impl <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html" class="struct" title="struct arrow_csv::reader::records::RecordDecoder">RecordDecoder</a>

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#method.new" class="fn">new</a>(delimiter: Reader, num_columns: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, truncated_rows: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#method.decode" class="fn">decode</a>( &mut self, input: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], to_read: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>), ArrowError\>

Decodes records from `input` returning the number of records and bytes read

Note: this expects to be called with an empty `input` to signal EOF

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the current number of buffered records

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the decoder is empty

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#method.clear" class="fn">clear</a>(&mut self)

Clears the current contents of the decoder

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#method.flush" class="fn">flush</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html" class="struct" title="struct arrow_csv::reader::records::StringRecords">StringRecords</a>\<'\_\>, ArrowError\>

Flushes the current contents of the reader

## Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#trait-implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#impl-Debug-for-RecordDecoder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html" class="struct" title="struct arrow_csv::reader::records::RecordDecoder">RecordDecoder</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.RecordDecoder.html#blanket-implementations" class="anchor">Â§</a>
