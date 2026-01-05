# Struct AtomicContentLength Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/atomic_util.rs.html#29" class="src">Source</a>

``` rust
pub struct AtomicContentLength(/* private fields */);
```

Expand description

AtomicContentLength is a wrapper of AtomicU64 that used to store content length.

It provides a way to store and load content length in atomic way, so caller donâ€™t need to use `Mutex` or `RwLock` to protect the content length.

We use value `u64::MAX` to represent unknown size, itâ€™s impossible for us to handle a file that has `u64::MAX` bytes.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#impl-AtomicContentLength" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html" class="struct" title="struct opendal::raw::AtomicContentLength">AtomicContentLength</a>

#### pub const fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#method.new" class="fn">new</a>() -\> Self

Create a new AtomicContentLength.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#method.load" class="fn">load</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Load content length from AtomicU64.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#method.store" class="fn">store</a>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

Store content length to AtomicU64.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#impl-Debug-for-AtomicContentLength" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html" class="struct" title="struct opendal::raw::AtomicContentLength">AtomicContentLength</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#impl-Default-for-AtomicContentLength" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html" class="struct" title="struct opendal::raw::AtomicContentLength">AtomicContentLength</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html#blanket-implementations" class="anchor">Â§</a>
