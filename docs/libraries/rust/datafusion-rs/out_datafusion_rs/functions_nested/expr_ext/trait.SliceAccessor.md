# Trait SliceAccessor Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/expr_ext.rs.html#73" class="src">Source</a>

``` rust
pub trait SliceAccessor {
    // Required method
    fn range(self, start: Expr, stop: Expr) -> Expr;
}
```

Available on **crate feature `nested_expressions`** only.

Expand description

Return elements between `1` based `start` and `stop`, for example `expr[1:3]`

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.SliceAccessor.html#example-access-element-2-3-4-from-column-c1" class="doc-anchor">§</a>Example: Access element 2, 3, 4 from column “c1”

For example if column “c1” holds documents like this

``` json
[10, 20, 30, 40]
```

You can access the value `[20, 30, 40]` with

``` rust
let expr = col("c1")
   .range(lit(2), lit(4));
assert_eq!(expr.schema_name().to_string(), "c1[Int32(2):Int32(4)]");
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.SliceAccessor.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.SliceAccessor.html#tymethod.range" class="fn">range</a>(self, start: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, stop: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.SliceAccessor.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.SliceAccessor.html#impl-SliceAccessor-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.SliceAccessor.html" class="trait" title="trait datafusion::functions_nested::expr_ext::SliceAccessor">SliceAccessor</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>
