# Enum AmazonS3ConfigKey Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/aws/builder.rs.html#204-399" class="src">Source</a>

``` rust
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

Configuration keys for [`AmazonS3Builder`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html "struct object_store::aws::AmazonS3Builder")

Configuration via keys can be done via [`AmazonS3Builder::with_config`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_config "method object_store::aws::AmazonS3Builder::with_config")

## <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#example" class="doc-anchor">§</a>Example

``` rust
let builder = AmazonS3Builder::new()
    .with_config("aws_access_key_id".parse().unwrap(), "my-access-key-id")
    .with_config(AmazonS3ConfigKey::DefaultRegion, "my-default-region");
```

## Variants (Non-exhaustive)<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.AccessKeyId" class="anchor">§</a>

### AccessKeyId

AWS Access Key

See [`AmazonS3Builder::with_access_key_id`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_access_key_id "method object_store::aws::AmazonS3Builder::with_access_key_id") for details.

Supported keys:

- `aws_access_key_id`
- `access_key_id`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.SecretAccessKey" class="anchor">§</a>

### SecretAccessKey

Secret Access Key

See [`AmazonS3Builder::with_secret_access_key`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_secret_access_key "method object_store::aws::AmazonS3Builder::with_secret_access_key") for details.

Supported keys:

- `aws_secret_access_key`
- `secret_access_key`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.Region" class="anchor">§</a>

### Region

Region

See [`AmazonS3Builder::with_region`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_region "method object_store::aws::AmazonS3Builder::with_region") for details.

Supported keys:

- `aws_region`
- `region`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.DefaultRegion" class="anchor">§</a>

### DefaultRegion

Default region

See [`AmazonS3Builder::with_region`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_region "method object_store::aws::AmazonS3Builder::with_region") for details.

Supported keys:

- `aws_default_region`
- `default_region`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.Bucket" class="anchor">§</a>

### Bucket

Bucket name

See [`AmazonS3Builder::with_bucket_name`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_bucket_name "method object_store::aws::AmazonS3Builder::with_bucket_name") for details.

Supported keys:

- `aws_bucket`
- `aws_bucket_name`
- `bucket`
- `bucket_name`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.Endpoint" class="anchor">§</a>

### Endpoint

Sets custom endpoint for communicating with AWS S3.

See [`AmazonS3Builder::with_endpoint`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_endpoint "method object_store::aws::AmazonS3Builder::with_endpoint") for details.

Supported keys:

- `aws_endpoint`
- `aws_endpoint_url`
- `endpoint`
- `endpoint_url`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.Token" class="anchor">§</a>

### Token

Token to use for requests (passed to underlying provider)

See [`AmazonS3Builder::with_token`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_token "method object_store::aws::AmazonS3Builder::with_token") for details.

Supported keys:

- `aws_session_token`
- `aws_token`
- `session_token`
- `token`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.ImdsV1Fallback" class="anchor">§</a>

### ImdsV1Fallback

Fall back to ImdsV1

See [`AmazonS3Builder::with_imdsv1_fallback`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_imdsv1_fallback "method object_store::aws::AmazonS3Builder::with_imdsv1_fallback") for details.

Supported keys:

- `aws_imdsv1_fallback`
- `imdsv1_fallback`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.VirtualHostedStyleRequest" class="anchor">§</a>

### VirtualHostedStyleRequest

If virtual hosted style request has to be used

See [`AmazonS3Builder::with_virtual_hosted_style_request`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_virtual_hosted_style_request "method object_store::aws::AmazonS3Builder::with_virtual_hosted_style_request") for details.

Supported keys:

- `aws_virtual_hosted_style_request`
- `virtual_hosted_style_request`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.UnsignedPayload" class="anchor">§</a>

### UnsignedPayload

Avoid computing payload checksum when calculating signature.

See [`AmazonS3Builder::with_unsigned_payload`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_unsigned_payload "method object_store::aws::AmazonS3Builder::with_unsigned_payload") for details.

Supported keys:

- `aws_unsigned_payload`
- `unsigned_payload`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.Checksum" class="anchor">§</a>

### Checksum

Set the checksum algorithm for this client

See [`AmazonS3Builder::with_checksum_algorithm`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_checksum_algorithm "method object_store::aws::AmazonS3Builder::with_checksum_algorithm")

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.MetadataEndpoint" class="anchor">§</a>

### MetadataEndpoint

Set the instance metadata endpoint

See [`AmazonS3Builder::with_metadata_endpoint`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html#method.with_metadata_endpoint "method object_store::aws::AmazonS3Builder::with_metadata_endpoint") for details.

Supported keys:

- `aws_metadata_endpoint`
- `metadata_endpoint`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.ContainerCredentialsRelativeUri" class="anchor">§</a>

### ContainerCredentialsRelativeUri

Set the container credentials relative URI when used in ECS

<https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.ContainerCredentialsFullUri" class="anchor">§</a>

### ContainerCredentialsFullUri

Set the container credentials full URI when used in EKS

<https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.ContainerAuthorizationTokenFile" class="anchor">§</a>

### ContainerAuthorizationTokenFile

Set the authorization token in plain text when used in EKS to authenticate with ContainerCredentialsFullUri

<https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.WebIdentityTokenFile" class="anchor">§</a>

### WebIdentityTokenFile

Web identity token file path for AssumeRoleWithWebIdentity

Supported keys:

- `aws_web_identity_token_file`
- `web_identity_token_file`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.RoleArn" class="anchor">§</a>

### RoleArn

Role ARN to assume when using web identity token

Supported keys:

- `aws_role_arn`
- `role_arn`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.RoleSessionName" class="anchor">§</a>

### RoleSessionName

Session name for web identity role assumption

Supported keys:

- `aws_role_session_name`
- `role_session_name`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.StsEndpoint" class="anchor">§</a>

### StsEndpoint

Custom STS endpoint for web identity token exchange

Supported keys:

- `aws_endpoint_url_sts`
- `endpoint_url_sts`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.CopyIfNotExists" class="anchor">§</a>

### CopyIfNotExists

Configure how to provide `copy_if_not_exists`

See [`S3CopyIfNotExists`](https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html "enum object_store::aws::S3CopyIfNotExists")

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.ConditionalPut" class="anchor">§</a>

### ConditionalPut

Configure how to provide conditional put operations

See [`S3ConditionalPut`](https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html "enum object_store::aws::S3ConditionalPut")

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.SkipSignature" class="anchor">§</a>

### SkipSignature

Skip signing request

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.DisableTagging" class="anchor">§</a>

### DisableTagging

Disable tagging objects

This can be desirable if not supported by the backing store

Supported keys:

- `aws_disable_tagging`
- `disable_tagging`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.S3Express" class="anchor">§</a>

### S3Express

Enable Support for S3 Express One Zone

Supported keys:

- `aws_s3_express`
- `s3_express`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.RequestPayer" class="anchor">§</a>

### RequestPayer

Enable Support for S3 Requester Pays

Supported keys:

- `aws_request_payer`
- `request_payer`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.Client" class="anchor">§</a>

### Client(<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>)

Client options

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#variant.Encryption" class="anchor">§</a>

### Encryption(S3EncryptionConfigKey)

Encryption options

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#impl-AsRef%3Cstr%3E-for-AmazonS3ConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#impl-Clone-for-AmazonS3ConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#impl-Debug-for-AmazonS3ConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#impl-Deserialize%3C&#39;de%3E-for-AmazonS3ConfigKey" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#impl-FromStr-for-AmazonS3ConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<Self, Self::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#impl-Hash-for-AmazonS3ConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#impl-PartialEq-for-AmazonS3ConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#impl-Serialize-for-AmazonS3ConfigKey" class="anchor">§</a>

### impl <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde_core::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html" class="trait" title="trait serde_core::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#impl-Copy-for-AmazonS3ConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#impl-Eq-for-AmazonS3ConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#impl-StructuralPartialEq-for-AmazonS3ConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html#blanket-implementations" class="anchor">§</a>
