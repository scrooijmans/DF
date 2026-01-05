# Trait ExprFunctionExt Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#752" class="src">Source</a>

``` rust
pub trait ExprFunctionExt {
    // Required methods
    fn order_by(self, order_by: Vec<Sort>) -> ExprFuncBuilder;
    fn filter(self, filter: Expr) -> ExprFuncBuilder;
    fn distinct(self) -> ExprFuncBuilder;
    fn null_treatment(
        self,
        null_treatment: impl Into<Option<NullTreatment>>,
    ) -> ExprFuncBuilder;
    fn partition_by(self, partition_by: Vec<Expr>) -> ExprFuncBuilder;
    fn window_frame(self, window_frame: WindowFrame) -> ExprFuncBuilder;
}
```

Expand description

Extensions for configuring [`Expr::AggregateFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.AggregateFunction "variant datafusion::prelude::Expr::AggregateFunction") or [`Expr::WindowFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.WindowFunction "variant datafusion::prelude::Expr::WindowFunction")

Adds methods to [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") that make it easy to set optional options such as `ORDER BY`, `FILTER` and `DISTINCT`

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#example" class="doc-anchor">§</a>Example

``` rust
unimplemented!() }
// Create an aggregate count, filtering on column y > 5
let agg = count(col("x")).filter(col("y").gt(lit(5))).build()?;

// Find the first value in an aggregate sorted by column y
// equivalent to:
// `FIRST_VALUE(x ORDER BY y ASC IGNORE NULLS)`
let sort_expr = col("y").sort(true, true);
let agg = first_value(col("x"))
    .order_by(vec![sort_expr])
    .null_treatment(NullTreatment::IgnoreNulls)
    .build()?;

// Create a window expression for percent rank partitioned on column a
// equivalent to:
// `PERCENT_RANK() OVER (PARTITION BY a ORDER BY b ASC NULLS LAST IGNORE NULLS)`
// percent_rank is an udwf function in another crate
unimplemented!() }
let window = percent_rank()
    .partition_by(vec![col("a")])
    .order_by(vec![col("b").sort(true, true)])
    .null_treatment(NullTreatment::IgnoreNulls)
    .build()?;
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#tymethod.order_by" class="fn">order_by</a>(self, order_by: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `ORDER BY <order_by>`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#tymethod.filter" class="fn">filter</a>(self, filter: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `FILTER <filter>`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#tymethod.distinct" class="fn">distinct</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `DISTINCT`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#tymethod.null_treatment" class="fn">null_treatment</a>( self, null_treatment: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.NullTreatment.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::NullTreatment">NullTreatment</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `RESPECT NULLS` or `IGNORE NULLS`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#tymethod.partition_by" class="fn">partition_by</a>(self, partition_by: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `PARTITION BY`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#tymethod.window_frame" class="fn">window_frame</a>(self, window_frame: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add appropriate window frame conditions

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#impl-ExprFunctionExt-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html" class="trait" title="trait datafusion::prelude::ExprFunctionExt">ExprFunctionExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprFunctionExt.html#impl-ExprFunctionExt-for-ExprFuncBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html" class="trait" title="trait datafusion::prelude::ExprFunctionExt">ExprFunctionExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>
