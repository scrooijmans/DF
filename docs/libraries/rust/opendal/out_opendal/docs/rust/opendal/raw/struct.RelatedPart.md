# Struct RelatedPart Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/multipart.rs.html#527-531" class="src">Source</a>

``` rust
pub struct RelatedPart { /* private fields */ }
```

Expand description

RelatedPart is a builder for multipart/related part.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#impl-RelatedPart" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html" class="struct" title="struct opendal::raw::RelatedPart">RelatedPart</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#method.new" class="fn">new</a>() -\> Self

Create a new related

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#method.from_request" class="fn">from_request</a>(req: Request\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> Self

Build a mixed part from a request.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#method.into_response" class="fn">into_response</a>(self) -\> Response\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Consume a mixed part to build a response.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#method.header" class="fn">header</a>(self, key: HeaderName, value: HeaderValue) -\> Self

Insert a header into part.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#method.content" class="fn">content</a>(self, content: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> Self

Set the content for this part.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#impl-Default-for-RelatedPart" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html" class="struct" title="struct opendal::raw::RelatedPart">RelatedPart</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#impl-Part-for-RelatedPart" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html" class="trait" title="trait opendal::raw::Part">Part</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html" class="struct" title="struct opendal::raw::RelatedPart">RelatedPart</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#associatedconstant.TYPE" class="anchor">Â§</a>

#### const <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE" class="constant">TYPE</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "related"

TYPE is the type of multipart. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#method.format" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#tymethod.format" class="fn">format</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#" class="tooltip" data-notable-ty="Buffer">â“˜</a>

format will generates the bytes.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#method.parse" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#tymethod.parse" class="fn">parse</a>(\_s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

parse will parse the bytes into a part.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html#blanket-implementations" class="anchor">Â§</a>
