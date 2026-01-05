# Trait NormalizeEq Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/cse.rs.html#70" class="src">Source</a>

``` rust
pub trait NormalizeEq: Eq + Normalizeable {
    // Required method
    fn normalize_eq(&self, other: &Self) -> bool;
}
```

Expand description

The `NormalizeEq` trait extends `Eq` and `Normalizeable` to provide a method for comparing normalized nodes in optimizations like Common Subexpression Elimination (CSE).

The `normalize_eq` method ensures that two nodes that are semantically equivalent (after normalization) are considered equal in CSE optimization, even if their original forms differ.

This trait allows for equality comparisons between nodes with equivalent semantics, regardless of their internal representations.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html#tymethod.normalize_eq" class="fn">normalize_eq</a>(&self, other: &Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html#impl-NormalizeEq-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html" class="trait" title="trait datafusion::common::cse::NormalizeEq">NormalizeEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html#impl-NormalizeEq-for-Subquery" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html" class="trait" title="trait datafusion::common::cse::NormalizeEq">NormalizeEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Subquery.html" class="struct" title="struct datafusion::logical_expr::Subquery">Subquery</a>
