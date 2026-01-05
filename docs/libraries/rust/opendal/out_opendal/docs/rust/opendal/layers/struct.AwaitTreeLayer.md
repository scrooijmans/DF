# Struct AwaitTreeLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/await_tree.rs.html#49" class="src">Source</a>

``` rust
pub struct AwaitTreeLayer {}
```

Available on **crate feature `layers-await-tree`** only.

Expand description

Add an Instrument await-tree for actor-based applications to the underlying services.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#awaittree" class="doc-anchor">Â§</a>AwaitTree

await-tree allows developers to dump this execution tree at runtime, with the span of each Future annotated by instrument_await. Read more about [await-tree](https://docs.rs/await-tree/latest/await_tree/)

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust

let _ = Operator::new(services::Memory::default())?
    .layer(AwaitTreeLayer::new())
    .finish();
Ok(())
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#impl-AwaitTreeLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html" class="struct" title="struct opendal::layers::AwaitTreeLayer">AwaitTreeLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#method.new" class="fn">new</a>() -\> Self

Create a new `AwaitTreeLayer`.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#impl-Clone-for-AwaitTreeLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html" class="struct" title="struct opendal::layers::AwaitTreeLayer">AwaitTreeLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html" class="struct" title="struct opendal::layers::AwaitTreeLayer">AwaitTreeLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#impl-Default-for-AwaitTreeLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html" class="struct" title="struct opendal::layers::AwaitTreeLayer">AwaitTreeLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html" class="struct" title="struct opendal::layers::AwaitTreeLayer">AwaitTreeLayer</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#impl-Layer%3CA%3E-for-AwaitTreeLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html" class="struct" title="struct opendal::layers::AwaitTreeLayer">AwaitTreeLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = AwaitTreeAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, accessor: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html#blanket-implementations" class="anchor">Â§</a>
