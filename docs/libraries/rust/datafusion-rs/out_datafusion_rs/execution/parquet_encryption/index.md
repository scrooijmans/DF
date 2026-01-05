# Module parquet_encryption Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/lib.rs.html#35" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html" class="struct" title="struct datafusion::execution::parquet_encryption::EncryptionFactoryRegistry">EncryptionFactoryRegistry</a>  
Stores [`EncryptionFactory`](https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html "trait datafusion::execution::parquet_encryption::EncryptionFactory") implementations that can be retrieved by a unique string identifier

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html" class="trait" title="trait datafusion::execution::parquet_encryption::EncryptionFactory">EncryptionFactory</a>  
Trait for types that generate file encryption and decryption properties to write and read encrypted Parquet files. This allows flexibility in how encryption keys are managed, for example, to integrate with a user’s key management service (KMS). For example usage, see the [`parquet_encrypted_with_kms` example](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/parquet_encrypted_with_kms.rs).
