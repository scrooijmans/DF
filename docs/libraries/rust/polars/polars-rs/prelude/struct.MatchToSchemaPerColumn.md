# Struct MatchToSchemaPerColumn Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/match_to_schema.rs.html#23" class="src">Source</a>

``` rust
pub struct MatchToSchemaPerColumn {
    pub missing_columns: MissingColumnsPolicyOrExpr,
    pub missing_struct_fields: MissingColumnsPolicy,
    pub extra_struct_fields: ExtraColumnsPolicy,
    pub integer_cast: UpcastOrForbid,
    pub float_cast: UpcastOrForbid,
}
```

Available on **crate feature `lazy`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#structfield.missing_columns" class="anchor field">§</a>`missing_columns: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.MissingColumnsPolicyOrExpr.html" class="enum" title="enum polars::prelude::MissingColumnsPolicyOrExpr"><code>MissingColumnsPolicyOrExpr</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#structfield.missing_struct_fields" class="anchor field">§</a>`missing_struct_fields: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.MissingColumnsPolicy.html" class="enum" title="enum polars::prelude::MissingColumnsPolicy"><code>MissingColumnsPolicy</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#structfield.extra_struct_fields" class="anchor field">§</a>`extra_struct_fields: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ExtraColumnsPolicy.html" class="enum" title="enum polars::prelude::ExtraColumnsPolicy"><code>ExtraColumnsPolicy</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#structfield.integer_cast" class="anchor field">§</a>`integer_cast: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.UpcastOrForbid.html" class="enum" title="enum polars::prelude::UpcastOrForbid"><code>UpcastOrForbid</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#structfield.float_cast" class="anchor field">§</a>`float_cast: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.UpcastOrForbid.html" class="enum" title="enum polars::prelude::UpcastOrForbid"><code>UpcastOrForbid</code></a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#impl-Clone-for-MatchToSchemaPerColumn" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn">MatchToSchemaPerColumn</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn">MatchToSchemaPerColumn</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#impl-Debug-for-MatchToSchemaPerColumn" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn">MatchToSchemaPerColumn</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#impl-Deserialize%3C&#39;de%3E-for-MatchToSchemaPerColumn" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn">MatchToSchemaPerColumn</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn">MatchToSchemaPerColumn</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#impl-PartialEq-for-MatchToSchemaPerColumn" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn">MatchToSchemaPerColumn</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn">MatchToSchemaPerColumn</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#impl-Serialize-for-MatchToSchemaPerColumn" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn">MatchToSchemaPerColumn</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#impl-Eq-for-MatchToSchemaPerColumn" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn">MatchToSchemaPerColumn</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#impl-StructuralPartialEq-for-MatchToSchemaPerColumn" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn">MatchToSchemaPerColumn</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html#blanket-implementations" class="anchor">§</a>
