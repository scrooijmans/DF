# Type Alias AzureCredentialProvider Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/azure/mod.rs.html#50" class="src">Source</a>

``` rust
pub type AzureCredentialProvider = Arc<dyn CredentialProvider<Credential = AzureCredential>>;
```

Available on **crate feature `azure`** only.

Expand description

[`CredentialProvider`](https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html "trait object_store::client::CredentialProvider") for [`MicrosoftAzure`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzure.html "struct object_store::azure::MicrosoftAzure")

## Aliased Type<a href="https://docs.rs/object_store/latest/object_store/azure/type.AzureCredentialProvider.html#aliased-type" class="anchor">§</a>

``` rust
pub struct AzureCredentialProvider { /* private fields */ }
```
