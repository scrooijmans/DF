# Struct StringRecords Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/records.rs.html#250-255" class="src">Source</a>

``` rust
pub struct StringRecords<'a> {
    num_columns: usize,
    num_rows: usize,
    offsets: &'a [usize],
    data: &'a str,
}
```

Expand description

A collection of parsed, UTF-8 CSV records

## Fields<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#fields" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#structfield.num_columns" class="anchor field">Â§</a>`num_columns: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#structfield.num_rows" class="anchor field">Â§</a>`num_rows: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#structfield.offsets" class="anchor field">Â§</a>`offsets: &'a [`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`]`<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#structfield.data" class="anchor field">Â§</a>`data: &'a `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

## Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#impl-StringRecords%3C&#39;a%3E" class="anchor">Â§</a>

### impl\<'a\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html" class="struct" title="struct arrow_csv::reader::records::StringRecords">StringRecords</a>\<'a\>

#### fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#method.get" class="fn">get</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html" class="struct" title="struct arrow_csv::reader::records::StringRecord">StringRecord</a>\<'a\>

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#method.iter" class="fn">iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html" class="struct" title="struct arrow_csv::reader::records::StringRecord">StringRecord</a>\<'a\>\> + '\_

## Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#trait-implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#impl-Debug-for-StringRecords%3C&#39;a%3E" class="anchor">Â§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html" class="struct" title="struct arrow_csv::reader::records::StringRecords">StringRecords</a>\<'a\>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecords.html#blanket-implementations" class="anchor">Â§</a>
