# Module rfc_3232_align_list_api Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#202" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Align list API

- Proposal Name: `align_list_api`
- Start Date: 2023-10-07
- RFC PR: [apache/opendal#3232](https://github.com/apache/opendal/pull/3232)
- Tracking Issue: [apache/opendal#3236](https://github.com/apache/opendal/issues/3236)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3232_align_list_api/index.html#summary" class="doc-anchor">Â§</a>Summary

Refactor internal `Page` API to `List` API.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3232_align_list_api/index.html#motivation" class="doc-anchor">Â§</a>Motivation

OpenDALâ€™s `Lister` is implemented by `Page`:

``` rust
#[async_trait]
pub trait Page: Send + Sync + 'static {
    /// Fetch a new page of [`Entry`]
    ///
    /// `Ok(None)` means all pages have been returned. Any following call
    /// to `next` will always get the same result.
    async fn next(&mut self) -> Result<Option<Vec<Entry>>>;
}
```

Each call to `next` will retrieve a page of `Entry` objects. This design is modeled after the `list_object` API used in object storage services. However, this design has several drawbacks:

- Services like `fs`, `hdfs` needs to buffer the whole page in memory before returning it to the caller.
- `Page` is not aligned with `opendal::Lister` make it hard to understand the code.
- `Page` is not aligned with `Read` & `Write` which is poll based.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3232_align_list_api/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

No user-facing changes.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3232_align_list_api/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

We will rename `Page` to `List` and change the API to:

``` rust
pub trait List: Send + Sync + 'static {
    /// Fetch a new [`Entry`]
    ///
    /// `Ok(None)` means all entries have been returned. Any following call
    /// to `next` will always get the same result.
    fn poll_next(&mut self, cx: &mut Context<'_>) -> Result<Option<Entry>>;
}
```

All `page` related code will be replaced by `list`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3232_align_list_api/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

Breaking changes for raw API.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3232_align_list_api/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3232_align_list_api/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3232_align_list_api/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3232_align_list_api/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

None
