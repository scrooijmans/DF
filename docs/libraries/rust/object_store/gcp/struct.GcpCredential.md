# Struct GcpCredential Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/gcp/credential.rs.html#173-176" class="src">Source</a>

``` rust
pub struct GcpCredential {
    pub bearer: String,
}
```

Available on **crate feature `gcp`** only.

Expand description

A Google Cloud Storage Credential

## Fields<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#structfield.bearer" class="anchor field">§</a>`bearer: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

An HTTP bearer token

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#impl-Debug-for-GcpCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html" class="struct" title="struct object_store::gcp::GcpCredential">GcpCredential</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#impl-PartialEq-for-GcpCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html" class="struct" title="struct object_store::gcp::GcpCredential">GcpCredential</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html" class="struct" title="struct object_store::gcp::GcpCredential">GcpCredential</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#impl-Eq-for-GcpCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html" class="struct" title="struct object_store::gcp::GcpCredential">GcpCredential</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#impl-StructuralPartialEq-for-GcpCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html" class="struct" title="struct object_store::gcp::GcpCredential">GcpCredential</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html#blanket-implementations" class="anchor">§</a>
