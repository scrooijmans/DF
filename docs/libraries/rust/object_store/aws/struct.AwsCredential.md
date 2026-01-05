# Struct AwsCredential Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/aws/credential.rs.html#71-78" class="src">Source</a>

``` rust
pub struct AwsCredential {
    pub key_id: String,
    pub secret_key: String,
    pub token: Option<String>,
}
```

Available on **crate feature `aws`** only.

Expand description

A set of AWS security credentials

## Fields<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#structfield.key_id" class="anchor field">§</a>`key_id: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

AWS_ACCESS_KEY_ID

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#structfield.secret_key" class="anchor field">§</a>`secret_key: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

AWS_SECRET_ACCESS_KEY

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#structfield.token" class="anchor field">§</a>`token: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

AWS_SESSION_TOKEN

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#impl-Debug-for-AwsCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html" class="struct" title="struct object_store::aws::AwsCredential">AwsCredential</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#impl-PartialEq-for-AwsCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html" class="struct" title="struct object_store::aws::AwsCredential">AwsCredential</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html" class="struct" title="struct object_store::aws::AwsCredential">AwsCredential</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#impl-Eq-for-AwsCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html" class="struct" title="struct object_store::aws::AwsCredential">AwsCredential</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#impl-StructuralPartialEq-for-AwsCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html" class="struct" title="struct object_store::aws::AwsCredential">AwsCredential</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html#blanket-implementations" class="anchor">§</a>
