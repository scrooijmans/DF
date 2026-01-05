# Trait FieldAccessor Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/core/expr_ext.rs.html#46" class="src">Source</a>

``` rust
pub trait FieldAccessor {
    // Required method
    fn field(self, name: impl Literal) -> Expr;
}
```

Expand description

Return access to the named field. Example `expr["name"]`

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/expr_ext/trait.FieldAccessor.html#access-field-my_field-from-column-c1" class="doc-anchor">§</a>Access field “my_field” from column “c1”

For example if column “c1” holds documents like this

``` json
{
  "my_field": 123.34,
  "other_field": "Boston",
}
```

You can access column “my_field” with

``` rust
let expr = col("c1")
   .field("my_field");
assert_eq!(expr.schema_name().to_string(), "c1[my_field]");
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/expr_ext/trait.FieldAccessor.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/expr_ext/trait.FieldAccessor.html#tymethod.field" class="fn">field</a>(self, name: impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/expr_ext/trait.FieldAccessor.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/expr_ext/trait.FieldAccessor.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/expr_ext/trait.FieldAccessor.html#impl-FieldAccessor-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/expr_ext/trait.FieldAccessor.html" class="trait" title="trait datafusion::functions::core::expr_ext::FieldAccessor">FieldAccessor</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>
