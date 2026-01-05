# Module rfc_2299_chain_based_operator_api Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#170" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Chain based operator API

- Proposal Name: `chain_based_operator_api`
- Start Date: 2023-05-23
- RFC PR: [apache/opendal#2299](https://github.com/apache/opendal/pull/2299)
- Tracking Issue: [apache/opendal#2300](https://github.com/apache/opendal/issues/2300)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html#summary" class="doc-anchor">Â§</a>Summary

Add chain based Operator API to replace `OpXxx`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html#motivation" class="doc-anchor">Â§</a>Motivation

OpenDAL provides `xxx_with` API for users to add more options for requests:

``` rust
let bs = op.read_with("path/to/file", OpRead::new().with_range(0..=1024)).await?;
```

However, the APIâ€™s usability is hindered as users are required to create a new `OpXxx` struct. The API call can be excessively verbose:

``` rust
let bs = op.read_with(
  "path/to/file",
  OpRead::new()
    .with_range(0..=1024)
    .with_if_match("<etag>")
    .with_if_none_match("<etag>")
    .with_override_cache_control("<cache_control>")
    .with_override_content_disposition("<content_disposition>")
  ).await?;
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

In this proposal, I plan to introduce chain based `Operator` API to make them more friendly to use:

``` rust
let bs = op.read_with("path/to/file")
  .range(0..=1024)
  .if_match("<etag>")
  .if_none_match("<etag>")
  .override_cache_control("<cache_control>")
  .override_content_disposition("<content_disposition>")
  .await?;
```

By eliminating the usage of `OpXxx`, our users can write code that is more readable.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

To implement chain based API, we will change `read_with` as following:

``` diff
- pub async fn read_with(&self, path: &str, args: OpRead) -> Result<Vec<u8>>
+ pub fn read_with(&self, path: &str) -> FutureRead
```

`FutureRead` will implement `Future<Output=Result<Vec<u8>>>`, so that users can still call `read_with` like the following:

``` rust
let bs = op.read_with("path/to/file").await?;
```

For blocking operations, we will change `read_with` as following:

``` diff
- pub fn read_with(&self, path: &str, args: OpRead) -> Result<Vec<u8>>
+ pub fn read_with(&self, path: &str) -> FunctionRead
```

`FunctionRead` will implement `call(self) -> Result<Vec<u8>>`, so that users can call `read_with` like the following:

``` rust
let bs = op.read_with("path/to/file").call()?;
```

After this change, all `OpXxx` will be moved as raw API.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html#change-api-after-fn_traits-stabilized" class="doc-anchor">Â§</a>Change API after fn_traits stabilized

After [fn_traits](https://github.com/rust-lang/rust/issues/29625) get stabilized, we will implement `FnOnce` for `FunctionXxx` instead of `call`.
