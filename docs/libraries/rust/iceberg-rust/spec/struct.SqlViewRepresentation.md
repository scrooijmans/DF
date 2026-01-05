# Struct SqlViewRepresentation Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/view_version.rs.html#209-216" class="src">Source</a>

``` rust
pub struct SqlViewRepresentation {
    pub sql: String,
    pub dialect: String,
}
```

Expand description

The SQL representation stores the view definition as a SQL SELECT, with metadata such as the SQL dialect.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#structfield.sql" class="anchor field">§</a>`sql: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The SQL SELECT statement that defines the view.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#structfield.dialect" class="anchor field">§</a>`dialect: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The dialect of the sql SELECT statement (e.g., “trino” or “spark”)

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#impl-Clone-for-SqlViewRepresentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#impl-Debug-for-SqlViewRepresentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#impl-Deserialize%3C&#39;de%3E-for-SqlViewRepresentation" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#impl-From%3CSqlViewRepresentation%3E-for-ViewRepresentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ViewRepresentation.html" class="enum" title="enum iceberg::spec::ViewRepresentation">ViewRepresentation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(sql: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#impl-PartialEq-for-SqlViewRepresentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#impl-Serialize-for-SqlViewRepresentation" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#impl-Eq-for-SqlViewRepresentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#impl-StructuralPartialEq-for-SqlViewRepresentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html#blanket-implementations" class="anchor">§</a>
