# Struct TestTableFactory Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test_util/mod.rs.html#176" class="src">Source</a>

``` rust
pub struct TestTableFactory {}
```

Expand description

TableFactory for tests

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html#impl-Debug-for-TestTableFactory" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html" class="struct" title="struct datafusion::test_util::TestTableFactory">TestTableFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html#impl-Default-for-TestTableFactory" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html" class="struct" title="struct datafusion::test_util::TestTableFactory">TestTableFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html" class="struct" title="struct datafusion::test_util::TestTableFactory">TestTableFactory</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html#impl-TableProviderFactory-for-TestTableFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html" class="struct" title="struct datafusion::test_util::TestTableFactory">TestTableFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html#method.create" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html#tymethod.create" class="fn">create</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, \_: &'life1 dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, cmd: &'life2 <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Create a TableProvider with the given url

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html#blanket-implementations" class="anchor">§</a>
