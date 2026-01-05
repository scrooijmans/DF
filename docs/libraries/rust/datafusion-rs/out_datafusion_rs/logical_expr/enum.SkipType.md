# Enum SkipType Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/plan.rs.html#3278" class="src">Source</a>

``` rust
pub enum SkipType {
    Literal(usize),
    UnsupportedExpr,
}
```

Expand description

Different types of skip expression in Limit plan.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SkipType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SkipType.html#variant.Literal" class="anchor">§</a>

### Literal(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

The skip expression is a literal value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SkipType.html#variant.UnsupportedExpr" class="anchor">§</a>

### UnsupportedExpr

Currently only supports expressions that can be folded into constants.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SkipType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.SkipType.html#blanket-implementations" class="anchor">§</a>
