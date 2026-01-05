# Module rfc_0977_refactor_error Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#134" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Refactor error

- Proposal Name: `refactor-error`
- Start Date: 2022-11-21
- RFC PR: [apache/opendal#977](https://github.com/apache/opendal/pull/977)
- Tracking Issue: [apache/opendal#976](https://github.com/apache/opendal/pull/976)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#summary" class="doc-anchor">Â§</a>Summary

Use a separate error instead of `std::io::Error`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#motivation" class="doc-anchor">Â§</a>Motivation

OpenDAL is used to use `std::io::Error` for all functions. This design is natural and easy to use. But there are many problems with the usage:

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#not-friendly-for-retry" class="doc-anchor">Â§</a>Not friendly for retry

`io::Error` canâ€™t carry retry-related information. In [RFC-0247: Retryable Error](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/0247-retryable-error.md), we use `io::ErrorKind::Interrupt` to indicate this error is retryable. But this change will hide the real error kind from the underlying. To mark this error has been retried, we have to add another new error wrapper:

``` rust
#[derive(thiserror::Error, Debug)]
#[error("permanent error: still failing after retry, source: {source}")]
struct PermanentError {
    source: Error,
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#errorkind-is-inaccurate" class="doc-anchor">Â§</a>ErrorKind is inaccurate

`std::io::ErrorKind` is used to represent errors returned from system io, which is unsuitable for mistakes that have business semantics. For example, users canâ€™t distinguish `ObjectNotFound` or `BucketNotFound` from `ErrorKind::NotFound`.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#errorkind-is-incomplete" class="doc-anchor">Â§</a>ErrorKind is incomplete

OpenDAL has been waiting for features [`io_error_more`](https://github.com/rust-lang/rust/issues/86442) to be stabilized for a long time. But there is no progress so far, which makes it impossible to return `ErrorKind::IsADirectory` or `ErrorKind::NotADirectory` on stable rust.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#error-is-not-easy-to-carry-context" class="doc-anchor">Â§</a>Error is not easy to carry context

To carry context inside `std::io::Error`, we have to check and make sure all functions are constructed `ObjectError` or `BackendError`:

``` rust
#[derive(Error, Debug)]
#[error("object error: (op: {op}, path: {path}, source: {source})")]
pub struct ObjectError {
    op: Operation,
    path: String,
    source: anyhow::Error,
}
```

To make everything worse, we canâ€™t prevent opendal returns raw io errors directly. For example, in `Object::range_read`:

``` rust
pub async fn range_read(&self, range: impl RangeBounds<u64>) -> Result<Vec<u8>> {
    ...

    io::copy(s, &mut bs).await?;

    Ok(bs.into_inner())
}
```

We leaked the `io::Error` without any context.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

Thus, I propose to add `opendal::Error` back with everything improved.

Users will have similar usage as before:

``` rust
if let Err(e) = op.object("test_file").metadata().await {
    if e.kind() == ErrorKind::ObjectNotFound {
        println!("object not exist")
    }
}
```

Users can check if this error a `temporary`:

``` rust
if err.is_temporary() {
    // retry the operation
}
```

Users can print error messages via `Display`:

``` rust
> println!("{}", err);

ObjectNotFound (permanent) at read, context: { service: S3, path: path/to/file } => status code: 404, headers: {"x-amz-request-id": "GCYDTQX51YRSF4ZF", "x-amz-id-2": "EH0vV6lTwWk+lFXqCMCBSk1oovqhG4bzALU9+sUudyw7TEVrfWm2o/AFJKhYKpdGqOoBZGgMTC0=", "content-type": "application/xml", "date": "Mon, 21 Nov 2022 05:26:37 GMT", "server": "AmazonS3"}, body: ""
```

Also, users can choose to print the more verbose message via `Debug`:

``` rust
> println!("{:?}", err);

ObjectNotFound (permanent) at read => status code: 404, headers: {"x-amz-request-id": "GCYDTQX51YRSF4ZF", "x-amz-id-2": "EH0vV6lTwWk+lFXqCMCBSk1oovqhG4bzALU9+sUudyw7TEVrfWm2o/AFJKhYKpdGqOoBZGgMTC0=", "content-type": "application/xml", "date": "Mon, 21 Nov 2022 05:26:37 GMT", "server": "AmazonS3"}, body: ""

Context:
    service: S3
    path: path/to/file

Source: <source error>

Backtrace: <backtrace if we have>
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

We will add new `Error` and `ErrorKind` in opendal:

``` rust
pub struct Error {
    kind: ErrorKind,
    message: String,

    status: ErrorStatus,
    operation: &'static str,
    context: Vec<(&'static str, String)>,
    source: Option<anyhow::Error>,
}
```

- status: the status of this error, which indicates if this error is temporary
- operation: the operation which generates this error
- context: the context related to this error
- source: the underlying source error

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#breaking-changes" class="doc-anchor">Â§</a>Breaking changes

This RFC will lead to a breaking at user side.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html#more-errorkind" class="doc-anchor">Â§</a>More ErrorKind

We can add more error kinds to make it possible for users to check.
