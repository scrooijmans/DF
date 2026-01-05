# Struct CountLines Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/parser.rs.html#663" class="src">Source</a>

``` rust
pub struct CountLines { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/struct.CountLines.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/struct.CountLines.html#impl-CountLines" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/struct.CountLines.html" class="struct" title="struct polars::prelude::_csv_read_internal::CountLines">CountLines</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/struct.CountLines.html#method.new" class="fn">new</a>(quote_char: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>, eol_char: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/struct.CountLines.html" class="struct" title="struct polars::prelude::_csv_read_internal::CountLines">CountLines</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/struct.CountLines.html#method.analyze_chunk" class="fn">analyze_chunk</a>(&self, bytes: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> \[LineStats; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">2</a>\]

Analyzes a chunk of CSV data.

Returns (newline_count, last_newline_offset, end_inside_string) twice, the first is assuming the start of the chunk is *not* inside a string, the second assuming the start is inside a string.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/struct.CountLines.html#method.find_next" class="fn">find_next</a>(&self, bytes: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], chunk_size: &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/struct.CountLines.html#method.count" class="fn">count</a>(&self, bytes: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/struct.CountLines.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/_csv_read_internal/struct.CountLines.html#blanket-implementations" class="anchor">§</a>
