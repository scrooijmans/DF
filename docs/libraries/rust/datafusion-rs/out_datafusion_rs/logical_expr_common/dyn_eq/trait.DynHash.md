# Trait DynHash Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/dyn_eq.rs.html#49" class="src">Source</a>

``` rust
pub trait DynHash: HashSealed {
    // Required method
    fn dyn_hash(&self, _state: &mut dyn Hasher);
}
```

Expand description

A dyn-compatible version of [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") trait. If two values are equal according to [`DynEq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html "trait datafusion::logical_expr_common::dyn_eq::DynEq"), they must produce the same hash value.

Note: This trait should not be implemented directly. Implement `Hash` and `Any` and use the blanket implementation.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynHash.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynHash.html#tymethod.dyn_hash" class="fn">dyn_hash</a>(&self, \_state: &mut dyn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>)

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynHash.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynHash.html#impl-DynHash-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynHash.html" class="trait" title="trait datafusion::logical_expr_common::dyn_eq::DynHash">DynHash</a> for T

where T: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>,
