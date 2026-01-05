# Trait CatalogBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/catalog/mod.rs.html#114-123" class="src">Source</a>

``` rust
pub trait CatalogBuilder:
    Default
    + Debug
    + Send
    + Sync {
    type C: Catalog;

    // Required method
    fn load(
        self,
        name: impl Into<String>,
        props: HashMap<String, String>,
    ) -> impl Future<Output = Result<Self::C>> + Send;
}
```

Expand description

Common interface for all catalog builders.

## Required Associated Types<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#associatedtype.C" class="associatedtype">C</a>: <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.Catalog.html" class="trait" title="trait iceberg::Catalog">Catalog</a>

The catalog type that this builder creates.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#tymethod.load" class="fn">load</a>( self, name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, props: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#associatedtype.C" class="associatedtype" title="type iceberg::CatalogBuilder::C">C</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>

Create a new catalog instance.

## Dyn Compatibility<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#impl-CatalogBuilder-for-MemoryCatalogBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html" class="trait" title="trait iceberg::CatalogBuilder">CatalogBuilder</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html" class="struct" title="struct iceberg::memory::MemoryCatalogBuilder">MemoryCatalogBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#associatedtype.C-1" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#associatedtype.C" class="associatedtype">C</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MemoryCatalog.html" class="struct" title="struct iceberg::MemoryCatalog">MemoryCatalog</a>
