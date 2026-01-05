# Struct Certificate Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/mod.rs.html#199" class="src">Source</a>

``` rust
pub struct Certificate(/* private fields */);
```

Available on **crate feature `cloud` and non-WebAssembly** only.

Expand description

Represents a CA certificate provided by the user.

This is used to configure the client to trust a specific certificate. See [Self::from_pem](https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#method.from_pem "associated function object_store::client::Certificate::from_pem") for an example

## Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#impl-Certificate" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html" class="struct" title="struct object_store::client::Certificate">Certificate</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#method.from_pem" class="fn">from_pem</a>(pem: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<Self\>

Create a `Certificate` from a PEM encoded certificate.

##### <a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#example-from-a-pem-file" class="doc-anchor">§</a>Example from a PEM file

``` rust
let mut buf = Vec::new();
File::open("my_cert.pem").unwrap()
  .read_to_end(&mut buf).unwrap();
let cert = Certificate::from_pem(&buf).unwrap();
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#method.from_pem_bundle" class="fn">from_pem_bundle</a>(pem_bundle: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self\>\>

Create a collection of `Certificate` from a PEM encoded certificate bundle.

Files that contain such collections have extensions such as `.crt`, `.cer` and `.pem` files.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#method.from_der" class="fn">from_der</a>(der: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<Self\>

Create a `Certificate` from a binary DER encoded certificate.

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#impl-Clone-for-Certificate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html" class="struct" title="struct object_store::client::Certificate">Certificate</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html" class="struct" title="struct object_store::client::Certificate">Certificate</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#impl-Debug-for-Certificate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html" class="struct" title="struct object_store::client::Certificate">Certificate</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html#blanket-implementations" class="anchor">§</a>
