# Struct GcpSigningCredential Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/gcp/credential.rs.html#108-121" class="src">Source</a>

``` rust
pub struct GcpSigningCredential {
    pub email: String,
    pub private_key: Option<ServiceAccountKey>,
}
```

Available on **crate feature `gcp`** only.

Expand description

A Google Cloud Storage Credential for signing

## Fields<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpSigningCredential.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpSigningCredential.html#structfield.email" class="anchor field">§</a>`email: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The email of the service account

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpSigningCredential.html#structfield.private_key" class="anchor field">§</a>`private_key: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html" class="struct" title="struct object_store::gcp::ServiceAccountKey"><code>ServiceAccountKey</code></a>`>`

An optional RSA private key

If provided this will be used to sign the URL, otherwise a call will be made to [`iam.serviceAccounts.signBlob`](https://cloud.google.com/storage/docs/authentication/creating-signatures). This allows supporting credential sources that don’t expose the service account private key, e.g. [IMDS](https://cloud.google.com/docs/authentication/get-id-token#metadata-server).

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpSigningCredential.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpSigningCredential.html#impl-Debug-for-GcpSigningCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpSigningCredential.html" class="struct" title="struct object_store::gcp::GcpSigningCredential">GcpSigningCredential</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpSigningCredential.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpSigningCredential.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpSigningCredential.html#blanket-implementations" class="anchor">§</a>
