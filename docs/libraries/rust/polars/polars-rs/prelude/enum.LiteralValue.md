# Enum LiteralValue Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/plans/lit.rs.html#71" class="src">Source</a>

``` rust
pub enum LiteralValue {
    Dyn(DynLiteralValue),
    Scalar(Scalar),
    Series(SpecialEq<Series>),
    Range(RangeLiteralValue),
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#variant.Dyn" class="anchor">§</a>

### Dyn(<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/lit/enum.DynLiteralValue.html" class="enum" title="enum polars_plan::plans::lit::DynLiteralValue">DynLiteralValue</a>)

A dynamically inferred literal value. This needs to be materialized into a specific type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#variant.Scalar" class="anchor">§</a>

### Scalar(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#variant.Series" class="anchor">§</a>

### Series(<a href="https://docs.rs/polars/latest/polars/prelude/struct.SpecialEq.html" class="struct" title="struct polars::prelude::SpecialEq">SpecialEq</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#variant.Range" class="anchor">§</a>

### Range(<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/lit/struct.RangeLiteralValue.html" class="struct" title="struct polars_plan::plans::lit::RangeLiteralValue">RangeLiteralValue</a>)

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#impl-LiteralValue" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.extract_usize" class="fn">extract_usize</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.extract_i64" class="fn">extract_i64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.materialize" class="fn">materialize</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.is_scalar" class="fn">is_scalar</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.to_any_value" class="fn">to_any_value</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.get_datatype" class="fn">get_datatype</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Getter for the `DataType` of the value

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.new_idxsize" class="fn">new_idxsize</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.extract_str" class="fn">extract_str</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.extract_binary" class="fn">extract_binary</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.is_null" class="fn">is_null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.bool" class="fn">bool</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.untyped_null" class="fn">untyped_null</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.implode" class="fn">implode</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#impl-Clone-for-LiteralValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#impl-Debug-for-LiteralValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#impl-Deserialize%3C&#39;de%3E-for-LiteralValue" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#impl-From%3CAnyValue%3C&#39;_%3E%3E-for-LiteralValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#impl-From%3CScalar%3E-for-LiteralValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#impl-Hash-for-LiteralValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#impl-Literal-for-LiteralValue" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.lit" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

[Literal](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Literal "variant polars::prelude::Expr::Literal") expression.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#impl-PartialEq-for-LiteralValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#impl-Serialize-for-LiteralValue" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#impl-StructuralPartialEq-for-LiteralValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html#blanket-implementations" class="anchor">§</a>
