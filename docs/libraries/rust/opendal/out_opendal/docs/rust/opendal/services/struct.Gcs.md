# Struct Gcs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/gcs/backend.rs.html#47-53" class="src">Source</a>

``` rust
pub struct Gcs { /* private fields */ }
```

Expand description

[Google Cloud Storage](https://cloud.google.com/storage) services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☐ rename
- ☒ list
- ☒ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for backend
- `bucket`: Set the container name for backend
- `endpoint`: Customizable endpoint setting
- `credential`: Service Account or External Account JSON, in base64
- `credential_path`: local path to Service Account or External Account JSON file
- `service_account`: name of Service Account
- `predefined_acl`: Predefined ACL for GCS
- `default_storage_class`: Default storage class for GCS

Refer to public API docs for more information. For authentication related options, read on.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#options-to-authenticate-to-gcs" class="doc-anchor">Â§</a>Options to authenticate to GCS

OpenDAL supports the following authentication options:

1.  Provide a base64-ed JSON key string with `credential`
2.  Provide a JSON key file at explicit path with `credential_path`
3.  Provide a JSON key file at implicit path
    - `GcsBackend` will attempt to load Service Account key from [ADC well-known places](https://cloud.google.com/docs/authentication/application-default-credentials).
4.  Fetch access token from [VM metadata](https://cloud.google.com/docs/authentication/rest#metadata-server)
    - Only works when running inside Google Cloud.
    - If a non-default Service Account name is required, set with `service_account`. Otherwise, nothing need to be set.
5.  A custom `TokenLoader` via `GcsBuilder.customized_token_loader()`

Notes:

- When a Service Account key is provided, it will be used to create access tokens (VM metadata will not be used).
- Explicit Service Account key, in json or path, always take precedence over ADC-defined key paths.
- Due to [limitation in GCS](https://cloud.google.com/storage/docs/authentication/signatures#signing-process), a private key is required to create Pre-signed URL. Currently, OpenDAL only supports Service Account key.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Gcs;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Gcs::default()
       // set the storage bucket for OpenDAL
       .bucket("test")
       // set the working directory root for GCS
       // all operations will happen within it
       .root("/path/to/dir")
       // set the credentials with service account
       .credential("service account JSON in base64")
       // set the predefined ACL for GCS
       .predefined_acl("publicRead")
       // set the default storage class for GCS
       .default_storage_class("STANDARD");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#impl-GcsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html" class="struct" title="struct opendal::services::Gcs">GcsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gcs`** only.

set the working directory root of backend

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.bucket" class="fn">bucket</a>(self, bucket: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gcs`** only.

set the containerâ€™s name

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.scope" class="fn">scope</a>(self, scope: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gcs`** only.

set the GCS service scope

If not set, we will use `https://www.googleapis.com/auth/devstorage.read_write`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#valid-scope-examples" class="doc-anchor">Â§</a>Valid scope examples

- read-only: `https://www.googleapis.com/auth/devstorage.read_only`
- read-write: `https://www.googleapis.com/auth/devstorage.read_write`
- full-control: `https://www.googleapis.com/auth/devstorage.full_control`

Reference: [Cloud Storage authentication](https://cloud.google.com/storage/docs/authentication)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.service_account" class="fn">service_account</a>(self, service_account: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gcs`** only.

Set the GCS service account.

service account will be used for fetch token from vm metadata. If not set, we will try to fetch with `default` service account.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gcs`** only.

set the endpoint GCS service uses

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.credential" class="fn">credential</a>(self, credential: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gcs`** only.

set the base64 hashed credentials string used for OAuth2 authentication.

this method allows to specify the credentials directly as a base64 hashed string. alternatively, you can use `credential_path()` to provide the local path to a credentials file. we will use one of `credential` and `credential_path` to complete the OAuth2 authentication.

Reference: [Google Cloud Storage Authentication](https://cloud.google.com/docs/authentication).

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.credential_path" class="fn">credential_path</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gcs`** only.

set the local path to credentials file which is used for OAuth2 authentication.

credentials file contains the original credentials that have not been base64 hashed. we will use one of `credential` and `credential_path` to complete the OAuth2 authentication.

Reference: [Google Cloud Storage Authentication](https://cloud.google.com/docs/authentication).

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-gcs`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.customized_token_loader" class="fn">customized_token_loader</a>( self, token_load: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn GoogleTokenLoad\>, ) -\> Self

Available on **crate feature `services-gcs`** only.

Specify the customized token loader used by this service.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.token" class="fn">token</a>(self, token: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Available on **crate feature `services-gcs`** only.

Provide the OAuth2 token to use.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.disable_vm_metadata" class="fn">disable_vm_metadata</a>(self) -\> Self

Available on **crate feature `services-gcs`** only.

Disable attempting to load credentials from the GCE metadata server.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.disable_config_load" class="fn">disable_config_load</a>(self) -\> Self

Available on **crate feature `services-gcs`** only.

Disable loading configuration from the environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.predefined_acl" class="fn">predefined_acl</a>(self, acl: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gcs`** only.

Set the predefined acl for GCS.

Available values are:

- `authenticatedRead`
- `bucketOwnerFullControl`
- `bucketOwnerRead`
- `private`
- `projectPrivate`
- `publicRead`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.default_storage_class" class="fn">default_storage_class</a>(self, class: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gcs`** only.

Set the default storage class for GCS.

Available values are:

- `STANDARD`
- `NEARLINE`
- `COLDLINE`
- `ARCHIVE`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.allow_anonymous" class="fn">allow_anonymous</a>(self) -\> Self

Available on **crate feature `services-gcs`** only.

Allow anonymous requests.

This is typically used for buckets which are open to the public or GCS storage emulators.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#impl-Builder-for-GcsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html" class="struct" title="struct opendal::services::Gcs">GcsBuilder</a>

Available on **crate feature `services-gcs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GcsConfig.html" class="struct" title="struct opendal::services::GcsConfig">GcsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#impl-Debug-for-GcsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html" class="struct" title="struct opendal::services::Gcs">GcsBuilder</a>

Available on **crate feature `services-gcs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#impl-Default-for-GcsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html" class="struct" title="struct opendal::services::Gcs">GcsBuilder</a>

Available on **crate feature `services-gcs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html" class="struct" title="struct opendal::services::Gcs">GcsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html#blanket-implementations" class="anchor">Â§</a>
