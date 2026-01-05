# Struct Capability Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/capability.rs.html#66-186" class="src">Source</a>

``` rust
pub struct Capability {Show 53 fields
    pub stat: bool,
    pub stat_with_if_match: bool,
    pub stat_with_if_none_match: bool,
    pub stat_with_if_modified_since: bool,
    pub stat_with_if_unmodified_since: bool,
    pub stat_with_override_cache_control: bool,
    pub stat_with_override_content_disposition: bool,
    pub stat_with_override_content_type: bool,
    pub stat_with_version: bool,
    pub read: bool,
    pub read_with_if_match: bool,
    pub read_with_if_none_match: bool,
    pub read_with_if_modified_since: bool,
    pub read_with_if_unmodified_since: bool,
    pub read_with_override_cache_control: bool,
    pub read_with_override_content_disposition: bool,
    pub read_with_override_content_type: bool,
    pub read_with_version: bool,
    pub write: bool,
    pub write_can_multi: bool,
    pub write_can_empty: bool,
    pub write_can_append: bool,
    pub write_with_content_type: bool,
    pub write_with_content_disposition: bool,
    pub write_with_content_encoding: bool,
    pub write_with_cache_control: bool,
    pub write_with_if_match: bool,
    pub write_with_if_none_match: bool,
    pub write_with_if_not_exists: bool,
    pub write_with_user_metadata: bool,
    pub write_multi_max_size: Option<usize>,
    pub write_multi_min_size: Option<usize>,
    pub write_total_max_size: Option<usize>,
    pub create_dir: bool,
    pub delete: bool,
    pub delete_with_version: bool,
    pub delete_max_size: Option<usize>,
    pub copy: bool,
    pub copy_with_if_not_exists: bool,
    pub rename: bool,
    pub list: bool,
    pub list_with_limit: bool,
    pub list_with_start_after: bool,
    pub list_with_recursive: bool,
    pub list_with_version: bool,
    pub list_with_versions: bool,
    pub list_with_deleted: bool,
    pub presign: bool,
    pub presign_read: bool,
    pub presign_stat: bool,
    pub presign_write: bool,
    pub presign_delete: bool,
    pub shared: bool,
}
```

Expand description

Capability defines the supported operations and their constraints for a storage Operator.

## <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#overview" class="doc-anchor">Â§</a>Overview

This structure provides a comprehensive description of an Operatorâ€™s capabilities, including:

- Basic operations support (read, write, delete, etc.)
- Advanced operation variants (conditional operations, metadata handling)
- Operational constraints (size limits, batch limitations)

## <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#capability-types" class="doc-anchor">Â§</a>Capability Types

Every operator maintains two capability sets:

