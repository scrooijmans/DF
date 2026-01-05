# Trait UrlTableFactory Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#181" class="src">Source</a>

``` rust
pub trait UrlTableFactory:
    Debug
    + Sync
    + Send {
    // Required method
    fn try_new<'life0, 'life1, 'async_trait>(
        &'life0 self,
        url: &'life1 str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Arc<dyn TableProvider>>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             Self: 'async_trait;
}
```

Expand description

[UrlTableFactory](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.UrlTableFactory.html "trait datafusion::catalog::UrlTableFactory") is a factory that can create a table provider from the given url.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.UrlTableFactory.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.UrlTableFactory.html#tymethod.try_new" class="fn">try_new</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, url: &'life1 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

create a new table provider from the provided url

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.UrlTableFactory.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.UrlTableFactory.html#impl-UrlTableFactory-for-DynamicListTableFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.UrlTableFactory.html" class="trait" title="trait datafusion::catalog::UrlTableFactory">UrlTableFactory</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/dynamic_file/struct.DynamicListTableFactory.html" class="struct" title="struct datafusion::datasource::dynamic_file::DynamicListTableFactory">DynamicListTableFactory</a>
