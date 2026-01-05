# Struct DefaultObjectStoreRegistry Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/registry.rs.html#112-115" class="src">Source</a>

``` rust
pub struct DefaultObjectStoreRegistry { /* private fields */ }
```

Expand description

An [`ObjectStoreRegistry`](https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html "trait object_store::registry::ObjectStoreRegistry") that uses [`parse_url_opts`](https://docs.rs/object_store/latest/object_store/fn.parse_url_opts.html "fn object_store::parse_url_opts") to create stores based on the environment

## Implementations<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#impl-DefaultObjectStoreRegistry" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct object_store::registry::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#method.new" class="fn">new</a>() -\> Self

Create a new [`DefaultObjectStoreRegistry`](https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html "struct object_store::registry::DefaultObjectStoreRegistry")

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#impl-Debug-for-DefaultObjectStoreRegistry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct object_store::registry::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#impl-Default-for-DefaultObjectStoreRegistry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct object_store::registry::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct object_store::registry::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#impl-ObjectStoreRegistry-for-DefaultObjectStoreRegistry" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html" class="trait" title="trait object_store::registry::ObjectStoreRegistry">ObjectStoreRegistry</a> for <a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct object_store::registry::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#method.register" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html#tymethod.register" class="fn">register</a>( &self, url: <a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>, store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>\>

Register a new store for the provided store URL [Read more](https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html#tymethod.register)

<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#method.resolve" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html#tymethod.resolve" class="fn">resolve</a>(&self, to_resolve: &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>)\>

Resolve an object URL [Read more](https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html#tymethod.resolve)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html#blanket-implementations" class="anchor">§</a>
