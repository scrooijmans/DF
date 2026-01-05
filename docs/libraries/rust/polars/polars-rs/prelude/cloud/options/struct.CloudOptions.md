# Struct CloudOptions Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/cloud/options.rs.html#82" class="src">Source</a>

``` rust
pub struct CloudOptions {
    pub max_retries: usize,
    pub file_cache_ttl: u64,
    /* private fields */
}
```

Available on **crate feature `polars-io`** only.

Expand description

Options to connect to various cloud providers.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#structfield.max_retries" class="anchor field">§</a>`max_retries: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#structfield.file_cache_ttl" class="anchor field">§</a>`file_cache_ttl: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#impl-CloudOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.default_static_ref" class="fn">default_static_ref</a>() -\> &'static <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#impl-CloudOptions-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.with_max_retries" class="fn">with_max_retries</a>(self, max_retries: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

Set the maximum number of retries.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.with_credential_provider" class="fn">with_credential_provider</a>( self, credential_provider: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/enum.PlCredentialProvider.html" class="enum" title="enum polars::prelude::cloud::credential_provider::PlCredentialProvider">PlCredentialProvider</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.build_http" class="fn">build_http</a>(&self, url: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<impl <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.from_untyped_config" class="fn">from_untyped_config</a>\<I\>( url: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, config: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>)\>,

Parse a configuration from a Hashmap. This is the interface from Python.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#impl-Clone-for-CloudOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#impl-Debug-for-CloudOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#impl-Default-for-CloudOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#impl-Deserialize%3C&#39;de%3E-for-CloudOptions" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#impl-Hash-for-CloudOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#impl-PartialEq-for-CloudOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#impl-Serialize-for-CloudOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#impl-Eq-for-CloudOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#impl-StructuralPartialEq-for-CloudOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/struct.CloudOptions.html#blanket-implementations" class="anchor">§</a>
