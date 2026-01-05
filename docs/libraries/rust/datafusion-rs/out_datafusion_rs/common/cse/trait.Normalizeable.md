# Trait Normalizeable Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/cse.rs.html#58" class="src">Source</a>

``` rust
pub trait Normalizeable {
    // Required method
    fn can_normalize(&self) -> bool;
}
```

Expand description

The `Normalizeable` trait defines a method to determine whether a node can be normalized.

Normalization is the process of converting a node into a canonical form that can be used to compare nodes for equality. This is useful in optimizations like Common Subexpression Elimination (CSE), where semantically equivalent nodes (e.g., `a + b` and `b + a`) should be treated as equal.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.Normalizeable.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.Normalizeable.html#tymethod.can_normalize" class="fn">can_normalize</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.Normalizeable.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.Normalizeable.html#impl-Normalizeable-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.Normalizeable.html" class="trait" title="trait datafusion::common::cse::Normalizeable">Normalizeable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.Normalizeable.html#impl-Normalizeable-for-Subquery" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.Normalizeable.html" class="trait" title="trait datafusion::common::cse::Normalizeable">Normalizeable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Subquery.html" class="struct" title="struct datafusion::logical_expr::Subquery">Subquery</a>
