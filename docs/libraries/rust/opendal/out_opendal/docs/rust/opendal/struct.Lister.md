# Struct Lister Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/list.rs.html#33-38" class="src">Source</a>

``` rust
pub struct Lister { /* private fields */ }
```

Expand description

Lister is designed to list entries at given path in an asynchronous manner.

- Lister implements `Stream<Item = Result<Entry>>`.
- Lister will return `None` if there is no more entries or error has been returned.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html#impl-Stream-for-Lister" class="anchor">Â§</a>

### impl Stream for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html" class="struct" title="struct opendal::Lister">Lister</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html#associatedtype.Item" class="anchor">Â§</a>

#### type Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>, <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>\>

Values yielded by the stream.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html#method.poll_next" class="anchor">Â§</a>

#### fn poll_next( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::Item\>\>

Attempt to pull out the next value of this stream, registering the current task for wakeup if the value is not yet available, and returning `None` if the stream is exhausted. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html#method.size_hint" class="anchor">Â§</a>

#### fn size_hint(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Returns the bounds on the remaining length of the stream. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html#impl-Sync-for-Lister" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html" class="struct" title="struct opendal::Lister">Lister</a>

#### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html#safety" class="doc-anchor">Â§</a>Safety

Lister will only be accessed by `&mut Self`

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Lister.html#blanket-implementations" class="anchor">Â§</a>
