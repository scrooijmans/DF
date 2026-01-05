# Struct EncryptionFactoryRegistry Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/parquet_encryption.rs.html#56" class="src">Source</a>

``` rust
pub struct EncryptionFactoryRegistry { /* private fields */ }
```

Expand description

Stores [`EncryptionFactory`](https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html "trait datafusion::execution::parquet_encryption::EncryptionFactory") implementations that can be retrieved by a unique string identifier

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#impl-EncryptionFactoryRegistry" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html" class="struct" title="struct datafusion::execution::parquet_encryption::EncryptionFactoryRegistry">EncryptionFactoryRegistry</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#method.register_factory" class="fn">register_factory</a>( &self, id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html" class="trait" title="trait datafusion::execution::parquet_encryption::EncryptionFactory">EncryptionFactory</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html" class="trait" title="trait datafusion::execution::parquet_encryption::EncryptionFactory">EncryptionFactory</a>\>\>

Register an [`EncryptionFactory`](https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html "trait datafusion::execution::parquet_encryption::EncryptionFactory") with an associated identifier that can be later used to configure encryption when reading or writing Parquet. If an encryption factory with the same identifier was already registered, it is replaced and returned.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#method.get_factory" class="fn">get_factory</a>( &self, id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html" class="trait" title="trait datafusion::execution::parquet_encryption::EncryptionFactory">EncryptionFactory</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Retrieve an [`EncryptionFactory`](https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html "trait datafusion::execution::parquet_encryption::EncryptionFactory") by its identifier

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#impl-Clone-for-EncryptionFactoryRegistry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html" class="struct" title="struct datafusion::execution::parquet_encryption::EncryptionFactoryRegistry">EncryptionFactoryRegistry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html" class="struct" title="struct datafusion::execution::parquet_encryption::EncryptionFactoryRegistry">EncryptionFactoryRegistry</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#impl-Debug-for-EncryptionFactoryRegistry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html" class="struct" title="struct datafusion::execution::parquet_encryption::EncryptionFactoryRegistry">EncryptionFactoryRegistry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#impl-Default-for-EncryptionFactoryRegistry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html" class="struct" title="struct datafusion::execution::parquet_encryption::EncryptionFactoryRegistry">EncryptionFactoryRegistry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html" class="struct" title="struct datafusion::execution::parquet_encryption::EncryptionFactoryRegistry">EncryptionFactoryRegistry</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html#blanket-implementations" class="anchor">§</a>
