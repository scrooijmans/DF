# Enum PrimitiveType Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/datatypes.rs.html#210-250" class="src">Source</a>

``` rust
pub enum PrimitiveType {
Show 16 variants    Boolean,
    Int,
    Long,
    Float,
    Double,
    Decimal {
        precision: u32,
        scale: u32,
    },
    Date,
    Time,
    Timestamp,
    Timestamptz,
    TimestampNs,
    TimestamptzNs,
    String,
    Uuid,
    Fixed(u64),
    Binary,
}
```

Expand description

Primitive data types

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Boolean" class="anchor">§</a>

### Boolean

True or False

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Int" class="anchor">§</a>

### Int

32-bit signed integer

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Long" class="anchor">§</a>

### Long

64-bit signed integer

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Float" class="anchor">§</a>

### Float

32-bit IEEE 754 floating point.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Double" class="anchor">§</a>

### Double

64-bit IEEE 754 floating point.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Decimal" class="anchor">§</a>

### Decimal

Fixed point decimal

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Decimal.field.precision" class="anchor field">§</a>`precision: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>

Precision, must be 38 or less

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Decimal.field.scale" class="anchor field">§</a>`scale: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>

Scale

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Date" class="anchor">§</a>

### Date

Calendar date without timezone or time.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Time" class="anchor">§</a>

### Time

Time of day in microsecond precision, without date or timezone.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Timestamp" class="anchor">§</a>

### Timestamp

Timestamp in microsecond precision, without timezone

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Timestamptz" class="anchor">§</a>

### Timestamptz

Timestamp in microsecond precision, with timezone

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.TimestampNs" class="anchor">§</a>

### TimestampNs

Timestamp in nanosecond precision, without timezone

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.TimestamptzNs" class="anchor">§</a>

### TimestamptzNs

Timestamp in nanosecond precision with timezone

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.String" class="anchor">§</a>

### String

Arbitrary-length character sequences encoded in utf-8

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Uuid" class="anchor">§</a>

### Uuid

Universally Unique Identifiers, should use 16-byte fixed

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Fixed" class="anchor">§</a>

### Fixed(<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

Fixed length byte array

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#variant.Binary" class="anchor">§</a>

### Binary

Arbitrary-length byte array.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-PrimitiveType" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.serialize" class="fn">serialize</a>\<\_\_S\>( \_\_self: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-PrimitiveType-1" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-PrimitiveType-2" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.compatible" class="fn">compatible</a>(&self, literal: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check whether literal is compatible with the type.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-Clone-for-PrimitiveType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-Debug-for-PrimitiveType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-Deserialize%3C&#39;de%3E-for-PrimitiveType" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.deserialize-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>(deserializer: D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-Display-for-PrimitiveType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-From%3CPrimitiveType%3E-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-Hash-for-PrimitiveType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-PartialEq-for-PrimitiveType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-Serialize-for-PrimitiveType" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#method.serialize-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>(&self, serializer: S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-Eq-for-PrimitiveType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#impl-StructuralPartialEq-for-PrimitiveType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html#blanket-implementations" class="anchor">§</a>
