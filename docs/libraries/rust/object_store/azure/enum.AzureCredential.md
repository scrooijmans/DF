# Enum AzureCredential Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/azure/credential.rs.html#127-140" class="src">Source</a>

``` rust
pub enum AzureCredential {
    AccessKey(AzureAccessKey),
    SASToken(Vec<(String, String)>),
    BearerToken(String),
}
```

Available on **crate feature `azure`** only.

Expand description

An Azure storage credential

## Variants<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#variants" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#variant.AccessKey" class="anchor">§</a>

### AccessKey(<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html" class="struct" title="struct object_store::azure::AzureAccessKey">AzureAccessKey</a>)

A shared access key

<https://learn.microsoft.com/en-us/rest/api/storageservices/authorize-with-shared-key>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#variant.SASToken" class="anchor">§</a>

### SASToken(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)\>)

A shared access signature

<https://learn.microsoft.com/en-us/rest/api/storageservices/delegate-access-with-shared-access-signature>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#variant.BearerToken" class="anchor">§</a>

### BearerToken(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

An authorization token

<https://learn.microsoft.com/en-us/rest/api/storageservices/authorize-with-azure-active-directory>

## Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#impl-AzureCredential" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html" class="enum" title="enum object_store::azure::AzureCredential">AzureCredential</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#method.sensitive_request" class="fn">sensitive_request</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determines if the credential requires the request be treated as sensitive

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#impl-Debug-for-AzureCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html" class="enum" title="enum object_store::azure::AzureCredential">AzureCredential</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#impl-PartialEq-for-AzureCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html" class="enum" title="enum object_store::azure::AzureCredential">AzureCredential</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html" class="enum" title="enum object_store::azure::AzureCredential">AzureCredential</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#impl-Eq-for-AzureCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html" class="enum" title="enum object_store::azure::AzureCredential">AzureCredential</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#impl-StructuralPartialEq-for-AzureCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html" class="enum" title="enum object_store::azure::AzureCredential">AzureCredential</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html#blanket-implementations" class="anchor">§</a>
