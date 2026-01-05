# Struct MemoryCatalogBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/catalog/memory/catalog.rs.html#43" class="src">Source</a>

``` rust
pub struct MemoryCatalogBuilder(/* private fields */);
```

Expand description

Builder for [`MemoryCatalog`](https://docs.rs/iceberg/0.7.0/iceberg/struct.MemoryCatalog.html "struct iceberg::MemoryCatalog").

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html#impl-CatalogBuilder-for-MemoryCatalogBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html" class="trait" title="trait iceberg::CatalogBuilder">CatalogBuilder</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html" class="struct" title="struct iceberg::memory::MemoryCatalogBuilder">MemoryCatalogBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html#associatedtype.C" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#associatedtype.C" class="associatedtype">C</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.MemoryCatalog.html" class="struct" title="struct iceberg::MemoryCatalog">MemoryCatalog</a>

The catalog type that this builder creates.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html#method.load" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#tymethod.load" class="fn">load</a>( self, name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, props: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/trait.CatalogBuilder.html#associatedtype.C" class="associatedtype" title="type iceberg::CatalogBuilder::C">C</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>

Create a new catalog instance.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html#impl-Debug-for-MemoryCatalogBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html" class="struct" title="struct iceberg::memory::MemoryCatalogBuilder">MemoryCatalogBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html#impl-Default-for-MemoryCatalogBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html" class="struct" title="struct iceberg::memory::MemoryCatalogBuilder">MemoryCatalogBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/memory/struct.MemoryCatalogBuilder.html#blanket-implementations" class="anchor">§</a>
