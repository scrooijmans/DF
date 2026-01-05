# Enum S3CopyIfNotExists Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/aws/precondition.rs.html#29-74" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum S3CopyIfNotExists {
    Header(String, String),
    HeaderWithStatus(String, String, StatusCode),
    Multipart,
    Dynamo(DynamoCommit),
}
```

Available on **crate feature `aws`** only.

Expand description

Configure how to provide [`ObjectStore::copy_if_not_exists`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.copy_if_not_exists "method object_store::ObjectStore::copy_if_not_exists") for [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3").

## Variants (Non-exhaustive)<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#variant.Header" class="anchor">§</a>

### Header(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Some S3-compatible stores, such as Cloudflare R2, support copy if not exists semantics through custom headers.

If set, [`ObjectStore::copy_if_not_exists`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.copy_if_not_exists "method object_store::ObjectStore::copy_if_not_exists") will perform a normal copy operation with the provided header pair, and expect the store to fail with `412 Precondition Failed` if the destination file already exists.

Encoded as `header:<HEADER_NAME>:<HEADER_VALUE>` ignoring whitespace

For example `header: cf-copy-destination-if-none-match: *`, would set the header `cf-copy-destination-if-none-match` to `*`

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#variant.HeaderWithStatus" class="anchor">§</a>

### HeaderWithStatus(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/status/struct.StatusCode.html" class="struct" title="struct http::status::StatusCode">StatusCode</a>)

The same as [`S3CopyIfNotExists::Header`](https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#variant.Header "variant object_store::aws::S3CopyIfNotExists::Header") but allows custom status code checking, for object stores that return values other than 412.

Encoded as `header-with-status:<HEADER_NAME>:<HEADER_VALUE>:<STATUS>` ignoring whitespace

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#variant.Multipart" class="anchor">§</a>

### Multipart

Native Amazon S3 supports copy if not exists through a multipart upload where the upload copies an existing object and is completed only if the new object does not already exist.

WARNING: When using this mode, `copy_if_not_exists` does not copy tags or attributes from the source object.

WARNING: When using this mode, `copy_if_not_exists` makes only a best effort attempt to clean up the multipart upload if the copy operation fails. Consider using a lifecycle rule to automatically clean up abandoned multipart uploads. See [the module docs](https://docs.rs/object_store/latest/object_store/aws/index.html#multipart-uploads "mod object_store::aws") for details.

Encoded as `multipart` ignoring whitespace.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#variant.Dynamo" class="anchor">§</a>

### Dynamo(<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html" class="struct" title="struct object_store::aws::DynamoCommit">DynamoCommit</a>)

👎Deprecated: Use S3CopyIfNotExists::Multipart

The name of a DynamoDB table to use for coordination

Encoded as either `dynamo:<TABLE_NAME>` or `dynamo:<TABLE_NAME>:<TIMEOUT_MILLIS>` ignoring whitespace. The default timeout is used if not specified

See [`DynamoCommit`](https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html "struct object_store::aws::DynamoCommit") for more information

This will use the same region, credentials and endpoint as configured for S3

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#impl-Clone-for-S3CopyIfNotExists" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html" class="enum" title="enum object_store::aws::S3CopyIfNotExists">S3CopyIfNotExists</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html" class="enum" title="enum object_store::aws::S3CopyIfNotExists">S3CopyIfNotExists</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#impl-Debug-for-S3CopyIfNotExists" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html" class="enum" title="enum object_store::aws::S3CopyIfNotExists">S3CopyIfNotExists</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#impl-Display-for-S3CopyIfNotExists" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html" class="enum" title="enum object_store::aws::S3CopyIfNotExists">S3CopyIfNotExists</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#impl-PartialEq-for-S3CopyIfNotExists" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html" class="enum" title="enum object_store::aws::S3CopyIfNotExists">S3CopyIfNotExists</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html" class="enum" title="enum object_store::aws::S3CopyIfNotExists">S3CopyIfNotExists</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#impl-Eq-for-S3CopyIfNotExists" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html" class="enum" title="enum object_store::aws::S3CopyIfNotExists">S3CopyIfNotExists</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#impl-StructuralPartialEq-for-S3CopyIfNotExists" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html" class="enum" title="enum object_store::aws::S3CopyIfNotExists">S3CopyIfNotExists</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html#blanket-implementations" class="anchor">§</a>
