# Module path Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/path/mod.rs.html#18-614" class="src">Source</a>

Expand description

Path abstraction for Object Storage

## Structs<a href="https://docs.rs/object_store/latest/object_store/path/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html" class="struct" title="struct object_store::path::InvalidPart">InvalidPart</a>  
Error returned by [`PathPart::parse`](https://docs.rs/object_store/latest/object_store/path/struct.PathPart.html#method.parse "associated function object_store::path::PathPart::parse")

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>  
A parsed path representation that can be safely written to object storage

<a href="https://docs.rs/object_store/latest/object_store/path/struct.PathPart.html" class="struct" title="struct object_store::path::PathPart">PathPart</a>  
The PathPart type exists to validate the directory/file names that form part of a path.

## Enums<a href="https://docs.rs/object_store/latest/object_store/path/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>  
Error returned by [`Path::parse`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.parse "associated function object_store::path::Path::parse")

## Constants<a href="https://docs.rs/object_store/latest/object_store/path/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/path/constant.DELIMITER.html" class="constant" title="constant object_store::path::DELIMITER">DELIMITER</a>  
The delimiter to separate object namespaces, creating a directory structure.

<a href="https://docs.rs/object_store/latest/object_store/path/constant.DELIMITER_BYTE.html" class="constant" title="constant object_store::path::DELIMITER_BYTE">DELIMITER_BYTE</a>  
The path delimiter as a single byte
