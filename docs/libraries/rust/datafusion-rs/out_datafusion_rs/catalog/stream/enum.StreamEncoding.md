# Enum StreamEncoding Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/stream.rs.html#87" class="src">Source</a>

``` rust
pub enum StreamEncoding {
    Csv,
    Json,
}
```

Expand description

The data encoding for [`StreamTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamTable.html "struct datafusion::datasource::stream::StreamTable")

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#variant.Csv" class="anchor">§</a>

### Csv

CSV records

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#variant.Json" class="anchor">§</a>

### Json

Newline-delimited JSON records

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#impl-Clone-for-StreamEncoding" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/enum.StreamEncoding.html" class="enum" title="enum datafusion::datasource::stream::StreamEncoding">StreamEncoding</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/enum.StreamEncoding.html" class="enum" title="enum datafusion::datasource::stream::StreamEncoding">StreamEncoding</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#impl-Debug-for-StreamEncoding" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/enum.StreamEncoding.html" class="enum" title="enum datafusion::datasource::stream::StreamEncoding">StreamEncoding</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#impl-FromStr-for-StreamEncoding" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/enum.StreamEncoding.html" class="enum" title="enum datafusion::datasource::stream::StreamEncoding">StreamEncoding</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/enum.StreamEncoding.html" class="enum" title="enum datafusion::datasource::stream::StreamEncoding">StreamEncoding</a>, \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/enum.StreamEncoding.html" class="enum" title="enum datafusion::datasource::stream::StreamEncoding">StreamEncoding</a> as <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/enum.StreamEncoding.html#blanket-implementations" class="anchor">§</a>
