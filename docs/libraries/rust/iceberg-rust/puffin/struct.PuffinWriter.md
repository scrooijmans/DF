# Struct PuffinWriter Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/puffin/writer.rs.html#29-37" class="src">Source</a>

``` rust
pub struct PuffinWriter { /* private fields */ }
```

Expand description

Puffin writer

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinWriter.html#impl-PuffinWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinWriter.html" class="struct" title="struct iceberg::puffin::PuffinWriter">PuffinWriter</a>

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinWriter.html#method.new" class="fn">new</a>( output_file: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html" class="struct" title="struct iceberg::io::OutputFile">OutputFile</a>, properties: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, compress_footer: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Returns a new Puffin writer

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinWriter.html#method.add" class="fn">add</a>( &mut self, blob: <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html" class="struct" title="struct iceberg::puffin::Blob">Blob</a>, compression_codec: <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/enum.CompressionCodec.html" class="enum" title="enum iceberg::puffin::CompressionCodec">CompressionCodec</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Adds blob to Puffin file

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinWriter.html#method.close" class="fn">close</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Finalizes the Puffin file

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinWriter.html#blanket-implementations" class="anchor">§</a>
