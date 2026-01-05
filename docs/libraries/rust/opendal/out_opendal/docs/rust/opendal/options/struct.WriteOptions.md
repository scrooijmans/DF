# Struct WriteOptions Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/options.rs.html#301-524" class="src">Source</a>

``` rust
pub struct WriteOptions {
    pub append: bool,
    pub cache_control: Option<String>,
    pub content_type: Option<String>,
    pub content_disposition: Option<String>,
    pub content_encoding: Option<String>,
    pub user_metadata: Option<HashMap<String, String>>,
    pub if_match: Option<String>,
    pub if_none_match: Option<String>,
    pub if_not_exists: bool,
    pub concurrent: usize,
    pub chunk: Option<usize>,
}
```

Expand description

Options for write operations.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#structfield.append" class="anchor field">Â§</a>`append: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Sets append mode for this operation.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#capability" class="doc-anchor">Â§</a>Capability

Check \[`Capability::write_can_append`\] before using this option.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#behavior" class="doc-anchor">Â§</a>Behavior

- By default, write operations overwrite existing files
- When append is set to true:
  - New data will be appended to the end of existing file
  - If file doesnâ€™t exist, it will be created
- If not supported, will return an error

This operation allows adding data to existing files instead of overwriting them.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#structfield.cache_control" class="anchor field">Â§</a>`cache_control: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Sets Cache-Control header for this write operation.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#capability-1" class="doc-anchor">Â§</a>Capability

Check \[`Capability::write_with_cache_control`\] before using this feature.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#behavior-1" class="doc-anchor">Â§</a>Behavior

- If supported, sets Cache-Control as system metadata on the target file
- The value should follow HTTP Cache-Control header format
- If not supported, the value will be ignored

This operation allows controlling caching behavior for the written content.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#use-cases" class="doc-anchor">Â§</a>Use Cases

- Setting browser cache duration
- Configuring CDN behavior
- Optimizing content delivery
- Managing cache invalidation

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#references" class="doc-anchor">Â§</a>References

