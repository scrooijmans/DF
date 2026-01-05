# Enum S3ConditionalPut Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/aws/precondition.rs.html#135-158" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum S3ConditionalPut {
    ETagMatch,
    Dynamo(DynamoCommit),
    Disabled,
}
```

Available on **crate feature `aws`** only.

Expand description

Configure how to provide conditional put support for [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3").

## Variants (Non-exhaustive)<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#variant.ETagMatch" class="anchor">§</a>

### ETagMatch

Some S3-compatible stores, such as Cloudflare R2 and minio support conditional put using the standard [HTTP precondition](https://datatracker.ietf.org/doc/html/rfc9110#name-preconditions) headers If-Match and If-None-Match

Encoded as `etag` ignoring whitespace

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#variant.Dynamo" class="anchor">§</a>

### Dynamo(<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html" class="struct" title="struct object_store::aws::DynamoCommit">DynamoCommit</a>)

👎Deprecated: Use S3ConditionalPut::ETagMatch

The name of a DynamoDB table to use for coordination

Encoded as either `dynamo:<TABLE_NAME>` or `dynamo:<TABLE_NAME>:<TIMEOUT_MILLIS>` ignoring whitespace. The default timeout is used if not specified

See [`DynamoCommit`](https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html "struct object_store::aws::DynamoCommit") for more information

This will use the same region, credentials and endpoint as configured for S3

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#variant.Disabled" class="anchor">§</a>

### Disabled

Disable `conditional put`

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#impl-Clone-for-S3ConditionalPut" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#impl-Debug-for-S3ConditionalPut" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#impl-Default-for-S3ConditionalPut" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#impl-Display-for-S3ConditionalPut" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#impl-PartialEq-for-S3ConditionalPut" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#impl-Eq-for-S3ConditionalPut" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#impl-StructuralPartialEq-for-S3ConditionalPut" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html#blanket-implementations" class="anchor">§</a>
