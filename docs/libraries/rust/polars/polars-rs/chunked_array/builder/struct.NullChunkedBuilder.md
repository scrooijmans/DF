# Struct NullChunkedBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/builder/null.rs.html#7" class="src">Source</a>

``` rust
pub struct NullChunkedBuilder { /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#impl-NullChunkedBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::NullChunkedBuilder">NullChunkedBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#method.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::NullChunkedBuilder">NullChunkedBuilder</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Appends a null slot into the builder

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#impl-Clone-for-NullChunkedBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::NullChunkedBuilder">NullChunkedBuilder</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::NullChunkedBuilder">NullChunkedBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html#blanket-implementations" class="anchor">§</a>
