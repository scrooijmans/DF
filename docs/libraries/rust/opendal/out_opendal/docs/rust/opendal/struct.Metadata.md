# Struct Metadata Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/metadata.rs.html#48-66" class="src">Source</a>

``` rust
pub struct Metadata { /* private fields */ }
```

Expand description

Metadata contains all the information related to a specific path.

Depending on the context of the requests, the metadata for the same path may vary. For example, two versions of the same path might have different content lengths. Keep in mind that metadata is always tied to the given context and is not a global state.

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#file-versions" class="doc-anchor">Â§</a>File Versions

In systems that support versioning, such as AWS S3, the metadata may represent a specific version of a file.

Users can access [`Metadata::version`](https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.version "method opendal::Metadata::version") to retrieve the fileâ€™s version, if available. They can also use [`Metadata::is_current`](https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.is_current "method opendal::Metadata::is_current") and [`Metadata::is_deleted`](https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.is_deleted "method opendal::Metadata::is_deleted") to determine whether the metadata corresponds to the latest version or a deleted one.

The all possible combinations of `is_current` and `is_deleted` are as follows:

| `is_current` | `is_deleted` | description |
|----|----|----|
| `Some(true)` | `false` | **The metadataâ€™s associated version is the latest, current version.** This is the normal state, indicating that this version is the most up-to-date and accessible version. |
| `Some(true)` | `true` | **The metadataâ€™s associated version is the latest, deleted version (Latest Delete Marker or Soft Deleted).** This is particularly important in object storage systems like S3. It signifies that this version is the **most recent delete marker**, indicating the object has been deleted. Subsequent GET requests will return 404 errors unless a specific version ID is provided. |
| `Some(false)` | `false` | **The metadataâ€™s associated version is neither the latest version nor deleted.** This indicates that this version is a previous version, still accessible by specifying its version ID. |
| `Some(false)` | `true` | **The metadataâ€™s associated version is not the latest version and is deleted.** This represents a historical version that has been marked for deletion. Users will need to specify the version ID to access it, and accessing it may be subject to specific delete marker behavior (e.g., in S3, it might not return actual data but a specific delete marker response). |
| `None` | `false` | **The metadataâ€™s associated file is not deleted, but its version status is either unknown or it is not the latest version.** This likely indicates that versioning is not enabled for this file, or versioning information is unavailable. |
| `None` | `true` | **The metadataâ€™s associated file is deleted, but its version status is either unknown or it is not the latest version.** This typically means the file was deleted without versioning enabled, or its versioning information is unavailable. This may represent an actual data deletion operation rather than an S3 delete marker. |

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#impl-Metadata" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.new" class="fn">new</a>(mode: <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>) -\> Self

Create a new metadata

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.mode" class="fn">mode</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

mode represent this entryâ€™s mode.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_mode" class="fn">set_mode</a>(&mut self, v: <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>) -\> &mut Self

Set mode for entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_mode" class="fn">with_mode</a>(self, v: <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>) -\> Self

Set mode for entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.is_file" class="fn">is_file</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if this metadata is for a file.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.is_dir" class="fn">is_dir</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if this metadata is for a directory.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.is_current" class="fn">is_current</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Checks whether the metadata corresponds to the most recent version of the file.

This function is particularly useful when working with versioned objects, such as those stored in systems like AWS S3 with versioning enabled. It helps determine if the retrieved metadata represents the current state of the file or an older version.

Refer to docs in [`Metadata`](https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html "struct opendal::Metadata") for more information about file versions.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#return-value" class="doc-anchor">Â§</a>Return Value

The function returns an `Option<bool>` which can have the following values:

- `Some(true)`: Indicates that the metadata **is** associated with the latest version of the file. The metadata is current and reflects the most up-to-date state.
- `Some(false)`: Indicates that the metadata **is not** associated with the latest version of the file. The metadata belongs to an older version, and there might be a more recent version available.
- `None`: Indicates that the currency of the metadata **cannot be determined**. This might occur if versioning is not supported or enabled, or if there is insufficient information to ascertain the version status.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_is_current" class="fn">set_is_current</a>(&mut self, is_current: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> &mut Self

Set the `is_current` status of this entry.

By default, this value will be `None`. Please avoid using this API if itâ€™s unclear whether the entry is current. Set it to `true` if it is known to be the latest; otherwise, set it to `false`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_is_current" class="fn">with_is_current</a>(self, is_current: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>) -\> Self

Set the `is_current` status of this entry.

By default, this value will be `None`. Please avoid using this API if itâ€™s unclear whether the entry is current. Set it to `true` if it is known to be the latest; otherwise, set it to `false`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.is_deleted" class="fn">is_deleted</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks if the file (or version) associated with this metadata has been deleted.

This function returns `true` if the file represented by this metadata has been marked for deletion or has been permanently deleted. It returns `false` otherwise, indicating that the file (or version) is still present and accessible.

Refer to docs in [`Metadata`](https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html "struct opendal::Metadata") for more information about file versions.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#returns" class="doc-anchor">Â§</a>Returns

`bool`: `true` if the object is considered deleted, `false` otherwise.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_is_deleted" class="fn">set_is_deleted</a>(&mut self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> &mut Self

Set the deleted status of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_is_deleted" class="fn">with_is_deleted</a>(self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set the deleted status of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.cache_control" class="fn">cache_control</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Cache control of this entry.

