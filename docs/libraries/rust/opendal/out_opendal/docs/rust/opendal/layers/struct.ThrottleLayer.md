# Struct ThrottleLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/throttle.rs.html#67-70" class="src">Source</a>

``` rust
pub struct ThrottleLayer { /* private fields */ }
```

Available on **crate feature `layers-throttle`** only.

Expand description

Add a bandwidth rate limiter to the underlying services.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#throttle" class="doc-anchor">Â§</a>Throttle

There are several algorithms when it come to rate limiting techniques. This throttle layer uses Generic Cell Rate Algorithm (GCRA) provided by [Governor](https://docs.rs/governor/latest/governor/index.html). By setting the `bandwidth` and `burst`, we can control the byte flow rate of underlying services.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#note" class="doc-anchor">Â§</a>Note

When setting the ThrottleLayer, always consider the largest possible operation size as the burst size, as **the burst size should be larger than any possible byte length to allow it to pass through**.

Read more about [Quota](https://docs.rs/governor/latest/governor/struct.Quota.html#examples)

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#examples" class="doc-anchor">Â§</a>Examples

This example limits bandwidth to 10 KiB/s and burst size to 10 MiB.

``` rust

let _ = Operator::new(services::Memory::default())
    .expect("must init")
    .layer(ThrottleLayer::new(10 * 1024, 10000 * 1024))
    .finish();
Ok(())
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#impl-ThrottleLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html" class="struct" title="struct opendal::layers::ThrottleLayer">ThrottleLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#method.new" class="fn">new</a>(bandwidth: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, burst: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> Self

Create a new `ThrottleLayer` with given bandwidth and burst.

- bandwidth: the maximum number of bytes allowed to pass through per second.
- burst: the maximum number of bytes allowed to pass through at once.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#impl-Clone-for-ThrottleLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html" class="struct" title="struct opendal::layers::ThrottleLayer">ThrottleLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html" class="struct" title="struct opendal::layers::ThrottleLayer">ThrottleLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#impl-Layer%3CA%3E-for-ThrottleLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html" class="struct" title="struct opendal::layers::ThrottleLayer">ThrottleLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = ThrottleAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, accessor: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html#blanket-implementations" class="anchor">Â§</a>
