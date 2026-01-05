# Module rfc_0926_object_reader Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#130" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Object reader

- Proposal Name: `object_reader`
- Start Date: 2022-11-13
- RFC PR: [apache/opendal#926](https://github.com/apache/opendal/pull/926)
- Tracking Issue: [apache/opendal#927](https://github.com/apache/opendal/issues/927)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0926_object_reader/index.html#summary" class="doc-anchor">Â§</a>Summary

Returning reading related object meta in the reader.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0926_object_reader/index.html#motivation" class="doc-anchor">Â§</a>Motivation

Some services like s3 could return object meta while issuing reading requests.

In `GetObject`, we could get:

- Last-Modified
- Content-Length
- ETag
- Content-Range
- Content-Type
- Expires

We can avoid extra `HeadObject` calls by reusing that meta wisely, which could take 50ms. For example, `Content-Range` returns the content range of this read in the whole object: `<unit> <range-start>-<range-end>/<size>`. By using the content range, we can avoid `HeadObject` to get this objectâ€™s total size, which means a lot for the content cache.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0926_object_reader/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

`reader` and all its related API will return `ObjectReader` instead:

``` diff
- pub async fn reader(&self) -> Result<impl BytesRead> {}
+ pub async fn reader(&self) -> Result<ObjectReader> {}
```

`ObjectReader` impls `BytesRead` too, so existing code will keep working. And `ObjectReader` will provide similar APIs to `Entry`, for example:

``` rust
pub async fn content_length(&self) -> Option<u64> {}
pub async fn last_modified(&self) -> Option<OffsetDateTime> {}
pub async fn etag(&self) -> Option<String> {}
```

Note:

- All fields are optional, as services like fs could not return them.
- `content_length` here is this read requestâ€™s length, not the objectâ€™s length.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0926_object_reader/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

We will change the API signature of `Accessor`:

``` diff
- async fn read(&self, path: &str, args: OpRead) -> Result<BytesReader> {}
+ async fn read(&self, path: &str, args: OpRead) -> Result<ObjectReader> {}
```

`ObjectReader` is a wrapper of `BytesReader` and `ObjectMeta`:

``` rust
pub struct ObjectReader {
    inner: BytesReader
    meta: ObjectMetadata,
}

impl ObjectReader {
    pub async fn content_length(&self) -> Option<u64> {}
    pub async fn last_modified(&self) -> Option<OffsetDateTime> {}
    pub async fn etag(&self) -> Option<String> {}
}
```

Services can decide whether or not to fill them.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0926_object_reader/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0926_object_reader/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0926_object_reader/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0926_object_reader/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0926_object_reader/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0926_object_reader/index.html#add-content-range-support" class="doc-anchor">Â§</a>Add content-range support

We can add `content-range` in `ObjectMeta` so that users can fetch and use them.
