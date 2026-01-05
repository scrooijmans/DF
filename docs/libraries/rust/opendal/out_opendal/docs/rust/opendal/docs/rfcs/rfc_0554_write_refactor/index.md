# Module rfc_0554_write_refactor Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#102" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Write refactor

- Proposal Name: `write_refactor`
- Start Date: 2022-08-22
- RFC PR: [apache/opendal#554](https://github.com/apache/opendal/pull/554)
- Tracking Issue: [apache/opendal#555](https://github.com/apache/opendal/issues/555)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/index.html#summary" class="doc-anchor">Â§</a>Summary

Refactor `write` operation to accept a `BytesReader` instead.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/index.html#motivation" class="doc-anchor">Â§</a>Motivation

To simulate the similar operation like POSIX fs, OpenDAL returns `BytesWriter` for users to write, flush and close:

``` rust
pub trait Accessor {
    async fn write(&self, args: &OpWrite) -> Result<BytesWriter> {}
}
```

`Operator` builds the high level APIs upon this:

``` rust
impl Object {
    pub async fn write(&self, bs: impl AsRef<[u8]>) -> Result<()> {}
    
    pub async fn writer(&self, size: u64) -> Result<impl BytesWrite> {}
}
```

However, we are meeting the following problems:

- Performance: HTTP body channel is mush slower than read from Reader directly.
- Complicity: Service implementer have to deal with APIs like `new_http_channel`.
- Extensibility: Current design canâ€™t be extended to multipart APIs.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

Underlying `write` implementations will be replaced by:

``` rust
pub trait Accessor {
    async fn write(&self, args: &OpWrite, r: BytesReader) -> Result<u64> {}
}
```

Existing API will have no changes, and we will add a new API:

``` rust
impl Object {
    pub async fn write_from(&self, size: u64, r: impl BytesRead) -> Result<u64> {}
}
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

`Accessor`â€™s `write` API will be changed to accept a `BytesReader`:

``` rust
pub trait Accessor {
    async fn write(&self, args: &OpWrite, r: BytesReader) -> Result<u64> {}
}
```

We will provide `Operator::writer` based on this new API instead.

[RFC-0438: Multipart](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/0438-multipart.md) will also be updated to:

``` rust
pub trait Accessor {
    async fn write_multipart(&self, args: &OpWriteMultipart, r: BytesReader) -> Result<u64> {}
}
```

In this way, we donâ€™t need to introduce a `PartWriter`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/index.html#layer-api-breakage" class="doc-anchor">Â§</a>Layer API breakage

This change will introduce break changes to layers.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

- [RFC-0191: Async Streaming IO](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/0191-async-streaming-io.md)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

None.
