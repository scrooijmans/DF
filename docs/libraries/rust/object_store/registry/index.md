# Module registry Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/registry.rs.html#18-340" class="src">Source</a>

Expand description

Map object URLs to [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore")

## Structs<a href="https://docs.rs/object_store/latest/object_store/registry/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct object_store::registry::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>  
An [`ObjectStoreRegistry`](https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html "trait object_store::registry::ObjectStoreRegistry") that uses [`parse_url_opts`](https://docs.rs/object_store/latest/object_store/fn.parse_url_opts.html "fn object_store::parse_url_opts") to create stores based on the environment

## Traits<a href="https://docs.rs/object_store/latest/object_store/registry/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html" class="trait" title="trait object_store::registry::ObjectStoreRegistry">ObjectStoreRegistry</a>  
[`ObjectStoreRegistry`](https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html "trait object_store::registry::ObjectStoreRegistry") maps a URL to an [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") instance
