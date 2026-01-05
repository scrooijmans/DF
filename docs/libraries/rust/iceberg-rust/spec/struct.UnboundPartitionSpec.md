# Struct UnboundPartitionSpec Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/partition.rs.html#235-240" class="src">Source</a>

``` rust
pub struct UnboundPartitionSpec { /* private fields */ }
```

Expand description

Unbound partition spec can be built without a schema and later bound to a schema. They are used to transport schema information as part of the REST specification. The main difference to [`PartitionSpec`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html "struct iceberg::spec::PartitionSpec") is that the field ids are optional.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#impl-UnboundPartitionSpec" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.builder" class="fn">builder</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpecBuilder">UnboundPartitionSpecBuilder</a>

Create unbound partition spec builder

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.bind" class="fn">bind</a>(self, schema: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>\>

Bind this unbound partition spec to a schema.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.spec_id" class="fn">spec_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

Spec id of the partition spec

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.fields" class="fn">fields</a>(&self) -\> &\[<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionField.html" class="struct" title="struct iceberg::spec::UnboundPartitionField">UnboundPartitionField</a>\]

Fields of the partition spec

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.with_spec_id" class="fn">with_spec_id</a>(self, spec_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> Self

Change the spec id of the partition spec

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#impl-Clone-for-UnboundPartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#impl-Debug-for-UnboundPartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#impl-Default-for-UnboundPartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#impl-Deserialize%3C&#39;de%3E-for-UnboundPartitionSpec" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#impl-From%3CPartitionSpec%3E-for-UnboundPartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(spec: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#impl-PartialEq-for-UnboundPartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#impl-Serialize-for-UnboundPartitionSpec" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#impl-Eq-for-UnboundPartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#impl-StructuralPartialEq-for-UnboundPartitionSpec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html#blanket-implementations" class="anchor">§</a>
