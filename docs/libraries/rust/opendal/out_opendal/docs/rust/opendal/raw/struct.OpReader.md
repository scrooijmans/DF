# Struct OpReader Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/ops.rs.html#433-442" class="src">Source</a>

``` rust
pub struct OpReader { /* private fields */ }
```

Expand description

Args for reader operation.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#impl-OpReader" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html" class="struct" title="struct opendal::raw::OpReader">OpReader</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.new" class="fn">new</a>() -\> Self

Create a new `OpReader`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.with_concurrent" class="fn">with_concurrent</a>(self, concurrent: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set the concurrent of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.concurrent" class="fn">concurrent</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get concurrent from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.with_chunk" class="fn">with_chunk</a>(self, chunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set the chunk of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.chunk" class="fn">chunk</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Get chunk from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.with_gap" class="fn">with_gap</a>(self, gap: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set the gap of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.gap" class="fn">gap</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Get gap from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.with_prefetch" class="fn">with_prefetch</a>(self, prefetch: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set the prefetch of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.prefetch" class="fn">prefetch</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get prefetch from option

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#impl-Clone-for-OpReader" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html" class="struct" title="struct opendal::raw::OpReader">OpReader</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html" class="struct" title="struct opendal::raw::OpReader">OpReader</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#impl-Debug-for-OpReader" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html" class="struct" title="struct opendal::raw::OpReader">OpReader</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#impl-Default-for-OpReader" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html" class="struct" title="struct opendal::raw::OpReader">OpReader</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html#blanket-implementations" class="anchor">Â§</a>
