# Module rfc_2774_lister_api Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#182" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Lister API

- Proposal Name: `lister_api`
- Start Date: 2023-08-04
- RFC PR: [apache/opendal#2774](https://github.com/apache/opendal/pull/2774)
- Tracking Issue: [apache/opendal#2775](https://github.com/apache/opendal/issues/2775)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2774_lister_api/index.html#summary" class="doc-anchor">Â§</a>Summary

Add `lister` API to align with other OpenDAL APIs like `read`/`reader`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2774_lister_api/index.html#motivation" class="doc-anchor">Â§</a>Motivation

Currently OpenDAL has `list` APIs like:

``` rust
let lister = op.list().await?;
```

This is inconsistent with APIs like `read`/`reader` and can confuse users.

We should add a new `lister` API and change the `list` to:

- Align with other OpenDAL APIs
- Simplify usage

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2774_lister_api/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

The new APIs will be:

``` rust
let entries = op.list().await?; // Get entries directly

let lister = op.lister().await?; // Get lister
```

- `op.list()` returns entries directly.
- `op.lister()` returns a lister that users can list entries on demand.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2774_lister_api/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

We will:

- Rename existing `list` to `lister`
- Add new `list` method to call `lister` and return all entries
- Merge `scan` into `list_with` with `delimiter("")`

This keeps the pagination logic encapsulated in `lister`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2774_lister_api/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2774_lister_api/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2774_lister_api/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2774_lister_api/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2774_lister_api/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

None
