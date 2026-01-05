# Struct ConcurrentLimitLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/concurrent_limit.rs.html#86-89" class="src">Source</a>

``` rust
pub struct ConcurrentLimitLayer { /* private fields */ }
```

Expand description

Add concurrent request limit.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#notes" class="doc-anchor">Â§</a>Notes

Users can control how many concurrent connections could be established between OpenDAL and underlying storage services.

All operators wrapped by this layer will share a common semaphore. This allows you to reuse the same layer across multiple operators, ensuring that the total number of concurrent requests across the entire application does not exceed the limit.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#examples" class="doc-anchor">Â§</a>Examples

Add a concurrent limit layer to the operator:

``` rust

let _ = Operator::new(services::Memory::default())?
    .layer(ConcurrentLimitLayer::new(1024))
    .finish();
Ok(())
```

Share a concurrent limit layer between the operators:

``` rust

let limit = ConcurrentLimitLayer::new(1024);

let _operator_a = Operator::new(services::Memory::default())?
    .layer(limit.clone())
    .finish();
let _operator_b = Operator::new(services::Memory::default())?
    .layer(limit.clone())
    .finish();

Ok(())
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#impl-ConcurrentLimitLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html" class="struct" title="struct opendal::layers::ConcurrentLimitLayer">ConcurrentLimitLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#method.new" class="fn">new</a>(permits: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Create a new ConcurrentLimitLayer will specify permits.

This permits will applied to all operations.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#method.with_http_concurrent_limit" class="fn">with_http_concurrent_limit</a>(self, permits: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set a concurrent limit for HTTP requests.

This will limit the number of concurrent HTTP requests made by the operator.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#impl-Clone-for-ConcurrentLimitLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html" class="struct" title="struct opendal::layers::ConcurrentLimitLayer">ConcurrentLimitLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html" class="struct" title="struct opendal::layers::ConcurrentLimitLayer">ConcurrentLimitLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#impl-Layer%3CA%3E-for-ConcurrentLimitLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html" class="struct" title="struct opendal::layers::ConcurrentLimitLayer">ConcurrentLimitLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = ConcurrentLimitAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html#blanket-implementations" class="anchor">Â§</a>
