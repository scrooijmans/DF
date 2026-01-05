# Struct PooledBuf Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/buf/pooled_buf.rs.html#31-35" class="src">Source</a>

``` rust
pub struct PooledBuf { /* private fields */ }
```

Expand description

PooledBuf is a buffer pool that designed for reusing already allocated bufs.

It works as best-effort that tries to reuse the buffer if possible. It wonâ€™t block the thread if the pool is locked, just returning a new buffer or dropping existing buffer.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html#impl-PooledBuf" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html" class="struct" title="struct opendal::raw::oio::PooledBuf">PooledBuf</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html#method.new" class="fn">new</a>(size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Create a new buffer pool with a given size.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html#method.with_initial_capacity" class="fn">with_initial_capacity</a>(self, initial_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set the initial capacity of the buffer.

The default value is 0.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html#method.get" class="fn">get</a>(&self) -\> BytesMut

Get a \[`BytesMut`\] from the pool.

Itâ€™s guaranteed that the buffer is empty.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html#method.put" class="fn">put</a>(&self, buf: BytesMut)

Put a \[`BytesMut`\] back to the pool.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html#impl-Debug-for-PooledBuf" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html" class="struct" title="struct opendal::raw::oio::PooledBuf">PooledBuf</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html#blanket-implementations" class="anchor">Â§</a>
