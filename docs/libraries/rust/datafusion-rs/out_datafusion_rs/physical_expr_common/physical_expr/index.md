# Module physical_expr Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/lib.rs.html#35" class="src">Source</a>

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/trait.DynEq.html" class="trait" title="trait datafusion::physical_expr_common::physical_expr::DynEq">DynEq</a>  
A dyn-compatible version of [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") trait. The implementation constraints for this trait are the same as for [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"): the implementation must be reflexive, symmetric, and transitive. Additionally, if two values can be compared with [`DynEq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html "trait datafusion::logical_expr_common::dyn_eq::DynEq") and [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") then they must be [`DynEq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html "trait datafusion::logical_expr_common::dyn_eq::DynEq")-equal if and only if they are [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")-equal. It is therefore strongly discouraged to implement this trait for types that implement `PartialEq<Other>` or `Eq<Other>` for any type `Other` other than `Self`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/trait.DynHash.html" class="trait" title="trait datafusion::physical_expr_common::physical_expr::DynHash">DynHash</a>  
A dyn-compatible version of [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") trait. If two values are equal according to [`DynEq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/dyn_eq/trait.DynEq.html "trait datafusion::logical_expr_common::dyn_eq::DynEq"), they must produce the same hash value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr_common::physical_expr::PhysicalExpr">PhysicalExpr</a>  
[`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr")s represent expressions such as `A + 1` or `CAST(c1 AS int)`.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.fmt_sql.html" class="fn" title="fn datafusion::physical_expr_common::physical_expr::fmt_sql">fmt_sql</a>  
Prints a [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") in a SQL-like format

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.format_physical_expr_list.html" class="fn" title="fn datafusion::physical_expr_common::physical_expr::format_physical_expr_list">format_physical_expr_list</a>  
Returns [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") able a list of [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.is_dynamic_physical_expr.html" class="fn" title="fn datafusion::physical_expr_common::physical_expr::is_dynamic_physical_expr">is_dynamic_physical_expr</a>  
Check if the given `PhysicalExpr` is dynamic. Internally this calls [`snapshot_generation`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.snapshot_generation.html "fn datafusion::physical_expr_common::physical_expr::snapshot_generation") to check if the generation is non-zero, any dynamic `PhysicalExpr` should have a non-zero generation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.is_volatile.html" class="fn" title="fn datafusion::physical_expr_common::physical_expr::is_volatile">is_volatile</a>  
Returns true if the expression is volatile, i.e. whether it can return different results when evaluated multiple times with the same input.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.snapshot_generation.html" class="fn" title="fn datafusion::physical_expr_common::physical_expr::snapshot_generation">snapshot_generation</a>  
Check the generation of this `PhysicalExpr`. Dynamic `PhysicalExpr`s may have a generation that is incremented every time the state of the `PhysicalExpr` changes. If the generation changes that means this `PhysicalExpr` or one of its children has changed since the last time it was evaluated.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.snapshot_physical_expr.html" class="fn" title="fn datafusion::physical_expr_common::physical_expr::snapshot_physical_expr">snapshot_physical_expr</a>  
Take a snapshot of the given `PhysicalExpr` if it is dynamic.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.with_new_children_if_necessary.html" class="fn" title="fn datafusion::physical_expr_common::physical_expr::with_new_children_if_necessary">with_new_children_if_necessary</a>  
Returns a copy of this expr if we change any child according to the pointer comparison. The size of `children` must be equal to the size of `PhysicalExpr::children()`.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/type.PhysicalExprRef.html" class="type" title="type datafusion::physical_expr_common::physical_expr::PhysicalExprRef">PhysicalExprRef</a>  
Shared [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr").
