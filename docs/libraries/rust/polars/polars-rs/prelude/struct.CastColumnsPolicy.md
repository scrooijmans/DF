# Struct CastColumnsPolicy Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/file_scan/mod.rs.html#166" class="src">Source</a>

``` rust
pub struct CastColumnsPolicy {
    pub integer_upcast: bool,
    pub float_upcast: bool,
    pub float_downcast: bool,
    pub datetime_nanoseconds_downcast: bool,
    pub datetime_microseconds_downcast: bool,
    pub datetime_convert_timezone: bool,
    pub null_upcast: bool,
    pub missing_struct_fields: MissingColumnsPolicy,
    pub extra_struct_fields: ExtraColumnsPolicy,
}
```

Available on **crate feature `lazy`** only.

Expand description

Used by scans.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#structfield.integer_upcast" class="anchor field">§</a>`integer_upcast: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Allow casting when target dtype is lossless supertype

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#structfield.float_upcast" class="anchor field">§</a>`float_upcast: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Allow Float32 -\> Float64

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#structfield.float_downcast" class="anchor field">§</a>`float_downcast: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Allow Float64 -\> Float32

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#structfield.datetime_nanoseconds_downcast" class="anchor field">§</a>`datetime_nanoseconds_downcast: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Allow datetime\[ns\] to be casted to any lower precision. Important for being able to read datasets written by spark.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#structfield.datetime_microseconds_downcast" class="anchor field">§</a>`datetime_microseconds_downcast: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Allow datetime\[us\] to datetime\[ms\]

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#structfield.datetime_convert_timezone" class="anchor field">§</a>`datetime_convert_timezone: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Allow casting to change time units.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#structfield.null_upcast" class="anchor field">§</a>`null_upcast: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

DataType::Null to any

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#structfield.missing_struct_fields" class="anchor field">§</a>`missing_struct_fields: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.MissingColumnsPolicy.html" class="enum" title="enum polars::prelude::MissingColumnsPolicy"><code>MissingColumnsPolicy</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#structfield.extra_struct_fields" class="anchor field">§</a>`extra_struct_fields: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ExtraColumnsPolicy.html" class="enum" title="enum polars::prelude::ExtraColumnsPolicy"><code>ExtraColumnsPolicy</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#impl-CastColumnsPolicy" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

#### pub const <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#associatedconstant.ERROR_ON_MISMATCH" class="constant">ERROR_ON_MISMATCH</a>: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

Configuration variant that defaults to raising on mismatch.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#impl-CastColumnsPolicy-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#method.should_cast_column" class="fn">should_cast_column</a>( &self, column_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, target_dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, incoming_dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Checks if casting can be done to a dtype with a configured policy.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#returns" class="doc-anchor">§</a>Returns

- Ok(true): Cast needed to target dtype
- Ok(false): No casting needed
- Err(\_): Forbidden by configuration, or incompatible types.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#impl-Clone-for-CastColumnsPolicy" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#impl-Debug-for-CastColumnsPolicy" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#impl-Default-for-CastColumnsPolicy" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#impl-Deserialize%3C&#39;de%3E-for-CastColumnsPolicy" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#impl-Hash-for-CastColumnsPolicy" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#impl-PartialEq-for-CastColumnsPolicy" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#impl-Serialize-for-CastColumnsPolicy" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#impl-Eq-for-CastColumnsPolicy" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#impl-StructuralPartialEq-for-CastColumnsPolicy" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html" class="struct" title="struct polars::prelude::CastColumnsPolicy">CastColumnsPolicy</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CastColumnsPolicy.html#blanket-implementations" class="anchor">§</a>
