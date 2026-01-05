# Enum BoundPredicate Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/expr/predicate.rs.html#707-724" class="src">Source</a>

``` rust
pub enum BoundPredicate {
    AlwaysTrue,
    AlwaysFalse,
    And(LogicalExpression<BoundPredicate, 2>),
    Or(LogicalExpression<BoundPredicate, 2>),
    Not(LogicalExpression<BoundPredicate, 1>),
    Unary(UnaryExpression<BoundReference>),
    Binary(BinaryExpression<BoundReference>),
    Set(SetExpression<BoundReference>),
}
```

Expand description

Bound predicate expression after binding to a schema.

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#variant.AlwaysTrue" class="anchor">§</a>

### AlwaysTrue

An expression always evaluates to true.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#variant.AlwaysFalse" class="anchor">§</a>

### AlwaysFalse

An expression always evaluates to false.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#variant.And" class="anchor">§</a>

### And(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>, 2\>)

An expression combined by `AND`, for example, `a > 10 AND b < 20`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#variant.Or" class="anchor">§</a>

### Or(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>, 2\>)

An expression combined by `OR`, for example, `a > 10 OR b < 20`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#variant.Not" class="anchor">§</a>

### Not(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>, 1\>)

An expression combined by `NOT`, for example, `NOT (a > 10)`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#variant.Unary" class="anchor">§</a>

### Unary(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.BoundReference.html" class="struct" title="struct iceberg::expr::BoundReference">BoundReference</a>\>)

Unary expression, for example, `a IS NULL`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#variant.Binary" class="anchor">§</a>

### Binary(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.BinaryExpression.html" class="struct" title="struct iceberg::expr::BinaryExpression">BinaryExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.BoundReference.html" class="struct" title="struct iceberg::expr::BoundReference">BoundReference</a>\>)

Binary expression, for example, `a > 10`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#variant.Set" class="anchor">§</a>

### Set(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.BoundReference.html" class="struct" title="struct iceberg::expr::BoundReference">BoundReference</a>\>)

Set predicates, for example, `a IN (1, 2, 3)`.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#impl-BoundPredicate" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#method.rewrite_not" class="fn">rewrite_not</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

Simplifies the expression by removing `NOT` predicates, directly negating the inner expressions instead. The transformation applies logical laws (such as De Morgan’s laws) to recursively negate and simplify inner expressions within `NOT` predicates.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#example" class="doc-anchor">§</a>Example

``` rust
use std::ops::Not;

use iceberg::expr::{Bind, BoundPredicate, Reference};
use iceberg::spec::Datum;

// This would need to be bound first, but the concept is:
// let expression = bound_predicate.not();
// let result = expression.rewrite_not();
```

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#impl-Clone-for-BoundPredicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#impl-Debug-for-BoundPredicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#impl-Deserialize%3C&#39;de%3E-for-BoundPredicate" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#impl-Display-for-BoundPredicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#impl-PartialEq-for-BoundPredicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#impl-Serialize-for-BoundPredicate" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#impl-StructuralPartialEq-for-BoundPredicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html#blanket-implementations" class="anchor">§</a>
