# Struct PuffinReader Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/puffin/reader.rs.html#26-29" class="src">Source</a>

``` rust
pub struct PuffinReader { /* private fields */ }
```

Expand description

Puffin reader

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinReader.html#impl-PuffinReader" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinReader.html" class="struct" title="struct iceberg::puffin::PuffinReader">PuffinReader</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinReader.html#method.new" class="fn">new</a>(input_file: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html" class="struct" title="struct iceberg::io::InputFile">InputFile</a>) -\> Self

Returns a new Puffin reader

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinReader.html#method.file_metadata" class="fn">file_metadata</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.FileMetadata.html" class="struct" title="struct iceberg::puffin::FileMetadata">FileMetadata</a>\>

Returns file metadata

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinReader.html#method.blob" class="fn">blob</a>(&self, blob_metadata: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html" class="struct" title="struct iceberg::puffin::Blob">Blob</a>\>

Returns blob

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinReader.html#blanket-implementations" class="anchor">§</a>
