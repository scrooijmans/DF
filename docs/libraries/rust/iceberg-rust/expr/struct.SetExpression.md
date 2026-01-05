# Struct SetExpression Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/expr/predicate.rs.html#242-249" class="src">Source</a>

``` rust
pub struct SetExpression<T> { /* private fields */ }
```

Expand description

Set predicates, for example, `a in (1, 2, 3)`.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#impl-SetExpression%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.new" class="fn">new</a>(op: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>, term: T, literals: <a href="https://docs.rs/fnv/1.0.7/x86_64-unknown-linux-gnu/fnv/type.FnvHashSet.html" class="type" title="type fnv::FnvHashSet">FnvHashSet</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\>) -\> Self

Creates a set expression with the given operator, term and literal.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#example" class="doc-anchor">§</a>Example

``` rust
use fnv::FnvHashSet;
use iceberg::expr::{PredicateOperator, Reference, SetExpression};
use iceberg::spec::Datum;

SetExpression::new(
    PredicateOperator::In,
    Reference::new("a"),
    FnvHashSet::from_iter(vec![Datum::int(1)]),
);
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.op" class="fn">op</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

Return the operator of this predicate.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.literals" class="fn">literals</a>(&self) -\> &<a href="https://docs.rs/fnv/1.0.7/x86_64-unknown-linux-gnu/fnv/type.FnvHashSet.html" class="type" title="type fnv::FnvHashSet">FnvHashSet</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\>

Return the hash set of values compared against the term in this expression.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.term" class="fn">term</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>

Return the term of this predicate.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#impl-Bind-for-SetExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#associatedtype.Bound" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<\<T as <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\>::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>\>

The type of the bound result.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.bind" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#tymethod.bind" class="fn">bind</a>(&self, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, case_sensitive: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>\>

Bind an expression to a schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#impl-Clone-for-SetExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#impl-Debug-for-SetExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#impl-Deserialize%3C&#39;de%3E-for-SetExpression%3CT%3E" class="anchor">§</a>

### impl\<'de, T\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>

where T: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\>,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#impl-Display-for-SetExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> + <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#impl-PartialEq-for-SetExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#impl-Serialize-for-SetExpression%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>

where T: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a>,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#impl-StructuralPartialEq-for-SetExpression%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html#blanket-implementations" class="anchor">§</a>
