# Module multipart Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/multipart.rs.html#18-84" class="src">Source</a>

Expand description

Cloud Multipart Upload

This crate provides an asynchronous interface for multipart file uploads to cloud storage services. It’s designed to offer efficient, non-blocking operations, especially useful when dealing with large files or high-throughput systems.

## Structs<a href="https://docs.rs/object_store/latest/object_store/multipart/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/multipart/struct.PartId.html" class="struct" title="struct object_store::multipart::PartId">PartId</a>  
Represents a part of a file that has been successfully uploaded in a multipart upload process.

## Traits<a href="https://docs.rs/object_store/latest/object_store/multipart/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html" class="trait" title="trait object_store::multipart::MultipartStore">MultipartStore</a>  
A low-level interface for interacting with multipart upload APIs
