# Struct ReadOptions Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/options.rs.html#69-159" class="src">Source</a>

``` rust
pub struct ReadOptions {
    pub range: BytesRange,
    pub version: Option<String>,
    pub if_match: Option<String>,
    pub if_none_match: Option<String>,
    pub if_modified_since: Option<Timestamp>,
    pub if_unmodified_since: Option<Timestamp>,
    pub concurrent: usize,
    pub chunk: Option<usize>,
    pub gap: Option<usize>,
    pub override_content_type: Option<String>,
    pub override_cache_control: Option<String>,
    pub override_content_disposition: Option<String>,
}
```

Expand description

Options for read operations.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.range" class="anchor field">Â§</a>`range: `<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange"><code>BytesRange</code></a>

Set `range` for this operation.

If we have a file with size `n`.

- `..` means read bytes in range `[0, n)` of file.
- `0..1024` and `..1024` means read bytes in range `[0, 1024)` of file
- `1024..` means read bytes in range `[1024, n)` of file

The type implements `From<RangeBounds<u64>>`, so users can use `(1024..).into()` instead.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.version" class="anchor field">Â§</a>`version: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Set `version` for this operation.

This option can be used to retrieve the data of a specified version of the given path.

If the version doesnâ€™t exist, an error with kind \[`ErrorKind::NotFound`\] will be returned.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.if_match" class="anchor field">Â§</a>`if_match: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Set `if_match` for this operation.

This option can be used to check if the fileâ€™s `ETag` matches the given `ETag`.

If file exists and itâ€™s etag doesnâ€™t match, an error with kind \[`ErrorKind::ConditionNotMatch`\] will be returned.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.if_none_match" class="anchor field">Â§</a>`if_none_match: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Set `if_none_match` for this operation.

This option can be used to check if the fileâ€™s `ETag` doesnâ€™t match the given `ETag`.

If file exists and itâ€™s etag match, an error with kind \[`ErrorKind::ConditionNotMatch`\] will be returned.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.if_modified_since" class="anchor field">Â§</a>`if_modified_since: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp"><code>Timestamp</code></a>`>`

Set `if_modified_since` for this operation.

This option can be used to check if the file has been modified since the given timestamp.

If file exists and it hasnâ€™t been modified since the specified time, an error with kind \[`ErrorKind::ConditionNotMatch`\] will be returned.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.if_unmodified_since" class="anchor field">Â§</a>`if_unmodified_since: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp"><code>Timestamp</code></a>`>`

Set `if_unmodified_since` for this operation.

This feature can be used to check if the file hasnâ€™t been modified since the given timestamp.

If file exists and it has been modified since the specified time, an error with kind \[`ErrorKind::ConditionNotMatch`\] will be returned.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.concurrent" class="anchor field">Â§</a>`concurrent: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Set `concurrent` for the operation.

OpenDAL by default to read file without concurrent. This is not efficient for cases when users read large chunks of data. By setting `concurrent`, opendal will reading files concurrently on support storage services.

By setting `concurrent`, opendal will fetch chunks concurrently with the give chunk size.

Refer to [`crate::docs::performance`](https://opendal.apache.org/docs/rust/opendal/docs/performance/index.html "mod opendal::docs::performance") for more details.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.chunk" class="anchor field">Â§</a>`chunk: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Set `chunk` for the operation.

OpenDAL will use servicesâ€™ preferred chunk size by default. Users can set chunk based on their own needs.

Refer to [`crate::docs::performance`](https://opendal.apache.org/docs/rust/opendal/docs/performance/index.html "mod opendal::docs::performance") for more details.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.gap" class="anchor field">Â§</a>`gap: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Controls the optimization strategy for range reads in \[`Reader::fetch`\].

When performing range reads, if the gap between two requested ranges is smaller than the configured `gap` size, OpenDAL will merge these ranges into a single read request and discard the unrequested data in between. This helps reduce the number of API calls to remote storage services.

This optimization is particularly useful when performing multiple small range reads that are close to each other, as it reduces the overhead of multiple network requests at the cost of transferring some additional data.

Refer to [`crate::docs::performance`](https://opendal.apache.org/docs/rust/opendal/docs/performance/index.html "mod opendal::docs::performance") for more details.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.override_content_type" class="anchor field">Â§</a>`override_content_type: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Specify the content-type header that should be sent back by the operation.

This option is only meaningful when used along with presign.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.override_cache_control" class="anchor field">Â§</a>`override_cache_control: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Specify the `cache-control` header that should be sent back by the operation.

This option is only meaningful when used along with presign.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#structfield.override_content_disposition" class="anchor field">Â§</a>`override_content_disposition: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Specify the `content-disposition` header that should be sent back by the operation.

This option is only meaningful when used along with presign.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#impl-Clone-for-ReadOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#impl-Debug-for-ReadOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#impl-Default-for-ReadOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#impl-From%3CReadOptions%3E-for-(OpRead,+OpReader)" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>\> for (<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>, <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html" class="struct" title="struct opendal::raw::OpReader">OpReader</a>)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#impl-PartialEq-for-ReadOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#impl-Eq-for-ReadOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#impl-StructuralPartialEq-for-ReadOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html#blanket-implementations" class="anchor">Â§</a>
