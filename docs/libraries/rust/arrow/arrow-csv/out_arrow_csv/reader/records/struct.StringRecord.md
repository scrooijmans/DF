# Struct StringRecord Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/records.rs.html#277-280" class="src">Source</a>

``` rust
pub struct StringRecord<'a> {
    data: &'a str,
    offsets: &'a [usize],
}
```

Expand description

A single parsed, UTF-8 CSV record

## Fields<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#fields" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#structfield.data" class="anchor field">Â§</a>`data: &'a `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a><a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#structfield.offsets" class="anchor field">Â§</a>`offsets: &'a [`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`]`

## Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#impl-StringRecord%3C&#39;a%3E" class="anchor">Â§</a>

### impl\<'a\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html" class="struct" title="struct arrow_csv::reader::records::StringRecord">StringRecord</a>\<'a\>

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#method.get" class="fn">get</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

## Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#trait-implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#impl-Clone-for-StringRecord%3C&#39;a%3E" class="anchor">Â§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html" class="struct" title="struct arrow_csv::reader::records::StringRecord">StringRecord</a>\<'a\>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html" class="struct" title="struct arrow_csv::reader::records::StringRecord">StringRecord</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#impl-Debug-for-StringRecord%3C&#39;a%3E" class="anchor">Â§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html" class="struct" title="struct arrow_csv::reader::records::StringRecord">StringRecord</a>\<'a\>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#impl-Display-for-StringRecord%3C&#39;_%3E" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html" class="struct" title="struct arrow_csv::reader::records::StringRecord">StringRecord</a>\<'\_\>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#method.fmt-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#impl-Copy-for-StringRecord%3C&#39;a%3E" class="anchor">Â§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html" class="struct" title="struct arrow_csv::reader::records::StringRecord">StringRecord</a>\<'a\>

## Auto Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/records/struct.StringRecord.html#blanket-implementations" class="anchor">Â§</a>
