# Enum Attribute Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/attributes.rs.html#25-60" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum Attribute {
    ContentDisposition,
    ContentEncoding,
    ContentLanguage,
    ContentType,
    CacheControl,
    StorageClass,
    Metadata(Cow<'static, str>),
}
```

Expand description

Additional object attribute types

## Variants (Non-exhaustive)<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#variant.ContentDisposition" class="anchor">§</a>

### ContentDisposition

Specifies how the object should be handled by a browser

See [Content-Disposition](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition)

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#variant.ContentEncoding" class="anchor">§</a>

### ContentEncoding

Specifies the encodings applied to the object

See [Content-Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding)

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#variant.ContentLanguage" class="anchor">§</a>

### ContentLanguage

Specifies the language of the object

See [Content-Language](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Language)

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#variant.ContentType" class="anchor">§</a>

### ContentType

Specifies the MIME type of the object

This takes precedence over any [ClientOptions](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html "struct object_store::client::ClientOptions") configuration

See [Content-Type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type)

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#variant.CacheControl" class="anchor">§</a>

### CacheControl

Overrides cache control policy of the object

See [Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control)

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#variant.StorageClass" class="anchor">§</a>

### StorageClass

Specifies the storage class of the object.

See [AWS](https://aws.amazon.com/s3/storage-classes/), [GCP](https://cloud.google.com/storage/docs/storage-classes), and [Azure](https://learn.microsoft.com/en-us/rest/api/storageservices/set-blob-tier).  
`StorageClass` is used as the name for this attribute because 2 of the 3 storage providers use that name

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#variant.Metadata" class="anchor">§</a>

### Metadata(<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'static, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>)

Specifies a user-defined metadata field for the object

The String is a user-defined key

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#impl-Clone-for-Attribute" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#impl-Debug-for-Attribute" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#impl-Hash-for-Attribute" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#impl-PartialEq-for-Attribute" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#impl-Eq-for-Attribute" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#impl-StructuralPartialEq-for-Attribute" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html#blanket-implementations" class="anchor">§</a>
