# Struct FormDataPart Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/multipart.rs.html#168-172" class="src">Source</a>

``` rust
pub struct FormDataPart { /* private fields */ }
```

Expand description

FormDataPart is a builder for multipart/form-data part.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#impl-FormDataPart" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html" class="struct" title="struct opendal::raw::FormDataPart">FormDataPart</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#method.new" class="fn">new</a>(name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Create a new part builder

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#panics" class="doc-anchor">Â§</a>Panics

Input name must be percent encoded.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#method.header" class="fn">header</a>(self, key: HeaderName, value: HeaderValue) -\> Self

Insert a header into part.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#method.content" class="fn">content</a>(self, content: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> Self

Set the content for this part.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#impl-Part-for-FormDataPart" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html" class="trait" title="trait opendal::raw::Part">Part</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html" class="struct" title="struct opendal::raw::FormDataPart">FormDataPart</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#associatedconstant.TYPE" class="anchor">Â§</a>

#### const <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE" class="constant">TYPE</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "form-data"

TYPE is the type of multipart. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#method.format" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#tymethod.format" class="fn">format</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#" class="tooltip" data-notable-ty="Buffer">â“˜</a>

format will generates the bytes.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#method.parse" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#tymethod.parse" class="fn">parse</a>(\_: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

parse will parse the bytes into a part.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html#blanket-implementations" class="anchor">Â§</a>
