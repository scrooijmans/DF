# Module rfc_3017_remove_write_copy_from Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#194" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Remove write copy from

- Proposal Name: `remove_write_copy_from`
- Start Date: 2023-09-06
- RFC PR: [apache/opendal#3017](https://github.com/apache/opendal/pull/3017)
- Tracking Issue: [apache/opendal#3017](https://github.com/apache/opendal/issues/3017)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/index.html#summary" class="doc-anchor">Â§</a>Summary

Remove the `oio::Write::copy_from()` API pending a more thoughtful design.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/index.html#motivation" class="doc-anchor">Â§</a>Motivation

In [RFC-2083: Writer Sink API](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/2083_writer_sink_api.md), we launched an API, initially named `sink` and changed to `copy_from`, that enables data writing from a `Reader` to a `Writer` object.

The current API signature is:

``` rust
pub trait Write: Unpin + Send + Sync {
    /// Copies data from the given reader to the writer.
    ///
    /// # Behavior
    ///
    /// - `Ok(n)` indicates successful writing of `n` bytes.
    /// - `Err(err)` indicates a failure, resulting in zero bytes written.
    ///
    /// A situation where `n < size` may arise; the caller should then transmit the remaining bytes until the full amount is written.
    async fn copy_from(&mut self, size: u64, src: oio::Reader) -> Result<u64>;
}
```

The API has the following limitations:

- Incompatibility with existing buffering and retry mechanisms.
- Imposes ownership requirements on the reader, complicating its use. The reader must be recreated for every write operation.

Due to restrictions in both Rust and Hyperâ€™s APIs, the following ideal implementation is currently unattainable:

``` rust
pub trait Write: Unpin + Send + Sync {
    async fn copy_from(&mut self, size: u64, src: &mut impl oio::Read) -> Result<u64>;
}
```

- Rust doesnâ€™t allow us to have `impl oio::Read` in trait method if we want object safe.
- hyper doesnâ€™t allow us to use reference here, it requires `impl Stream + 'static`.

Given these constraints, the proposal is to remove `oio::Write::copy_from` until a more fitting design becomes feasible.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

The `Writer::sink()` and `Writer::copy()` methods will be kept, but itâ€™s internal implementation will be changed to use `AsyncWrite` instead. For example:

``` diff
pub async fn copy<R>(&mut self, size: u64, read_from: R) -> Result<u64>
where
    R: futures::AsyncRead + Send + Sync + Unpin + 'static,
{
    if let State::Idle(Some(w)) = &mut self.state {
        let r = Box::new(oio::into_streamable_read(
            oio::into_read_from_file(read_from, 0, size),
            64 * 1024,
        ));
-       w.copy_from(size, r).await
+       futures::io::copy(&mut r, w).await
    } else {
        unreachable!(
            "writer state invalid while copy, expect Idle, actual {}",
            self.state
        );
    }
}
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

The method `oio::Write::copy_from` will be removed.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

The deprecation eliminates the ability to stream data uploads. A viable alternative is to directly use `AsyncWrite` offered by `Writer`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and Alternatives

N/A

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/index.html#prior-art" class="doc-anchor">Â§</a>Prior Art

N/A

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved Questions

N/A

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/index.html#future-possibilities" class="doc-anchor">Â§</a>Future Possibilities

Introduce utility functions such as `Writer::copy_from(r: &dyn AsyncRead)` when possible.
