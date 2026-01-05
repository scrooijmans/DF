# Struct OpWriter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/ops.rs.html#824-826" class="src">Source</a>

``` rust
pub struct OpWriter { /* private fields */ }
```

Expand description

Args for `writer` operation.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#impl-OpWriter" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html" class="struct" title="struct opendal::raw::OpWriter">OpWriter</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#method.new" class="fn">new</a>() -\> Self

Create a new `OpWriter`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#method.chunk" class="fn">chunk</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Get the chunk from op.

The chunk is used by service to decide the chunk size of the underlying writer.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#method.with_chunk" class="fn">with_chunk</a>(self, chunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set the chunk of op.

If chunk is set, the data will be chunked by the underlying writer.

###### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#note" class="doc-anchor">Â§</a>NOTE

Service could have their own minimum chunk size while perform write operations like multipart uploads. So the chunk size may be larger than the given buffer size.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#impl-Clone-for-OpWriter" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html" class="struct" title="struct opendal::raw::OpWriter">OpWriter</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html" class="struct" title="struct opendal::raw::OpWriter">OpWriter</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#impl-Debug-for-OpWriter" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html" class="struct" title="struct opendal::raw::OpWriter">OpWriter</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#impl-Default-for-OpWriter" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html" class="struct" title="struct opendal::raw::OpWriter">OpWriter</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html" class="struct" title="struct opendal::raw::OpWriter">OpWriter</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html#blanket-implementations" class="anchor">Â§</a>
