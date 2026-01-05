# Type Alias GcpCredentialProvider Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/gcp/mod.rs.html#69" class="src">Source</a>

``` rust
pub type GcpCredentialProvider = Arc<dyn CredentialProvider<Credential = GcpCredential>>;
```

Available on **crate feature `gcp`** only.

Expand description

[`CredentialProvider`](https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html "trait object_store::client::CredentialProvider") for [`GoogleCloudStorage`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html "struct object_store::gcp::GoogleCloudStorage")

## Aliased Type<a href="https://docs.rs/object_store/latest/object_store/gcp/type.GcpCredentialProvider.html#aliased-type" class="anchor">§</a>

``` rust
pub struct GcpCredentialProvider { /* private fields */ }
```
