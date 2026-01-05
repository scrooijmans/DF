# Enum Predicate Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/expr/predicate.rs.html#321-338" class="src">Source</a>

``` rust
pub enum Predicate {
    AlwaysTrue,
    AlwaysFalse,
    And(LogicalExpression<Predicate, 2>),
    Or(LogicalExpression<Predicate, 2>),
    Not(LogicalExpression<Predicate, 1>),
    Unary(UnaryExpression<Reference>),
    Binary(BinaryExpression<Reference>),
    Set(SetExpression<Reference>),
}
```

Expand description

Unbound predicate expression before binding to a schema.

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#variant.AlwaysTrue" class="anchor">§</a>

### AlwaysTrue

AlwaysTrue predicate, for example, `TRUE`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#variant.AlwaysFalse" class="anchor">§</a>

### AlwaysFalse

AlwaysFalse predicate, for example, `FALSE`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#variant.And" class="anchor">§</a>

### And(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>, 2\>)

And predicate, for example, `a > 10 AND b < 20`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#variant.Or" class="anchor">§</a>

### Or(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>, 2\>)

Or predicate, for example, `a > 10 OR b < 20`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#variant.Not" class="anchor">§</a>

### Not(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>, 1\>)

Not predicate, for example, `NOT (a > 10)`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#variant.Unary" class="anchor">§</a>

### Unary(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>\>)

Unary expression, for example, `a IS NULL`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#variant.Binary" class="anchor">§</a>

### Binary(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.BinaryExpression.html" class="struct" title="struct iceberg::expr::BinaryExpression">BinaryExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>\>)

Binary expression, for example, `a > 10`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#variant.Set" class="anchor">§</a>

### Set(<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>\>)

Set predicates, for example, `a in (1, 2, 3)`.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#impl-Predicate" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.and" class="fn">and</a>(self, other: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Combines two predicates with `AND`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#example" class="doc-anchor">§</a>Example

``` rust
use std::ops::Bound::Unbounded;

use iceberg::expr::BoundPredicate::Unary;
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr1 = Reference::new("a").less_than(Datum::long(10));

let expr2 = Reference::new("b").less_than(Datum::long(20));

let expr = expr1.and(expr2);

assert_eq!(&format!("{expr}"), "(a < 10) AND (b < 20)");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.or" class="fn">or</a>(self, other: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Combines two predicates with `OR`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#example-1" class="doc-anchor">§</a>Example

``` rust
use std::ops::Bound::Unbounded;

use iceberg::expr::BoundPredicate::Unary;
use iceberg::expr::Reference;
use iceberg::spec::Datum;
let expr1 = Reference::new("a").less_than(Datum::long(10));

let expr2 = Reference::new("b").less_than(Datum::long(20));

let expr = expr1.or(expr2);

assert_eq!(&format!("{expr}"), "(a < 10) OR (b < 20)");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.negate" class="fn">negate</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Returns a predicate representing the negation (‘NOT’) of this one, by using inverse predicates rather than wrapping in a `NOT`. Used for `NOT` elimination.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#example-2" class="doc-anchor">§</a>Example

``` rust
use std::ops::Bound::Unbounded;

use iceberg::expr::BoundPredicate::Unary;
use iceberg::expr::{LogicalExpression, Predicate, Reference};
use iceberg::spec::Datum;
let expr1 = Reference::new("a").less_than(Datum::long(10));
let expr2 = Reference::new("b")
    .less_than(Datum::long(5))
    .and(Reference::new("c").less_than(Datum::long(10)));

let result = expr1.negate();
assert_eq!(&format!("{result}"), "a >= 10");

let result = expr2.negate();
assert_eq!(&format!("{result}"), "(b >= 5) OR (c >= 10)");
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.rewrite_not" class="fn">rewrite_not</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Simplifies the expression by removing `NOT` predicates, directly negating the inner expressions instead. The transformation applies logical laws (such as De Morgan’s laws) to recursively negate and simplify inner expressions within `NOT` predicates.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#example-3" class="doc-anchor">§</a>Example

``` rust
use std::ops::Not;

use iceberg::expr::{LogicalExpression, Predicate, Reference};
use iceberg::spec::Datum;

let expression = Reference::new("a").less_than(Datum::long(5)).not();
let result = expression.rewrite_not();

assert_eq!(&format!("{result}"), "a >= 5");
```

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#impl-Bind-for-Predicate" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#associatedtype.Bound" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

The type of the bound result.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.bind" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#tymethod.bind" class="fn">bind</a>( &self, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, case_sensitive: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>\>

Bind an expression to a schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#impl-Clone-for-Predicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#impl-Debug-for-Predicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#impl-Deserialize%3C&#39;de%3E-for-Predicate" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#impl-Display-for-Predicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#impl-Not-for-Predicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html" class="trait" title="trait core::ops::bit::Not">Not</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.not" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not" class="fn">not</a>(self) -\> Self::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::Not::Output">Output</a>

Create a predicate which is the reverse of this predicate. For example: `NOT (a > 10)`.

This is different from [`Predicate::negate()`](https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.negate "method iceberg::expr::Predicate::negate") since it doesn’t rewrite expression, but just adds a `NOT` operator.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#example-4" class="doc-anchor">§</a>Example

``` rust
 use std::ops::Bound::Unbounded;

 use iceberg::expr::BoundPredicate::Unary;
 use iceberg::expr::Reference;
 use iceberg::spec::Datum;
 let expr1 = Reference::new("a").less_than(Datum::long(10));

 let expr = !expr1;

 assert_eq!(&format!("{expr}"), "NOT (a < 10)");
```

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

The resulting type after applying the `!` operator.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#impl-PartialEq-for-Predicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#impl-Serialize-for-Predicate" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#impl-StructuralPartialEq-for-Predicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html#blanket-implementations" class="anchor">§</a>
