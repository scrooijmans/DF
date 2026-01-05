# Struct PresignedRequest Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/rps.rs.html#55-59" class="src">Source</a>

``` rust
pub struct PresignedRequest { /* private fields */ }
```

Expand description

PresignedRequest is a presigned request return by `presign`.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#impl-PresignedRequest" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#method.new" class="fn">new</a>(method: Method, uri: Uri, headers: HeaderMap) -\> Self

Create a new PresignedRequest

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#method.method" class="fn">method</a>(&self) -\> &Method

Return requestâ€™s method.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#method.uri" class="fn">uri</a>(&self) -\> &Uri

Return requestâ€™s uri.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#method.header" class="fn">header</a>(&self) -\> &HeaderMap

Return requestâ€™s header.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#impl-Clone-for-PresignedRequest" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#impl-Debug-for-PresignedRequest" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#impl-From%3CPresignedRequest%3E-for-Request%3CT%3E" class="anchor">Â§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>\> for Request\<T\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html#blanket-implementations" class="anchor">Â§</a>
