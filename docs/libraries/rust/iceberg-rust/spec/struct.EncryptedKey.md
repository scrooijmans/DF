# Struct EncryptedKey Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/encrypted_key.rs.html#26-39" class="src">Source</a>

``` rust
pub struct EncryptedKey { /* private fields */ }
```

Expand description

Keys used for table encryption

Serializing of `encrypted_key_metadata` is done using base64 encoding.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#impl-EncryptedKey" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.builder" class="fn">builder</a>() -\> EncryptedKeyBuilder\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>)\>

Create a builder for building `EncryptedKey`. On the builder, call `.key_id(...)`, `.encrypted_key_metadata(...)`, `.encrypted_by_id(...)`, `.properties(...)`(optional) to set the values of the fields. Finally, call `.build()` to create the instance of `EncryptedKey`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#impl-EncryptedKey-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.key_id" class="fn">key_id</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns the key ID

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.encrypted_key_metadata" class="fn">encrypted_key_metadata</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the encrypted key metadata

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.encrypted_by_id" class="fn">encrypted_by_id</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns the ID of the entity that encrypted this key

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.properties" class="fn">properties</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns the properties map

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#impl-Clone-for-EncryptedKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#impl-Debug-for-EncryptedKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#impl-Deserialize%3C&#39;de%3E-for-EncryptedKey" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>(deserializer: D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#impl-PartialEq-for-EncryptedKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#impl-Serialize-for-EncryptedKey" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>(&self, serializer: S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#impl-Eq-for-EncryptedKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#impl-StructuralPartialEq-for-EncryptedKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html#blanket-implementations" class="anchor">§</a>