Cache-Control is defined by [RFC 7234](https://httpwg.org/specs/rfc7234.html#header.cache-control) Refer to [MDN Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) for more information.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_cache_control" class="fn">set_cache_control</a>(&mut self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &mut Self

Set cache control of this entry.

Cache-Control is defined by [RFC 7234](https://httpwg.org/specs/rfc7234.html#header.cache-control) Refer to [MDN Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) for more information.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_cache_control" class="fn">with_cache_control</a>(self, v: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set cache control of this entry.

Cache-Control is defined by [RFC 7234](https://httpwg.org/specs/rfc7234.html#header.cache-control) Refer to [MDN Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) for more information.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.content_length" class="fn">content_length</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Content length of this entry.

`Content-Length` is defined by [RFC 7230](https://httpwg.org/specs/rfc7230.html#header.content-length)

Refer to [MDN Content-Length](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Length) for more information.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#returns-1" class="doc-anchor">Â§</a>Returns

Content length of this entry. It will be `0` if the content length is not set by the storage services.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_content_length" class="fn">set_content_length</a>(&mut self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> &mut Self

Set content length of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_content_length" class="fn">with_content_length</a>(self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> Self

Set content length of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.content_md5" class="fn">content_md5</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Content MD5 of this entry.

Content MD5 is defined by [RFC 2616](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html). And removed by [RFC 7231](https://www.rfc-editor.org/rfc/rfc7231).

OpenDAL will try its best to set this value, but not guarantee this value is the md5 of content.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_content_md5" class="fn">set_content_md5</a>(&mut self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &mut Self

Set content MD5 of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_content_md5" class="fn">with_content_md5</a>(self, v: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set content MD5 of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.content_type" class="fn">content_type</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Content Type of this entry.

Content Type is defined by [RFC 9110](https://httpwg.org/specs/rfc9110.html#field.content-type).

Refer to [MDN Content-Type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type) for more information.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_content_type" class="fn">set_content_type</a>(&mut self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &mut Self

Set Content Type of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_content_type" class="fn">with_content_type</a>(self, v: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set Content Type of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.content_encoding" class="fn">content_encoding</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Content Encoding of this entry.

Content Encoding is defined by [RFC 7231](https://httpwg.org/specs/rfc7231.html#header.content-encoding)

Refer to [MDN Content-Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding) for more information.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_content_encoding" class="fn">set_content_encoding</a>(&mut self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &mut Self

Set Content Encoding of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.content_range" class="fn">content_range</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>\>

Content Range of this entry.

Content Range is defined by [RFC 9110](https://httpwg.org/specs/rfc9110.html#field.content-range).

Refer to [MDN Content-Range](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Range) for more information.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_content_range" class="fn">set_content_range</a>(&mut self, v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>) -\> &mut Self

Set Content Range of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_content_range" class="fn">with_content_range</a>(self, v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>) -\> Self

Set Content Range of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.last_modified" class="fn">last_modified</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>\>

Last modified of this entry.

`Last-Modified` is defined by [RFC 7232](https://httpwg.org/specs/rfc7232.html#header.last-modified)

Refer to [MDN Last-Modified](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Last-Modified) for more information.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_last_modified" class="fn">set_last_modified</a>(&mut self, v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>) -\> &mut Self

Set Last modified of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_last_modified" class="fn">with_last_modified</a>(self, v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>) -\> Self

Set Last modified of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.etag" class="fn">etag</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

ETag of this entry.

`ETag` is defined by [RFC 7232](https://httpwg.org/specs/rfc7232.html#header.etag)

Refer to [MDN ETag](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag) for more information.

OpenDAL will return this value AS-IS like the following:

- `"33a64df551425fcc55e4d42a148795d9f25f89d4"`
- `W/"0815"`

`"` is part of etag.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_etag" class="fn">set_etag</a>(&mut self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &mut Self

Set ETag of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_etag" class="fn">with_etag</a>(self, v: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set ETag of this entry.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.content_disposition" class="fn">content_disposition</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Content-Disposition of this entry

`Content-Disposition` is defined by [RFC 2616](https://www.rfc-editor/rfcs/2616) and clarified usage in [RFC 6266](https://www.rfc-editor/6266).

Refer to [MDN Content-Disposition](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition) for more information.

OpenDAL will return this value AS-IS like the following:

- â€œinlineâ€?
- â€œattachmentâ€?
- â€œattachment; filename="filename.jpg"â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_content_disposition" class="fn">set_content_disposition</a>(&mut self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &mut Self

Set Content-Disposition of this entry

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_content_disposition" class="fn">with_content_disposition</a>(self, v: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set Content-Disposition of this entry

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.version" class="fn">version</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Retrieves the `version` of the file, if available.

The version is typically used in systems that support object versioning, such as AWS S3.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#returns-2" class="doc-anchor">Â§</a>Returns

- `Some(&str)`: If the file has a version associated with it, this function returns `Some` containing a reference to the version ID string.
- `None`: If the file does not have a version, or if versioning is not supported or enabled for the underlying storage system, this function returns `None`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.set_version" class="fn">set_version</a>(&mut self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &mut Self

Set the version of the file

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_version" class="fn">with_version</a>(self, v: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

With the version of the file.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.user_metadata" class="fn">user_metadata</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>

User defined metadata of this entry

The prefix of the user defined metadata key(for example: in oss, itâ€™s x-oss-meta-) is remove from the key

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.with_user_metadata" class="fn">with_user_metadata</a>(self, data: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

With user defined metadata of this entry

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#impl-Clone-for-Metadata" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#impl-Debug-for-Metadata" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#impl-Default-for-Metadata" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#impl-From%3CMetaData%3E-for-Metadata" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<MetaData\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>

Available on **crate feature `services-sftp`** only.

REMOVE ME: we should not implement `From<SftpMeta> for Metadata`.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(meta: SftpMeta) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#impl-PartialEq-for-Metadata" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#impl-Eq-for-Metadata" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#impl-StructuralPartialEq-for-Metadata" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html#blanket-implementations" class="anchor">Â§</a>
