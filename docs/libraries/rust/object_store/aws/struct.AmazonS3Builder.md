# Struct AmazonS3Builder Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/aws/builder.rs.html#124-189" class="src">Source</a>

``` rust
pub struct AmazonS3Builder { /* private fields */ }
```

Available on **crate feature `aws`** only.

Expand description

Configure a connection to Amazon S3 using the specified credentials in the specified Amazon region and bucket.

## <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#example" class="doc-anchor">§</a>Example

``` rust
let s3 = AmazonS3Builder::new()
 .with_region(REGION)
 .with_bucket_name(BUCKET_NAME)
 .with_access_key_id(ACCESS_KEY_ID)
 .with_secret_access_key(SECRET_KEY)
 .build();
```

## Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#impl-AmazonS3Builder" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html" class="struct" title="struct object_store::aws::AmazonS3Builder">AmazonS3Builder</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.new" class="fn">new</a>() -\> Self

Create a new [`AmazonS3Builder`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html "struct object_store::aws::AmazonS3Builder") with default values.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.from_env" class="fn">from_env</a>() -\> Self

Fill the [`AmazonS3Builder`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html "struct object_store::aws::AmazonS3Builder") with regular AWS environment variables

All environment variables starting with `AWS_` will be evaluated. Names must match acceptable input to [`AmazonS3ConfigKey::from_str`](https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.from_str "associated function object_store::aws::AmazonS3ConfigKey::from_str"). Only upper-case environment variables are accepted.

Some examples of variables extracted from environment:

