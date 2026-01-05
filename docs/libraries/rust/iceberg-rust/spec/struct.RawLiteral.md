# Struct RawLiteral Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/values.rs.html#2269" class="src">Source</a>

``` rust
pub struct RawLiteral(/* private fields */);
```

Expand description

Raw literal representation used for serde. The serialize way is used for Avro serializer.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#impl-RawLiteral" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html" class="struct" title="struct iceberg::spec::RawLiteral">RawLiteral</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#method.try_from" class="fn">try_from</a>(literal: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>, ty: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>\>

Covert literal to raw literal.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#method.try_into" class="fn">try_into</a>(self, ty: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>\>, <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>\>

Convert raw literal to literal.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#impl-Debug-for-RawLiteral" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html" class="struct" title="struct iceberg::spec::RawLiteral">RawLiteral</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#impl-Deserialize%3C&#39;de%3E-for-RawLiteral" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html" class="struct" title="struct iceberg::spec::RawLiteral">RawLiteral</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#impl-Serialize-for-RawLiteral" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html" class="struct" title="struct iceberg::spec::RawLiteral">RawLiteral</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html#blanket-implementations" class="anchor">§</a>
