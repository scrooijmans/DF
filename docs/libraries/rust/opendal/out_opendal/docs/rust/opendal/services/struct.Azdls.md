# Struct Azdls Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/azdls/backend.rs.html#59-64" class="src">Source</a>

``` rust
pub struct Azdls { /* private fields */ }
```

Expand description

Azure Data Lake Storage Gen2 Support. As known as `abfs`, `azdls` or `azdls`.

This service will visit the [ABFS](https://learn.microsoft.com/en-us/azure/storage/blobs/data-lake-storage-abfs-driver) URI supported by [Azure Data Lake Storage Gen2](https://learn.microsoft.com/en-us/azure/storage/blobs/data-lake-storage-introduction).

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#notes" class="doc-anchor">Â§</a>Notes

`azdls` is different from `azfile` service which used to visit [Azure File Storage](https://azure.microsoft.com/en-us/services/storage/files/).

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☐ copy
- ☒ rename
- ☒ list
- ☐ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work dir for backend.
- `filesystem`: Set the filesystem name for backend.
- `endpoint`: Set the endpoint for backend.
- `account_name`: Set the account_name for backend.
- `account_key`: Set the account_key for backend.

Refer to public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Azdls;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Create azdls backend builder.
    let mut builder = Azdls::default()
        // Set the root for azdls, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        .root("/path/to/dir")
        // Set the filesystem name, this is required.
        .filesystem("test")
        // Set the endpoint, this is required.
        //
        // For examples:
        // - "https://accountname.dfs.core.windows.net"
        .endpoint("https://accountname.dfs.core.windows.net")
        // Set the account_name and account_key.
        //
        // OpenDAL will try load credential from the env.
        // If credential not set and no valid credential in env, OpenDAL will
        // send request without signing like anonymous user.
        .account_name("account_name")
        .account_key("account_key");

    // `Accessor` provides the low level APIs, we will use `Operator` normally.
    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#impl-AzdlsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html" class="struct" title="struct opendal::services::Azdls">AzdlsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azdls`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.filesystem" class="fn">filesystem</a>(self, filesystem: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azdls`** only.

Set filesystem name of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azdls`** only.

Set endpoint of this backend.

Endpoint must be full uri, e.g.

- Azblob: `https://accountname.blob.core.windows.net`
- Azurite: `http://127.0.0.1:10000/devstoreaccount1`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.account_name" class="fn">account_name</a>(self, account_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azdls`** only.

Set account_name of this backend.

- If account_name is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.account_key" class="fn">account_key</a>(self, account_key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azdls`** only.

Set account_key of this backend.

- If account_key is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.client_secret" class="fn">client_secret</a>(self, client_secret: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azdls`** only.

Set client_secret of this backend.

- If client_secret is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.
- required for client_credentials authentication

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.tenant_id" class="fn">tenant_id</a>(self, tenant_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azdls`** only.

Set tenant_id of this backend.

- If tenant_id is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.
- required for client_credentials authentication

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.client_id" class="fn">client_id</a>(self, client_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azdls`** only.

Set client_id of this backend.

- If client_id is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.
- required for client_credentials authentication

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.sas_token" class="fn">sas_token</a>(self, sas_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azdls`** only.

Set the sas_token of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.authority_host" class="fn">authority_host</a>(self, authority_host: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-azdls`** only.

Set authority_host of this backend.

- If authority_host is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.
- default value: `https://login.microsoftonline.com`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-azdls`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#notes-1" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.from_connection_string" class="fn">from_connection_string</a>(conn_str: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Available on **crate feature `services-azdls`** only.

Create a new `AzdlsBuilder` instance from an [Azure Storage connection string](https://learn.microsoft.com/en-us/azure/storage/common/storage-configure-connection-string).

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#example-1" class="doc-anchor">Â§</a>Example

``` rust
use opendal::Builder;
use opendal::services::Azdls;

let conn_str = "AccountName=example;DefaultEndpointsProtocol=https;EndpointSuffix=core.windows.net";

let mut config = Azdls::from_connection_string(&conn_str)
    .unwrap()
    // Add additional configuration if needed
    .filesystem("myFilesystem")
    .client_id("myClientId")
    .client_secret("myClientSecret")
    .tenant_id("myTenantId")
    .build()
    .unwrap();
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#impl-Builder-for-AzdlsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html" class="struct" title="struct opendal::services::Azdls">AzdlsBuilder</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#impl-Clone-for-AzdlsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html" class="struct" title="struct opendal::services::Azdls">AzdlsBuilder</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html" class="struct" title="struct opendal::services::Azdls">AzdlsBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#impl-Debug-for-AzdlsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html" class="struct" title="struct opendal::services::Azdls">AzdlsBuilder</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#impl-Default-for-AzdlsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html" class="struct" title="struct opendal::services::Azdls">AzdlsBuilder</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html" class="struct" title="struct opendal::services::Azdls">AzdlsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html#blanket-implementations" class="anchor">Â§</a>
