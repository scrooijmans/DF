# Struct FlexBuf Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/buf/flex_buf.rs.html#26-33" class="src">Source</a>

``` rust
pub struct FlexBuf { /* private fields */ }
```

Expand description

FlexBuf is a buffer that support frozen bytes and reuse existing allocated memory.

Itâ€™s useful when we want to freeze the buffer and reuse the memory for the next buffer.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#impl-FlexBuf" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html" class="struct" title="struct opendal::raw::oio::FlexBuf">FlexBuf</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#method.new" class="fn">new</a>(cap: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Initializes a new `FlexBuf` with the given capacity.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#method.put" class="fn">put</a>(&mut self, bs: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Put slice into flex buf.

Return 0 means the buffer is frozen.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#method.freeze" class="fn">freeze</a>(&mut self)

Freeze the buffer no matter itâ€™s full or not.

Itâ€™s a no-op if the buffer has already been frozen.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#method.get" class="fn">get</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Bytes\>

Get the frozen buffer.

Return `None` if the buffer is not frozen.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#notes" class="doc-anchor">Â§</a>Notes

This operation did nothing to the buffer. We use `&mut self` just for make the API consistent with other APIs.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#method.advance" class="fn">advance</a>(&mut self, cnt: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#panics" class="doc-anchor">Â§</a>Panics

Panic if the buffer is not frozen.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#method.clean" class="fn">clean</a>(&mut self)

Cleanup the buffer, reset to the initial state.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html#blanket-implementations" class="anchor">Â§</a>
