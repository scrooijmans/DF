# Module limit Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/limit.rs.html#18-320" class="src">Source</a>

Expand description

An object store that limits the maximum concurrency of the wrapped implementation

## Structs<a href="https://docs.rs/object_store/latest/object_store/limit/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html" class="struct" title="struct object_store::limit::LimitStore">LimitStore</a>  
Store wrapper that wraps an inner store and limits the maximum number of concurrent object store operations. Where each call to an [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") member function is considered a single operation, even if it may result in more than one network call

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html" class="struct" title="struct object_store::limit::LimitUpload">LimitUpload</a>  
An [`MultipartUpload`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html "trait object_store::MultipartUpload") wrapper that limits the maximum number of concurrent requests
