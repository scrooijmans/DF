# Trait IndexAccessor Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/expr_ext.rs.html#43" class="src">Source</a>

``` rust
pub trait IndexAccessor {
    // Required method
    fn index(self, key: Expr) -> Expr;
}
```

Available on **crate feature `nested_expressions`** only.

Expand description

Return access to the element field. Example `expr["name"]`

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.IndexAccessor.html#example-access-element-2-from-column-c1" class="doc-anchor">§</a>Example Access element 2 from column “c1”

For example if column “c1” holds documents like this

``` json
[10, 20, 30, 40]
```

You can access the value “30” with

``` rust
let expr = col("c1")
   .index(lit(3));
assert_eq!(expr.schema_name().to_string(), "c1[Int32(3)]");
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.IndexAccessor.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.IndexAccessor.html#tymethod.index" class="fn">index</a>(self, key: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.IndexAccessor.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.IndexAccessor.html#impl-IndexAccessor-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.IndexAccessor.html" class="trait" title="trait datafusion::functions_nested::expr_ext::IndexAccessor">IndexAccessor</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>
