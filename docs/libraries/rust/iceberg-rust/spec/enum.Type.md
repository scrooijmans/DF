# Enum Type Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/datatypes.rs.html#84-93" class="src">Source</a>

``` rust
pub enum Type {
    Primitive(PrimitiveType),
    Struct(StructType),
    List(ListType),
    Map(MapType),
}
```

Expand description

All data types are either primitives or nested types, which are maps, lists, or structs.

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#variant.Primitive" class="anchor">§</a>

### Primitive(<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>)

Primitive types

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#variant.Struct" class="anchor">§</a>

### Struct(<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StructType.html" class="struct" title="struct iceberg::spec::StructType">StructType</a>)

Struct type

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#variant.List" class="anchor">§</a>

### List(<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>)

List type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#variant.Map" class="anchor">§</a>

### Map(<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>)

Map type

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-Type" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.is_primitive" class="fn">is_primitive</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether the type is primitive type.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.is_struct" class="fn">is_struct</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether the type is struct type.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.is_nested" class="fn">is_nested</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether the type is nested type.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.as_primitive_type" class="fn">as_primitive_type</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>\>

Convert Type to reference of PrimitiveType

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.to_struct_type" class="fn">to_struct_type</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StructType.html" class="struct" title="struct iceberg::spec::StructType">StructType</a>\>

Convert Type to StructType

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.decimal_max_precision" class="fn">decimal_max_precision</a>(num_bytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

Return max precision for decimal given \[`num_bytes`\] bytes.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.decimal_required_bytes" class="fn">decimal_required_bytes</a>(precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

Returns minimum bytes required for decimal with \[`precision`\].

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.decimal" class="fn">decimal</a>(precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, scale: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates decimal type.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.is_floating_type" class="fn">is_floating_type</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if it’s float or double type.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-Clone-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-Debug-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-Deserialize%3C&#39;de%3E-for-Type" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>(deserializer: D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-Display-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-From%3CListType%3E-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-From%3CMapType%3E-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-From%3CPrimitiveType%3E-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-From%3CStructType%3E-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StructType.html" class="struct" title="struct iceberg::spec::StructType">StructType</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StructType.html" class="struct" title="struct iceberg::spec::StructType">StructType</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-PartialEq-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-Serialize-for-Type" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>(&self, serializer: S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-Eq-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#impl-StructuralPartialEq-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html#blanket-implementations" class="anchor">§</a>
