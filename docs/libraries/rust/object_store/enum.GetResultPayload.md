# Enum GetResultPayload Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/lib.rs.html#1059-1065" class="src">Source</a>

``` rust
pub enum GetResultPayload {
    File(File, PathBuf),
    Stream(BoxStream<'static, Result<Bytes>>),
}
```

Expand description

The kind of a [`GetResult`](https://docs.rs/object_store/latest/object_store/struct.GetResult.html "struct object_store::GetResult")

This special cases the case of a local file, as some systems may be able to optimise the case of a file already present on local disk

## Variants<a href="https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html#variants" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html#variant.File" class="anchor">§</a>

### File(<a href="https://doc.rust-lang.org/nightly/std/fs/struct.File.html" class="struct" title="struct std::fs::File">File</a>, <a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>)

Available on **crate feature `fs` and non-WebAssembly** only.

The file, path

<a href="https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html#variant.Stream" class="anchor">§</a>

### Stream(<a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/type.BoxStream.html" class="type" title="type futures_core::stream::BoxStream">BoxStream</a>\<'static, <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>\>)

An opaque stream of bytes

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html#impl-Debug-for-GetResultPayload" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html" class="enum" title="enum object_store::GetResultPayload">GetResultPayload</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html#blanket-implementations" class="anchor">§</a>
