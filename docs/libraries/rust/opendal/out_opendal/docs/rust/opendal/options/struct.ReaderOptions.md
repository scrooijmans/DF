# Struct ReaderOptions Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/options.rs.html#163-244" class="src">Source</a>

``` rust
pub struct ReaderOptions {
    pub version: Option<String>,
    pub if_match: Option<String>,
    pub if_none_match: Option<String>,
    pub if_modified_since: Option<Timestamp>,
    pub if_unmodified_since: Option<Timestamp>,
    pub concurrent: usize,
    pub chunk: Option<usize>,
    pub gap: Option<usize>,
    pub prefetch: usize,
}
```

Expand description

Options for reader operations.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#structfield.version" class="anchor field">Â§</a>`version: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Set `version` for this operation.

This option can be used to retrieve the data of a specified version of the given path.

If the version doesnâ€™t exist, an error with kind \[`ErrorKind::NotFound`\] will be returned.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#structfield.if_match" class="anchor field">Â§</a>`if_match: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Set `if_match` for this operation.

This option can be used to check if the fileâ€™s `ETag` matches the given `ETag`.

If file exists and itâ€™s etag doesnâ€™t match, an error with kind \[`ErrorKind::ConditionNotMatch`\] will be returned.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#structfield.if_none_match" class="anchor field">Â§</a>`if_none_match: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Set `if_none_match` for this operation.

This option can be used to check if the fileâ€™s `ETag` doesnâ€™t match the given `ETag`.

If file exists and itâ€™s etag match, an error with kind \[`ErrorKind::ConditionNotMatch`\] will be returned.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#structfield.if_modified_since" class="anchor field">Â§</a>`if_modified_since: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp"><code>Timestamp</code></a>`>`

Set `if_modified_since` for this operation.

This option can be used to check if the file has been modified since the given timestamp.

If file exists and it hasnâ€™t been modified since the specified time, an error with kind \[`ErrorKind::ConditionNotMatch`\] will be returned.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#structfield.if_unmodified_since" class="anchor field">Â§</a>`if_unmodified_since: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp"><code>Timestamp</code></a>`>`

Set `if_unmodified_since` for this operation.

This feature can be used to check if the file hasnâ€™t been modified since the given timestamp.

If file exists and it has been modified since the specified time, an error with kind \[`ErrorKind::ConditionNotMatch`\] will be returned.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#structfield.concurrent" class="anchor field">Â§</a>`concurrent: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Set `concurrent` for the operation.

OpenDAL by default to read file without concurrent. This is not efficient for cases when users read large chunks of data. By setting `concurrent`, opendal will reading files concurrently on support storage services.

By setting `concurrent`, opendal will fetch chunks concurrently with the give chunk size.

Refer to [`crate::docs::performance`](https://opendal.apache.org/docs/rust/opendal/docs/performance/index.html "mod opendal::docs::performance") for more details.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#structfield.chunk" class="anchor field">Â§</a>`chunk: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Set `chunk` for the operation.

OpenDAL will use servicesâ€™ preferred chunk size by default. Users can set chunk based on their own needs.

Refer to [`crate::docs::performance`](https://opendal.apache.org/docs/rust/opendal/docs/performance/index.html "mod opendal::docs::performance") for more details.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#structfield.gap" class="anchor field">Â§</a>`gap: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Controls the optimization strategy for range reads in \[`Reader::fetch`\].

When performing range reads, if the gap between two requested ranges is smaller than the configured `gap` size, OpenDAL will merge these ranges into a single read request and discard the unrequested data in between. This helps reduce the number of API calls to remote storage services.

This optimization is particularly useful when performing multiple small range reads that are close to each other, as it reduces the overhead of multiple network requests at the cost of transferring some additional data.

Refer to [`crate::docs::performance`](https://opendal.apache.org/docs/rust/opendal/docs/performance/index.html "mod opendal::docs::performance") for more details.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#structfield.prefetch" class="anchor field">Â§</a>`prefetch: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Controls the number of prefetched bytes ranges that can be buffered in memory during concurrent reading.

When performing concurrent reads with `Reader`, this option limits how many completed-but-not-yet-read chunks can be buffered. Once the number of buffered chunks reaches this limit, no new read tasks will be spawned until some of the buffered chunks are consumed.

- Default value: 0 (no prefetching, strict back-pressure control)
- Set to a higher value to allow more aggressive prefetching at the cost of memory

This option helps prevent memory exhaustion when reading large files with high concurrency settings.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#impl-Clone-for-ReaderOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#impl-Debug-for-ReaderOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#impl-Default-for-ReaderOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#impl-From%3CReaderOptions%3E-for-(OpRead,+OpReader)" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>\> for (<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>, <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html" class="struct" title="struct opendal::raw::OpReader">OpReader</a>)

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#impl-PartialEq-for-ReaderOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#impl-Eq-for-ReaderOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>

<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#impl-StructuralPartialEq-for-ReaderOptions" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html#blanket-implementations" class="anchor">Â§</a>
