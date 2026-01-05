# Module rfc_5556_write_returns_metadata Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#262" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Write Returns Metadata

- Proposal Name: `write_returns_metadata`
- Start Date: 2025-01-16
- RFC PR: [apache/opendal#5556](https://github.com/apache/opendal/pull/5556)
- Tracking Issue: [apache/opendal#5557](https://github.com/apache/opendal/issues/5557)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#summary" class="doc-anchor">Â§</a>Summary

Enhance write operations by returning metadata after successful writes.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#motivation" class="doc-anchor">Â§</a>Motivation

Currently, write operations (`write`, `write_with`, `writer`, `writer_with`) only return `Result<()>` or `Result<Writer>`. Users who need metadata after writing (like `ETag` or `version_id`) must make an additional `stat()` call. This is inefficient and can lead to race conditions if the file is modified between the write and stat operations.

Many storage services (like S3, GCS, Azure Blob) return metadata in their write responses. We should expose this information to users directly after write operations.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

The write operations will be enhanced to return metadata:

``` rust
// Before
op.write("path/to/file", data).await?;
let meta = op.stat("path/to/file").await?;
if Some(etag) = meta.etag() {
    println!("File ETag: {}", etag);
}

// After
let meta = op.write("path/to/file", data).await?;
if Some(etag) = meta.etag() {
    println!("File ETag: {}", etag);
}
```

For writer operations:

``` rust
// Before
let mut writer = op.writer("path/to/file").await?;
writer.write(data).await?;
writer.close().await?;
let meta = op.stat("path/to/file").await?;
if Some(etag) = meta.etag() {
    println!("File ETag: {}", etag);
}

// After
let mut writer = op.writer("path/to/file").await?;
writer.write(data).await?;
let meta = writer.close().await?;
if Some(etag) = meta.etag() {
    println!("File ETag: {}", etag);
}
```

The behavior remains unchanged if users donâ€™t need the metadata - they can simply ignore the return value.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#changes-to-operator-api" class="doc-anchor">Â§</a>Changes to `Operator` API

The following functions will be modified to return `Result<Metadata>` instead of `Result<()>`:

- `write()`
- `write_with()`

The `writer()` and `writer_with()` return types remain unchanged as they return `Result<Writer>`.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#changes-to-struct-writer" class="doc-anchor">Â§</a>Changes to struct `Writer`

The `Writer` struct will be modified to return `Result<Metadata>` instead of `Result<()>` for the `close()` function.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#changes-to-trait-oiowrite-and-trait-oiomultipartwrite" class="doc-anchor">Â§</a>Changes to trait `oio::Write` and trait `oio::MultipartWrite`

The `Write` trait will be modified to return `Result<Metadata>` instead of `Result<()>` for the `close()` function.

The `MultipartWrite` trait will be modified to return `Result<Metadata>` instead of `Result<()>` for the `complete_part()` and `write_once` functions.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#implementation-details" class="doc-anchor">Â§</a>Implementation Details

For services that return metadata in their write responses:

- The metadata will be captured from the service response
- All available fields (etag, version_id, etc.) will be populated

For services that donâ€™t return metadata in write responses:

- for `fs`: we can use `stat` to retrieve the metadata before returning. since the metadata is cached by the kernel, this wonâ€™t cause a performance issue.
- for other services: A default metadata object will be returned.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

- Minor breaking change for users who explicitly type the return value of write operations
- Additional complexity in the Writer implementation

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

- Provides a clean, consistent API
- Maintains backward compatibility for users who ignore the return value
- Improves performance by avoiding additional stat calls when possible

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

Similar patterns exist in other storage SDKs:

- `object_store` crate returns metadata in `PutResult` after calling `put_opts`
- AWS SDK returns metadata in `PutObjectOutput`
- Azure SDK returns `UploadFileResponse` after uploads

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

- None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

- None
