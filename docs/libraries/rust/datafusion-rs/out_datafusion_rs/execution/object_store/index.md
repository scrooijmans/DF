# Module object_store Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/lib.rs.html#33" class="src">Source</a>

Expand description

ObjectStoreRegistry holds all the object stores at Runtime with a scheme for each store. This allows the user to extend DataFusion with different storage systems such as S3 or HDFS and query data inside these systems.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/object_store/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/object_store/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct datafusion::execution::object_store::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>  
The default [`ObjectStoreRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html "trait datafusion::datasource::object_store::ObjectStoreRegistry")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/object_store/struct.ObjectStoreUrl.html" class="struct" title="struct datafusion::execution::object_store::ObjectStoreUrl">ObjectStoreUrl</a>  
A parsed URL identifying a particular [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") instance

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/object_store/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/object_store/trait.ObjectStoreRegistry.html" class="trait" title="trait datafusion::execution::object_store::ObjectStoreRegistry">ObjectStoreRegistry</a>  
[`ObjectStoreRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html "trait datafusion::datasource::object_store::ObjectStoreRegistry") maps a URL to an [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") instance, and allows DataFusion to read from different [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") instances. For example DataFusion might be configured so that
