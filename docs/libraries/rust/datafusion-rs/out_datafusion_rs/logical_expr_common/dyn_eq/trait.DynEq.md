# Trait DynEq Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/dyn_eq.rs.html#32" class="src">Source</a>

``` rust
pub trait DynEq: EqSealed {
    // Required method
    fn dyn_eq(&self, other: &(dyn Any + 'static)) -> bool;
}
```

Expand description

A dyn-compatible version of [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") trait. The implementation constraints for this trait are the same as for [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"): the implementation must be reflexive, symmetric, and transitive. Additionally, if two values can be compared with [`DynEq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html "trait datafusion::logical_expr_common::dyn_eq::DynEq") and [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") then they must be [`DynEq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html "trait datafusion::logical_expr_common::dyn_eq::DynEq")-equal if and only if they are [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")-equal. It is therefore strongly discouraged to implement this trait for types that implement `PartialEq<Other>` or `Eq<Other>` for any type `Other` other than `Self`.

Note: This trait should not be implemented directly. Implement `Eq` and `Any` and use the blanket implementation.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html#tymethod.dyn_eq" class="fn">dyn_eq</a>(&self, other: &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html#impl-DynEq-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html" class="trait" title="trait datafusion::logical_expr_common::dyn_eq::DynEq">DynEq</a> for T

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>,
