# Module rfc_2852_native_capability Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#190" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Native capability

- Proposal Name: `native_capability`
- Start Date: 2023-08-11
- RFC PR: [apache/opendal#2852](https://github.com/apache/opendal/pull/2852)
- Tracking Issue: [apache/opendal#2859](https://github.com/apache/opendal/issues/2859)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2852_native_capability/index.html#summary" class="doc-anchor">Â§</a>Summary

Add `native_capability` and `full_capability` to `Operator` so that users can make more informed decisions.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2852_native_capability/index.html#motivation" class="doc-anchor">Â§</a>Motivation

OpenDAL adds `Capability` to inform users whether a service supports a specific feature. However, this is not enough for users to make decisions. OpenDAL doesnâ€™t simply expose the servicesâ€™ API directly; instead, it simulates the behavior to make it more useful.

For example, `s3` doesnâ€™t support seek operations like a local file system. But itâ€™s a quite common operation for users. So OpenDAL will try to simulate the behavior by calculating the correct offset and reading the data from that offset instead. After this simulation, the `s3` service has the `read_can_seek` capability now.

As another example, most services like `s3` donâ€™t support blocking operations. OpenDAL implements a `BlockingLayer` to make it possible. After this implementation, the `s3` service has the `blocking` capability now.

However, these capabilities alone are insufficient for users to make informed decisions. Take the `s3` serviceâ€™s `blocking` capability as an example. Users are unable to determine whether it is a native capability or not, which may result in them unknowingly utilizing this feature in performance-sensitive scenarios, leading to significantly poor performance.

So this proposal intends to address this issue by adding `native_capability` and `full_capability` to `OperatorInfo`. Users can use `native_capability` to determine whether a capability is native or not.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2852_native_capability/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

We will add two new APIs `native_capability()` and `full_capability()` in `OperatorInfo`, and remove the `capability()` and related `can_xxx()` API.

``` diff
+ pub fn native_capability(&self) -> Capability
+ pub fn full_capability(&self) -> Capability
- pub fn capability(&self) -> Capability
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2852_native_capability/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

We will add two new fields `native_capability` and `full_capability` in `AccessorInfo`:

- Services SHOULD only set `native_capability`, and `full_capability` will be the same as `native_capability`.
- Layers MAY change `full_capability` and MUST NOT modify `native_capability`.
- `OperatorInfo` should forward `native_capability()` and `full_capability()` to `AccessorInfo`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2852_native_capability/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2852_native_capability/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2852_native_capability/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2852_native_capability/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2852_native_capability/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

None
