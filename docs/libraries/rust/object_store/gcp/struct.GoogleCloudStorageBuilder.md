# Struct GoogleCloudStorageBuilder Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/gcp/builder.rs.html#96-119" class="src">Source</a>

``` rust
pub struct GoogleCloudStorageBuilder { /* private fields */ }
```

Available on **crate feature `gcp`** only.

Expand description

Configure a connection to Google Cloud Storage.

If no credentials are explicitly provided, they will be sourced from the environment as documented [here](https://cloud.google.com/docs/authentication/application-default-credentials).

## <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
let gcs = GoogleCloudStorageBuilder::from_env().with_bucket_name(BUCKET_NAME).build();
```

## Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#impl-GoogleCloudStorageBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html" class="struct" title="struct object_store::gcp::GoogleCloudStorageBuilder">GoogleCloudStorageBuilder</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.new" class="fn">new</a>() -\> Self

Create a new [`GoogleCloudStorageBuilder`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html "struct object_store::gcp::GoogleCloudStorageBuilder") with default values.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.from_env" class="fn">from_env</a>() -\> Self

Create an instance of [`GoogleCloudStorageBuilder`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html "struct object_store::gcp::GoogleCloudStorageBuilder") with values pre-populated from environment variables.

Variables extracted from environment:

- GOOGLE_SERVICE_ACCOUNT: location of service account file
- GOOGLE_SERVICE_ACCOUNT_PATH: (alias) location of service account file
- SERVICE_ACCOUNT: (alias) location of service account file
- GOOGLE_SERVICE_ACCOUNT_KEY: JSON serialized service account key
- GOOGLE_BUCKET: bucket name
- GOOGLE_BUCKET_NAME: (alias) bucket name

##### <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#example-1" class="doc-anchor">§</a>Example

``` rust
use object_store::gcp::GoogleCloudStorageBuilder;

let gcs = GoogleCloudStorageBuilder::from_env()
    .with_bucket_name("foo")
    .build();
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_url" class="fn">with_url</a>(self, url: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Parse available connection info form a well-known storage URL.

The supported url schemes are:

- `gs://<bucket>/<path>`

Note: Settings derived from the URL will override any others set on this builder

##### <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#example-2" class="doc-anchor">§</a>Example

``` rust
use object_store::gcp::GoogleCloudStorageBuilder;

let gcs = GoogleCloudStorageBuilder::from_env()
    .with_url("gs://bucket/path")
    .build();
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_config" class="fn">with_config</a>(self, key: <a href="https://docs.rs/object_store/latest/object_store/gcp/enum.GoogleConfigKey.html" class="enum" title="enum object_store::gcp::GoogleConfigKey">GoogleConfigKey</a>, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set an option on the builder via a key - value pair.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.get_config_value" class="fn">get_config_value</a>(&self, key: &<a href="https://docs.rs/object_store/latest/object_store/gcp/enum.GoogleConfigKey.html" class="enum" title="enum object_store::gcp::GoogleConfigKey">GoogleConfigKey</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get config value via a [`GoogleConfigKey`](https://docs.rs/object_store/latest/object_store/gcp/enum.GoogleConfigKey.html "enum object_store::gcp::GoogleConfigKey").

##### <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#example-3" class="doc-anchor">§</a>Example

``` rust
use object_store::gcp::{GoogleCloudStorageBuilder, GoogleConfigKey};

let builder = GoogleCloudStorageBuilder::from_env()
    .with_service_account_key("foo");
let service_account_key = builder.get_config_value(&GoogleConfigKey::ServiceAccountKey).unwrap_or_default();
assert_eq!("foo", &service_account_key);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_bucket_name" class="fn">with_bucket_name</a>(self, bucket_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the bucket name (required)

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_service_account_path" class="fn">with_service_account_path</a>( self, service_account_path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set the path to the service account file.

This or [`GoogleCloudStorageBuilder::with_service_account_key`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_service_account_key "method object_store::gcp::GoogleCloudStorageBuilder::with_service_account_key") must be set.

Example `"/tmp/gcs.json"`.

Example contents of `gcs.json`:

``` json
{
   "gcs_base_url": "https://localhost:4443",
   "disable_oauth": true,
   "client_email": "",
   "private_key": ""
}
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_service_account_key" class="fn">with_service_account_key</a>( self, service_account: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set the service account key. The service account must be in the JSON format.

This or [`GoogleCloudStorageBuilder::with_service_account_path`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_service_account_path "method object_store::gcp::GoogleCloudStorageBuilder::with_service_account_path") must be set.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_application_credentials" class="fn">with_application_credentials</a>( self, application_credentials_path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set the path to the application credentials file.

<https://cloud.google.com/docs/authentication/provide-credentials-adc>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_skip_signature" class="fn">with_skip_signature</a>(self, skip_signature: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

If enabled, [`GoogleCloudStorage`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html "struct object_store::gcp::GoogleCloudStorage") will not fetch credentials and will not sign requests.

This can be useful when interacting with public GCS buckets that deny authorized requests.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_credentials" class="fn">with_credentials</a>(self, credentials: <a href="https://docs.rs/object_store/latest/object_store/gcp/type.GcpCredentialProvider.html" class="type" title="type object_store::gcp::GcpCredentialProvider">GcpCredentialProvider</a>) -\> Self

Set the credential provider overriding any other options

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_retry" class="fn">with_retry</a>(self, retry_config: <a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html" class="struct" title="struct object_store::RetryConfig">RetryConfig</a>) -\> Self

Set the retry configuration

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_proxy_url" class="fn">with_proxy_url</a>(self, proxy_url: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the proxy_url to be used by the underlying client

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_proxy_ca_certificate" class="fn">with_proxy_ca_certificate</a>( self, proxy_ca_certificate: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set a trusted proxy CA certificate

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_proxy_excludes" class="fn">with_proxy_excludes</a>(self, proxy_excludes: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set a list of hosts to exclude from proxy connections

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_client_options" class="fn">with_client_options</a>(self, options: <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>) -\> Self

Sets the client options, overriding any already set

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.with_http_connector" class="fn">with_http_connector</a>\<C: <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html" class="trait" title="trait object_store::client::HttpConnector">HttpConnector</a>\>(self, connector: C) -\> Self

The [`HttpConnector`](https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html "trait object_store::client::HttpConnector") to use

On non-WASM32 platforms uses [`reqwest`](https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/index.html "mod reqwest") by default, on WASM32 platforms must be provided

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html" class="struct" title="struct object_store::gcp::GoogleCloudStorage">GoogleCloudStorage</a>\>

Configure a connection to Google Cloud Storage, returning a new [`GoogleCloudStorage`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html "struct object_store::gcp::GoogleCloudStorage") and consuming `self`

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#impl-Clone-for-GoogleCloudStorageBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html" class="struct" title="struct object_store::gcp::GoogleCloudStorageBuilder">GoogleCloudStorageBuilder</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html" class="struct" title="struct object_store::gcp::GoogleCloudStorageBuilder">GoogleCloudStorageBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#impl-Debug-for-GoogleCloudStorageBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html" class="struct" title="struct object_store::gcp::GoogleCloudStorageBuilder">GoogleCloudStorageBuilder</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#impl-Default-for-GoogleCloudStorageBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html" class="struct" title="struct object_store::gcp::GoogleCloudStorageBuilder">GoogleCloudStorageBuilder</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html#blanket-implementations" class="anchor">§</a>
