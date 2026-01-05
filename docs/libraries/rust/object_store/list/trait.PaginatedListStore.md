# Trait PaginatedListStore Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/list.rs.html#72-85" class="src">Source</a>

``` rust
pub trait PaginatedListStore:
    Send
    + Sync
    + 'static {
    // Required method
    fn list_paginated<'life0, 'life1, 'async_trait>(
        &'life0 self,
        prefix: Option<&'life1 str>,
        opts: PaginatedListOptions,
    ) -> Pin<Box<dyn Future<Output = Result<PaginatedListResult>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
}
```

Expand description

A low-level interface for interacting with paginated listing APIs

Most use-cases should prefer [`ObjectStore::list`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.list "method object_store::ObjectStore::list") as this is supported by more backends, including [`LocalFileSystem`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem"), however, [`PaginatedListStore`](https://docs.rs/object_store/latest/object_store/list/trait.PaginatedListStore.html "trait object_store::list::PaginatedListStore") can be used where stateless pagination or non-path segment based listing is required

## Required Methods<a href="https://docs.rs/object_store/latest/object_store/list/trait.PaginatedListStore.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/list/trait.PaginatedListStore.html#tymethod.list_paginated" class="fn">list_paginated</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'life1 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, opts: <a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html" class="struct" title="struct object_store::list::PaginatedListOptions">PaginatedListOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListResult.html" class="struct" title="struct object_store::list::PaginatedListResult">PaginatedListResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Perform a paginated list request

Note: the order of returned objects is not guaranteed and unlike [`ObjectStore::list`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.list "method object_store::ObjectStore::list") a trailing delimiter is not automatically added to `prefix`

## Implementors<a href="https://docs.rs/object_store/latest/object_store/list/trait.PaginatedListStore.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/list/trait.PaginatedListStore.html#impl-PaginatedListStore-for-AmazonS3" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/list/trait.PaginatedListStore.html" class="trait" title="trait object_store::list::PaginatedListStore">PaginatedListStore</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html" class="struct" title="struct object_store::aws::AmazonS3">AmazonS3</a>

Available on **crate feature `aws`** only.

<a href="https://docs.rs/object_store/latest/object_store/list/trait.PaginatedListStore.html#impl-PaginatedListStore-for-MicrosoftAzure" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/list/trait.PaginatedListStore.html" class="trait" title="trait object_store::list::PaginatedListStore">PaginatedListStore</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzure.html" class="struct" title="struct object_store::azure::MicrosoftAzure">MicrosoftAzure</a>

Available on **crate feature `azure`** only.

<a href="https://docs.rs/object_store/latest/object_store/list/trait.PaginatedListStore.html#impl-PaginatedListStore-for-GoogleCloudStorage" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/list/trait.PaginatedListStore.html" class="trait" title="trait object_store::list::PaginatedListStore">PaginatedListStore</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html" class="struct" title="struct object_store::gcp::GoogleCloudStorage">GoogleCloudStorage</a>

Available on **crate feature `gcp`** only.
