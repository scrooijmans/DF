# Struct MicrosoftAzureBuilder Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/azure/builder.rs.html#122-183" class="src">Source</a>

``` rust
pub struct MicrosoftAzureBuilder { /* private fields */ }
```

Available on **crate feature `azure`** only.

Expand description

Configure a connection to Microsoft Azure Blob Storage container using the specified credentials.

## <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
let azure = MicrosoftAzureBuilder::new()
 .with_account(ACCOUNT)
 .with_access_key(ACCESS_KEY)
 .with_container_name(BUCKET_NAME)
 .build();
```

## Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#impl-MicrosoftAzureBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html" class="struct" title="struct object_store::azure::MicrosoftAzureBuilder">MicrosoftAzureBuilder</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.new" class="fn">new</a>() -\> Self

Create a new [`MicrosoftAzureBuilder`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html "struct object_store::azure::MicrosoftAzureBuilder") with default values.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.from_env" class="fn">from_env</a>() -\> Self

Create an instance of [`MicrosoftAzureBuilder`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html "struct object_store::azure::MicrosoftAzureBuilder") with values pre-populated from environment variables.

Variables extracted from environment:

- AZURE_STORAGE_ACCOUNT_NAME: storage account name
- AZURE_STORAGE_ACCOUNT_KEY: storage account master key
- AZURE_STORAGE_ACCESS_KEY: alias for AZURE_STORAGE_ACCOUNT_KEY
- AZURE_STORAGE_CLIENT_ID -\> client id for service principal authorization
- AZURE_STORAGE_CLIENT_SECRET -\> client secret for service principal authorization
- AZURE_STORAGE_TENANT_ID -\> tenant id used in oauth flows

##### <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#example-1" class="doc-anchor">§</a>Example

``` rust
use object_store::azure::MicrosoftAzureBuilder;

let azure = MicrosoftAzureBuilder::from_env()
    .with_container_name("foo")
    .build();
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_url" class="fn">with_url</a>(self, url: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Parse available connection info form a well-known storage URL.

The supported url schemes are:

