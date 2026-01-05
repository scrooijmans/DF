# Struct ObjectStoreBuilder Copy item path

<a href="https://opendal.apache.org/docs/object-store-opendal/src/object_store_opendal/service/mod.rs.html#49-51" class="src">Source</a>

``` rust
pub struct ObjectStoreBuilder { /* private fields */ }
```

Expand description

ObjectStore backend builder

## Implementations<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#impl-ObjectStoreBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html" class="struct" title="struct object_store_opendal::ObjectStoreBuilder">ObjectStoreBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#method.new" class="fn">new</a>(store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn ObjectStore + 'static\>) -\> Self

Set the object store instance

## Trait Implementations<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#impl-Builder-for-ObjectStoreBuilder" class="anchor">Â§</a>

### impl Builder for <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html" class="struct" title="struct object_store_opendal::ObjectStoreBuilder">ObjectStoreBuilder</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#associatedtype.Config" class="anchor">Â§</a>

#### type Config = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#method.build" class="anchor">Â§</a>

#### fn build(self) -\> Result\<impl Access\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#impl-Debug-for-ObjectStoreBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html" class="struct" title="struct object_store_opendal::ObjectStoreBuilder">ObjectStoreBuilder</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#impl-Default-for-ObjectStoreBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html" class="struct" title="struct object_store_opendal::ObjectStoreBuilder">ObjectStoreBuilder</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html" class="struct" title="struct object_store_opendal::ObjectStoreBuilder">ObjectStoreBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreBuilder.html#blanket-implementations" class="anchor">Â§</a>
