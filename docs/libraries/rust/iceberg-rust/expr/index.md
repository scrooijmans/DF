# Module expr Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/expr/mod.rs.html#18-204" class="src">Source</a>

Expand description

This module contains expressions.

## Structs<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.BinaryExpression.html" class="struct" title="struct iceberg::expr::BinaryExpression">BinaryExpression</a>  
Binary predicate, for example, `a > 10`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.BoundReference.html" class="struct" title="struct iceberg::expr::BoundReference">BoundReference</a>  
A named reference in a bound expression after binding to a schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.LogicalExpression.html" class="struct" title="struct iceberg::expr::LogicalExpression">LogicalExpression</a>  
Logical expression, such as `AND`, `OR`, `NOT`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.Reference.html" class="struct" title="struct iceberg::expr::Reference">Reference</a>  
A named reference in an unbound expression. For example, `a` in `a > 10`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.SetExpression.html" class="struct" title="struct iceberg::expr::SetExpression">SetExpression</a>  
Set predicates, for example, `a in (1, 2, 3)`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/struct.UnaryExpression.html" class="struct" title="struct iceberg::expr::UnaryExpression">UnaryExpression</a>  
Unary predicate, for example, `a IS NULL`.

## Enums<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>  
Bound predicate expression after binding to a schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>  
Unbound predicate expression before binding to a schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.PredicateOperator.html" class="enum" title="enum iceberg::expr::PredicateOperator">PredicateOperator</a>  
Predicate operators used in expressions.

## Traits<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/trait.Bind.html" class="trait" title="trait iceberg::expr::Bind">Bind</a>  
Bind expression to a schema.

## Type Aliases<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/type.BoundTerm.html" class="type" title="type iceberg::expr::BoundTerm">BoundTerm</a>  
Bound term after binding to a schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/type.Term.html" class="type" title="type iceberg::expr::Term">Term</a>  
Unbound term before binding to a schema.
