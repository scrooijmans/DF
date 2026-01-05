# Struct NamePreserver Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_rewriter/mod.rs.html#284" class="src">Source</a>

``` rust
pub struct NamePreserver { /* private fields */ }
```

Expand description

Handles ensuring the name of rewritten expressions is not changed.

This is important when optimizing plans to ensure the output schema of plan nodes don’t change after optimization. For example, if an expression `1 + 2` is rewritten to `3`, the name of the expression should be preserved: `3 as "1 + 2"`

See <https://github.com/apache/datafusion/issues/3555> for details

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/struct.NamePreserver.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/struct.NamePreserver.html#impl-NamePreserver" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/struct.NamePreserver.html" class="struct" title="struct datafusion::logical_expr::expr_rewriter::NamePreserver">NamePreserver</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/struct.NamePreserver.html#method.new" class="fn">new</a>(plan: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/struct.NamePreserver.html" class="struct" title="struct datafusion::logical_expr::expr_rewriter::NamePreserver">NamePreserver</a>

Create a new NamePreserver for rewriting the `expr` that is part of the specified plan

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/struct.NamePreserver.html#method.new_for_projection" class="fn">new_for_projection</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/struct.NamePreserver.html" class="struct" title="struct datafusion::logical_expr::expr_rewriter::NamePreserver">NamePreserver</a>

Create a new NamePreserver for rewriting the `expr`s in `Projection`

This will use aliases

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/struct.NamePreserver.html#method.save" class="fn">save</a>(&self, expr: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/enum.SavedName.html" class="enum" title="enum datafusion::logical_expr::expr_rewriter::SavedName">SavedName</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/struct.NamePreserver.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/struct.NamePreserver.html#blanket-implementations" class="anchor">§</a>
