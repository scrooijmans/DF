# Struct QueueBuf Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/buf/queue_buf.rs.html#36" class="src">Source</a>

``` rust
pub struct QueueBuf(/* private fields */);
```

Expand description

QueueBuf is a queue of [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer").

Itâ€™s designed to allow storing multiple buffers without copying underlying bytes and consume them in order.

QueueBuf mainly provides the following operations:

- `push`: Push a new buffer in the queue.
- `collect`: Collect all buffer in the queue as a new [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer")
- `advance`: Advance the queue by `cnt` bytes.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#impl-QueueBuf" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html" class="struct" title="struct opendal::raw::oio::QueueBuf">QueueBuf</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#method.new" class="fn">new</a>() -\> Self

Create a new buffer queue.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#method.push" class="fn">push</a>(&mut self, buf: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>)

Push new [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer") into the queue.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Total bytes size inside the buffer queue.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Is the buffer queue empty.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#method.take" class="fn">take</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html" class="struct" title="struct opendal::raw::oio::QueueBuf">QueueBuf</a>

Take the entire buffer queue and leave `self` in empty states.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#method.collect" class="fn">collect</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#" class="tooltip" data-notable-ty="Buffer">â“˜</a>

Build a new [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer") from the queue.

If the queue is empty, it will return an empty buffer. Otherwise, it will iterate over all buffers and collect them into a new buffer.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#notes" class="doc-anchor">Â§</a>Notes

There are allocation overheads when collecting multiple buffers into a new buffer. But most of them should be acceptable since we can expect the item length of buffers are slower than 4k.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#method.advance" class="fn">advance</a>(&mut self, cnt: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Advance the buffer queue by `cnt` bytes.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#method.clear" class="fn">clear</a>(&mut self)

Clear the buffer queue.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#impl-Clone-for-QueueBuf" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html" class="struct" title="struct opendal::raw::oio::QueueBuf">QueueBuf</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html" class="struct" title="struct opendal::raw::oio::QueueBuf">QueueBuf</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#impl-Default-for-QueueBuf" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html" class="struct" title="struct opendal::raw::oio::QueueBuf">QueueBuf</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html" class="struct" title="struct opendal::raw::oio::QueueBuf">QueueBuf</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html#blanket-implementations" class="anchor">Â§</a>
