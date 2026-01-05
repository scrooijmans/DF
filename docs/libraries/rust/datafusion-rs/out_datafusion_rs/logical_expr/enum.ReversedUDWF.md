# Enum ReversedUDWF Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/udwf.rs.html#419" class="src">Source</a>

``` rust
pub enum ReversedUDWF {
    Identical,
    NotSupported,
    Reversed(Arc<WindowUDF>),
}
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDWF.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDWF.html#variant.Identical" class="anchor">§</a>

### Identical

The result of evaluating the user-defined window function remains identical when reversed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDWF.html#variant.NotSupported" class="anchor">§</a>

### NotSupported

A window function which does not support evaluating the result in reverse order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDWF.html#variant.Reversed" class="anchor">§</a>

### Reversed(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html" class="struct" title="struct datafusion::logical_expr::WindowUDF">WindowUDF</a>\>)

Customize the user-defined window function for evaluating the result in reverse order.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDWF.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ReversedUDWF.html#blanket-implementations" class="anchor">§</a>
