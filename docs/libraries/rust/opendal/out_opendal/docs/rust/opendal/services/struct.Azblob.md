# Struct Azblob Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/azblob/backend.rs.html#61-66" class="src">Source</a>

``` rust
pub struct Azblob { /* private fields */ }
```

Expand description

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ append
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☐ rename
- ☒ list
- ☒ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work dir for backend.
- `container`: Set the container name for backend.
- `endpoint`: Set the endpoint for backend.
- `account_name`: Set the account_name for backend.
- `account_key`: Set the account_key for backend.

Refer to public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#examples" class="doc-anchor">Â§</a>Examples

This example works on [Azurite](https://github.com/Azure/Azurite) for local developments.

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#start-local-blob-service" class="doc-anchor">Â§</a>Start local blob service

``` shell
docker run -p 10000:10000 mcr.microsoft.com/azure-storage/azurite
az storage container create --name test --connection-string "DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;BlobEndpoint=http://127.0.0.1:10000/devstoreaccount1;"
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#init-opendal-operator" class="doc-anchor">Â§</a>Init OpenDAL Operator

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Azblob;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Create azblob backend builder.
    let mut builder = Azblob::default()
        // Set the root for azblob, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        .root("/path/to/dir")
        // Set the container name, this is required.
        .container("test")
        // Set the endpoint, this is required.
        //
        // For examples:
        // - "http://127.0.0.1:10000/devstoreaccount1"
        // - "https://accountname.blob.core.windows.net"
        .endpoint("http://127.0.0.1:10000/devstoreaccount1")
        // Set the account_name and account_key.
        //
        // OpenDAL will try load credential from the env.
        // If credential not set and no valid credential in env, OpenDAL will
        // send request without signing like anonymous user.
        .account_name("devstoreaccount1")
        .account_key("Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==");

    // `Accessor` provides the low level APIs, we will use `Operator` normally.
    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#impl-AzblobBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html" class="struct" title="struct opendal::services::Azblob">AzblobBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azblob`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.container" class="fn">container</a>(self, container: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azblob`** only.

Set container name of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azblob`** only.

Set endpoint of this backend

Endpoint must be full uri, e.g.

- Azblob: `https://accountname.blob.core.windows.net`
- Azurite: `http://127.0.0.1:10000/devstoreaccount1`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.account_name" class="fn">account_name</a>(self, account_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azblob`** only.

Set account_name of this backend.

- If account_name is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.account_key" class="fn">account_key</a>(self, account_key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azblob`** only.

Set account_key of this backend.

- If account_key is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.encryption_key" class="fn">encryption_key</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azblob`** only.

Set encryption_key of this backend.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#args" class="doc-anchor">Â§</a>Args

`v`: Base64-encoded key that matches algorithm specified in `encryption_algorithm`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#note" class="doc-anchor">Â§</a>Note

This function is the low-level setting for SSE related features.

SSE related options should be set carefully to make them works. Please use `server_side_encryption_with_*` helpers if even possible.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.encryption_key_sha256" class="fn">encryption_key_sha256</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azblob`** only.

Set encryption_key_sha256 of this backend.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#args-1" class="doc-anchor">Â§</a>Args

`v`: Base64-encoded SHA256 digest of the key specified in encryption_key.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#note-1" class="doc-anchor">Â§</a>Note

This function is the low-level setting for SSE related features.

SSE related options should be set carefully to make them works. Please use `server_side_encryption_with_*` helpers if even possible.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.encryption_algorithm" class="fn">encryption_algorithm</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azblob`** only.

Set encryption_algorithm of this backend.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#args-2" class="doc-anchor">Â§</a>Args

`v`: server-side encryption algorithm. (Available values: `AES256`)

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#note-2" class="doc-anchor">Â§</a>Note

This function is the low-level setting for SSE related features.

SSE related options should be set carefully to make them works. Please use `server_side_encryption_with_*` helpers if even possible.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.server_side_encryption_with_customer_key" class="fn">server_side_encryption_with_customer_key</a>(self, key: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> Self

Available on **crate feature `services-azblob`** only.

Enable server side encryption with customer key.

As known as: CPK

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#args-3" class="doc-anchor">Â§</a>Args

`key`: Base64-encoded SHA256 digest of the key specified in encryption_key.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#note-3" class="doc-anchor">Â§</a>Note

Function that helps the user to set the server-side customer-provided encryption key, the keyâ€™s SHA256, and the algorithm. See [Server-side encryption with customer-provided keys (CPK)](https://learn.microsoft.com/en-us/azure/storage/blobs/encryption-customer-provided-keys) for more info.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.sas_token" class="fn">sas_token</a>(self, sas_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azblob`** only.

Set sas_token of this backend.

- If sas_token is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

See [Grant limited access to Azure Storage resources using shared access signatures (SAS)](https://learn.microsoft.com/en-us/azure/storage/common/storage-sas-overview) for more info.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-azblob`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.batch_max_operations" class="fn">batch_max_operations</a>(self, batch_max_operations: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Available on **crate feature `services-azblob`** only.

Set maximum batch operations of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.from_connection_string" class="fn">from_connection_string</a>(conn: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Available on **crate feature `services-azblob`** only.

from_connection_string will make a builder from connection string

connection string looks like:

``` txt
DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;
AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;
BlobEndpoint=http://127.0.0.1:10000/devstoreaccount1;
QueueEndpoint=http://127.0.0.1:10001/devstoreaccount1;
TableEndpoint=http://127.0.0.1:10002/devstoreaccount1;
```

Or

``` txt
DefaultEndpointsProtocol=https;
AccountName=storagesample;
AccountKey=<account-key>;
EndpointSuffix=core.chinacloudapi.cn;
```

For reference: [Configure Azure Storage connection strings](https://learn.microsoft.com/en-us/azure/storage/common/storage-configure-connection-string)

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#note-4" class="doc-anchor">Â§</a>Note

Connection strings can only configure the endpoint, account name and authentication information. Users still need to configure container name.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#impl-Builder-for-AzblobBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html" class="struct" title="struct opendal::services::Azblob">AzblobBuilder</a>

Available on **crate feature `services-azblob`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzblobConfig.html" class="struct" title="struct opendal::services::AzblobConfig">AzblobConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#impl-Clone-for-AzblobBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html" class="struct" title="struct opendal::services::Azblob">AzblobBuilder</a>

Available on **crate feature `services-azblob`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html" class="struct" title="struct opendal::services::Azblob">AzblobBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#impl-Debug-for-AzblobBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html" class="struct" title="struct opendal::services::Azblob">AzblobBuilder</a>

Available on **crate feature `services-azblob`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#impl-Default-for-AzblobBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html" class="struct" title="struct opendal::services::Azblob">AzblobBuilder</a>

Available on **crate feature `services-azblob`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html" class="struct" title="struct opendal::services::Azblob">AzblobBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html#blanket-implementations" class="anchor">Â§</a>
