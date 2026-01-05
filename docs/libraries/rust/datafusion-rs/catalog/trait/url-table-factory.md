# UrlTableFactory in datafusion::catalog - Rust

[datafusion](../index.html)::[catalog](index.html)

## Trait UrlTableFactory 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#181)

```
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

[UrlTableFactory](trait.UrlTableFactory.html "trait datafusion::catalog::UrlTableFactory") is a factory that can create a table provider from the given url.

## Required Methods[§](#required-methods)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#183-186)

#### fn [try_new](#tymethod.try_new)<'life0, 'life1, 'async_trait>( &'life0 self, url: &'life1 [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html "struct alloc::boxed::Box")<dyn [Future](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html "trait core::future::future::Future")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Arc](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")<dyn [TableProvider](../datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider")\>>, [DataFusionError](../error/enum.DataFusionError.html "enum datafusion::error::DataFusionError")\>> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'async_trait>>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

create a new table provider from the provided url

## Implementors[§](#implementors)

[Source](about:blank/src/datafusion/datasource/dynamic_file.rs.html#54-89)
[§](#impl-UrlTableFactory-for-DynamicListTableFactory)

### impl [UrlTableFactory](trait.UrlTableFactory.html "trait datafusion::catalog::UrlTableFactory") for [DynamicListTableFactory](../datasource/dynamic_file/struct.DynamicListTableFactory.html "struct datafusion::datasource::dynamic_file::DynamicListTableFactory")
