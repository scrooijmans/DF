# Trait AwsCredentialLoad Copy item path

<a href="https://docs.rs/reqsign/0.16.5/x86_64-unknown-linux-gnu/src/reqsign/aws/credential.rs.html#68" class="src">Source</a>

``` rust
pub trait AwsCredentialLoad:
    'static
    + Send
    + Sync {
    // Required method
    fn load_credential<'life0, 'async_trait>(
        &'life0 self,
        client: Client,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Credential>, Error>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             Self: 'async_trait;
}
```

Expand description

Loader trait will try to load credential from different sources.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html#tymethod.load_credential" class="fn">load_credential</a>\<'life0, 'async_trait\>( &'life0 self, client: <a href="https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/async_impl/client/struct.Client.html" class="struct" title="struct reqwest::async_impl::client::Client">Client</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html" class="struct" title="struct iceberg::io::AwsCredential">Credential</a>\>, <a href="https://docs.rs/anyhow/1.0.99/x86_64-unknown-linux-gnu/anyhow/struct.Error.html" class="struct" title="struct anyhow::Error">Error</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, Self: 'async_trait,

Load credential from sources.

- If succeed, return `Ok(Some(cred))`
- If not found, return `Ok(None)`
- If unexpected errors happened, return `Err(err)`

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html#impl-AwsCredentialLoad-for-AwsAssumeRoleLoader" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html" class="trait" title="trait iceberg::io::AwsCredentialLoad">CredentialLoad</a> for <a href="https://docs.rs/reqsign/0.16.5/x86_64-unknown-linux-gnu/reqsign/aws/credential/struct.AssumeRoleLoader.html" class="struct" title="struct reqsign::aws::credential::AssumeRoleLoader">AssumeRoleLoader</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html#impl-AwsCredentialLoad-for-AwsDefaultLoader" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html" class="trait" title="trait iceberg::io::AwsCredentialLoad">CredentialLoad</a> for <a href="https://docs.rs/reqsign/0.16.5/x86_64-unknown-linux-gnu/reqsign/aws/credential/struct.DefaultLoader.html" class="struct" title="struct reqsign::aws::credential::DefaultLoader">DefaultLoader</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html#impl-AwsCredentialLoad-for-CustomAwsCredentialLoader" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html" class="trait" title="trait iceberg::io::AwsCredentialLoad">CredentialLoad</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html" class="struct" title="struct iceberg::io::CustomAwsCredentialLoader">CustomAwsCredentialLoader</a>
