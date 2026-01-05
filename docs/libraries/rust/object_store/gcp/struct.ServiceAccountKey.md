# Struct ServiceAccountKey Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/gcp/credential.rs.html#125" class="src">Source</a>

``` rust
pub struct ServiceAccountKey(/* private fields */);
```

Available on **crate feature `gcp`** only.

Expand description

A private RSA key for a service account

## Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html#impl-ServiceAccountKey" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html" class="struct" title="struct object_store::gcp::ServiceAccountKey">ServiceAccountKey</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html#method.from_pem" class="fn">from_pem</a>(encoded: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, Error\>

Parses a pem-encoded RSA key

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html#method.from_pkcs8" class="fn">from_pkcs8</a>(key: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, Error\>

Parses an unencrypted PKCS#8-encoded RSA private key.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html#method.from_der" class="fn">from_der</a>(key: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, Error\>

Parses an unencrypted PKCS#8-encoded RSA private key.

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html#impl-Debug-for-ServiceAccountKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html" class="struct" title="struct object_store::gcp::ServiceAccountKey">ServiceAccountKey</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html#blanket-implementations" class="anchor">§</a>
