# Trait FunctionRewrite Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_rewriter/mod.rs.html#46" class="src">Source</a>

``` rust
pub trait FunctionRewrite: Debug {
    // Required methods
    fn name(&self) -> &str;
    fn rewrite(
        &self,
        expr: Expr,
        schema: &DFSchema,
        config: &ConfigOptions,
    ) -> Result<Transformed<Expr>, DataFusionError>;
}
```

Expand description

Trait for rewriting [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s into function calls.

This trait is used with `FunctionRegistry::register_function_rewrite` to to evaluating `Expr`s using functions that may not be built in to DataFusion

For example, concatenating arrays `a || b` is represented as `Operator::ArrowAt`, but can be implemented by calling a function `array_concat` from the `functions-nested` crate.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return a human readable name for this rewrite

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html#tymethod.rewrite" class="fn">rewrite</a>( &self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Potentially rewrite `expr` to some other expression

Note that recursion is handled by the caller – this method should only handle `expr`, not recurse to its children.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html#implementors" class="anchor">§</a>
