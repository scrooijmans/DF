# Struct CapabilityCheckLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/capability_check.rs.html#59" class="src">Source</a>

``` rust
pub struct CapabilityCheckLayer;
```

Expand description

Add an extra capability check layer for every operation

Similar to `CorrectnessChecker`, Before performing any operations, this layer will first verify its arguments against the capability of the underlying service. If the arguments is not supported, an error will be returned directly.

Notes

There are two main differences between this checker with the `CorrectnessChecker`:

1.  This checker provides additional checks for capabilities like write_with_content_type and list_with_versions, among others. These capabilities do not affect data integrity, even if the underlying storage services do not support them.

2.  OpenDAL doesnâ€™t apply this checker by default. Users can enable this layer if they want to enforce stricter requirements.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html#examples" class="doc-anchor">Â§</a>examples

``` rust

use opendal::layers::CapabilityCheckLayer;
let _ = Operator::new(services::Memory::default())?
    .layer(CapabilityCheckLayer)
    .finish();
Ok(())
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html#impl-Default-for-CapabilityCheckLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html" class="struct" title="struct opendal::layers::CapabilityCheckLayer">CapabilityCheckLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html" class="struct" title="struct opendal::layers::CapabilityCheckLayer">CapabilityCheckLayer</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html#impl-Layer%3CA%3E-for-CapabilityCheckLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html" class="struct" title="struct opendal::layers::CapabilityCheckLayer">CapabilityCheckLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = CapabilityAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html#blanket-implementations" class="anchor">Â§</a>