- `AWS_ACCESS_KEY_ID` -\> access_key_id
- `AWS_SECRET_ACCESS_KEY` -\> secret_access_key
- `AWS_DEFAULT_REGION` -\> region
- `AWS_ENDPOINT` -\> endpoint
- `AWS_SESSION_TOKEN` -\> token
- `AWS_WEB_IDENTITY_TOKEN_FILE` -\> path to file containing web identity token for AssumeRoleWithWebIdentity
- `AWS_ROLE_ARN` -\> ARN of the role to assume when using web identity token
- `AWS_ROLE_SESSION_NAME` -\> optional session name for web identity role assumption (defaults to “WebIdentitySession”)
- `AWS_ENDPOINT_URL_STS` -\> optional custom STS endpoint for web identity token exchange (defaults to “https://sts.{region}.amazonaws.com”)
- `AWS_CONTAINER_CREDENTIALS_RELATIVE_URI` -\> <https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html>
- `AWS_CONTAINER_CREDENTIALS_FULL_URI` -\> <https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html>
- `AWS_CONTAINER_AUTHORIZATION_TOKEN_FILE` -\> <https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html>
- `AWS_ALLOW_HTTP` -\> set to “true” to permit HTTP connections without TLS
- `AWS_REQUEST_PAYER` -\> set to “true” to permit operations on requester-pays buckets.

##### <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#example-1" class="doc-anchor">§</a>Example

``` rust
use object_store::aws::AmazonS3Builder;

let s3 = AmazonS3Builder::from_env()
    .with_bucket_name("foo")
    .build();
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_url" class="fn">with_url</a>(self, url: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Parse available connection info form a well-known storage URL.

The supported url schemes are:

- `s3://<bucket>/<path>`
- `s3a://<bucket>/<path>`
- `https://s3.<region>.amazonaws.com/<bucket>`
- `https://<bucket>.s3.<region>.amazonaws.com`
- `https://ACCOUNT_ID.r2.cloudflarestorage.com/bucket`

Note: Settings derived from the URL will override any others set on this builder

##### <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#example-2" class="doc-anchor">§</a>Example

``` rust
use object_store::aws::AmazonS3Builder;

let s3 = AmazonS3Builder::from_env()
    .with_url("s3://bucket/path")
    .build();
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_config" class="fn">with_config</a>( self, key: <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set an option on the builder via a key - value pair.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.get_config_value" class="fn">get_config_value</a>(&self, key: &<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get config value via a [`AmazonS3ConfigKey`](https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html "enum object_store::aws::AmazonS3ConfigKey").

##### <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#example-3" class="doc-anchor">§</a>Example

``` rust
use object_store::aws::{AmazonS3Builder, AmazonS3ConfigKey};

let builder = AmazonS3Builder::from_env()
    .with_bucket_name("foo");
let bucket_name = builder.get_config_value(&AmazonS3ConfigKey::Bucket).unwrap_or_default();
assert_eq!("foo", &bucket_name);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_access_key_id" class="fn">with_access_key_id</a>(self, access_key_id: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the AWS Access Key

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_secret_access_key" class="fn">with_secret_access_key</a>( self, secret_access_key: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set the AWS Secret Access Key

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_token" class="fn">with_token</a>(self, token: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the AWS Session Token to use for requests

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_region" class="fn">with_region</a>(self, region: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the region, defaults to `us-east-1`

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_bucket_name" class="fn">with_bucket_name</a>(self, bucket_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the bucket_name (required)

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_endpoint" class="fn">with_endpoint</a>(self, endpoint: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Sets the endpoint for communicating with AWS S3, defaults to the [region endpoint](https://docs.aws.amazon.com/general/latest/gr/s3.html)

For example, this might be set to `"http://localhost:4566:` for testing against a localstack instance.

The `endpoint` field should be consistent with [`Self::with_virtual_hosted_style_request`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_virtual_hosted_style_request "method object_store::aws::AmazonS3Builder::with_virtual_hosted_style_request"), i.e. if `virtual_hosted_style_request` is set to true then `endpoint` should have the bucket name included.

By default, only HTTPS schemes are enabled. To connect to an HTTP endpoint, enable [`Self::with_allow_http`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_allow_http "method object_store::aws::AmazonS3Builder::with_allow_http").

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_credentials" class="fn">with_credentials</a>(self, credentials: <a href="https://docs.rs/object_store/latest/object_store/aws/type.AwsCredentialProvider.html" class="type" title="type object_store::aws::AwsCredentialProvider">AwsCredentialProvider</a>) -\> Self

Set the credential provider overriding any other options

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_allow_http" class="fn">with_allow_http</a>(self, allow_http: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Sets what protocol is allowed. If `allow_http` is :

- false (default): Only HTTPS are allowed
- true: HTTP and HTTPS are allowed

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_virtual_hosted_style_request" class="fn">with_virtual_hosted_style_request</a>( self, virtual_hosted_style_request: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> Self

Sets if virtual hosted style request has to be used.

If `virtual_hosted_style_request` is:

- false (default): Path style request is used
- true: Virtual hosted style request is used

If the `endpoint` is provided then it should be consistent with `virtual_hosted_style_request`. i.e. if `virtual_hosted_style_request` is set to true then `endpoint` should have bucket name included.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_s3_express" class="fn">with_s3_express</a>(self, s3_express: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Configure this as an S3 Express One Zone Bucket

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_retry" class="fn">with_retry</a>(self, retry_config: <a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html" class="struct" title="struct object_store::RetryConfig">RetryConfig</a>) -\> Self

Set the retry configuration

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_imdsv1_fallback" class="fn">with_imdsv1_fallback</a>(self) -\> Self

