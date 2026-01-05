# Struct StaticCredentialProvider Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/mod.rs.html#816-818" class="src">Source</a>

``` rust
pub struct StaticCredentialProvider<T> { /* private fields */ }
```

Available on **crate feature `cloud`** only.

Expand description

A static set of credentials

## Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html#impl-StaticCredentialProvider%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html" class="struct" title="struct object_store::client::StaticCredentialProvider">StaticCredentialProvider</a>\<T\>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html#method.new" class="fn">new</a>(credential: T) -\> Self

A [`CredentialProvider`](https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html "trait object_store::client::CredentialProvider") for a static credential of type `T`

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html#impl-CredentialProvider-for-StaticCredentialProvider%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html" class="trait" title="trait object_store::client::CredentialProvider">CredentialProvider</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html" class="struct" title="struct object_store::client::StaticCredentialProvider">StaticCredentialProvider</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html#associatedtype.Credential" class="anchor">§</a>

#### type <a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html#associatedtype.Credential" class="associatedtype">Credential</a> = T

The type of credential returned by this provider

<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html#method.get_credential" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html#tymethod.get_credential" class="fn">get_credential</a>\<'life0, 'async_trait\>( &'life0 self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Return a credential

<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html#impl-Debug-for-StaticCredentialProvider%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html" class="struct" title="struct object_store::client::StaticCredentialProvider">StaticCredentialProvider</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html#blanket-implementations" class="anchor">§</a>
