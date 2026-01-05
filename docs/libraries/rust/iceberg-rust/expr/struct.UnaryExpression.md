# Struct UnaryExpression Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/expr/predicate.rs.html#105-111" class="src">Source</a>

``` rust
pub struct UnaryExpression<T> { /* private fields */ }
```

Expand description

Unary predicate, for example, `a IS NULL`.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#impl-UnaryExpression%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.new" class="fn">new</a>(op: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>, term: T) -\> Self

Creates a unary expression with the given operator and term.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#example" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::{PredicateOperator, Reference, UnaryExpression};

UnaryExpression::new(PredicateOperator::IsNull, Reference::new("c"));
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.op" class="fn">op</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

Return the operator of this predicate.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.term" class="fn">term</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>

Return the term of this predicate.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#impl-Bind-for-UnaryExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#associatedtype.Bound" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<\<T as <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\>::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>\>

The type of the bound result.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.bind" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#tymethod.bind" class="fn">bind</a>(&self, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, case_sensitive: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>\>

Bind an expression to a schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#impl-Clone-for-UnaryExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#impl-Debug-for-UnaryExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#impl-Deserialize%3C&#39;de%3E-for-UnaryExpression%3CT%3E" class="anchor">§</a>

### impl\<'de, T\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>

where T: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\>,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#impl-Display-for-UnaryExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#impl-PartialEq-for-UnaryExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#impl-Serialize-for-UnaryExpression%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>

where T: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a>,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#impl-StructuralPartialEq-for-UnaryExpression%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html#blanket-implementations" class="anchor">§</a>
