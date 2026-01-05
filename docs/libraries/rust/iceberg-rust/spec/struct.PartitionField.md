# Struct PartitionField Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/partition.rs.html#38-48" class="src">Source</a>

``` rust
pub struct PartitionField {
    pub source_id: i32,
    pub field_id: i32,
    pub name: String,
    pub transform: Transform,
}
```

Expand description

Partition fields capture the transform from table data to partition values.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#structfield.source_id" class="anchor field">§</a>`source_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

A source column id from the table’s schema

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#structfield.field_id" class="anchor field">§</a>`field_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

A partition field id that is used to identify a partition field and is unique within a partition spec. In v2 table metadata, it is unique across all partition specs.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#structfield.name" class="anchor field">§</a>`name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

A partition name.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#structfield.transform" class="anchor field">§</a>`transform: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform"><code>Transform</code></a>

A transform that is applied to the source column to produce a partition value.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#impl-PartitionField" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#method.builder" class="fn">builder</a>() -\> PartitionFieldBuilder\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>)\>

Create a builder for building `PartitionField`. On the builder, call `.source_id(...)`, `.field_id(...)`, `.name(...)`, `.transform(...)` to set the values of the fields. Finally, call `.build()` to create the instance of `PartitionField`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#impl-PartitionField-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#method.into_unbound" class="fn">into_unbound</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionField.html" class="struct" title="struct iceberg::spec::UnboundPartitionField">UnboundPartitionField</a>

To unbound partition field

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#impl-Clone-for-PartitionField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#impl-Debug-for-PartitionField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#impl-Deserialize%3C&#39;de%3E-for-PartitionField" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#impl-From%3CPartitionField%3E-for-UnboundPartitionField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionField.html" class="struct" title="struct iceberg::spec::UnboundPartitionField">UnboundPartitionField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(field: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#impl-PartialEq-for-PartitionField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#impl-Serialize-for-PartitionField" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#impl-Eq-for-PartitionField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#impl-StructuralPartialEq-for-PartitionField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html#blanket-implementations" class="anchor">§</a>