1.  [`OperatorInfo::native_capability`](https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#method.native_capability "method opendal::OperatorInfo::native_capability"): Represents operations natively supported by the storage backend.

2.  [`OperatorInfo::full_capability`](https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#method.full_capability "method opendal::OperatorInfo::full_capability"): Represents all available operations, including those implemented through alternative mechanisms.

## <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#implementation-details" class="doc-anchor">Â§</a>Implementation Details

Some operations might be available even when not natively supported by the backend. For example:

- Blocking operations are provided through the BlockingLayer

Developers should:

- Use `full_capability` to determine available operations
- Use `native_capability` to identify optimized operations

## <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#field-naming-conventions" class="doc-anchor">Â§</a>Field Naming Conventions

Fields follow these naming patterns:

- Basic operations: Simple lowercase (e.g., `read`, `write`)
- Compound operations: Underscore-separated (e.g., `presign_read`)
- Variants: Capability description (e.g., `write_can_empty`)
- Parameterized operations: With-style (e.g., `read_with_if_match`)
- Limitations: Constraint description (e.g., `write_multi_max_size`)
- Metadata Results: Returning metadata capabilities (e.g., `stat_has_content_length`)

All capability fields are public and can be accessed directly.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.stat" class="anchor field">Â§</a>`stat: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if the operator supports metadata retrieval operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.stat_with_if_match" class="anchor field">Â§</a>`stat_with_if_match: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if conditional stat operations using If-Match are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.stat_with_if_none_match" class="anchor field">Â§</a>`stat_with_if_none_match: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if conditional stat operations using If-None-Match are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.stat_with_if_modified_since" class="anchor field">Â§</a>`stat_with_if_modified_since: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if conditional stat operations using If-Modified-Since are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.stat_with_if_unmodified_since" class="anchor field">Â§</a>`stat_with_if_unmodified_since: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if conditional stat operations using If-Unmodified-Since are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.stat_with_override_cache_control" class="anchor field">Â§</a>`stat_with_override_cache_control: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if Cache-Control header override is supported during stat operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.stat_with_override_content_disposition" class="anchor field">Â§</a>`stat_with_override_content_disposition: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if Content-Disposition header override is supported during stat operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.stat_with_override_content_type" class="anchor field">Â§</a>`stat_with_override_content_type: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if Content-Type header override is supported during stat operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.stat_with_version" class="anchor field">Â§</a>`stat_with_version: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if versions stat operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.read" class="anchor field">Â§</a>`read: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if the operator supports read operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.read_with_if_match" class="anchor field">Â§</a>`read_with_if_match: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if conditional read operations using If-Match are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.read_with_if_none_match" class="anchor field">Â§</a>`read_with_if_none_match: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if conditional read operations using If-None-Match are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.read_with_if_modified_since" class="anchor field">Â§</a>`read_with_if_modified_since: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if conditional read operations using If-Modified-Since are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.read_with_if_unmodified_since" class="anchor field">Â§</a>`read_with_if_unmodified_since: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if conditional read operations using If-Unmodified-Since are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.read_with_override_cache_control" class="anchor field">Â§</a>`read_with_override_cache_control: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if Cache-Control header override is supported during read operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.read_with_override_content_disposition" class="anchor field">Â§</a>`read_with_override_content_disposition: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if Content-Disposition header override is supported during read operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.read_with_override_content_type" class="anchor field">Â§</a>`read_with_override_content_type: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if Content-Type header override is supported during read operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.read_with_version" class="anchor field">Â§</a>`read_with_version: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if versions read operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write" class="anchor field">Â§</a>`write: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if the operator supports write operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_can_multi" class="anchor field">Â§</a>`write_can_multi: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if multiple write operations can be performed on the same object.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_can_empty" class="anchor field">Â§</a>`write_can_empty: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if writing empty content is supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_can_append" class="anchor field">Â§</a>`write_can_append: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if append operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_with_content_type" class="anchor field">Â§</a>`write_with_content_type: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if Content-Type can be specified during write operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_with_content_disposition" class="anchor field">Â§</a>`write_with_content_disposition: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if Content-Disposition can be specified during write operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_with_content_encoding" class="anchor field">Â§</a>`write_with_content_encoding: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if Content-Encoding can be specified during write operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_with_cache_control" class="anchor field">Â§</a>`write_with_cache_control: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if Cache-Control can be specified during write operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_with_if_match" class="anchor field">Â§</a>`write_with_if_match: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if conditional write operations using If-Match are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_with_if_none_match" class="anchor field">Â§</a>`write_with_if_none_match: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if conditional write operations using If-None-Match are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_with_if_not_exists" class="anchor field">Â§</a>`write_with_if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if write operations can be conditional on object non-existence.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_with_user_metadata" class="anchor field">Â§</a>`write_with_user_metadata: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if custom user metadata can be attached during write operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_multi_max_size" class="anchor field">Â§</a>`write_multi_max_size: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Maximum size supported for multipart uploads. For example, AWS S3 supports up to 5GiB per part in multipart uploads.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_multi_min_size" class="anchor field">Â§</a>`write_multi_min_size: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Minimum size required for multipart uploads (except for the last part). For example, AWS S3 requires at least 5MiB per part.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write_total_max_size" class="anchor field">Â§</a>`write_total_max_size: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Maximum total size supported for write operations. For example, Cloudflare D1 has a 1MB total size limit.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.create_dir" class="anchor field">Â§</a>`create_dir: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if directory creation is supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.delete" class="anchor field">Â§</a>`delete: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if delete operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.delete_with_version" class="anchor field">Â§</a>`delete_with_version: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if versions delete operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.delete_max_size" class="anchor field">Â§</a>`delete_max_size: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Maximum size supported for single delete operations.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.copy" class="anchor field">Â§</a>`copy: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if copy operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.copy_with_if_not_exists" class="anchor field">Â§</a>`copy_with_if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if conditional copy operations with if-not-exists are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.rename" class="anchor field">Â§</a>`rename: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if rename operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.list" class="anchor field">Â§</a>`list: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if list operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.list_with_limit" class="anchor field">Â§</a>`list_with_limit: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if list operations support result limiting.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.list_with_start_after" class="anchor field">Â§</a>`list_with_start_after: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if list operations support continuation from a specific point.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.list_with_recursive" class="anchor field">Â§</a>`list_with_recursive: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if recursive listing is supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.list_with_version" class="anchor field">Â§</a>`list_with_version: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

ðŸ‘ŽDeprecated since 0.51.1: use with_versions instead

Indicates if versions listing is supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.list_with_versions" class="anchor field">Â§</a>`list_with_versions: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if listing with versions included is supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.list_with_deleted" class="anchor field">Â§</a>`list_with_deleted: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if listing with deleted files included is supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.presign" class="anchor field">Â§</a>`presign: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if presigned URL generation is supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.presign_read" class="anchor field">Â§</a>`presign_read: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if presigned URLs for read operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.presign_stat" class="anchor field">Â§</a>`presign_stat: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if presigned URLs for stat operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.presign_write" class="anchor field">Â§</a>`presign_write: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if presigned URLs for write operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.presign_delete" class="anchor field">Â§</a>`presign_delete: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates if presigned URLs for delete operations are supported.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.shared" class="anchor field">Â§</a>`shared: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicate if the operator supports shared access.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#impl-Clone-for-Capability" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#impl-Debug-for-Capability" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#impl-Default-for-Capability" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#impl-Copy-for-Capability" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#blanket-implementations" class="anchor">Â§</a>
