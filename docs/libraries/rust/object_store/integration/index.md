# Module integration Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/integration.rs.html#18-1324" class="src">Source</a>

Available on **crate feature `integration`** only.

Expand description

Integration tests for custom object store implementations

NB: These tests will delete everything present in the provided [`DynObjectStore`](https://docs.rs/object_store/latest/object_store/type.DynObjectStore.html "type object_store::DynObjectStore").

These tests are not a stable part of the public API and breaking changes may be made in patch releases.

They are intended solely for testing purposes.

## Functions<a href="https://docs.rs/object_store/latest/object_store/integration/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.copy_if_not_exists.html" class="fn" title="fn object_store::integration::copy_if_not_exists">copy_if_not_exists</a>  
Tests copy if not exists

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.copy_rename_nonexistent_object.html" class="fn" title="fn object_store::integration::copy_rename_nonexistent_object">copy_rename_nonexistent_object</a>  
Tests copy and renaming behaviour of non-existent objects

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.get_nonexistent_object.html" class="fn" title="fn object_store::integration::get_nonexistent_object">get_nonexistent_object</a>  
Tests fetching a non-existent object returns a not found error

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.get_opts.html" class="fn" title="fn object_store::integration::get_opts">get_opts</a>  
Tests conditional read requests

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.list_paginated.html" class="fn" title="fn object_store::integration::list_paginated">list_paginated</a>  
Tests [`PaginatedListStore`](https://docs.rs/object_store/latest/object_store/list/trait.PaginatedListStore.html "trait object_store::list::PaginatedListStore")

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.list_uses_directories_correctly.html" class="fn" title="fn object_store::integration::list_uses_directories_correctly">list_uses_directories_correctly</a>  
Tests that directories are transparent

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.list_with_delimiter.html" class="fn" title="fn object_store::integration::list_with_delimiter">list_with_delimiter</a>  
Tests listing with delimiter

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.multipart.html" class="fn" title="fn object_store::integration::multipart">multipart</a>  
Tests [`MultipartStore`](https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html "trait object_store::multipart::MultipartStore")

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.multipart_out_of_order.html" class="fn" title="fn object_store::integration::multipart_out_of_order">multipart_out_of_order</a>  
Tests performing out of order multipart uploads

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.multipart_race_condition.html" class="fn" title="fn object_store::integration::multipart_race_condition">multipart_race_condition</a>  
Tests a race condition where 2 threads are performing multipart writes to the same path

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.put_get_attributes.html" class="fn" title="fn object_store::integration::put_get_attributes">put_get_attributes</a>  
Tests the ability to read and write [`Attributes`](https://docs.rs/object_store/latest/object_store/struct.Attributes.html "struct object_store::Attributes")

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.put_get_delete_list.html" class="fn" title="fn object_store::integration::put_get_delete_list">put_get_delete_list</a>  
Tests basic read/write and listing operations

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.put_opts.html" class="fn" title="fn object_store::integration::put_opts">put_opts</a>  
Tests conditional writes

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.rename_and_copy.html" class="fn" title="fn object_store::integration::rename_and_copy">rename_and_copy</a>  
Tests copying

<a href="https://docs.rs/object_store/latest/object_store/integration/fn.stream_get.html" class="fn" title="fn object_store::integration::stream_get">stream_get</a>  
Tests the ability to perform multipart writes
