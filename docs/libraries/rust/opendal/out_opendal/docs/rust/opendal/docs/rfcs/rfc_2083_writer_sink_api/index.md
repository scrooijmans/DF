# Module rfc_2083_writer_sink_api Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#162" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Writer sink API

- Proposal Name: `writer_sink_api`
- Start Date: 2023-04-23
- RFC PR: [apache/opendal#2083](https://github.com/apache/opendal/pull/2083)
- Tracking Issue: [apache/opendal#2084](https://github.com/apache/opendal/issues/2084)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#summary" class="doc-anchor">Â§</a>Summary

Include a `sink` API within the `Writer` to enable streaming writing.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#motivation" class="doc-anchor">Â§</a>Motivation

OpenDAL does not support streaming data uploads. Users must first load the data into memory and then send it to the `writer`.

``` rust
let bs = balabala();
w.write(bs).await?;
let bs = daladala();
w.write(bs).await?;
...
w.close().await?;
```

There are two main drawbacks to OpenDAL:

- high memory usage, as reported in issue \#1821 on GitHub
- low performance due to the need to buffer user data before sending it over the network.

To address this issue, it would be beneficial for OpenDAL to provide an API that allows users to pass a stream or reader directly into the writer.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

I propose to add the following API to `Writer`:

``` rust
impl Writer {
    pub async fn copy_from<R>(&mut self, size: u64, r: R) -> Result<()>
    where
        R: futures::AsyncRead + Send + Sync + 'static;

    pub async fn pipe_from<S>(&mut self, size: u64, s: S) -> Result<()>
    where
        S: futures::TryStream + Send + Sync + 'static
        Bytes: From<S::Ok>;
}
```

Users can now upload data in a streaming way:

``` rust
// Start writing the 5 TiB file.
let w = op.writer_with(
    OpWrite::default()
        .with_content_length(5 * 1024 * 1024 * 1024 * 1024));

let r = balabala();
// Send to network directly without in-memory buffer.
w.copy_from(size, r).await?;
// repeat...
...

// Close the write once we are ready!
w.close().await?;
```

The underlying services will handle this stream in the most efficient way possible.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

To support `Wrtier::copy_from` and `Writer::pipe_from`, we will add a new API called `sink` inside `oio::Writer`:

``` rust
#[async_trait]
pub trait Write: Unpin + Send + Sync {
    async fn sink(&mut self, size: u64, s: Box<dyn futures::TryStream<Ok=Bytes> + Send + Sync>) -> Result<()>;
}
```

OpenDAL converts the user input reader and stream into a byte stream for `oio::Write`. Services that support streaming upload natively can directly pass the stream. If not, they can use `write` repeatedly to write the entire stream.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#whats-the-different-of-opwritecontent_length-and-sink-size" class="doc-anchor">Â§</a>Whatâ€™s the different of `OpWrite::content_length` and `sink` size?

The `OpWrite::content_length` parameter specifies the total length of the file to be written, while the `size` argument in the `sink` API indicates the size of the reader or stream provided. Certain services may optimize by writing all content in a single request if `content_length` is the same with given `size`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#retry-for-the-sink-api" class="doc-anchor">Â§</a>Retry for the `sink` API

Itâ€™s impossible to retry the `sink` API itself, but we can provide a wrapper to retry the streamâ€™s call of `next`. If we met a retryable error, we can call `next` again by crate like `backon`.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html#blocking-support-for-sink" class="doc-anchor">Â§</a>Blocking support for sink

We will add async support first.
