# Enum PredicateOperator Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/expr/mod.rs.html#40-60" class="src">Source</a>

``` rust
#[non_exhaustive]#[repr(u16)]pub enum PredicateOperator {
Show 14 variants    IsNull = 101,
    NotNull = 102,
    IsNan = 103,
    NotNan = 104,
    LessThan = 201,
    LessThanOrEq = 202,
    GreaterThan = 203,
    GreaterThanOrEq = 204,
    Eq = 205,
    NotEq = 206,
    StartsWith = 207,
    NotStartsWith = 208,
    In = 301,
    NotIn = 302,
}
```

Expand description

Predicate operators used in expressions.

The discriminant of this enum is used for determining the type of the operator, see [`PredicateOperator::is_unary`](https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.is_unary "method iceberg::expr::PredicateOperator::is_unary"), [`PredicateOperator::is_binary`](https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.is_binary "method iceberg::expr::PredicateOperator::is_binary"), [`PredicateOperator::is_set`](https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.is_set "method iceberg::expr::PredicateOperator::is_set")

## Variants (Non-exhaustive)<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.IsNull" class="anchor">§</a>

### IsNull = 101

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.NotNull" class="anchor">§</a>

### NotNull = 102

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.IsNan" class="anchor">§</a>

### IsNan = 103

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.NotNan" class="anchor">§</a>

### NotNan = 104

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.LessThan" class="anchor">§</a>

### LessThan = 201

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.LessThanOrEq" class="anchor">§</a>

### LessThanOrEq = 202

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.GreaterThan" class="anchor">§</a>

### GreaterThan = 203

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.GreaterThanOrEq" class="anchor">§</a>

### GreaterThanOrEq = 204

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.Eq" class="anchor">§</a>

### Eq = 205

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.NotEq" class="anchor">§</a>

### NotEq = 206

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.StartsWith" class="anchor">§</a>

### StartsWith = 207

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.NotStartsWith" class="anchor">§</a>

### NotStartsWith = 208

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.In" class="anchor">§</a>

### In = 301

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#variant.NotIn" class="anchor">§</a>

### NotIn = 302

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#impl-PredicateOperator" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.is_unary" class="fn">is_unary</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this operator is unary operator.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#example" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::PredicateOperator;
assert!(PredicateOperator::IsNull.is_unary());
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.is_binary" class="fn">is_binary</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this operator is binary operator.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#example-1" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::PredicateOperator;
assert!(PredicateOperator::LessThan.is_binary());
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.is_set" class="fn">is_set</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this operator is set operator.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#example-2" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::PredicateOperator;
assert!(PredicateOperator::In.is_set());
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.negate" class="fn">negate</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

Returns the predicate that is the inverse of self

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#example-3" class="doc-anchor">§</a>Example

``` rust
use iceberg::expr::PredicateOperator;
assert_eq!(
    PredicateOperator::IsNull.negate(),
    PredicateOperator::NotNull
);
assert_eq!(PredicateOperator::IsNan.negate(), PredicateOperator::NotNan);
assert_eq!(
    PredicateOperator::LessThan.negate(),
    PredicateOperator::GreaterThanOrEq
);
assert_eq!(
    PredicateOperator::GreaterThan.negate(),
    PredicateOperator::LessThanOrEq
);
assert_eq!(PredicateOperator::Eq.negate(), PredicateOperator::NotEq);
assert_eq!(PredicateOperator::In.negate(), PredicateOperator::NotIn);
assert_eq!(
    PredicateOperator::StartsWith.negate(),
    PredicateOperator::NotStartsWith
);
```

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#impl-Clone-for-PredicateOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#impl-Debug-for-PredicateOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#impl-Deserialize%3C&#39;de%3E-for-PredicateOperator" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#impl-Display-for-PredicateOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#impl-PartialEq-for-PredicateOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#impl-Serialize-for-PredicateOperator" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#impl-Copy-for-PredicateOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#impl-StructuralPartialEq-for-PredicateOperator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html#blanket-implementations" class="anchor">§</a>
