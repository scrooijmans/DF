# Module rfc_2758_merge_append_into_write Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#178" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Merge append into write

- Proposal Name: `merge_append_into_write`
- Start Date: 2023-08-02
- RFC PR: [apache/opendal#2758](https://github.com/apache/opendal/pull/2758)
- Tracking Issue: [apache/opendal#2760](https://github.com/apache/opendal/issues/2760)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2758_merge_append_into_write/index.html#summary" class="doc-anchor">Â§</a>Summary

Merge the `appender` API into `writer` by introducing a new `writer_with.append(true)` method to enable append mode.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2758_merge_append_into_write/index.html#motivation" class="doc-anchor">Â§</a>Motivation

Currently OpenDAL has separate `appender` and `writer` APIs:

``` rust
let mut appender = op.appender_with("file.txt").await?; 

appender.append(bs).await?;
appender.append(bs).await?;
```

This duplication forces users to learn two different APIs for writing data.

By adding this change, we can:

- Simpler API surface - users only need to learn one writing API.
- Reduce code duplication between append and write implementations.
- Atomic append semantics are handled internally in `writer`.
- Reuse the `sink` api for both `overwrite` and `append` mode.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2758_merge_append_into_write/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

The new approach is to enable append mode on `writer`:

``` rust
let mut writer = op.writer_with("file.txt").append(true).await?;

writer.write(bs).await?; // appends to file
writer.write(bs).await?; // appends again
```

Calling `writer_with.append(true)` will start the writer in append mode. Subsequent `write()` calls will append rather than overwrite.

There is no longer a separate `appender` API.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2758_merge_append_into_write/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

We will add an `append` flag into `OpWrite`:

``` rust
impl OpWrite {   
    pub fn with_append(mut self, append: bool) -> Self {
        self.append = append;
        self
    }
}
```

All services need to check `append` flag and handle append mode accordingly. Services that not support append should return an `Unsupported` error instead.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2758_merge_append_into_write/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

- `writer` API is more complex with the append mode flag.
- Internal implementation must handle both overwrite and append logic.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2758_merge_append_into_write/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2758_merge_append_into_write/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

Pythonâ€™s file open() supports an `"a"` mode flag to enable append-only writing.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2758_merge_append_into_write/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2758_merge_append_into_write/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

None
