# Module rfc_1085_object_handler Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#138" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Object handler

- Proposal Name: `object_handler`
- Start Date: 2022-12-19
- RFC PR: [apache/opendal#1085](https://github.com/apache/opendal/pull/1085)
- Tracking Issue: [apache/opendal#1085](https://github.com/apache/opendal/issues/1085)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1085_object_handler/index.html#summary" class="doc-anchor">Â§</a>Summary

Returning a `file description` to users for native seek support.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1085_object_handler/index.html#motivation" class="doc-anchor">Â§</a>Motivation

OpenDALâ€™s goal is to `access data freely, painlessly, and efficiently`, so we build an operation first API which means we provide operation instead of the file description. Users donâ€™t need to call `open` before `read`; OpenDAL will handle all the open and close functions.

However, our users do want to control the complex behavior of that:

- Some storage backends have native `seek` support, but OpenDAL canâ€™t fully use them.
- Users want to improve performance by reusing the same file description without `open` and `close` for every read operation.

This RFC will fill this gap.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1085_object_handler/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

Users can get an object handler like:

``` rust
let oh: ObjectHandler = op.object("path/to/file").open().await?;
```

`ObjectHandler` will implement `AsyncRead` and `AsyncSeek` so it can be used like `tokio::fs::File`. If the backend supports native seek operation, we will take the native process; otherwise, we will fall back to simulation implementations.

The blocking version will be provided by:

``` rust
let boh: BlockingObjectHandler = op.object("path/to/file").blocking_open()?;
```

And `BlockingObjectHandler` will implement `Read` and `Seek` so it can be used like `std::fs::File`. If the backend supports native seek operation, we will take the native process; otherwise, we will fall back to simulation implementations.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1085_object_handler/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

This RFC will add a new API `open` in `Accessor`:

``` rust
pub trait Accessor {
    async fn open(&self, path: &str, args: OpOpen) -> Result<(RpOpen, BytesHandler)>;
}
```

Only services that support native `seek` operations can implement this API, like `fs` and `hdfs`. For services that do not support native `seek` operations like `s3` and `azblob`, we will fall back to the simulation implementations: maintaining an in-memory index instead.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1085_object_handler/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1085_object_handler/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1085_object_handler/index.html#how-about-writing-operations" class="doc-anchor">Â§</a>How about writing operations?

Ideally, writing on `ObjectHandler` should also be supported. But we still donâ€™t know how this API will be used. Letâ€™s apply this API for `read` first.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1085_object_handler/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1085_object_handler/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1085_object_handler/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

- Add write support
- Adopt native `pread`
