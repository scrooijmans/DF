# Trait CredentialProvider Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/mod.rs.html#806-812" class="src">Source</a>

``` rust
pub trait CredentialProvider:
    Debug
    + Send
    + Sync {
    type Credential;

    // Required method
    fn get_credential<'life0, 'async_trait>(
        &'life0 self,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<Self::Credential>>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait;
}
```

Available on **crate feature `cloud`** only.

Expand description

Provides credentials for use when signing requests

## Required Associated Types<a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html#associatedtype.Credential" class="associatedtype">Credential</a>

The type of credential returned by this provider

## Required Methods<a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html#tymethod.get_credential" class="fn">get_credential</a>\<'life0, 'async_trait\>( &'life0 self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self::<a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html#associatedtype.Credential" class="associatedtype" title="type object_store::client::CredentialProvider::Credential">Credential</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Return a credential

## Implementors<a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html#impl-CredentialProvider-for-StaticCredentialProvider%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html" class="trait" title="trait object_store::client::CredentialProvider">CredentialProvider</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html" class="struct" title="struct object_store::client::StaticCredentialProvider">StaticCredentialProvider</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

<a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html#associatedtype.Credential-1" class="anchor">§</a>

#### type <a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html#associatedtype.Credential" class="associatedtype">Credential</a> = T