By default instance credentials will only be fetched over [IMDSv2](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-instance-metadata-service.html), as AWS recommends against having IMDSv1 enabled on EC2 instances as it is vulnerable to [SSRF attack](https://aws.amazon.com/blogs/security/defense-in-depth-open-firewalls-reverse-proxies-ssrf-vulnerabilities-ec2-instance-metadata-service/)

However, certain deployment environments, such as those running old versions of kube2iam, may not support IMDSv2. This option will enable automatic fallback to using IMDSv1 if the token endpoint returns a 403 error indicating that IMDSv2 is not supported.

This option has no effect if not using instance credentials

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_unsigned_payload" class="fn">with_unsigned_payload</a>(self, unsigned_payload: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Sets if unsigned payload option has to be used. See [unsigned payload option](https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html)

- false (default): Signed payload option is used, where the checksum for the request body is computed and included when constructing a canonical request.
- true: Unsigned payload option is used. `UNSIGNED-PAYLOAD` literal is included when constructing a canonical request,

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_skip_signature" class="fn">with_skip_signature</a>(self, skip_signature: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

If enabled, [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3") will not fetch credentials and will not sign requests

This can be useful when interacting with public S3 buckets that deny authorized requests

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_checksum_algorithm" class="fn">with_checksum_algorithm</a>(self, checksum_algorithm: <a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>) -\> Self

Sets the [checksum algorithm](https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html) which has to be used for object integrity check during upload.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_metadata_endpoint" class="fn">with_metadata_endpoint</a>(self, endpoint: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the [instance metadata endpoint](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html), used primarily within AWS EC2.

This defaults to the IPv4 endpoint: http://169.254.169.254. One can alternatively use the IPv6 endpoint http://fd00:ec2::254.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_proxy_url" class="fn">with_proxy_url</a>(self, proxy_url: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the proxy_url to be used by the underlying client

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_proxy_ca_certificate" class="fn">with_proxy_ca_certificate</a>( self, proxy_ca_certificate: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Set a trusted proxy CA certificate

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_proxy_excludes" class="fn">with_proxy_excludes</a>(self, proxy_excludes: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set a list of hosts to exclude from proxy connections

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_client_options" class="fn">with_client_options</a>(self, options: <a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>) -\> Self

Sets the client options, overriding any already set

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_copy_if_not_exists" class="fn">with_copy_if_not_exists</a>(self, config: <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html" class="enum" title="enum object_store::aws::S3CopyIfNotExists">S3CopyIfNotExists</a>) -\> Self

Configure how to provide `copy_if_not_exists`

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_conditional_put" class="fn">with_conditional_put</a>(self, config: <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>) -\> Self

Configure how to provide conditional put operations. if not set, the default value will be `S3ConditionalPut::ETagMatch`

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_disable_tagging" class="fn">with_disable_tagging</a>(self, ignore: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

If set to `true` will ignore any tags provided to put_opts

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_sse_kms_encryption" class="fn">with_sse_kms_encryption</a>(self, kms_key_id: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Use SSE-KMS for server side encryption.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_dsse_kms_encryption" class="fn">with_dsse_kms_encryption</a>(self, kms_key_id: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Use dual server side encryption for server side encryption.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_ssec_encryption" class="fn">with_ssec_encryption</a>( self, customer_key_base64: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> Self

Use SSE-C for server side encryption. Must pass the *base64-encoded* 256-bit customer encryption key.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_bucket_key" class="fn">with_bucket_key</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set whether to enable bucket key for server side encryption. This overrides the bucket default setting for bucket keys.

When bucket keys are disabled, each object is encrypted with a unique data key. When bucket keys are enabled, a single data key is used for the entire bucket, reducing overhead of encryption.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_request_payer" class="fn">with_request_payer</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set whether to charge requester for bucket operations.

<https://docs.aws.amazon.com/AmazonS3/latest/userguide/RequesterPaysBuckets.html>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_http_connector" class="fn">with_http_connector</a>\<C: <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html" class="trait" title="trait object_store::client::HttpConnector">HttpConnector</a>\>(self, connector: C) -\> Self

The [`HttpConnector`](https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html "trait object_store::client::HttpConnector") to use

On non-WASM32 platforms uses [`reqwest`](https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/index.html "mod reqwest") by default, on WASM32 platforms must be provided

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html" class="struct" title="struct object_store::aws::AmazonS3">AmazonS3</a>\>

Create a [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3") instance from the provided values, consuming `self`.

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#impl-Clone-for-AmazonS3Builder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html" class="struct" title="struct object_store::aws::AmazonS3Builder">AmazonS3Builder</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html" class="struct" title="struct object_store::aws::AmazonS3Builder">AmazonS3Builder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#impl-Debug-for-AmazonS3Builder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html" class="struct" title="struct object_store::aws::AmazonS3Builder">AmazonS3Builder</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#impl-Default-for-AmazonS3Builder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html" class="struct" title="struct object_store::aws::AmazonS3Builder">AmazonS3Builder</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html" class="struct" title="struct object_store::aws::AmazonS3Builder">AmazonS3Builder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#blanket-implementations" class="anchor">§</a>
