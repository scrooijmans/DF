# AmazonS3Builder in object_store::aws - Rust

## Struct AmazonS3Builder 

[Source](about:blank/src/object_store/aws/builder.rs.html#124-189)

```
pub struct AmazonS3Builder { /* private fields */ }
```

Available on **crate feature `aws`** only.

Expand description

Configure a connection to Amazon S3 using the specified credentials in the specified Amazon region and bucket.

## [§](#example)Example

```
let s3 = AmazonS3Builder::new()
 .with_region(REGION)
 .with_bucket_name(BUCKET_NAME)
 .with_access_key_id(ACCESS_KEY_ID)
 .with_secret_access_key(SECRET_KEY)
 .build();
```

[Source](about:blank/src/object_store/aws/builder.rs.html#489-1174)
[§](#impl-AmazonS3Builder)

[Source](about:blank/src/object_store/aws/builder.rs.html#491-493)

[Source](about:blank/src/object_store/aws/builder.rs.html#524-538)

Fill the [`AmazonS3Builder`](struct.AmazonS3Builder.html "struct object_store::aws::AmazonS3Builder") with regular AWS environment variables

All environment variables starting with `AWS_` will be evaluated. Names must match acceptable input to [`AmazonS3ConfigKey::from_str`](about:blank/enum.AmazonS3ConfigKey.html#method.from_str "associated function object_store::aws::AmazonS3ConfigKey::from_str"). Only upper-case environment variables are accepted.

Some examples of variables extracted from environment:

- `AWS_ACCESS_KEY_ID` -> access_key_id
- `AWS_SECRET_ACCESS_KEY` -> secret_access_key
- `AWS_DEFAULT_REGION` -> region
- `AWS_ENDPOINT` -> endpoint
- `AWS_SESSION_TOKEN` -> token
- `AWS_WEB_IDENTITY_TOKEN_FILE` -> path to file containing web identity token for AssumeRoleWithWebIdentity
- `AWS_ROLE_ARN` -> ARN of the role to assume when using web identity token
- `AWS_ROLE_SESSION_NAME` -> optional session name for web identity role assumption (defaults to “WebIdentitySession”)
- `AWS_ENDPOINT_URL_STS` -> optional custom STS endpoint for web identity token exchange (defaults to “https://sts.{region}.amazonaws.com”)
- `AWS_CONTAINER_CREDENTIALS_RELATIVE_URI` -> [https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html)
- `AWS_CONTAINER_CREDENTIALS_FULL_URI` -> [https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html](https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html)
- `AWS_CONTAINER_AUTHORIZATION_TOKEN_FILE` -> [https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html](https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html)
- `AWS_ALLOW_HTTP` -> set to “true” to permit HTTP connections without TLS
- `AWS_REQUEST_PAYER` -> set to “true” to permit operations on requester-pays buckets.

##### [§](#example-1)Example

```
use object_store::aws::AmazonS3Builder;

let s3 = AmazonS3Builder::from_env()
    .with_bucket_name("foo")
    .build();
```

[Source](about:blank/src/object_store/aws/builder.rs.html#560-563)

Parse available connection info form a well-known storage URL.

The supported url schemes are:

- `s3://<bucket>/<path>`
- `s3a://<bucket>/<path>`
- `https://s3.<region>.amazonaws.com/<bucket>`
- `https://<bucket>.s3.<region>.amazonaws.com`
- `https://ACCOUNT_ID.r2.cloudflarestorage.com/bucket`

Note: Settings derived from the URL will override any others set on this builder

##### [§](#example-2)Example

```
use object_store::aws::AmazonS3Builder;

let s3 = AmazonS3Builder::from_env()
    .with_url("s3://bucket/path")
    .build();
```

[Source](about:blank/src/object_store/aws/builder.rs.html#566-636)

Set an option on the builder via a key - value pair.

[Source](about:blank/src/object_store/aws/builder.rs.html#649-702)

Get config value via a [`AmazonS3ConfigKey`](enum.AmazonS3ConfigKey.html "enum object_store::aws::AmazonS3ConfigKey").

##### [§](#example-3)Example

```
use object_store::aws::{AmazonS3Builder, AmazonS3ConfigKey};

let builder = AmazonS3Builder::from_env()
    .with_bucket_name("foo");
let bucket_name = builder.get_config_value(&AmazonS3ConfigKey::Bucket).unwrap_or_default();
assert_eq!("foo", &bucket_name);
```

[Source](about:blank/src/object_store/aws/builder.rs.html#754-757)

Set the AWS Access Key

[Source](about:blank/src/object_store/aws/builder.rs.html#760-763)

Set the AWS Secret Access Key

[Source](about:blank/src/object_store/aws/builder.rs.html#766-769)

Set the AWS Session Token to use for requests

[Source](about:blank/src/object_store/aws/builder.rs.html#772-775)

Set the region, defaults to `us-east-1`

[Source](about:blank/src/object_store/aws/builder.rs.html#778-781)

Set the bucket_name (required)

[Source](about:blank/src/object_store/aws/builder.rs.html#796-799)

Sets the endpoint for communicating with AWS S3, defaults to the [region endpoint](https://docs.aws.amazon.com/general/latest/gr/s3.html)

For example, this might be set to `"http://localhost:4566:` for testing against a localstack instance.

The `endpoint` field should be consistent with [`Self::with_virtual_hosted_style_request`](about:blank/struct.AmazonS3Builder.html#method.with_virtual_hosted_style_request "method object_store::aws::AmazonS3Builder::with_virtual_hosted_style_request"), i.e. if `virtual_hosted_style_request` is set to true then `endpoint` should have the bucket name included.

By default, only HTTPS schemes are enabled. To connect to an HTTP endpoint, enable [`Self::with_allow_http`](about:blank/struct.AmazonS3Builder.html#method.with_allow_http "method object_store::aws::AmazonS3Builder::with_allow_http").

[Source](about:blank/src/object_store/aws/builder.rs.html#802-805)

Set the credential provider overriding any other options

[Source](about:blank/src/object_store/aws/builder.rs.html#810-813)

Sets what protocol is allowed. If `allow_http` is :

- false (default): Only HTTPS are allowed
- true: HTTP and HTTPS are allowed

[Source](about:blank/src/object_store/aws/builder.rs.html#825-828)

Sets if virtual hosted style request has to be used.

If `virtual_hosted_style_request` is:

- false (default): Path style request is used
- true: Virtual hosted style request is used

If the `endpoint` is provided then it should be consistent with `virtual_hosted_style_request`. i.e. if `virtual_hosted_style_request` is set to true then `endpoint` should have bucket name included.

[Source](about:blank/src/object_store/aws/builder.rs.html#831-834)

Configure this as an S3 Express One Zone Bucket

[Source](about:blank/src/object_store/aws/builder.rs.html#837-840)

Set the retry configuration

[Source](about:blank/src/object_store/aws/builder.rs.html#854-857)

By default instance credentials will only be fetched over [IMDSv2](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-instance-metadata-service.html), as AWS recommends against having IMDSv1 enabled on EC2 instances as it is vulnerable to [SSRF attack](https://aws.amazon.com/blogs/security/defense-in-depth-open-firewalls-reverse-proxies-ssrf-vulnerabilities-ec2-instance-metadata-service/)

However, certain deployment environments, such as those running old versions of kube2iam, may not support IMDSv2. This option will enable automatic fallback to using IMDSv1 if the token endpoint returns a 403 error indicating that IMDSv2 is not supported.

This option has no effect if not using instance credentials

[Source](about:blank/src/object_store/aws/builder.rs.html#863-866)

Sets if unsigned payload option has to be used. See [unsigned payload option](https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html)

- false (default): Signed payload option is used, where the checksum for the request body is computed and included when constructing a canonical request.
- true: Unsigned payload option is used. `UNSIGNED-PAYLOAD` literal is included when constructing a canonical request,

[Source](about:blank/src/object_store/aws/builder.rs.html#871-874)

If enabled, [`AmazonS3`](struct.AmazonS3.html "struct object_store::aws::AmazonS3") will not fetch credentials and will not sign requests

This can be useful when interacting with public S3 buckets that deny authorized requests

[Source](about:blank/src/object_store/aws/builder.rs.html#879-883)

Sets the [checksum algorithm](https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html) which has to be used for object integrity check during upload.

[Source](about:blank/src/object_store/aws/builder.rs.html#890-893)

Set the [instance metadata endpoint](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html), used primarily within AWS EC2.

This defaults to the IPv4 endpoint: http://169.254.169.254. One can alternatively use the IPv6 endpoint http://fd00:ec2::254.

[Source](about:blank/src/object_store/aws/builder.rs.html#896-899)

Set the proxy_url to be used by the underlying client

[Source](about:blank/src/object_store/aws/builder.rs.html#902-907)

Set a trusted proxy CA certificate

[Source](about:blank/src/object_store/aws/builder.rs.html#910-913)

Set a list of hosts to exclude from proxy connections

[Source](about:blank/src/object_store/aws/builder.rs.html#916-919)

Sets the client options, overriding any already set

[Source](about:blank/src/object_store/aws/builder.rs.html#922-925)

Configure how to provide `copy_if_not_exists`

[Source](about:blank/src/object_store/aws/builder.rs.html#929-932)

Configure how to provide conditional put operations. if not set, the default value will be `S3ConditionalPut::ETagMatch`

[Source](about:blank/src/object_store/aws/builder.rs.html#935-938)

If set to `true` will ignore any tags provided to put_opts

[Source](about:blank/src/object_store/aws/builder.rs.html#941-947)

Use SSE-KMS for server side encryption.

[Source](about:blank/src/object_store/aws/builder.rs.html#950-956)

Use dual server side encryption for server side encryption.

[Source](about:blank/src/object_store/aws/builder.rs.html#960-964)

Use SSE-C for server side encryption. Must pass the _base64-encoded_ 256-bit customer encryption key.

[Source](about:blank/src/object_store/aws/builder.rs.html#972-975)

Set whether to enable bucket key for server side encryption. This overrides the bucket default setting for bucket keys.

When bucket keys are disabled, each object is encrypted with a unique data key. When bucket keys are enabled, a single data key is used for the entire bucket, reducing overhead of encryption.

[Source](about:blank/src/object_store/aws/builder.rs.html#980-983)

[Source](about:blank/src/object_store/aws/builder.rs.html#988-991)

The [`HttpConnector`](../client/trait.HttpConnector.html "trait object_store::client::HttpConnector") to use

On non-WASM32 platforms uses [`reqwest`](https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/index.html "mod reqwest") by default, on WASM32 platforms must be provided

[Source](about:blank/src/object_store/aws/builder.rs.html#995-1173)

Create a [`AmazonS3`](struct.AmazonS3.html "struct object_store::aws::AmazonS3") instance from the provided values, consuming `self`.

[§](#impl-Freeze-for-AmazonS3Builder)

[§](#impl-RefUnwindSafe-for-AmazonS3Builder)

[§](#impl-Send-for-AmazonS3Builder)

[§](#impl-Sync-for-AmazonS3Builder)

[§](#impl-Unpin-for-AmazonS3Builder)

[§](#impl-UnwindSafe-for-AmazonS3Builder)
