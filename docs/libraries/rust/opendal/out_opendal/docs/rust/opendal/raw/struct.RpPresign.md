# Struct RpPresign Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/rps.rs.html#37-39" class="src">Source</a>

``` rust
pub struct RpPresign { /* private fields */ }
```

Expand description

Reply for `presign` operation.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#impl-RpPresign" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#method.new" class="fn">new</a>(req: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>) -\> Self

Create a new reply for `presign`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#method.into_presigned_request" class="fn">into_presigned_request</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>

Consume reply to build a presigned request.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#impl-Clone-for-RpPresign" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#impl-Debug-for-RpPresign" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html#blanket-implementations" class="anchor">Â§</a>
