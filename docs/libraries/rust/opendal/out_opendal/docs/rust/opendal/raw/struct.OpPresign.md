# Struct OpPresign Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/ops.rs.html#236-240" class="src">Source</a>

``` rust
pub struct OpPresign { /* private fields */ }
```

Expand description

Args for `presign` operation.

The path must be normalized.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#impl-OpPresign" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#method.new" class="fn">new</a>(op: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>\>, expire: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Create a new `OpPresign`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#method.operation" class="fn">operation</a>(&self) -\> &<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>

Get operation from op.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#method.expire" class="fn">expire</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>

Get expire from op.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#method.into_parts" class="fn">into_parts</a>(self) -\> (<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>)

Consume OpPresign into (Duration, PresignOperation)

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#impl-Clone-for-OpPresign" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#impl-Debug-for-OpPresign" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html#blanket-implementations" class="anchor">Â§</a>
