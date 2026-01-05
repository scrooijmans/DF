# Trait EncryptionFactory Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/parquet_encryption.rs.html#37" class="src">Source</a>

``` rust
pub trait EncryptionFactory:
    Send
    + Sync
    + Debug
    + 'static {
    // Required methods
    fn get_file_encryption_properties<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        config: &'life1 EncryptionFactoryOptions,
        schema: &'life2 Arc<Schema>,
        file_path: &'life3 Path,
    ) -> Pin<Box<dyn Future<Output = Result<Option<FileEncryptionProperties>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait,
             'life3: 'async_trait,
             Self: 'async_trait;
    fn get_file_decryption_properties<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        config: &'life1 EncryptionFactoryOptions,
        file_path: &'life2 Path,
    ) -> Pin<Box<dyn Future<Output = Result<Option<FileDecryptionProperties>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait,
             Self: 'async_trait;
}
```

Expand description

Trait for types that generate file encryption and decryption properties to write and read encrypted Parquet files. This allows flexibility in how encryption keys are managed, for example, to integrate with a user’s key management service (KMS). For example usage, see the [`parquet_encrypted_with_kms` example](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/parquet_encrypted_with_kms.rs).

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html#tymethod.get_file_encryption_properties" class="fn">get_file_encryption_properties</a>\<'life0, 'life1, 'life2, 'life3, 'async_trait\>( &'life0 self, config: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>, schema: &'life2 <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, file_path: &'life3 <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileEncryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileEncryptionProperties">FileEncryptionProperties</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, 'life3: 'async_trait, Self: 'async_trait,

Generate file encryption properties to use when writing a Parquet file.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html#tymethod.get_file_decryption_properties" class="fn">get_file_decryption_properties</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, config: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>, file_path: &'life2 <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/struct.FileDecryptionProperties.html" class="struct" title="struct datafusion::common::encryption::FileDecryptionProperties">FileDecryptionProperties</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, Self: 'async_trait,

Generate file decryption properties to use when reading a Parquet file.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html#implementors" class="anchor">§</a>
