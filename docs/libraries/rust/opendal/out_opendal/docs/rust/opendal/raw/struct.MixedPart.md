# Struct MixedPart Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/multipart.rs.html#254-268" class="src">Source</a>

``` rust
pub struct MixedPart { /* private fields */ }
```

Expand description

MixedPart is a builder for multipart/mixed part.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#impl-MixedPart" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html" class="struct" title="struct opendal::raw::MixedPart">MixedPart</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#method.new" class="fn">new</a>(uri: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Create a new mixed part with given uri.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#method.from_request" class="fn">from_request</a>(req: Request\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> Self

Build a mixed part from a request.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#method.into_response" class="fn">into_response</a>(self) -\> Response\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Consume a mixed part to build a response.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#method.part_header" class="fn">part_header</a>(self, key: HeaderName, value: HeaderValue) -\> Self

Insert a part header into part.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#method.method" class="fn">method</a>(self, method: Method) -\> Self

Set the method for request in this part.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#method.version" class="fn">version</a>(self, version: Version) -\> Self

Set the version for request in this part.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#method.header" class="fn">header</a>(self, key: HeaderName, value: HeaderValue) -\> Self

Insert a header into part.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#method.content" class="fn">content</a>(self, content: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> Self

Set the content for this part.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#impl-Part-for-MixedPart" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html" class="trait" title="trait opendal::raw::Part">Part</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html" class="struct" title="struct opendal::raw::MixedPart">MixedPart</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#method.parse" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#tymethod.parse" class="fn">parse</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

TODO

This is a simple implementation and have a lot of space to improve.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#associatedconstant.TYPE" class="anchor">Â§</a>

#### const <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE" class="constant">TYPE</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "mixed"

TYPE is the type of multipart. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#method.format" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#tymethod.format" class="fn">format</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#" class="tooltip" data-notable-ty="Buffer">â“˜</a>

format will generates the bytes.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html#blanket-implementations" class="anchor">Â§</a>
