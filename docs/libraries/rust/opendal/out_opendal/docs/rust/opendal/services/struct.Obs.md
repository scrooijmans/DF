# Struct Obs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/obs/backend.rs.html#46-51" class="src">Source</a>

``` rust
pub struct Obs { /* private fields */ }
```

Expand description

Huawei-Cloud Object Storage Service (OBS) support

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for backend
- `bucket`: Set the container name for backend
- `endpoint`: Customizable endpoint setting
- `access_key_id`: Set the access_key_id for backend.
- `secret_access_key`: Set the secret_access_key for backend.

You can refer to [`ObsBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html "struct opendal::services::Obs")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Obs;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Obs::default()
        // set the storage bucket for OpenDAL
        .bucket("test")
        .endpoint("obs.cn-north-1.myhuaweicloud.com")
        // Set the access_key_id and secret_access_key.
        //
        // OpenDAL will try load credential from the env.
        // If credential not set and no valid credential in env, OpenDAL will
        // send request without signing like anonymous user.
        .access_key_id("access_key_id")
        .secret_access_key("secret_access_key");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#impl-ObsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html" class="struct" title="struct opendal::services::Obs">ObsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-obs`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-obs`** only.

Set endpoint of this backend.

Both huaweicloud default domain and user domain endpoints are allowed. Please DO NOT add the bucket name to the endpoint.

- `https://obs.cn-north-4.myhuaweicloud.com`
- `obs.cn-north-4.myhuaweicloud.com` (https by default)
- `https://custom.obs.com` (port should not be set)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.access_key_id" class="fn">access_key_id</a>(self, access_key_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-obs`** only.

Set access_key_id of this backend.

- If it is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.secret_access_key" class="fn">secret_access_key</a>(self, secret_access_key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-obs`** only.

Set secret_access_key of this backend.

- If it is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.bucket" class="fn">bucket</a>(self, bucket: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-obs`** only.

Set bucket of this backend. The param is required.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.enable_versioning" class="fn">enable_versioning</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Available on **crate feature `services-obs`** only.

Set bucket versioning status for this backend

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-obs`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#impl-Builder-for-ObsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html" class="struct" title="struct opendal::services::Obs">ObsBuilder</a>

Available on **crate feature `services-obs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.ObsConfig.html" class="struct" title="struct opendal::services::ObsConfig">ObsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#impl-Clone-for-ObsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html" class="struct" title="struct opendal::services::Obs">ObsBuilder</a>

Available on **crate feature `services-obs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html" class="struct" title="struct opendal::services::Obs">ObsBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#impl-Debug-for-ObsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html" class="struct" title="struct opendal::services::Obs">ObsBuilder</a>

Available on **crate feature `services-obs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#impl-Default-for-ObsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html" class="struct" title="struct opendal::services::Obs">ObsBuilder</a>

Available on **crate feature `services-obs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html" class="struct" title="struct opendal::services::Obs">ObsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html#blanket-implementations" class="anchor">Â§</a>
