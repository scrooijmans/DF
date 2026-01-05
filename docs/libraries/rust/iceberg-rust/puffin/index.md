# Module puffin Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/puffin/mod.rs.html#18-38" class="src">Source</a>

Expand description

Iceberg Puffin implementation.

## Structs<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.Blob.html" class="struct" title="struct iceberg::puffin::Blob">Blob</a>  
The blob

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.BlobMetadata.html" class="struct" title="struct iceberg::puffin::BlobMetadata">BlobMetadata</a>  
Metadata about a blob. For more information, see: https://iceberg.apache.org/puffin-spec/#blobmetadata

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.FileMetadata.html" class="struct" title="struct iceberg::puffin::FileMetadata">FileMetadata</a>  
Metadata about a puffin file.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinReader.html" class="struct" title="struct iceberg::puffin::PuffinReader">PuffinReader</a>  
Puffin reader

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/struct.PuffinWriter.html" class="struct" title="struct iceberg::puffin::PuffinWriter">PuffinWriter</a>  
Puffin writer

## Enums<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/enum.CompressionCodec.html" class="enum" title="enum iceberg::puffin::CompressionCodec">CompressionCodec</a>  
Data compression formats

## Constants<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/constant.APACHE_DATASKETCHES_THETA_V1.html" class="constant" title="constant iceberg::puffin::APACHE_DATASKETCHES_THETA_V1">APACHE_DATASKETCHES_THETA_V1</a>  
A serialized form of a “compact” Theta sketch produced by the Apache DataSketches library.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/constant.CREATED_BY_PROPERTY.html" class="constant" title="constant iceberg::puffin::CREATED_BY_PROPERTY">CREATED_BY_PROPERTY</a>  
Human-readable identification of the application writing the file, along with its version. Example: “Trino version 381”

<a href="https://docs.rs/iceberg/0.7.0/iceberg/puffin/constant.DELETION_VECTOR_V1.html" class="constant" title="constant iceberg::puffin::DELETION_VECTOR_V1">DELETION_VECTOR_V1</a>  
A serialized form of a deletion vector.
