# Module rfc_3898_concurrent_writer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#226" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Concurrent Writer

- Proposal Name: `concurrent_writer`
- Start Date: 2024-01-02
- RFC PR: [apache/opendal#3898](https://github.com/apache/opendal/pull/3898)
- Tracking Issue: [apache/opendal#3899](https://github.com/apache/opendal/issues/3899)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3898_concurrent_writer/index.html#summary" class="doc-anchor">Â§</a>Summary

Enhance the `Writer` by adding concurrent write capabilities.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3898_concurrent_writer/index.html#motivation" class="doc-anchor">Â§</a>Motivation

Certain services, such as S3, GCS, and AzBlob, offer the `multi_write` functionality, allowing users to perform multiple write operations for uploading of large files. If a service support `multi_write`, the [Capability::write_can_multi](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_can_multi) metadata should be set to `true`.

``` rust
    let mut writer = op.writer("path/to").await?; // a writers supports the `multi_write`.
    writer.write(part0).await?;
    writer.write(part1).await?; // It starts to upload after the `part0` is finished.
    writer.close().await?;
```

Currently, when invoking a `Writer` that supports the `multi_write` functionality, multiple writes are proceed serially, without fully leveraging the potential for improved throughput through concurrent uploads. We should enhance support to allow concurrent processing of multiple write operations.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3898_concurrent_writer/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

For users who want to concurrent writer, they will call the new API `concurrent`. And the default behavior remains unchanged, so users using `op.writer_with()` are not affected. The `concurrent` function will take a number as input, and this number will represent the maximum concurrent write task amount the writer can perform.

- If `concurrent` is set to 0 or 1, it functions with default behavior(writes serially).
- However, if `concurrent` is set to number larger than 1. It enables concurrent uploading of up to `concurrent` write tasks and allows users to initiate additional write tasks without waiting to complete the previous write operation, as long as the inner task queue still has available slots.

The concurrent write feature operate independently of other features.

``` rust
let mut w = op.writer_with(path).concurrent(8).await;
w.write(part0).await?;
w.write(part1).await?; // `write` won't wait for part0.
w.close().await?; // `close` will make sure all parts are finished.
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3898_concurrent_writer/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

The S3 and similar services use `MultipartUploadWriter`, while GCS uses `RangeWriter`. We can enhance these services by adding concurrent write features to them. A `concurrent` field of type `usize` will be introduced to `OpWrite` to allow the user to set the maximum concurrent write task amount. For other services that donâ€™t support `multi_write`, setting the concurrent parameter will have no effect, maintaining the default behavior.

This feature will be implemented in the `MultipartUploadWriter` and `RangeWriter`, which will utilize a `ConcurrentFutures<WriteTask>` as a task queue to store concurrent write tasks.

When the upper layer invokes `poll_write`, the `Writer` pushes write to the task queue (`ConcurrentFutures<WriteTask>`) if there are available slots, and then relinquishes control back to the upper layer. This allows for up to `concurrent` write tasks to uploaded concurrently without waiting. If the task queue is full, the `Writer` waits for the first task to yield results.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3898_concurrent_writer/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

- More memory usage
- More concurrent connections

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3898_concurrent_writer/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3898_concurrent_writer/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3898_concurrent_writer/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3898_concurrent_writer/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

- Adding `write_at` for `fs`.
- Use `ConcurrentFutureUnordered` instead of `ConcurrentFutures.`