- `abfs[s]://<container>/<path>` (according to [fsspec](https://github.com/fsspec/adlfs))
- `abfs[s]://<file_system>@<account_name>.dfs.core.windows.net/<path>`
- `abfs[s]://<file_system>@<account_name>.dfs.fabric.microsoft.com/<path>`
- `az://<container>/<path>` (according to [fsspec](https://github.com/fsspec/adlfs))
- `adl://<container>/<path>` (according to [fsspec](https://github.com/fsspec/adlfs))
- `azure://<container>/<path>` (custom)
- `https://<account>.dfs.core.windows.net`
- `https://<account>.blob.core.windows.net`
- `https://<account>.blob.core.windows.net/<container>`
- `https://<account>.dfs.fabric.microsoft.com`
- `https://<account>.dfs.fabric.microsoft.com/<container>`
- `https://<account>.blob.fabric.microsoft.com`
- `https://<account>.blob.fabric.microsoft.com/<container>`

Note: Settings derived from the URL will override any others set on this builder

##### <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#example-2" class="doc-anchor">§</a>Example

``` rust
use object_store::azure::MicrosoftAzureBuilder;

let azure = MicrosoftAzureBuilder::from_env()
    .with_url("abfss://file_system@account.dfs.core.windows.net/")
    .build();
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_config" class="fn">with_config</a>(self, key: <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set an option on the builder via a key - value pair.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.get_config_value" class="fn">get_config_value</a>(&self, key: &<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get config value via a [`AzureConfigKey`](https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html "enum object_store::azure::AzureConfigKey").

##### <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#example-3" class="doc-anchor">§</a>Example

``` rust
use object_store::azure::{MicrosoftAzureBuilder, AzureConfigKey};

let builder = MicrosoftAzureBuilder::from_env()
    .with_account("foo");
let account_name = builder.get_config_value(&AzureConfigKey::AccountName).unwrap_or_default();
assert_eq!("foo", &account_name);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_account" class="fn">with_account</a>(self, account: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the Azure Account (required)

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_container_name" class="fn">with_container_name</a>(self, container_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the Azure Container Name (required)

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_access_key" class="fn">with_access_key</a>(self, access_key: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the Azure Access Key (required - one of access key, bearer token, or client credentials)

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_bearer_token_authorization" class="fn">with_bearer_token_authorization</a>( self, bearer_token: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set a static bearer token to be used for authorizing requests

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_client_secret_authorization" class="fn">with_client_secret_authorization</a>( self, client_id: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, client_secret: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, tenant_id: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set a client secret used for client secret authorization

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_client_id" class="fn">with_client_id</a>(self, client_id: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Sets the client id for use in client secret or k8s federated credential flow

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_client_secret" class="fn">with_client_secret</a>(self, client_secret: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Sets the client secret for use in client secret flow

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_tenant_id" class="fn">with_tenant_id</a>(self, tenant_id: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Sets the tenant id for use in client secret or k8s federated credential flow

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_sas_authorization" class="fn">with_sas_authorization</a>( self, query_pairs: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)\>\>, ) -\> Self

Set query pairs appended to the url for shared access signature authorization

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_credentials" class="fn">with_credentials</a>(self, credentials: <a href="https://docs.rs/object_store/latest/object_store/azure/type.AzureCredentialProvider.html" class="type" title="type object_store::azure::AzureCredentialProvider">AzureCredentialProvider</a>) -\> Self

Set the credential provider overriding any other options

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_use_emulator" class="fn">with_use_emulator</a>(self, use_emulator: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set if the Azure emulator should be used (defaults to false)

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_endpoint" class="fn">with_endpoint</a>(self, endpoint: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Override the endpoint used to communicate with blob storage

Defaults to `https://{account}.blob.core.windows.net`

By default, only HTTPS schemes are enabled. To connect to an HTTP endpoint, enable [`Self::with_allow_http`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_allow_http "method object_store::azure::MicrosoftAzureBuilder::with_allow_http").

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_use_fabric_endpoint" class="fn">with_use_fabric_endpoint</a>(self, use_fabric_endpoint: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set if Microsoft Fabric url scheme should be used (defaults to false)

When disabled the url scheme used is `https://{account}.blob.core.windows.net` When enabled the url scheme used is `https://{account}.dfs.fabric.microsoft.com`

Note: [`Self::with_endpoint`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_endpoint "method object_store::azure::MicrosoftAzureBuilder::with_endpoint") will take precedence over this option

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_allow_http" class="fn">with_allow_http</a>(self, allow_http: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Sets what protocol is allowed

If `allow_http` is :

- false (default): Only HTTPS are allowed
- true: HTTP and HTTPS are allowed

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_authority_host" class="fn">with_authority_host</a>(self, authority_host: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Sets an alternative authority host for OAuth based authorization

Common hosts for azure clouds are defined in [authority_hosts](https://docs.rs/object_store/latest/object_store/azure/authority_hosts/index.html "mod object_store::azure::authority_hosts").

Defaults to <https://login.microsoftonline.com>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_retry" class="fn">with_retry</a>(self, retry_config: <a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html" class="struct" title="struct object_store::RetryConfig">RetryConfig</a>) -\> Self

Set the retry configuration

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_proxy_url" class="fn">with_proxy_url</a>(self, proxy_url: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the proxy_url to be used by the underlying client

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_proxy_ca_certificate" class="fn">with_proxy_ca_certificate</a>( self, proxy_ca_certificate: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set a trusted proxy CA certificate

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_proxy_excludes" class="fn">with_proxy_excludes</a>(self, proxy_excludes: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set a list of hosts to exclude from proxy connections

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_client_options" class="fn">with_client_options</a>(self, options: <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>) -\> Self

Sets the client options, overriding any already set

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_msi_endpoint" class="fn">with_msi_endpoint</a>(self, msi_endpoint: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Sets the endpoint for acquiring managed identity token

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_federated_token_file" class="fn">with_federated_token_file</a>( self, federated_token_file: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Sets a file path for acquiring azure federated identity token in k8s

requires `client_id` and `tenant_id` to be set

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_use_azure_cli" class="fn">with_use_azure_cli</a>(self, use_azure_cli: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set if the Azure Cli should be used for acquiring access token

<https://learn.microsoft.com/en-us/cli/azure/account?view=azure-cli-latest#az-account-get-access-token>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_skip_signature" class="fn">with_skip_signature</a>(self, skip_signature: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

If enabled, [`MicrosoftAzure`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzure.html "struct object_store::azure::MicrosoftAzure") will not fetch credentials and will not sign requests

This can be useful when interacting with public containers

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_disable_tagging" class="fn">with_disable_tagging</a>(self, ignore: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

If set to `true` will ignore any tags provided to put_opts

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_http_connector" class="fn">with_http_connector</a>\<C: <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html" class="trait" title="trait object_store::client::HttpConnector">HttpConnector</a>\>(self, connector: C) -\> Self

The [`HttpConnector`](https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html "trait object_store::client::HttpConnector") to use

On non-WASM32 platforms uses [`reqwest`](https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/index.html "mod reqwest") by default, on WASM32 platforms must be provided

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzure.html" class="struct" title="struct object_store::azure::MicrosoftAzure">MicrosoftAzure</a>\>

Configure a connection to container with given name on Microsoft Azure Blob store.

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#impl-Clone-for-MicrosoftAzureBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html" class="struct" title="struct object_store::azure::MicrosoftAzureBuilder">MicrosoftAzureBuilder</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html" class="struct" title="struct object_store::azure::MicrosoftAzureBuilder">MicrosoftAzureBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#impl-Debug-for-MicrosoftAzureBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html" class="struct" title="struct object_store::azure::MicrosoftAzureBuilder">MicrosoftAzureBuilder</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#impl-Default-for-MicrosoftAzureBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html" class="struct" title="struct object_store::azure::MicrosoftAzureBuilder">MicrosoftAzureBuilder</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html" class="struct" title="struct object_store::azure::MicrosoftAzureBuilder">MicrosoftAzureBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#blanket-implementations" class="anchor">§</a>
