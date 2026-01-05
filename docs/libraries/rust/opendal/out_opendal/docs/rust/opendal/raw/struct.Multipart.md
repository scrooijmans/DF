# Struct Multipart Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/multipart.rs.html#42-45" class="src">Source</a>

``` rust
pub struct Multipart<T: Part> { /* private fields */ }
```

Expand description

Multipart is a builder for multipart/form-data.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#impl-Multipart%3CT%3E" class="anchor">Â§</a>

### impl\<T: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html" class="trait" title="trait opendal::raw::Part">Part</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html" class="struct" title="struct opendal::raw::Multipart">Multipart</a>\<T\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#method.new" class="fn">new</a>() -\> Self

Create a new multipart with random boundary.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#method.with_boundary" class="fn">with_boundary</a>(self, boundary: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Set the boundary with given string.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#method.part" class="fn">part</a>(self, part: T) -\> Self

Insert a part into multipart.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#method.into_parts" class="fn">into_parts</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>

Into parts.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#method.parse" class="fn">parse</a>(self, bs: Bytes) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Parse a response with multipart body into Multipart.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#method.apply" class="fn">apply</a>(self, builder: Builder) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Request\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\>

Consume the input and generate a request with multipart body.

This function will make sure content_type and content_length set correctly.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#impl-Debug-for-Multipart%3CT%3E" class="anchor">Â§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html" class="trait" title="trait opendal::raw::Part">Part</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html" class="struct" title="struct opendal::raw::Multipart">Multipart</a>\<T\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#impl-Default-for-Multipart%3CT%3E" class="anchor">Â§</a>

### impl\<T: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html" class="trait" title="trait opendal::raw::Part">Part</a>\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html" class="struct" title="struct opendal::raw::Multipart">Multipart</a>\<T\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html#blanket-implementations" class="anchor">Â§</a>
