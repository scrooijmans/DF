# Trait TableProviderFactory Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#307" class="src">Source</a>

``` rust
pub trait TableProviderFactory:
    Debug
    + Sync
    + Send {
    // Required method
    fn create<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        state: &'life1 dyn Session,
        cmd: &'life2 CreateExternalTable,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn TableProvider>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait,
             Self: 'async_trait;
}
```

Expand description

A factory which creates [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider")s at runtime given a URL.

For example, this can be used to create a table “on the fly” from a directory of files only when that name is referenced.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html#tymethod.create" class="fn">create</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, state: &'life1 dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, cmd: &'life2 <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, Self: 'async_trait,

Create a TableProvider with the given url

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html#impl-TableProviderFactory-for-ListingTableFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing_table_factory/struct.ListingTableFactory.html" class="struct" title="struct datafusion::datasource::listing_table_factory::ListingTableFactory">ListingTableFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html#impl-TableProviderFactory-for-DefaultTableFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/provider/struct.DefaultTableFactory.html" class="struct" title="struct datafusion::datasource::provider::DefaultTableFactory">DefaultTableFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html#impl-TableProviderFactory-for-StreamTableFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamTableFactory.html" class="struct" title="struct datafusion::datasource::stream::StreamTableFactory">StreamTableFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html#impl-TableProviderFactory-for-TestTableFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html" class="struct" title="struct datafusion::test_util::TestTableFactory">TestTableFactory</a>
