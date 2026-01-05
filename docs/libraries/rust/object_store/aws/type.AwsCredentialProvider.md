# Type Alias AwsCredentialProvider Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/aws/mod.rs.html#79" class="src">Source</a>

``` rust
pub type AwsCredentialProvider = Arc<dyn CredentialProvider<Credential = AwsCredential>>;
```

Available on **crate feature `aws`** only.

Expand description

[`CredentialProvider`](https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html "trait object_store::client::CredentialProvider") for [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3")

## Aliased Type<a href="https://docs.rs/object_store/latest/object_store/aws/type.AwsCredentialProvider.html#aliased-type" class="anchor">§</a>

``` rust
pub struct AwsCredentialProvider { /* private fields */ }
```
