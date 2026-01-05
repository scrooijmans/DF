# Module object_store Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test/object_store.rs.html#18-193" class="src">Source</a>

Available on **non-WebAssembly** only.

Expand description

Object store implementation used for testing

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/object_store/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/object_store/fn.ensure_head_concurrency.html" class="fn" title="fn datafusion::test::object_store::ensure_head_concurrency">ensure_head_concurrency</a>  
Blocks the object_store `head` call until `concurrency` number of calls are pending.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/object_store/fn.local_unpartitioned_file.html" class="fn" title="fn datafusion::test::object_store::local_unpartitioned_file">local_unpartitioned_file</a>  
Helper method to fetch the file size and date at given path and create a `ObjectMeta`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/object_store/fn.make_test_store_and_state.html" class="fn" title="fn datafusion::test::object_store::make_test_store_and_state">make_test_store_and_state</a>  
Create a test object store with the provided files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/object_store/fn.register_test_store.html" class="fn" title="fn datafusion::test::object_store::register_test_store">register_test_store</a>  
Registers a test object store with the provided `ctx`
