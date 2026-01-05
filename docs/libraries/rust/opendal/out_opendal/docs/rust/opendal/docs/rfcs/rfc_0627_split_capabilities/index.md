# Module rfc_0627_split_capabilities Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#118" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Split capabilities

- Proposal Name: `split-capabilities`
- Start Date: 2022-09-04
- RFC PR: [apache/opendal#627](https://github.com/apache/opendal/pull/627)
- Tracking Issue: [apache/opendal#628](https://github.com/apache/opendal/issues/628)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html#summary" class="doc-anchor">Â§</a>Summary

Split basic operations into `read`, `write`, and `list` capabilities.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html#motivation" class="doc-anchor">Â§</a>Motivation

In [RFC-0409: Accessor Capabilities](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/0409-accessor-capabilities.md), we introduce the ideas of `Accessor Capabilities`. Services could have different capabilities, and users can check them via:

``` rust
let meta = op.metadata();
let _: bool = meta.can_presign();
let _: bool = meta.can_multipart(); 
```

If users call not supported capabilities, OpenDAL will return [`io::ErrorKind::Unsupported`](https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html#variant.Unsupported) instead.

Along with that RFC, we also introduce an idea about `Basic Operations`: the operations that all services must support, including:

- metadata
- create
- read
- write
- delete
- list

However, not all storage services support them. In our existing services, exception includes:

- HTTP services donâ€™t support `write`, `delete`, and `list`.
- IPFS HTTP gateway doesnâ€™t support `write` and `delete`.
  - NOTE: ipfs has a writable HTTP gateway, but there is no available instance.
- fs could be read-only if mounted as `RO`.
- object storage like `s3` and `gcs` could not have enough permission.
- cache services may not support `list`.

So in this RFC, we want to remove the idea about `Basic Operations` and convert them into different capabilities:

- `read`: `read` and `stat`
- `write`: `write` and `delete`
- `list`: `list`

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

No public API changes.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

This RFC will add three new capabilities:

- `read`: `read` and `stat`
- `write`: `write` and `delete`
- `list`: `list`

After this change, all services must declare the features they support.

Most of this RFCs work is to refactor the tests. This RFC will refactor the behavior tests into several parts based on capabilities.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html#read-only-services" class="doc-anchor">Â§</a>Read-only Services

OpenDAL can implement read-only services after this change:

- HTTP Service
- IPFS HTTP Gateway

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html#add-new-capabilities-with-layers" class="doc-anchor">Â§</a>Add new capabilities with Layers

We can implement a layer that can add `list` capability for underlying storage services. For example, `IndexLayer` for HTTP services.
