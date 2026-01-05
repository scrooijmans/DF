# Module rfc_0247_retryable_error Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#62" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Retryable error

- Proposal Name: `retryable_error`
- Start Date: 2022-04-12
- RFC PR: [apache/opendal#247](https://github.com/apache/opendal/pull/247)
- Tracking Issue: [apache/opendal#248](https://github.com/apache/opendal/issues/248)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#summary" class="doc-anchor">Â§</a>Summary

Treat `io::ErrorKind::Interrupt` as retryable error.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#motivation" class="doc-anchor">Â§</a>Motivation

Supports retry make our usersâ€™ lives easier:

> [Feature request: Custom retries for the s3 backend](https://github.com/apache/opendal/issues/196)
>
> While the reading/writing from/to s3, AWS occasionally returns errors that could be retried (at least 5xx?). Currently, in the databend, this will fail the whole execution of the statement (which may have been running for an extended time).

Most users may need this retry feature, like `decompress`. Implementing it in OpenDAL will make users no bother, no backoff logic.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

With the `retry` feature enabled:

``` toml
opendal = {version="0.5.2", features=["retry"]}
```

Users can configure the retry behavior easily:

``` rust
let backoff = ExponentialBackoff::default();
let op = op.with_backoff(backoff);
```

All requests sent by `op` will be automatically retried.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

We will implement retry features via adding a new `Layer`.

In the retry layer, we will support retrying all operations. To do our best to keep retrying read & write, we will implement `RetryableReader` and `RetryableWriter`, which will support retry while no actual IO happens.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#retry-operations" class="doc-anchor">Â§</a>Retry operations

Most operations are safe to retry, like `list`, `stat`, `delete` and `create`.

We will retry those operations via input backoff.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#retry-io-operations" class="doc-anchor">Â§</a>Retry IO operations

Retry IO operations are a bit complex because IO operations have side effects, especially for HTTP-based services like s3. We canâ€™t resume an operation during the reading process without sending new requests.

This proposal will do the best we can: retry the operation if no actual IO happens.

If we meet an internal error before reading/writing the userâ€™s buffer, itâ€™s safe and cheap to retry it with precisely the same argument.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#retryable-error" class="doc-anchor">Â§</a>Retryable Error

- Operator MAY retry `io::ErrorKind::Interrupt` errors.
- Services SHOULD return `io::ErrorKind::Interrupt` kind if the error is retryable.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#write-operation-cant-be-retried" class="doc-anchor">Â§</a>Write operation canâ€™t be retried

As we return `Writer` to users, there is no way for OpenDAL to get the input data again.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#implement-retry-at-operator-level" class="doc-anchor">Â§</a>Implement retry at operator level

We need to implement retry logic for every operator function, and canâ€™t address the same problem:

- `Reader` / `Writer` canâ€™t be retired.
- Intrusive design that users cannot expand on their own

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

- `read` and `write` canâ€™t be retried during IO.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

None
