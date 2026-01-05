# Struct OpRead Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/ops.rs.html#307-317" class="src">Source</a>

``` rust
pub struct OpRead { /* private fields */ }
```

Expand description

Args for `read` operation.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#impl-OpRead" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.new" class="fn">new</a>() -\> Self

Create a default `OpRead` which will read whole content of path.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.with_range" class="fn">with_range</a>(self, range: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>) -\> Self

Set the range of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.range" class="fn">range</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

Get range from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.with_override_content_disposition" class="fn">with_override_content_disposition</a>( self, content_disposition: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> Self

Sets the content-disposition header that should be sent back by the remote read operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.override_content_disposition" class="fn">override_content_disposition</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the content-disposition header that should be sent back by the remote read operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.with_override_cache_control" class="fn">with_override_cache_control</a>(self, cache_control: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Sets the cache-control header that should be sent back by the remote read operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.override_cache_control" class="fn">override_cache_control</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the cache-control header that should be sent back by the remote read operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.with_override_content_type" class="fn">with_override_content_type</a>(self, content_type: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Sets the content-type header that should be sent back by the remote read operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.override_content_type" class="fn">override_content_type</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the content-type header that should be sent back by the remote read operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.with_if_match" class="fn">with_if_match</a>(self, if_match: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Set the If-Match of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.if_match" class="fn">if_match</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get If-Match from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.with_if_none_match" class="fn">with_if_none_match</a>(self, if_none_match: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Set the If-None-Match of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.if_none_match" class="fn">if_none_match</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get If-None-Match from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.with_if_modified_since" class="fn">with_if_modified_since</a>(self, v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>) -\> Self

Set the If-Modified-Since of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.if_modified_since" class="fn">if_modified_since</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>\>

Get If-Modified-Since from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.with_if_unmodified_since" class="fn">with_if_unmodified_since</a>(self, v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>) -\> Self

Set the If-Unmodified-Since of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.if_unmodified_since" class="fn">if_unmodified_since</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>\>

Get If-Unmodified-Since from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.with_version" class="fn">with_version</a>(self, version: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Set the version of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.version" class="fn">version</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get version from option

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#impl-Clone-for-OpRead" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#impl-Debug-for-OpRead" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#impl-Default-for-OpRead" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#impl-From%3COpRead%3E-for-PresignOperation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html#blanket-implementations" class="anchor">Â§</a>
