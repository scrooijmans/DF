# Enum Operator Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/expr/mod.rs.html#588" class="src">Source</a>

``` rust
pub enum Operator {
Show 20 variants    Eq,
    EqValidity,
    NotEq,
    NotEqValidity,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Plus,
    Minus,
    Multiply,
    Divide,
    TrueDivide,
    FloorDivide,
    Modulus,
    And,
    Or,
    Xor,
    LogicalAnd,
    LogicalOr,
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.Eq" class="anchor">§</a>

### Eq

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.EqValidity" class="anchor">§</a>

### EqValidity

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.NotEq" class="anchor">§</a>

### NotEq

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.NotEqValidity" class="anchor">§</a>

### NotEqValidity

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.Lt" class="anchor">§</a>

### Lt

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.LtEq" class="anchor">§</a>

### LtEq

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.Gt" class="anchor">§</a>

### Gt

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.GtEq" class="anchor">§</a>

### GtEq

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.Plus" class="anchor">§</a>

### Plus

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.Minus" class="anchor">§</a>

### Minus

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.Multiply" class="anchor">§</a>

### Multiply

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.Divide" class="anchor">§</a>

### Divide

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.TrueDivide" class="anchor">§</a>

### TrueDivide

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.FloorDivide" class="anchor">§</a>

### FloorDivide

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.Modulus" class="anchor">§</a>

### Modulus

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.And" class="anchor">§</a>

### And

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.Or" class="anchor">§</a>

### Or

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.Xor" class="anchor">§</a>

### Xor

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.LogicalAnd" class="anchor">§</a>

### LogicalAnd

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#variant.LogicalOr" class="anchor">§</a>

### LogicalOr

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-Operator" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.is_comparison" class="fn">is_comparison</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.is_bitwise" class="fn">is_bitwise</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.is_comparison_or_bitwise" class="fn">is_comparison_or_bitwise</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.swap_operands" class="fn">swap_operands</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.is_arithmetic" class="fn">is_arithmetic</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-Clone-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-Debug-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-Deserialize%3C&#39;de%3E-for-Operator" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-Display-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-From%3CInequalityOperator%3E-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.InequalityOperator.html" class="enum" title="enum polars::prelude::InequalityOperator">InequalityOperator</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.InequalityOperator.html" class="enum" title="enum polars::prelude::InequalityOperator">InequalityOperator</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-Hash-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-PartialEq-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-Serialize-for-Operator" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-Copy-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-Eq-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#impl-StructuralPartialEq-for-Operator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html" class="enum" title="enum polars::prelude::Operator">Operator</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Operator.html#blanket-implementations" class="anchor">§</a>
