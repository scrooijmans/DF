# Struct Dropbox Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/dropbox/builder.rs.html#34-39" class="src">Source</a>

``` rust
pub struct Dropbox { /* private fields */ }
```

Expand description

[Dropbox](https://www.dropbox.com/) backend support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☒ list
- ☒ batch
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for this backend.

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#credentials-related" class="doc-anchor">Â§</a>Credentials related

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#just-provide-access-token-temporary" class="doc-anchor">Â§</a>Just provide Access Token (Temporary)

- `access_token`: set the access_token for this backend. Please notice its expiration.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#or-provide-client-id-and-client-secret-and-refresh-token-long-term" class="doc-anchor">Â§</a>Or provide Client ID and Client Secret and refresh token (Long Term)

If you want to let OpenDAL to refresh the access token automatically, please provide the following fields:

- `refresh_token`: set the refresh_token for dropbox api
- `client_id`: set the client_id for dropbox api
- `client_secret`: set the client_secret for dropbox api

OpenDAL is a library, it cannot do the first step of OAuth2 for you. You need to get authorization code from user by calling Dropboxâ€™s authorize url and exchange it for refresh token.

Please refer to [Dropbox OAuth2 Guide](https://www.dropbox.com/developers/reference/oauth-guide) for more information.

You can refer to [`DropboxBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html "struct opendal::services::Dropbox")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::raw::OpWrite;
use opendal::services::Dropbox;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Dropbox::default()
        .root("/opendal")
        .access_token("<token>");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#impl-DropboxBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html" class="struct" title="struct opendal::services::Dropbox">DropboxBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-dropbox`** only.

Set the root directory for dropbox.

Default to `/` if not set.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#method.access_token" class="fn">access_token</a>(self, access_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-dropbox`** only.

Access token is used for temporary access to the Dropbox API.

You can get the access token from [Dropbox App Console](https://www.dropbox.com/developers/apps)

NOTE: this token will be expired in 4 hours. If you are trying to use the Dropbox service in a long time, please set a refresh_token instead.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#method.refresh_token" class="fn">refresh_token</a>(self, refresh_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-dropbox`** only.

Refresh token is used for long term access to the Dropbox API.

You can get the refresh token via OAuth 2.0 Flow of Dropbox.

OpenDAL will use this refresh token to get a new access token when the old one is expired.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#method.client_id" class="fn">client_id</a>(self, client_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-dropbox`** only.

Set the client id for Dropbox.

This is required for OAuth 2.0 Flow to refresh the access token.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#method.client_secret" class="fn">client_secret</a>(self, client_secret: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-dropbox`** only.

Set the client secret for Dropbox.

This is required for OAuth 2.0 Flow with refresh the access token.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#method.http_client" class="fn">http_client</a>(self, http_client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-dropbox`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#impl-Builder-for-DropboxBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html" class="struct" title="struct opendal::services::Dropbox">DropboxBuilder</a>

Available on **crate feature `services-dropbox`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DropboxConfig.html" class="struct" title="struct opendal::services::DropboxConfig">DropboxConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#impl-Debug-for-DropboxBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html" class="struct" title="struct opendal::services::Dropbox">DropboxBuilder</a>

Available on **crate feature `services-dropbox`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#impl-Default-for-DropboxBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html" class="struct" title="struct opendal::services::Dropbox">DropboxBuilder</a>

Available on **crate feature `services-dropbox`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html" class="struct" title="struct opendal::services::Dropbox">DropboxBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html#blanket-implementations" class="anchor">Â§</a>
