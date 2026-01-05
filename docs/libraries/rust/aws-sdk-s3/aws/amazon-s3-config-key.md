# AmazonS3ConfigKey in object_store::aws - Rust

## Enum AmazonS3ConfigKey 

[Source](about:blank/src/object_store/aws/builder.rs.html#204-399)

```

#[non_exhaustive]pub enum AmazonS3ConfigKey {
Show 27 variants    AccessKeyId,
    SecretAccessKey,
    Region,
    DefaultRegion,
    Bucket,
    Endpoint,
    Token,
    ImdsV1Fallback,
    VirtualHostedStyleRequest,
    UnsignedPayload,
    Checksum,
    MetadataEndpoint,
    ContainerCredentialsRelativeUri,
    ContainerCredentialsFullUri,
    ContainerAuthorizationTokenFile,
    WebIdentityTokenFile,
    RoleArn,
    RoleSessionName,
    StsEndpoint,
    CopyIfNotExists,
    ConditionalPut,
    SkipSignature,
    DisableTagging,
    S3Express,
    RequestPayer,
    Client(ClientConfigKey),
    Encryption(S3EncryptionConfigKey),
}
```

Available on **crate feature `aws`** only.

Expand description

Configuration keys for [`AmazonS3Builder`](struct.AmazonS3Builder.html "struct object_store::aws::AmazonS3Builder")

Configuration via keys can be done via [`AmazonS3Builder::with_config`](about:blank/struct.AmazonS3Builder.html#method.with_config "method object_store::aws::AmazonS3Builder::with_config")

## [§](#example)Example

```
let builder = AmazonS3Builder::new()
    .with_config("aws_access_key_id".parse().unwrap(), "my-access-key-id")
    .with_config(AmazonS3ConfigKey::DefaultRegion, "my-default-region");
```

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

[§](#variant.AccessKeyId)

[§](#variant.SecretAccessKey)

[§](#variant.Region)

[§](#variant.DefaultRegion)

[§](#variant.Bucket)

[§](#variant.Endpoint)

Sets custom endpoint for communicating with AWS S3.

See [`AmazonS3Builder::with_endpoint`](about:blank/struct.AmazonS3Builder.html#method.with_endpoint "method object_store::aws::AmazonS3Builder::with_endpoint") for details.

Supported keys:

- `aws_endpoint`
- `aws_endpoint_url`
- `endpoint`
- `endpoint_url`

[§](#variant.Token)

Token to use for requests (passed to underlying provider)

See [`AmazonS3Builder::with_token`](about:blank/struct.AmazonS3Builder.html#method.with_token "method object_store::aws::AmazonS3Builder::with_token") for details.

Supported keys:

- `aws_session_token`
- `aws_token`
- `session_token`
- `token`

[§](#variant.ImdsV1Fallback)

[§](#variant.VirtualHostedStyleRequest)

[§](#variant.UnsignedPayload)

[§](#variant.Checksum)

[§](#variant.MetadataEndpoint)

[§](#variant.ContainerCredentialsRelativeUri)

[§](#variant.ContainerCredentialsFullUri)

[§](#variant.WebIdentityTokenFile)

Web identity token file path for AssumeRoleWithWebIdentity

Supported keys:

- `aws_web_identity_token_file`
- `web_identity_token_file`

[§](#variant.RoleArn)

Role ARN to assume when using web identity token

Supported keys:

- `aws_role_arn`
- `role_arn`

[§](#variant.RoleSessionName)

Session name for web identity role assumption

Supported keys:

- `aws_role_session_name`
- `role_session_name`

[§](#variant.StsEndpoint)

Custom STS endpoint for web identity token exchange

Supported keys:

- `aws_endpoint_url_sts`
- `endpoint_url_sts`

[§](#variant.CopyIfNotExists)

[§](#variant.ConditionalPut)

[§](#variant.SkipSignature)

Skip signing request

[§](#variant.DisableTagging)

Disable tagging objects

This can be desirable if not supported by the backing store

Supported keys:

- `aws_disable_tagging`
- `disable_tagging`

[§](#variant.S3Express)

Enable Support for S3 Express One Zone

Supported keys:

- `aws_s3_express`
- `s3_express`

[§](#variant.RequestPayer)

Enable Support for S3 Requester Pays

Supported keys:

- `aws_request_payer`
- `request_payer`

[§](#variant.Client)

Client options

[§](#variant.Encryption)

Encryption options

[§](#impl-Freeze-for-AmazonS3ConfigKey)

[§](#impl-RefUnwindSafe-for-AmazonS3ConfigKey)

[§](#impl-Send-for-AmazonS3ConfigKey)

[§](#impl-Sync-for-AmazonS3ConfigKey)

[§](#impl-Unpin-for-AmazonS3ConfigKey)

[§](#impl-UnwindSafe-for-AmazonS3ConfigKey)
