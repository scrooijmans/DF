# Trait Bind Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/expr/mod.rs.html#168-173" class="src">Source</a>

``` rust
pub trait Bind {
    type Bound;

    // Required method
    fn bind(
        &self,
        schema: SchemaRef,
        case_sensitive: bool,
    ) -> Result<Self::Bound>;
}
```

Expand description

Bind expression to a schema.

## Required Associated Types<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a>

The type of the bound result.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#tymethod.bind" class="fn">bind</a>(&self, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, case_sensitive: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>\>

Bind an expression to a schema.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#impl-Bind-for-Predicate" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound-1" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#impl-Bind-for-Reference" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound-2" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.BoundReference.html" class="struct" title="struct iceberg::expr::BoundReference">BoundReference</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#impl-Bind-for-BinaryExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.BinaryExpression.html" class="struct" title="struct iceberg::expr::BinaryExpression">BinaryExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound-3" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.BinaryExpression.html" class="struct" title="struct iceberg::expr::BinaryExpression">BinaryExpression</a>\<\<T as <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\>::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#impl-Bind-for-SetExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound-4" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>\<\<T as <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\>::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#impl-Bind-for-UnaryExpression%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<T\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound-5" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>\<\<T as <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\>::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#impl-Bind-for-LogicalExpression%3CT,+N%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<T, N\>

where T::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound-6" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype">Bound</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>\<\<T as <a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>\>::<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html#associatedtype.Bound" class="associatedtype" title="type iceberg::expr::Bind::Bound">Bound</a>, N\>
