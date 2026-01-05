# Trait AsString Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/string/mod.rs.html#330" class="src">Source</a>

``` rust
pub trait AsString {
    // Required method
    fn as_string(&self) -> &ChunkedArray<StringType>;
}
```

Available on **crate feature `temporal`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.AsString.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.AsString.html#tymethod.as_string" class="fn">as_string</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.AsString.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.AsString.html#impl-AsString-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/trait.AsString.html" class="trait" title="trait polars::prelude::chunkedarray::string::AsString">AsString</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>
