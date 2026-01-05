# Struct Onedrive Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/onedrive/builder.rs.html#40-43" class="src">Source</a>

``` rust
pub struct Onedrive { /* private fields */ }
```

Expand description

Microsoft [OneDrive](https://onedrive.com) backend support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☐ append
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☒ list
- ☐ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#notes" class="doc-anchor">Â§</a>Notes

Currently, OpenDAL supports OneDrive Personal only.

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#write-operations-and-onedrive-behavior" class="doc-anchor">Â§</a>Write Operations and OneDrive Behavior

For write-related operations, such as:

- write
- rename
- copy
- create_dir

OpenDALâ€™s OneDrive service replaces the destination folder instead of rename it.

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#consistency-issues-with-concurrent-requests" class="doc-anchor">Â§</a>Consistency Issues with Concurrent Requests

OneDrive does not guarantee consistency when handling a large number of concurrent requests write operations.

In some extreme cases, OneDrive may acknowledge an operation as successful but fail to commit the changes. This inconsistency can cause subsequent operations to fail, returning errors like:

- 400 Bad Request: OneDrive considers folders in the path are not there yet
- 404 Not Found: OneDrive doesnâ€™t recognize the created folder
- 409 Conflict: OneDrive canâ€™t replace an existing folder

You should consider \[`RetryLayer`\] and monitor your operations carefully.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#configuration" class="doc-anchor">Â§</a>Configuration

- `access_token`: Set a short-live access token for Microsoft Graph API (also, OneDrive API)
- `refresh_token`: Set a long term access token for Microsoft Graph API
- `client_id`: Set the client ID for a Microsoft Graph API application (available though Azureâ€™s registration portal)
- `client_secret`: Set the client secret for a Microsoft Graph API application
- `root`: Set the work directory for OneDrive backend
- `enable_versioning`: Enable versioning support for OneDrive items

The configuration for tokens is one of the following:

- `access_token` only, for short-lived access. Once the `access_token` expires, you must recreate the operator with a new token.
- `refresh_token`, `client_id`, and an optional `client_secret`, for long-lived access. The operator will automatically get and refresh the access token.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#how-to-get-tokens" class="doc-anchor">Â§</a>How to get tokens

1.  Create an application: navigate to [Microsoft Entra Admin Center](https://entra.microsoft.com/) -\> Applications -\> App Registrations -\> New registration
2.  In â€œSupported account typesâ€?, choose â€œAccounts in any organizational directory (Any Microsoft Entra ID tenant - Multitenant) and personal Microsoft accounts (e.g. Skype, Xbox)â€?
3.  Or in an existing application -\> manifest, make sure `signInAudience` is `"AzureADandPersonalMicrosoftAccount"`. Thatâ€™s because weâ€™re calling Graph API with `/common` path segment.
4.  The platform you choose determines whether you have to provide a `client_secret` or not. See [Public and confidential client apps](https://learn.microsoft.com/en-us/entra/identity-platform/msal-client-applications) for more information.
    1.  In short, if you choose â€œMobile and desktop applicationsâ€? or â€œSingle-page applicationâ€? (Public Client), you must not provide `client_secret`.
    2.  If you choose â€œWebâ€? (Confidential Client), create a secret in â€œCertificates & secrets -\> Client secrets -\> New client secretâ€?, and provide it as `client_secret`.
5.  Follow the [code grant flow](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow) or other flows to get the access_token. The minimum scope is `Files.ReadWrite`. And make sure the access token represents a user, because itâ€™s accessing the userâ€™s onedrive by `/me/drive`. So â€œclient credentials flowâ€? wonâ€™t work.
6.  If you need `refresh_token` for long-lived access, add an additional `offline_access` scope.

Read more at [`OnedriveBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html "struct opendal::services::Onedrive").

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#via-builder" class="doc-anchor">Â§</a>Via Builder

When you have a current access token:

``` rust
use anyhow::Result;
use opendal::services::Onedrive;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Onedrive::default()
        .access_token("my_access_token")
        .root("/root/folder/for/operator");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

When you have an Application with a refresh token:

``` rust
use anyhow::Result;
use opendal::services::Onedrive;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Onedrive::default()
        .refresh_token("my_refresh_token")
        .client_id("my_client_id")
        .root("/root/folder/for/operator");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#impl-OnedriveBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html" class="struct" title="struct opendal::services::Onedrive">OnedriveBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-onedrive`** only.

Set root path of OneDrive folder.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#method.http_client" class="fn">http_client</a>(self, http_client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-onedrive`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#notes-1" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#method.access_token" class="fn">access_token</a>(self, access_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-onedrive`** only.

Set the access token for a time limited access to Microsoft Graph API (also OneDrive).

Microsoft Graph API uses a typical OAuth 2.0 flow for authentication and authorization. You can get a access token from [Microsoft Graph Explore](https://developer.microsoft.com/en-us/graph/graph-explorer).

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#note" class="doc-anchor">Â§</a>Note

- An access token is short-lived.
- Use a refresh_token if you want to use OneDrive API for an extended period of time.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#method.refresh_token" class="fn">refresh_token</a>(self, refresh_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-onedrive`** only.

Set the refresh token for long term access to Microsoft Graph API.

OpenDAL will use a refresh token to maintain a fresh access token automatically.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#note-1" class="doc-anchor">Â§</a>Note

- A refresh token is available through a OAuth 2.0 flow, with an additional scope `offline_access`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#method.client_id" class="fn">client_id</a>(self, client_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-onedrive`** only.

Set the client_id for a Microsoft Graph API application (available though Azureâ€™s registration portal)

Required when using the refresh token.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#method.client_secret" class="fn">client_secret</a>(self, client_secret: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-onedrive`** only.

Set the client_secret for a Microsoft Graph API application

Required for Web app when using the refresh token. Donâ€™t use a client secret when use in a native app since the native app canâ€™t store the secret reliably.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#method.enable_versioning" class="fn">enable_versioning</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Available on **crate feature `services-onedrive`** only.

Enable versioning support for OneDrive

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#impl-Builder-for-OnedriveBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html" class="struct" title="struct opendal::services::Onedrive">OnedriveBuilder</a>

Available on **crate feature `services-onedrive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OnedriveConfig.html" class="struct" title="struct opendal::services::OnedriveConfig">OnedriveConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#impl-Debug-for-OnedriveBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html" class="struct" title="struct opendal::services::Onedrive">OnedriveBuilder</a>

Available on **crate feature `services-onedrive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#impl-Default-for-OnedriveBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html" class="struct" title="struct opendal::services::Onedrive">OnedriveBuilder</a>

Available on **crate feature `services-onedrive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html" class="struct" title="struct opendal::services::Onedrive">OnedriveBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html#blanket-implementations" class="anchor">Â§</a>
