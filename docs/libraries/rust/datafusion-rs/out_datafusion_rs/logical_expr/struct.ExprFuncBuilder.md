# Struct ExprFuncBuilder Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#780" class="src">Source</a>

``` rust
pub struct ExprFuncBuilder { /* private fields */ }
```

Expand description

Implementation of [`ExprFunctionExt`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html "trait datafusion::prelude::ExprFunctionExt").

See [`ExprFunctionExt`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html "trait datafusion::prelude::ExprFunctionExt") for usage and examples

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#impl-ExprFuncBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates and returns the in progress [`Expr::AggregateFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.AggregateFunction "variant datafusion::prelude::Expr::AggregateFunction") or [`Expr::WindowFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.WindowFunction "variant datafusion::prelude::Expr::WindowFunction")

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#errors" class="doc-anchor">§</a>Errors:

Returns an error if this builder [`ExprFunctionExt`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html "trait datafusion::prelude::ExprFunctionExt") was used with an `Expr` variant other than [`Expr::AggregateFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.AggregateFunction "variant datafusion::prelude::Expr::AggregateFunction") or [`Expr::WindowFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.WindowFunction "variant datafusion::prelude::Expr::WindowFunction")

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#impl-Clone-for-ExprFuncBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#impl-Debug-for-ExprFuncBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#impl-ExprFunctionExt-for-ExprFuncBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html" class="trait" title="trait datafusion::prelude::ExprFunctionExt">ExprFunctionExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#method.order_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.order_by" class="fn">order_by</a>(self, order_by: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `ORDER BY <order_by>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#method.filter" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.filter" class="fn">filter</a>(self, filter: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `FILTER <filter>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#method.distinct" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.distinct" class="fn">distinct</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `DISTINCT`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#method.null_treatment" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.null_treatment" class="fn">null_treatment</a>( self, null_treatment: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.NullTreatment.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::NullTreatment">NullTreatment</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `RESPECT NULLS` or `IGNORE NULLS`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#method.partition_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.partition_by" class="fn">partition_by</a>(self, partition_by: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `PARTITION BY`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#method.window_frame" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.window_frame" class="fn">window_frame</a>(self, window_frame: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add appropriate window frame conditions

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ExprFuncBuilder.html#blanket-implementations" class="anchor">§</a>
