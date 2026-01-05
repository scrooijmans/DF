# Enum ReversedUDAF Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/udaf.rs.html#1004" class="src">Source</a>

``` rust
pub enum ReversedUDAF {
    Identical,
    NotSupported,
    Reversed(Arc<AggregateUDF>),
}
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDAF.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDAF.html#variant.Identical" class="anchor">§</a>

### Identical

The expression is the same as the original expression, like SUM, COUNT

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDAF.html#variant.NotSupported" class="anchor">§</a>

### NotSupported

The expression does not support reverse calculation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDAF.html#variant.Reversed" class="anchor">§</a>

### Reversed(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>)

The expression is different from the original expression

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDAF.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDAF.html#blanket-implementations" class="anchor">§</a>
