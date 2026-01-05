# Struct RpRead Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/rps.rs.html#102-119" class="src">Source</a>

``` rust
pub struct RpRead { /* private fields */ }
```

Expand description

Reply for `read` operation.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#impl-RpRead" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#method.new" class="fn">new</a>() -\> Self

Create a new reply for `read`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Got the size of the reader returned by this read operation.

- `Some(size)` means the reader has at most size bytes.
- `None` means the reader has unknown size.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#method.with_size" class="fn">with_size</a>(self, size: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> Self

Set the size of the reader returned by this read operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#method.range" class="fn">range</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>\>

Got the range of the reader returned by this read operation.

- `Some(range)` means the reader has content range inside the whole file.
- `None` means the reader has unknown size.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#method.with_range" class="fn">with_range</a>(self, range: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>\>) -\> Self

Set the range of the reader returned by this read operation.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#impl-Clone-for-RpRead" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#impl-Debug-for-RpRead" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#impl-Default-for-RpRead" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html#blanket-implementations" class="anchor">Â§</a>
