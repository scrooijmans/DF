# Struct Gdrive Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/gdrive/builder.rs.html#42-47" class="src">Source</a>

``` rust
pub struct Gdrive { /* private fields */ }
```

Expand description

[GoogleDrive](https://drive.google.com/) backend support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ delete
- ☒ create_dir
- ☒ list
- ☒ copy
- ☒ rename
- ☐ batch

## <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for backend

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#credentials-related" class="doc-anchor">Â§</a>Credentials related

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#just-provide-access-token-temporary" class="doc-anchor">Â§</a>Just provide Access Token (Temporary)

- `access_token`: set the access_token for google drive api Please notice its expiration.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#or-provide-client-id-and-client-secret-and-refresh-token-long-term" class="doc-anchor">Â§</a>Or provide Client ID and Client Secret and refresh token (Long Term)

If you want to let OpenDAL to refresh the access token automatically, please provide the following fields:

- `refresh_token`: set the refresh_token for google drive api
- `client_id`: set the client_id for google drive api
- `client_secret`: set the client_secret for google drive api

OpenDAL is a library, it cannot do the first step of OAuth2 for you. You need to get authorization code from user by calling GoogleDriveâ€™s authorize url and exchange it for refresh token.

Make sure you have enabled Google Drive API in your Google Cloud Console. And your OAuth scope contains `https://www.googleapis.com/auth/drive`.

Please refer to [GoogleDrive OAuth2 Flow](https://developers.google.com/identity/protocols/oauth2/) for more information.

You can refer to [`GdriveBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html "struct opendal::services::Gdrive")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Gdrive;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Gdrive::default()
        .root("/test")
        .access_token("<token>");

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#impl-GdriveBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html" class="struct" title="struct opendal::services::Gdrive">GdriveBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gdrive`** only.

Set root path of GoogleDrive folder.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#method.access_token" class="fn">access_token</a>(self, access_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gdrive`** only.

Access token is used for temporary access to the GoogleDrive API.

You can get the access token from [GoogleDrive App Console](https://console.cloud.google.com/apis/credentials) or [GoogleDrive OAuth2 Playground](https://developers.google.com/oauthplayground/)

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#note" class="doc-anchor">Â§</a>Note

- An access token is valid for 1 hour.
- If you want to use the access token for a long time, you can use the refresh token to get a new access token.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#method.refresh_token" class="fn">refresh_token</a>(self, refresh_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gdrive`** only.

Refresh token is used for long term access to the GoogleDrive API.

You can get the refresh token via OAuth 2.0 Flow of GoogleDrive API.

OpenDAL will use this refresh token to get a new access token when the old one is expired.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#method.client_id" class="fn">client_id</a>(self, client_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gdrive`** only.

Set the client id for GoogleDrive.

This is required for OAuth 2.0 Flow to refresh the access token.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#method.client_secret" class="fn">client_secret</a>(self, client_secret: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gdrive`** only.

Set the client secret for GoogleDrive.

This is required for OAuth 2.0 Flow with refresh the access token.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#method.http_client" class="fn">http_client</a>(self, http_client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-gdrive`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#impl-Builder-for-GdriveBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html" class="struct" title="struct opendal::services::Gdrive">GdriveBuilder</a>

Available on **crate feature `services-gdrive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GdriveConfig.html" class="struct" title="struct opendal::services::GdriveConfig">GdriveConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#impl-Debug-for-GdriveBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html" class="struct" title="struct opendal::services::Gdrive">GdriveBuilder</a>

Available on **crate feature `services-gdrive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#impl-Default-for-GdriveBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html" class="struct" title="struct opendal::services::Gdrive">GdriveBuilder</a>

Available on **crate feature `services-gdrive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html" class="struct" title="struct opendal::services::Gdrive">GdriveBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html#blanket-implementations" class="anchor">Â§</a>
