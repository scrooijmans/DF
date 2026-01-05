# Struct TimeoutLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/timeout.rs.html#116-119" class="src">Source</a>

``` rust
pub struct TimeoutLayer { /* private fields */ }
```

Expand description

Add timeout for every operation to avoid slow or unexpected hang operations.

For example, a dead connection could hang a databases sql query. TimeoutLayer will break this connection and returns an error so users can handle it by retrying or print to users.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#notes" class="doc-anchor">Â§</a>Notes

`TimeoutLayer` treats all operations in two kinds:

- Non IO Operation like `stat`, `delete` they operate on a single file. We control them by setting `timeout`.
- IO Operation like `read`, `Reader::read` and `Writer::write`, they operate on data directly, we control them by setting `io_timeout`.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#default" class="doc-anchor">Â§</a>Default

- timeout: 60 seconds
- io_timeout: 10 seconds

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#panics" class="doc-anchor">Â§</a>Panics

TimeoutLayer will drop the future if the timeout is reached. This might cause the internal state of the future to be broken. If underlying future moves ownership into the future, it will be dropped and will neven return back.

For example, while using `TimeoutLayer` with `RetryLayer` at the same time, please make sure timeout layer showed up before retry layer.

``` rust


let op = Operator::new(services::Memory::default())?
    // This is fine, since timeout happen during retry.
    .layer(TimeoutLayer::new().with_io_timeout(Duration::from_nanos(1)))
    .layer(RetryLayer::new())
    // This is wrong. Since timeout layer will drop future, leaving retry layer in a bad state.
    .layer(TimeoutLayer::new().with_io_timeout(Duration::from_nanos(1)))
    .finish();
Ok(())
```

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#examples" class="doc-anchor">Â§</a>Examples

The following examples will create a timeout layer with 10 seconds timeout for all non-io operations, 3 seconds timeout for all io operations.

``` rust


let _ = Operator::new(services::Memory::default())?
    .layer(
        TimeoutLayer::default()
            .with_timeout(Duration::from_secs(10))
            .with_io_timeout(Duration::from_secs(3)),
    )
    .finish();
Ok(())
```

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#implementation-notes" class="doc-anchor">Â§</a>Implementation Notes

TimeoutLayer is using \[`tokio::time::timeout`\] to implement timeout for operations. And IO Operations insides `reader`, `writer` will use `Pin<Box<tokio::time::Sleep>>` to track the timeout.

This might introduce a bit overhead for IO operations, but itâ€™s the only way to implement timeout correctly. We used to implement timeout layer in zero cost way that only stores a [`std::time::Instant`](https://doc.rust-lang.org/nightly/std/time/struct.Instant.html "struct std::time::Instant") and check the timeout by comparing the instant with current time. However, it doesnâ€™t work for all cases.

For examples, users TCP connection could be in [Busy ESTAB](https://blog.cloudflare.com/when-tcp-sockets-refuse-to-die) state. In this state, no IO event will be emitted. The runtime will never poll our future again. From the application side, this future is hanging forever until this TCP connection is closed for reaching the linux [net.ipv4.tcp_retries2](https://man7.org/linux/man-pages/man7/tcp.7.html) times.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#impl-TimeoutLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html" class="struct" title="struct opendal::layers::TimeoutLayer">TimeoutLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#method.new" class="fn">new</a>() -\> Self

Create a new `TimeoutLayer` with default settings.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#method.with_timeout" class="fn">with_timeout</a>(self, timeout: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Set timeout for TimeoutLayer with given value.

This timeout is for all non-io operations like `stat`, `delete`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#method.with_io_timeout" class="fn">with_io_timeout</a>(self, timeout: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Set io timeout for TimeoutLayer with given value.

This timeout is for all io operations like `read`, `Reader::read` and `Writer::write`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#method.with_speed" class="fn">with_speed</a>(self, \_: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> Self

ðŸ‘ŽDeprecated: with speed is not supported anymore, please use with_io_timeout instead

Set speed for TimeoutLayer with given value.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#notes-1" class="doc-anchor">Â§</a>Notes

The speed should be the lower bound of the IO speed. Set this value too large could result in all write operations failing.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#panics-1" class="doc-anchor">Â§</a>Panics

This function will panic if speed is 0.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#impl-Clone-for-TimeoutLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html" class="struct" title="struct opendal::layers::TimeoutLayer">TimeoutLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html" class="struct" title="struct opendal::layers::TimeoutLayer">TimeoutLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#impl-Default-for-TimeoutLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html" class="struct" title="struct opendal::layers::TimeoutLayer">TimeoutLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#impl-Layer%3CA%3E-for-TimeoutLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html" class="struct" title="struct opendal::layers::TimeoutLayer">TimeoutLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = TimeoutAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html#blanket-implementations" class="anchor">Â§</a>
