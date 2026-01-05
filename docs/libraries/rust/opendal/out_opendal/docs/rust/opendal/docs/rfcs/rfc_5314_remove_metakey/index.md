# Module rfc_5314_remove_metakey Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#242" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Remove metakey

- Proposal Name: `remove_metakey`
- Start Date: 2024-11-12
- RFC PR: [apache/opendal#5313](https://github.com/apache/opendal/pull/5313)
- Tracking Issue: [apache/opendal#5314](https://github.com/apache/opendal/issues/5314)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5314_remove_metakey/index.html#summary" class="doc-anchor">Â§</a>Summary

Remove the `Metakey` concept from OpenDAL and replace it with a simpler and more predictable metadata handling mechanism.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5314_remove_metakey/index.html#motivation" class="doc-anchor">Â§</a>Motivation

The current `Metakey` design has several issues:

1.  Performance Impact: Users often initiate costly operations unintentionally, such as using `Metakey::Full`, which results in extra stat calls
2.  Usability Issues: Users often try to access metadata that hasnâ€™t been explicitly requested
3.  API Confusion: Thereâ€™s a conflict between `Metakey::Version` and the new `version(bool)` parameter
4.  Implementation Complexity: Service developers struggle to implement `Metakey` correctly

The goal is a simpler, more intuitive API that prevents common mistakes and improves performance as standard.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5314_remove_metakey/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

Instead of using `Metakey` to specify which metadata fields to fetch, services will now declare their metadata capabilities upfront through a new `MetadataCapability` struct:

``` rust
let entries = op.list("path").await?;
for entry in entries {
    if op.metadata_capability().content_type {
        println!("Content-Type: {}", entry.metadata().content_type());
    }
}
```

If users need additional metadata not provided by `list`:

``` rust
let entries = op.list("path").await?;
for entry in entries {
    let mut meta = entry.metadata();
    if !op.metadata_capability().etag {
        meta = op.stat(&entry.path()).await?;
    }
    println!("Content-Type: {}", meta.etag());
}
```

For existing OpenDAL users, the main changes are:

- Remove all `metakey()` calls from their code
- Use `metadata_capability()` to check available metadata
- Explicitly call `stat()` when needed

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5314_remove_metakey/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

The implementation involves:

1.  Remove the `Metakey` enum
2.  Add new `MetadataCapability` struct:

``` rust
pub struct MetadataCapability {
    pub content_length: bool,
    pub content_type: bool,
    pub last_modified: bool,
    pub etag: bool,
    pub mode: bool,
    pub version: bool,
    ...
}
```

3.  Add method to Operator to query capabilities:

``` rust
impl Operator {
    pub fn metadata_capability(&self) -> MetadataCapability;
}
```

4.  Modify list operation to avoid implicit stat calls
5.  Update all service implementations to declare their metadata capabilities

Each service implementation will need to:

- Remove `Metakey` handling logic
- Implement `metadata_capability()` to accurately indicate the metadata provided by default
- Ensure list operations return metadata thatâ€™s always available without extra API calls

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5314_remove_metakey/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

- Breaking change for existing users
- Loss of fine-grained control over metadata fetching
- Potential increased API calls if users need multiple metadata fields

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5314_remove_metakey/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

This design is superior because:

- Prevents performance pitfalls by default
- Makes metadata availability explicitly
- Simplifies service implementation
- Provides clearer mental model

Alternatives considered:

1.  Keep `Metakey` but make it more restrictive
2.  Add warnings for potentially costly operations
3.  Make stat calls async/lazy

Not making this change would continue the current issues of performance problems and API misuse.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5314_remove_metakey/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5314_remove_metakey/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5314_remove_metakey/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

- Add metadata prefetching optimization
- Add metadata caching layer
- Support for custom metadata fields
- Automated capability detection
