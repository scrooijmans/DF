# Struct S3 Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/s3/backend.rs.html#77-84" class="src">Source</a>

``` rust
pub struct S3 { /* private fields */ }
```

Expand description

Aws S3 and compatible services (including minio, digitalocean space, Tencent Cloud Object Storage(COS) and so on) support. For more information about s3-compatible services, refer to [Compatible Services](https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#compatible-services).

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work dir for backend.
- `bucket`: Set the container name for backend.
- `endpoint`: Set the endpoint for backend.
- `region`: Set the region for backend.
- `access_key_id`: Set the access_key_id for backend.
- `secret_access_key`: Set the secret_access_key for backend.
- `session_token`: Set the session_token for backend.
- `default_storage_class`: Set the default storage_class for backend.
- `server_side_encryption`: Set the server_side_encryption for backend.
- `server_side_encryption_aws_kms_key_id`: Set the server_side_encryption_aws_kms_key_id for backend.
- `server_side_encryption_customer_algorithm`: Set the server_side_encryption_customer_algorithm for backend.
- `server_side_encryption_customer_key`: Set the server_side_encryption_customer_key for backend.
- `server_side_encryption_customer_key_md5`: Set the server_side_encryption_customer_key_md5 for backend.
- `disable_config_load`: Disable aws config load from env.
- `enable_virtual_host_style`: Enable virtual host style.
- `disable_write_with_if_match`: Disable write with if match.
- `enable_request_payer`: Enable the request payer for backend.

Refer to [`S3Builder`](https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html "struct opendal::services::S3")â€™s public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#temporary-security-credentials" class="doc-anchor">Â§</a>Temporary security credentials

OpenDAL now provides support for S3 temporary security credentials in IAM.

The way to take advantage of this feature is to build your S3 backend with `Builder::session_token`.

But OpenDAL will not refresh the temporary security credentials, please keep in mind to refresh those credentials in time.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#server-side-encryption" class="doc-anchor">Â§</a>Server Side Encryption

OpenDAL provides full support of S3 Server Side Encryption(SSE) features.

The easiest way to configure them is to use helper functions like

- SSE-KMS: `server_side_encryption_with_aws_managed_kms_key`
- SSE-KMS: `server_side_encryption_with_customer_managed_kms_key`
- SSE-S3: `server_side_encryption_with_s3_key`
- SSE-C: `server_side_encryption_with_customer_key`

If those functions donâ€™t fulfill need, low-level options are also provided:

- Use service managed kms key
  - `server_side_encryption="aws:kms"`
- Use customer provided kms key
  - `server_side_encryption="aws:kms"`
  - `server_side_encryption_aws_kms_key_id="your-kms-key"`
- Use S3 managed key
  - `server_side_encryption="AES256"`
- Use customer key
  - `server_side_encryption_customer_algorithm="AES256"`
  - `server_side_encryption_customer_key="base64-of-your-aes256-key"`
  - `server_side_encryption_customer_key_md5="base64-of-your-aes256-key-md5"`

After SSE have been configured, all requests send by this backed will attach those headers.

Reference: [Protecting data using server-side encryption](https://docs.aws.amazon.com/AmazonS3/latest/userguide/serv-side-encryption.html)

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#example" class="doc-anchor">Â§</a>Example

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#via-builder" class="doc-anchor">Â§</a>Via Builder

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#basic-setup" class="doc-anchor">Â§</a>Basic Setup

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::S3;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Create s3 backend builder.
    let mut builder = S3::default()
      // Set the root for s3, all operations will happen under this root.
      //
      // NOTE: the root must be absolute path.
      .root("/path/to/dir")
      // Set the bucket name. This is required.
      .bucket("test")
      // Set the region. This is required for some services, if you don't care about it, for example Minio service, just set it to "auto", it will be ignored.
      .region("us-east-1")
      // Set the endpoint.
      //
      // For examples:
      // - "https://s3.amazonaws.com"
      // - "http://127.0.0.1:9000"
      // - "https://oss-ap-northeast-1.aliyuncs.com"
      // - "https://cos.ap-seoul.myqcloud.com"
      //
      // Default to "https://s3.amazonaws.com"
      .endpoint("https://s3.amazonaws.com")
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

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#s3-with-sse-c" class="doc-anchor">Â§</a>S3 with SSE-C

``` rust
use anyhow::Result;
use log::info;
use opendal::services::S3;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = S3::default()
      .root("/path/to/dir")
      .bucket("test")
      .region("us-east-1")
      .endpoint("https://s3.amazonaws.com")
      .access_key_id("access_key_id")
      .secret_access_key("secret_access_key")
      // Enable SSE-C
      .server_side_encryption_with_customer_key("AES256", "customer_key".as_bytes());

    let op = Operator::new(builder)?.finish();
    info!("operator: {:?}", op);

    // Writing your testing code here.

    Ok(())
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#s3-with-sse-kms-and-aws-managed-kms-key" class="doc-anchor">Â§</a>S3 with SSE-KMS and aws managed kms key

``` rust
use anyhow::Result;
use log::info;
use opendal::services::S3;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = S3::default()
      // Setup builders
      .root("/path/to/dir")
      .bucket("test")
      .region("us-east-1")
      .endpoint("https://s3.amazonaws.com")
      .access_key_id("access_key_id")
      .secret_access_key("secret_access_key")
      // Enable SSE-KMS with aws managed kms key
      .server_side_encryption_with_aws_managed_kms_key();

    let op = Operator::new(builder)?.finish();
    info!("operator: {:?}", op);

    // Writing your testing code here.

    Ok(())
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#s3-with-sse-kms-and-customer-managed-kms-key" class="doc-anchor">Â§</a>S3 with SSE-KMS and customer managed kms key

``` rust
use anyhow::Result;
use log::info;
use opendal::services::S3;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = S3::default()
      // Setup builders
      .root("/path/to/dir")
      .bucket("test")
      .region("us-east-1")
      .endpoint("https://s3.amazonaws.com")
      .access_key_id("access_key_id")
      .secret_access_key("secret_access_key")
      // Enable SSE-KMS with customer managed kms key
      .server_side_encryption_with_customer_managed_kms_key("aws_kms_key_id");

    let op = Operator::new(builder)?.finish();
    info!("operator: {:?}", op);

    // Writing your testing code here.

    Ok(())
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#s3-with-sse-s3" class="doc-anchor">Â§</a>S3 with SSE-S3

``` rust
use anyhow::Result;
use log::info;
use opendal::services::S3;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = S3::default()
      // Setup builders
      .root("/path/to/dir")
      .bucket("test")
      .region("us-east-1")
      .endpoint("https://s3.amazonaws.com")
      .access_key_id("access_key_id")
      .secret_access_key("secret_access_key")
      // Enable SSE-S3
      .server_side_encryption_with_s3_key();

    let op = Operator::new(builder)?.finish();
    info!("operator: {:?}", op);

    // Writing your testing code here.

    Ok(())
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#compatible-services" class="doc-anchor">Â§</a>Compatible Services

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#aws-s3" class="doc-anchor">Â§</a>AWS S3

[AWS S3](https://aws.amazon.com/s3/) is the default implementations of s3 services. Only `bucket` is required.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
builder.bucket("<bucket_name>");
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#alibaba-object-storage-service-oss" class="doc-anchor">Â§</a>Alibaba Object Storage Service (OSS)

[OSS](https://www.alibabacloud.com/product/object-storage-service) is a s3 compatible service provided by [Alibaba Cloud](https://www.alibabacloud.com).

To connect to OSS, we need to set:

- `endpoint`: The endpoint of oss, for example: `https://oss-cn-hangzhou.aliyuncs.com`
- `bucket`: The bucket name of oss.

> OSS provide internal endpoint for used at alibabacloud internally, please visit [OSS Regions and endpoints](https://www.alibabacloud.com/help/en/object-storage-service/latest/regions-and-endpoints) for more details.

> OSS only supports the virtual host style, users could meet errors like:
>
> ``` xml
> <?xml version="1.0" encoding="UTF-8"?>
> <Error>
>  <Code>SecondLevelDomainForbidden</Code>
>  <Message>The bucket you are attempting to access must be addressed using OSS third level domain.</Message>
>  <RequestId>62A1C265292C0632377F021F</RequestId>
>  <HostId>oss-cn-hangzhou.aliyuncs.com</HostId>
> </Error>
> ```
>
> In that case, please enable virtual host style for requesting.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
builder.endpoint("https://oss-cn-hangzhou.aliyuncs.com");
builder.region("<region>");
builder.bucket("<bucket_name>");
builder.enable_virtual_host_style();
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#minio" class="doc-anchor">Â§</a>Minio

[minio](https://min.io/) is an open-source s3 compatible services.

To connect to minio, we need to set:

- `endpoint`: The endpoint of minio, for example: `http://127.0.0.1:9000`
- `region`: The region of minio. If you donâ€™t care about it, just set it to â€œautoâ€?, it will be ignored.
- `bucket`: The bucket name of minio.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
builder.endpoint("http://127.0.0.1:9000");
builder.region("<region>");
builder.bucket("<bucket_name>");
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#qingstor-object-storage" class="doc-anchor">Â§</a>QingStor Object Storage

[QingStor Object Storage](https://www.qingcloud.com/products/qingstor) is a S3-compatible service provided by [QingCloud](https://www.qingcloud.com/).

To connect to QingStor Object Storage, we need to set:

- `endpoint`: The endpoint of QingStor s3 compatible endpoint, for example: `https://s3.pek3b.qingstor.com`
- `bucket`: The bucket name.

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#scaleway-object-storage" class="doc-anchor">Â§</a>Scaleway Object Storage

[Scaleway Object Storage](https://www.scaleway.com/en/object-storage/) is a S3-compatible and multi-AZ redundant object storage service.

To connect to Scaleway Object Storage, we need to set:

- `endpoint`: The endpoint of scaleway, for example: `https://s3.nl-ams.scw.cloud`
- `region`: The region of scaleway.
- `bucket`: The bucket name of scaleway.

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#tencent-cloud-object-storage-cos" class="doc-anchor">Â§</a>Tencent Cloud Object Storage (COS)

[COS](https://intl.cloud.tencent.com/products/cos) is a s3 compatible service provided by [Tencent Cloud](https://intl.cloud.tencent.com/).

To connect to COS, we need to set:

- `endpoint`: The endpoint of cos, for example: `https://cos.ap-beijing.myqcloud.com`
- `bucket`: The bucket name of cos.

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#wasabi-object-storage" class="doc-anchor">Â§</a>Wasabi Object Storage

[Wasabi](https://wasabi.com/) is a s3 compatible service.

> Cloud storage pricing that is 80% less than Amazon S3.

To connect to wasabi, we need to set:

- `endpoint`: The endpoint of wasabi, for example: `https://s3.us-east-2.wasabisys.com`
- `bucket`: The bucket name of wasabi.

> Refer to [What are the service URLs for Wasabiâ€™s different storage regions?](https://wasabi-support.zendesk.com/hc/en-us/articles/360015106031) for more details.

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#cloudflare-r2" class="doc-anchor">Â§</a>Cloudflare R2

[Cloudflare R2](https://developers.cloudflare.com/r2/) provides s3 compatible API.

> Cloudflare R2 Storage allows developers to store large amounts of unstructured data without the costly egress bandwidth fees associated with typical cloud storage services.

To connect to r2, we need to set:

- `endpoint`: The endpoint of r2, for example: `https://<account_id>.r2.cloudflarestorage.com`
- `bucket`: The bucket name of r2.
- `region`: When you create a new bucket, the data location is set to Automatic by default. So please use `auto` for region.
- `batch_max_operations`: R2â€™s delete objects will return `Internal Error` if the batch is larger than `700`. Please set this value `<= 700` to make sure batch delete work as expected.
- `enable_exact_buf_write`: R2 requires the non-tailing parts size to be exactly the same. Please enable this option to avoid the error `All non-trailing parts must have the same length`.

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#google-cloud-storage-xml-api" class="doc-anchor">Â§</a>Google Cloud Storage XML API

[Google Cloud Storage XML API](https://cloud.google.com/storage/docs/xml-api/overview) provides s3 compatible API.

- `endpoint`: The endpoint of Google Cloud Storage XML API, for example: `https://storage.googleapis.com`
- `bucket`: The bucket name.
- To access GCS via S3 API, please enable `features = ["native-tls"]` in your `Cargo.toml` to avoid connection being reset when using `rustls`. Tracking in <https://github.com/seanmonstar/reqwest/issues/1809>

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#ceph-rados-gateway" class="doc-anchor">Â§</a>Ceph Rados Gateway

Ceph supports a RESTful API that is compatible with the basic data access model of the Amazon S3 API.

For more information, refer: <https://docs.ceph.com/en/latest/radosgw/s3/>

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#impl-S3Builder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html" class="struct" title="struct opendal::services::S3">S3Builder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.bucket" class="fn">bucket</a>(self, bucket: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set bucket name of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set endpoint of this backend.

Endpoint must be full uri, e.g.

- AWS S3: `https://s3.amazonaws.com` or `https://s3.{region}.amazonaws.com`
- Cloudflare R2: `https://<ACCOUNT_ID>.r2.cloudflarestorage.com`
- Aliyun OSS: `https://{region}.aliyuncs.com`
- Tencent COS: `https://cos.{region}.myqcloud.com`
- Minio: `http://127.0.0.1:9000`

If user inputs endpoint without scheme like â€œs3.amazonaws.comâ€?, we will prepend â€œhttps://â€? before it.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.region" class="fn">region</a>(self, region: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Region represent the signing region of this endpoint. This is required if you are using the default AWS S3 endpoint.

If using a custom endpoint,

- If region is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.access_key_id" class="fn">access_key_id</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set access_key_id of this backend.

- If access_key_id is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.secret_access_key" class="fn">secret_access_key</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set secret_access_key of this backend.

- If secret_access_key is set, we will take userâ€™s input first.
- If not, we will try to load it from environment.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.role_arn" class="fn">role_arn</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set role_arn for this backend.

If `role_arn` is set, we will use already known config as source credential to assume role with `role_arn`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.external_id" class="fn">external_id</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set external_id for this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.role_session_name" class="fn">role_session_name</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set role_session_name for this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.default_storage_class" class="fn">default_storage_class</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set default storage_class for this backend.

Available values:

- `DEEP_ARCHIVE`
- `GLACIER`
- `GLACIER_IR`
- `INTELLIGENT_TIERING`
- `ONEZONE_IA`
- `OUTPOSTS`
- `REDUCED_REDUNDANCY`
- `STANDARD`
- `STANDARD_IA`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.server_side_encryption" class="fn">server_side_encryption</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set server_side_encryption for this backend.

Available values: `AES256`, `aws:kms`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#note" class="doc-anchor">Â§</a>Note

This function is the low-level setting for SSE related features.

SSE related options should be set carefully to make them works. Please use `server_side_encryption_with_*` helpers if even possible.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.server_side_encryption_aws_kms_key_id" class="fn">server_side_encryption_aws_kms_key_id</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set server_side_encryption_aws_kms_key_id for this backend

- If `server_side_encryption` set to `aws:kms`, and `server_side_encryption_aws_kms_key_id` is not set, S3 will use aws managed kms key to encrypt data.
- If `server_side_encryption` set to `aws:kms`, and `server_side_encryption_aws_kms_key_id` is a valid kms key id, S3 will use the provided kms key to encrypt data.
- If the `server_side_encryption_aws_kms_key_id` is invalid or not found, an error will be returned.
- If `server_side_encryption` is not `aws:kms`, setting `server_side_encryption_aws_kms_key_id` is a noop.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#note-1" class="doc-anchor">Â§</a>Note

This function is the low-level setting for SSE related features.

SSE related options should be set carefully to make them works. Please use `server_side_encryption_with_*` helpers if even possible.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.server_side_encryption_customer_algorithm" class="fn">server_side_encryption_customer_algorithm</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set server_side_encryption_customer_algorithm for this backend.

Available values: `AES256`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#note-2" class="doc-anchor">Â§</a>Note

This function is the low-level setting for SSE related features.

SSE related options should be set carefully to make them works. Please use `server_side_encryption_with_*` helpers if even possible.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.server_side_encryption_customer_key" class="fn">server_side_encryption_customer_key</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set server_side_encryption_customer_key for this backend.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#args" class="doc-anchor">Â§</a>Args

`v`: base64 encoded key that matches algorithm specified in `server_side_encryption_customer_algorithm`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#note-3" class="doc-anchor">Â§</a>Note

This function is the low-level setting for SSE related features.

SSE related options should be set carefully to make them works. Please use `server_side_encryption_with_*` helpers if even possible.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.server_side_encryption_customer_key_md5" class="fn">server_side_encryption_customer_key_md5</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set server_side_encryption_customer_key_md5 for this backend.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#args-1" class="doc-anchor">Â§</a>Args

`v`: MD5 digest of key specified in `server_side_encryption_customer_key`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#note-4" class="doc-anchor">Â§</a>Note

This function is the low-level setting for SSE related features.

SSE related options should be set carefully to make them works. Please use `server_side_encryption_with_*` helpers if even possible.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.server_side_encryption_with_aws_managed_kms_key" class="fn">server_side_encryption_with_aws_managed_kms_key</a>(self) -\> Self

Available on **crate feature `services-s3`** only.

Enable server side encryption with aws managed kms key

As known as: SSE-KMS

NOTE: This function should not be used along with other `server_side_encryption_with_` functions.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.server_side_encryption_with_customer_managed_kms_key" class="fn">server_side_encryption_with_customer_managed_kms_key</a>( self, aws_kms_key_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> Self

Available on **crate feature `services-s3`** only.

Enable server side encryption with customer managed kms key

As known as: SSE-KMS

NOTE: This function should not be used along with other `server_side_encryption_with_` functions.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.server_side_encryption_with_s3_key" class="fn">server_side_encryption_with_s3_key</a>(self) -\> Self

Available on **crate feature `services-s3`** only.

Enable server side encryption with s3 managed key

As known as: SSE-S3

NOTE: This function should not be used along with other `server_side_encryption_with_` functions.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.server_side_encryption_with_customer_key" class="fn">server_side_encryption_with_customer_key</a>( self, algorithm: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, key: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], ) -\> Self

Available on **crate feature `services-s3`** only.

Enable server side encryption with customer key.

As known as: SSE-C

NOTE: This function should not be used along with other `server_side_encryption_with_` functions.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.session_token" class="fn">session_token</a>(self, token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set temporary credential used in AWS S3 connections

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#warning" class="doc-anchor">Â§</a>Warning

session tokenâ€™s lifetime is short and requires users to refresh in time.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.security_token" class="fn">security_token</a>(self, token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

ðŸ‘ŽDeprecated: Please use `session_token` instead

Available on **crate feature `services-s3`** only.

Set temporary credential used in AWS S3 connections

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.disable_config_load" class="fn">disable_config_load</a>(self) -\> Self

Available on **crate feature `services-s3`** only.

Disable config load so that opendal will not load config from environment.

For examples:

- envs like `AWS_ACCESS_KEY_ID`
- files like `~/.aws/config`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.disable_list_objects_v2" class="fn">disable_list_objects_v2</a>(self) -\> Self

Available on **crate feature `services-s3`** only.

Disable list objects v2 so that opendal will not use the older List Objects V1 to list objects.

By default, OpenDAL uses List Objects V2 to list objects. However, some legacy services do not yet support V2.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.enable_request_payer" class="fn">enable_request_payer</a>(self) -\> Self

Available on **crate feature `services-s3`** only.

Enable request payer so that OpenDAL will send requests with `x-amz-request-payer` header.

With this option the client accepts to pay for the request and data transfer costs.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.disable_ec2_metadata" class="fn">disable_ec2_metadata</a>(self) -\> Self

Available on **crate feature `services-s3`** only.

Disable load credential from ec2 metadata.

This option is used to disable the default behavior of opendal to load credential from ec2 metadata, a.k.a, IMDSv2

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.allow_anonymous" class="fn">allow_anonymous</a>(self) -\> Self

Available on **crate feature `services-s3`** only.

Allow anonymous will allow opendal to send request without signing when credential is not loaded.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.enable_virtual_host_style" class="fn">enable_virtual_host_style</a>(self) -\> Self

Available on **crate feature `services-s3`** only.

Enable virtual host style so that opendal will send API requests in virtual host style instead of path style.

- By default, opendal will send API to `https://s3.us-east-1.amazonaws.com/bucket_name`
- Enabled, opendal will send API to `https://bucket_name.s3.us-east-1.amazonaws.com`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.disable_stat_with_override" class="fn">disable_stat_with_override</a>(self) -\> Self

Available on **crate feature `services-s3`** only.

Disable stat with override so that opendal will not send stat request with override queries.

For example, R2 doesnâ€™t support stat with `response_content_type` query.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.customized_credential_load" class="fn">customized_credential_load</a>( self, cred: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn AwsCredentialLoad\>, ) -\> Self

Available on **crate feature `services-s3`** only.

Adding a customized credential load for service.

If customized_credential_load has been set, we will ignore all other credential load methods.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-s3`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.enable_versioning" class="fn">enable_versioning</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set bucket versioning status for this backend

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.batch_max_operations" class="fn">batch_max_operations</a>(self, batch_max_operations: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

ðŸ‘ŽDeprecated since 0.52.0: Please use `delete_max_size` instead of `batch_max_operations`

Available on **crate feature `services-s3`** only.

Set maximum batch operations of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.delete_max_size" class="fn">delete_max_size</a>(self, delete_max_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set maximum delete operations of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.checksum_algorithm" class="fn">checksum_algorithm</a>(self, checksum_algorithm: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-s3`** only.

Set checksum algorithm of this backend. This is necessary when writing to AWS S3 Buckets with Object Lock enabled for example.

Available options:

- â€œcrc32câ€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.disable_write_with_if_match" class="fn">disable_write_with_if_match</a>(self) -\> Self

Available on **crate feature `services-s3`** only.

Disable write with if match so that opendal will not send write request with if match headers.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.enable_write_with_append" class="fn">enable_write_with_append</a>(self) -\> Self

Available on **crate feature `services-s3`** only.

Enable write with append so that opendal will send write request with append headers.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.detect_region" class="fn">detect_region</a>(endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, bucket: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Available on **crate feature `services-s3`** only.

Detect region of S3 bucket.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#args-2" class="doc-anchor">Â§</a>Args

- endpoint: the endpoint of S3 service
- bucket: the bucket of S3 service

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#return" class="doc-anchor">Â§</a>Return

- `Some(region)` means we detect the region successfully
- `None` means we canâ€™t detect the region or meeting errors.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#notes-1" class="doc-anchor">Â§</a>Notes

We will try to detect region by the following methods.

- Match endpoint with given rules to get region
  - Cloudflare R2
  - AWS S3
  - Aliyun OSS
- Send a `HEAD` request to endpoint with bucket name to get `x-amz-bucket-region`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#examples" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::services::S3;

let region: Option<String> = S3::detect_region("https://s3.amazonaws.com", "example").await;
```

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#reference" class="doc-anchor">Â§</a>Reference

- [Amazon S3 HeadBucket API](https://docs.aws.amazon.com/zh_cn/AmazonS3/latest/API/API_HeadBucket.html)

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#impl-Builder-for-S3Builder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html" class="struct" title="struct opendal::services::S3">S3Builder</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#impl-Debug-for-S3Builder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html" class="struct" title="struct opendal::services::S3">S3Builder</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#impl-Default-for-S3Builder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html" class="struct" title="struct opendal::services::S3">S3Builder</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html" class="struct" title="struct opendal::services::S3">S3Builder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html#blanket-implementations" class="anchor">Â§</a>
