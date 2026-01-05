# Struct FieldSummary Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest_list.rs.html#676-694" class="src">Source</a>

``` rust
pub struct FieldSummary {
    pub contains_null: bool,
    pub contains_nan: Option<bool>,
    pub lower_bound: Option<ByteBuf>,
    pub upper_bound: Option<ByteBuf>,
}
```

Expand description

Field summary for partition field in the spec.

Each field in the list corresponds to a field in the manifest file’s partition spec.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#structfield.contains_null" class="anchor field">§</a>`contains_null: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

field: 509

Whether the manifest contains at least one partition with a null value for the field

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#structfield.contains_nan" class="anchor field">§</a>`contains_nan: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

field: 518 Whether the manifest contains at least one partition with a NaN value for the field

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#structfield.lower_bound" class="anchor field">§</a>`lower_bound: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ByteBuf.html" class="struct" title="struct iceberg::spec::ByteBuf"><code>ByteBuf</code></a>`>`

field: 510 The minimum value for the field in the manifests partitions.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#structfield.upper_bound" class="anchor field">§</a>`upper_bound: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ByteBuf.html" class="struct" title="struct iceberg::spec::ByteBuf"><code>ByteBuf</code></a>`>`

field: 511 The maximum value for the field in the manifests partitions.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#impl-Clone-for-FieldSummary" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#impl-Debug-for-FieldSummary" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#impl-Default-for-FieldSummary" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#impl-Deserialize%3C&#39;de%3E-for-FieldSummary" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#impl-Hash-for-FieldSummary" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#impl-PartialEq-for-FieldSummary" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#impl-Serialize-for-FieldSummary" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#impl-Eq-for-FieldSummary" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#impl-StructuralPartialEq-for-FieldSummary" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html#blanket-implementations" class="anchor">§</a>
