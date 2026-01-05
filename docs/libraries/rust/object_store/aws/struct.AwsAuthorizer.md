# Struct AwsAuthorizer Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/aws/credential.rs.html#108-116" class="src">Source</a>

``` rust
pub struct AwsAuthorizer<'a> { /* private fields */ }
```

Available on **crate feature `aws`** only.

Expand description

Authorize a [`HttpRequest`](https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html "type object_store::client::HttpRequest") with an [`AwsCredential`](https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html "struct object_store::aws::AwsCredential") using [AWS SigV4](https://docs.aws.amazon.com/general/latest/gr/sigv4-calculate-signature.html)

## Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#impl-AwsAuthorizer%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html" class="struct" title="struct object_store::aws::AwsAuthorizer">AwsAuthorizer</a>\<'a\>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#method.new" class="fn">new</a>( credential: &'a <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html" class="struct" title="struct object_store::aws::AwsCredential">AwsCredential</a>, service: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, region: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> Self

Create a new [`AwsAuthorizer`](https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html "struct object_store::aws::AwsAuthorizer")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#method.with_sign_payload" class="fn">with_sign_payload</a>(self, signed: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Controls whether this [`AwsAuthorizer`](https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html "struct object_store::aws::AwsAuthorizer") will attempt to sign the request payload, the default is `true`

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#method.with_request_payer" class="fn">with_request_payer</a>(self, request_payer: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set whether to include requester pays headers

<https://docs.aws.amazon.com/AmazonS3/latest/userguide/ObjectsinRequesterPaysBuckets.html>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#method.authorize" class="fn">authorize</a>( &self, request: &mut <a href="https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html" class="type" title="type object_store::client::HttpRequest">HttpRequest</a>, pre_calculated_digest: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>, )

Authorize `request` with an optional pre-calculated SHA256 digest by attaching the relevant [AWS SigV4](https://docs.aws.amazon.com/IAM/latest/UserGuide/create-signed-request.html) headers

##### <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#payload-signature" class="doc-anchor">§</a>Payload Signature

AWS SigV4 requests must contain the `x-amz-content-sha256` header, it is set as follows:

- If not configured to sign payloads, it is set to `UNSIGNED-PAYLOAD`
- If a `pre_calculated_digest` is provided, it is set to the hex encoding of it
- If it is a streaming request, it is set to `STREAMING-AWS4-HMAC-SHA256-PAYLOAD`
- Otherwise it is set to the hex encoded SHA256 of the request body

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#impl-Debug-for-AwsAuthorizer%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html" class="struct" title="struct object_store::aws::AwsAuthorizer">AwsAuthorizer</a>\<'a\>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html#blanket-implementations" class="anchor">§</a>
