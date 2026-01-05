# Module rfc_2133_append_api Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#166" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Append API

- Proposal Name: `append_api`
- Start Date: 2023-04-26
- RFC PR: [apache/opendal#2133](https://github.com/apache/opendal/pull/2133)
- Tracking Issue: [apache/opendal#2163](https://github.com/apache/opendal/issues/2163)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2133_append_api/index.html#summary" class="doc-anchor">Â§</a>Summary

Introduce append operations for OpenDAL which allow users to add data to a file.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2133_append_api/index.html#motivation" class="doc-anchor">Â§</a>Motivation

OpenDAL has the write operation used to create a file and upload in parts. This is implemented based on multipart API. However, current approach has some limitations:

- Data could be lost and not readable before w.close() returned Ok(())
- File canâ€™t be appended again after w.close() returned Ok(())

To address these issues, I propose adding an append operation. Users can create an appender that provides a reentrant append operation. Each append operation will add data to the end of the file, which can be read immediately after the operation.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2133_append_api/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

The files created by the append operation can be appended via append API.

``` rust
async fn append_test(op: Operation) -> Result<()> {
  // create writer
  let append = op.append("path_to_file").await?;

  let bs = read_from_file();
  append.append(bs).await?;

  let bs = read_from_another_file();
  append.append(bs).await?;

  // close the file
  append.close().await?;
}
```

Difference between the write and append operation:

- write: Always create a new file, not readable until close.
- append: Can append existing appendable file, readable after append return.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2133_append_api/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

For underlay API, we will make these changes in Accessor:

``` rust
trait Accessor {
    type Appender: oio::Append;

    async fn append(&self, path: &str, args: OpAppend) -> Result<Self::Append>;
}
```

To implement this feature, we need to add a new API `append` into `oio::Append`.

``` rust
#[async_trait]
pub trait Append: Unpin + Send + Sync {
    /// Append data to the end of file.
    /// Users will call `append` multiple times. Please make sure `append` is safe to re-enter.
    async fn append(&mut self, bs: Bytes) -> Result<()>;

    /// Seal the file to mark it as unmodifiable.
    async fn close(&mut self) -> Result<()>;
}
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2133_append_api/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2133_append_api/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2133_append_api/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2133_append_api/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2133_append_api/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

We can use append API to implement for services that natively support append, such as [Azure blob](https://learn.microsoft.com/en-us/rest/api/storageservices/append-block?tabs=azure-ad) and [Alibaba cloud OSS](https://www.alibabacloud.com/help/en/object-storage-service/latest/appendobject). This will improve the performance and reliability of append operation.
