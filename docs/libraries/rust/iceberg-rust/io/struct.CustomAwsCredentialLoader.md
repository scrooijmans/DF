# Struct CustomAwsCredentialLoader Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/storage_s3.rs.html#190" class="src">Source</a>

``` rust
pub struct CustomAwsCredentialLoader(/* private fields */);
```

Expand description

Custom AWS credential loader. This can be used to load credentials from a custom source, such as the AWS SDK.

This should be set as an extension on `FileIOBuilder`.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#impl-CustomAwsCredentialLoader" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html" class="struct" title="struct iceberg::io::CustomAwsCredentialLoader">CustomAwsCredentialLoader</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#method.new" class="fn">new</a>(loader: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html" class="trait" title="trait iceberg::io::AwsCredentialLoad">AwsCredentialLoad</a>\>) -\> Self

Create a new custom AWS credential loader.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#method.into_opendal_loader" class="fn">into_opendal_loader</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html" class="trait" title="trait iceberg::io::AwsCredentialLoad">AwsCredentialLoad</a>\>

Convert this loader into an opendal compatible loader for customized AWS credentials.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#impl-AwsCredentialLoad-for-CustomAwsCredentialLoader" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html" class="trait" title="trait iceberg::io::AwsCredentialLoad">CredentialLoad</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html" class="struct" title="struct iceberg::io::CustomAwsCredentialLoader">CustomAwsCredentialLoader</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#method.load_credential" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html#tymethod.load_credential" class="fn">load_credential</a>\<'life0, 'async_trait\>( &'life0 self, client: <a href="https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/async_impl/client/struct.Client.html" class="struct" title="struct reqwest::async_impl::client::Client">Client</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/anyhow/1.0.99/x86_64-unknown-linux-gnu/anyhow/type.Result.html" class="type" title="type anyhow::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html" class="struct" title="struct iceberg::io::AwsCredential">AwsCredential</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Load credential from sources. [Read more](https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html#tymethod.load_credential)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#impl-Clone-for-CustomAwsCredentialLoader" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html" class="struct" title="struct iceberg::io::CustomAwsCredentialLoader">CustomAwsCredentialLoader</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html" class="struct" title="struct iceberg::io::CustomAwsCredentialLoader">CustomAwsCredentialLoader</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#impl-Debug-for-CustomAwsCredentialLoader" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html" class="struct" title="struct iceberg::io::CustomAwsCredentialLoader">CustomAwsCredentialLoader</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html#blanket-implementations" class="anchor">§</a>
