# Module dyn_eq Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/lib.rs.html#38" class="src">Source</a>

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html" class="trait" title="trait datafusion::logical_expr_common::dyn_eq::DynEq">DynEq</a>  
A dyn-compatible version of [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") trait. The implementation constraints for this trait are the same as for [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"): the implementation must be reflexive, symmetric, and transitive. Additionally, if two values can be compared with [`DynEq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html "trait datafusion::logical_expr_common::dyn_eq::DynEq") and [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") then they must be [`DynEq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html "trait datafusion::logical_expr_common::dyn_eq::DynEq")-equal if and only if they are [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")-equal. It is therefore strongly discouraged to implement this trait for types that implement `PartialEq<Other>` or `Eq<Other>` for any type `Other` other than `Self`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynHash.html" class="trait" title="trait datafusion::logical_expr_common::dyn_eq::DynHash">DynHash</a>  
A dyn-compatible version of [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") trait. If two values are equal according to [`DynEq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html "trait datafusion::logical_expr_common::dyn_eq::DynEq"), they must produce the same hash value.
