# Struct Oss Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/oss/backend.rs.html#47-52" class="src">Source</a>

``` rust
pub struct Oss { /* private fields */ }
```

Expand description

Aliyun Object Storage Service (OSS) support

## <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

## <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work dir for backend.
- `bucket`: Set the container name for backend.
- `endpoint`: Set the endpoint for backend.
- `addressing_style`: Set the addressing style for endpoint.
- `presign_endpoint`: Set the endpoint for presign.
- `presign_addressing_style`: Set the addressing style for presign endpoint.
- `access_key_id`: Set the access_key_id for backend.
- `access_key_secret`: Set the access_key_secret for backend.
- `role_arn`: Set the role of backend.
- `oidc_token`: Set the oidc_token for backend.
- `allow_anonymous`: Set the backend access OSS in anonymous way.

Refer to [`OssBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html "struct opendal::services::Oss")â€™s public API docs for more information.

## <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#example" class="doc-anchor">Â§</a>Example

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Oss;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Create OSS backend builder.
    let mut builder = Oss::default()
        // Set the root for oss, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        .root("/path/to/dir")
        // Set the bucket name, this is required.
        .bucket("test")
        // Set the endpoint.
        //
        // For example:
        // - "https://oss-ap-northeast-1.aliyuncs.com"
        // - "https://oss-hangzhou.aliyuncs.com"
        .endpoint("https://oss-cn-beijing.aliyuncs.com")
        // Set the access_key_id and access_key_secret.
        //
        // OpenDAL will try load credential from the env.
        // If credential not set and no valid credential in env, OpenDAL will
        // send request without signing like anonymous user.
        .access_key_id("access_key_id")
        .access_key_secret("access_key_secret");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#impl-OssBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html" class="struct" title="struct opendal::services::Oss">OssBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.bucket" class="fn">bucket</a>(self, bucket: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set bucket name of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set endpoint of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.addressing_style" class="fn">addressing_style</a>(self, addressing_style: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set addressing style for the endpoint.

Available values: `virtual`, `cname`, `path`.

- `virtual`: Use virtual addressing style, i.e. `http://bucket.oss-<region>.aliyuncs.com/object`

- `cname`: Use cname addressing style, i.e. `http://mydomain.com/object` with mydomain.com bound to your bucket.

- `path`: Use path addressing style. i.e. `http://oss-<region>.aliyuncs.com/bucket/object`

- If not set, default value is `virtual`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.enable_versioning" class="fn">enable_versioning</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set bucket versioning status for this backend

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.presign_endpoint" class="fn">presign_endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set an endpoint for generating presigned urls.

You can offer a public endpoint like <https://oss-cn-beijing.aliyuncs.com> to return a presinged url for public accessors, along with an internal endpoint like <https://oss-cn-beijing-internal.aliyuncs.com> to access objects in a faster path.

- If presign_endpoint is set, we will use presign_endpoint on generating presigned urls.
- if not, we will use endpoint as default.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.presign_addressing_style" class="fn">presign_addressing_style</a>(self, addressing_style: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set addressing style for presign endpoint.

Similar to setting addressing style for endpoint.

- If both presign_endpoint and presign_addressing_style are not set, they are the same as endpointâ€™s configurations.

- If presign_endpoint is set, but presign_addressing_style is not set, default value is `virtual`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.access_key_id" class="fn">access_key_id</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set access_key_id of this backend.

- If access_key_id is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.access_key_secret" class="fn">access_key_secret</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set access_key_secret of this backend.

- If access_key_secret is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.security_token" class="fn">security_token</a>(self, security_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set security_token for this backend.

- If security_token is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-oss`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.server_side_encryption" class="fn">server_side_encryption</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set server_side_encryption for this backend.

Available values: `AES256`, `KMS`.

Reference: <https://www.alibabacloud.com/help/en/object-storage-service/latest/server-side-encryption-5> Brief explanation: There are two server-side encryption methods available: SSE-AES256: 1. Configure the bucket encryption mode as OSS-managed and specify the encryption algorithm as AES256. 2. Include the `x-oss-server-side-encryption` parameter in the request and set its value to AES256. SSE-KMS: 1. To use this service, you need to first enable KMS. 2. Configure the bucket encryption mode as KMS, and specify the specific CMK ID for BYOK (Bring Your Own Key) or not specify the specific CMK ID for OSS-managed KMS key. 3. Include the `x-oss-server-side-encryption` parameter in the request and set its value to KMS. 4. If a specific CMK ID is specified, include the `x-oss-server-side-encryption-key-id` parameter in the request, and set its value to the specified CMK ID.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.server_side_encryption_key_id" class="fn">server_side_encryption_key_id</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set server_side_encryption_key_id for this backend.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#notes-1" class="doc-anchor">Â§</a>Notes

This option only takes effect when server_side_encryption equals to KMS.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.batch_max_operations" class="fn">batch_max_operations</a>(self, delete_max_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

ðŸ‘ŽDeprecated since 0.52.0: Please use `delete_max_size` instead of `batch_max_operations`

Available on **crate feature `services-oss`** only.

Set maximum batch operations of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.delete_max_size" class="fn">delete_max_size</a>(self, delete_max_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set maximum delete operations of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.allow_anonymous" class="fn">allow_anonymous</a>(self) -\> Self

Available on **crate feature `services-oss`** only.

Allow anonymous will allow opendal to send request without signing when credential is not loaded.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.role_arn" class="fn">role_arn</a>(self, role_arn: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set role_arn for this backend.

If `role_arn` is set, we will use already known config as source credential to assume role with `role_arn`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.role_session_name" class="fn">role_session_name</a>(self, role_session_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set role_session_name for this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.oidc_provider_arn" class="fn">oidc_provider_arn</a>(self, oidc_provider_arn: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set oidc_provider_arn for this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.oidc_token_file" class="fn">oidc_token_file</a>(self, oidc_token_file: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set oidc_token_file for this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.sts_endpoint" class="fn">sts_endpoint</a>(self, sts_endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-oss`** only.

Set sts_endpoint for this backend.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#impl-Builder-for-OssBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html" class="struct" title="struct opendal::services::Oss">OssBuilder</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#impl-Debug-for-OssBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html" class="struct" title="struct opendal::services::Oss">OssBuilder</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#impl-Default-for-OssBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html" class="struct" title="struct opendal::services::Oss">OssBuilder</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html" class="struct" title="struct opendal::services::Oss">OssBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html#blanket-implementations" class="anchor">Â§</a>