- [MDN Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control)
- [RFC 7234 Section 5.2](https://tools.ietf.org/html/rfc7234#section-5.2)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#structfield.content_type" class="anchor field">Â§</a>`content_type: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Sets `Content-Type` header for this write operation.

#### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#capability-2" class="doc-anchor">Â§</a>Capability

Check \[`Capability::write_with_content_type`\] before using this feature.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#behavior-2" class="doc-anchor">Â§</a>Behavior

- If supported, sets Content-Type as system metadata on the target file
- The value should follow MIME type format (e.g. â€œtext/plainâ€?, â€œimage/jpegâ€?)
- If not supported, the value will be ignored

This operation allows specifying the media type of the content being written.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#structfield.content_disposition" class="anchor field">Â§</a>`content_disposition: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Sets Content-Disposition header for this write request.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#capability-3" class="doc-anchor">Â§</a>Capability

Check \[`Capability::write_with_content_disposition`\] before using this feature.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#behavior-3" class="doc-anchor">Â§</a>Behavior

- If supported, sets Content-Disposition as system metadata on the target file
- The value should follow HTTP Content-Disposition header format
- Common values include:
  - `inline` - Content displayed within browser
  - `attachment` - Content downloaded as file
  - `attachment; filename="example.jpg"` - Downloaded with specified filename
- If not supported, the value will be ignored

This operation allows controlling how the content should be displayed or downloaded.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#structfield.content_encoding" class="anchor field">Â§</a>`content_encoding: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Sets Content-Encoding header for this write request.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#capability-4" class="doc-anchor">Â§</a>Capability

Check \[`Capability::write_with_content_encoding`\] before using this feature.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#behavior-4" class="doc-anchor">Â§</a>Behavior

- If supported, sets Content-Encoding as system metadata on the target file
- The value should follow HTTP Content-Encoding header format
- Common values include:
  - `gzip` - Content encoded using gzip compression
  - `deflate` - Content encoded using deflate compression
  - `br` - Content encoded using Brotli compression
  - `identity` - No encoding applied (default value)
- If not supported, the value will be ignored

This operation allows specifying the encoding applied to the content being written.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#structfield.user_metadata" class="anchor field">Â§</a>`user_metadata: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>>`

Sets user metadata for this write request.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#capability-5" class="doc-anchor">Â§</a>Capability

Check \[`Capability::write_with_user_metadata`\] before using this feature.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#behavior-5" class="doc-anchor">Â§</a>Behavior

- If supported, the user metadata will be attached to the object during write
- Accepts key-value pairs where both key and value are strings
- Keys are case-insensitive in most services
- Services may have limitations for user metadata, for example:
  - Key length is typically limited (e.g., 1024 bytes)
  - Value length is typically limited (e.g., 4096 bytes)
  - Total metadata size might be limited
  - Some characters might be forbidden in keys
- If not supported, the metadata will be ignored

User metadata provides a way to attach custom metadata to objects during write operations. This metadata can be retrieved later when reading the object.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#structfield.if_match" class="anchor field">Â§</a>`if_match: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Sets If-Match header for this write request.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#capability-6" class="doc-anchor">Â§</a>Capability

Check \[`Capability::write_with_if_match`\] before using this feature.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#behavior-6" class="doc-anchor">Â§</a>Behavior

- If supported, the write operation will only succeed if the targetâ€™s ETag matches the specified value
- The value should be a valid ETag string
- Common values include:
  - A specific ETag value like `"686897696a7c876b7e"`
  - `*` - Matches any existing resource
- If not supported, the value will be ignored

This operation provides conditional write functionality based on ETag matching, helping prevent unintended overwrites in concurrent scenarios.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#structfield.if_none_match" class="anchor field">Â§</a>`if_none_match: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Sets If-None-Match header for this write request.

Note: Certain services, like `s3`, support `if_not_exists` but not `if_none_match`. Use `if_not_exists` if you only want to check whether a file exists.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#capability-7" class="doc-anchor">Â§</a>Capability

Check \[`Capability::write_with_if_none_match`\] before using this feature.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#behavior-7" class="doc-anchor">Â§</a>Behavior

- If supported, the write operation will only succeed if the targetâ€™s ETag does not match the specified value
- The value should be a valid ETag string
- Common values include:
  - A specific ETag value like `"686897696a7c876b7e"`
  - `*` - Matches if the resource does not exist
- If not supported, the value will be ignored

This operation provides conditional write functionality based on ETag non-matching, useful for preventing overwriting existing resources or ensuring unique writes.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#structfield.if_not_exists" class="anchor field">Â§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Sets the condition that write operation will succeed only if target does not exist.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#capability-8" class="doc-anchor">Â§</a>Capability

Check \[`Capability::write_with_if_not_exists`\] before using this feature.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#behavior-8" class="doc-anchor">Â§</a>Behavior

- If supported, the write operation will only succeed if the target path does not exist
- Will return error if target already exists
- If not supported, the value will be ignored

This operation provides a way to ensure write operations only create new resources without overwriting existing ones, useful for implementing â€œcreate if not existsâ€? logic.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#structfield.concurrent" class="anchor field">Â§</a>`concurrent: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Sets concurrent write operations for this writer.

#### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#behavior-9" class="doc-anchor">Â§</a>Behavior

- By default, OpenDAL writes files sequentially
- When concurrent is set:
  - Multiple write operations can execute in parallel
  - Write operations return immediately without waiting if tasks space are available
  - Close operation ensures all writes complete in order
  - Memory usage increases with concurrency level
- If not supported, falls back to sequential writes

This feature significantly improves performance when:

- Writing large files
- Network latency is high
- Storage service supports concurrent uploads like multipart uploads

#### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#performance-impact" class="doc-anchor">Â§</a>Performance Impact

Setting appropriate concurrency can:

- Increase write throughput
- Reduce total write time
- Better utilize available bandwidth
- Trade memory for performance

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#structfield.chunk" class="anchor field">Â§</a>`chunk: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Sets chunk size for buffered writes.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#capability-9" class="doc-anchor">Â§</a>Capability

Check \[`Capability::write_multi_min_size`\] and \[`Capability::write_multi_max_size`\] for size limits.

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#behavior-10" class="doc-anchor">Â§</a>Behavior

- By default, OpenDAL sets optimal chunk size based on service capabilities
- When chunk size is set:
  - Data will be buffered until reaching chunk size
  - One API call will be made per chunk
  - Last chunk may be smaller than chunk size
- Important considerations:
  - Some services require minimum chunk sizes (e.g. S3â€™s EntityTooSmall error)
  - Smaller chunks increase API calls and costs
  - Larger chunks increase memory usage, but improve performance and reduce costs

##### <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#performance-impact-1" class="doc-anchor">Â§</a>Performance Impact

Setting appropriate chunk size can:

- Reduce number of API calls
- Improve overall throughput
- Lower operation costs
- Better utilize network bandwidth

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#impl-Clone-for-WriteOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#impl-Debug-for-WriteOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#impl-Default-for-WriteOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#impl-From%3CWriteOptions%3E-for-(OpWrite,+OpWriter)" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>\> for (<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>, <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html" class="struct" title="struct opendal::raw::OpWriter">OpWriter</a>)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#impl-PartialEq-for-WriteOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#impl-Eq-for-WriteOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#impl-StructuralPartialEq-for-WriteOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html#blanket-implementations" class="anchor">Â§</a>
