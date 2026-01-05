# Enum FetchType Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/plan.rs.html#3286" class="src">Source</a>

``` rust
pub enum FetchType {
    Literal(Option<usize>),
    UnsupportedExpr,
}
```

Expand description

Different types of fetch expression in Limit plan.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.FetchType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.FetchType.html#variant.Literal" class="anchor">§</a>

### Literal(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

The fetch expression is a literal value. `Literal(None)` means the fetch expression is not provided.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.FetchType.html#variant.UnsupportedExpr" class="anchor">§</a>

### UnsupportedExpr

Currently only supports expressions that can be folded into constants.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.FetchType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.FetchType.html#blanket-implementations" class="anchor">§</a>
