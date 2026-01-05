# Struct LogicalExpression Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/expr/predicate.rs.html#40-42" class="src">Source</a>

``` rust
pub struct LogicalExpression<T, const N: usize> { /* private fields */ }
```

Expand description

Logical expression, such as `AND`, `OR`, `NOT`.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#impl-LogicalExpression%3CT,+N%3E" class="anchor">§</a>

### impl\<T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<T, N\>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#method.inputs" class="fn">inputs</a>(&self) -\> \[<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]

Return inputs of this logical expression.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#impl-Bind-for-LogicalExpression%3CT,+N%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<T, N\>

where T::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#associatedtype.Bound" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<\<T as <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\>::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>, N\>

The type of the bound result.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#method.bind" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#tymethod.bind" class="fn">bind</a>(&self, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, case_sensitive: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>\>

Bind an expression to a schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#impl-Clone-for-LogicalExpression%3CT,+N%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<T, N\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<T, N\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#impl-Debug-for-LogicalExpression%3CT,+N%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<T, N\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#impl-Deserialize%3C&#39;de%3E-for-LogicalExpression%3CT,+N%3E" class="anchor">§</a>

### impl\<'de, T: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\>, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<T, N\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>(deserializer: D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#impl-PartialEq-for-LogicalExpression%3CT,+N%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<T, N\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<T, N\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#impl-Serialize-for-LogicalExpression%3CT,+N%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a>, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<T, N\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>(&self, serializer: S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#impl-StructuralPartialEq-for-LogicalExpression%3CT,+N%3E" class="anchor">§</a>

### impl\<T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<T, N\>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html#blanket-implementations" class="anchor">§</a>
