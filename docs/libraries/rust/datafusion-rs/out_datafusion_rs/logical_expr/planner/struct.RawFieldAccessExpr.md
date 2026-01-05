# Struct RawFieldAccessExpr Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/planner.rs.html#274" class="src">Source</a>

``` rust
pub struct RawFieldAccessExpr {
    pub field_access: GetFieldAccess,
    pub expr: Expr,
}
```

Expand description

An expression with GetFieldAccess to plan

This structure is used by [`ExprPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/trait.ExprPlanner.html "trait datafusion::logical_expr::planner::ExprPlanner") to plan operators with custom expressions.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html#structfield.field_access" class="anchor field">§</a>`field_access: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.GetFieldAccess.html" class="enum" title="enum datafusion::logical_expr::GetFieldAccess"><code>GetFieldAccess</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html#structfield.expr" class="anchor field">§</a>`expr: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr"><code>Expr</code></a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html#impl-Clone-for-RawFieldAccessExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawFieldAccessExpr">RawFieldAccessExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawFieldAccessExpr">RawFieldAccessExpr</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html#impl-Debug-for-RawFieldAccessExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html" class="struct" title="struct datafusion::logical_expr::planner::RawFieldAccessExpr">RawFieldAccessExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/planner/struct.RawFieldAccessExpr.html#blanket-implementations" class="anchor">§</a>
