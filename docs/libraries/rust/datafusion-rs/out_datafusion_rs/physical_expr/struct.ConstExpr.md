# Struct ConstExpr Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/equivalence/class.rs.html#88" class="src">Source</a>

``` rust
pub struct ConstExpr {
    pub expr: Arc<dyn PhysicalExpr>,
    pub across_partitions: AcrossPartitions,
}
```

Expand description

A structure representing a expression known to be constant in a physical execution plan.

The `ConstExpr` struct encapsulates an expression that is constant during the execution of a query. For example if a filter like `A = 5` appears earlier in the plan, `A` would become a constant in subsequent operations.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#fields-1" class="doc-anchor">§</a>Fields

- `expr`: Constant expression for a node in the physical plan.
- `across_partitions`: A boolean flag indicating whether the constant expression is the same across partitions. If set to `true`, the constant expression has same value for all partitions. If set to `false`, the constant expression may have different values for different partitions.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#example" class="doc-anchor">§</a>Example

``` rust
let col = lit(5);
// Create a constant expression from a physical expression:
let const_expr = ConstExpr::from(col);
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#structfield.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr"><code>PhysicalExpr</code></a>`>`

The expression that is known to be constant (e.g. a `Column`).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#structfield.across_partitions" class="anchor field">§</a>`across_partitions: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions"><code>AcrossPartitions</code></a>

Indicates whether the constant have the same value across all partitions.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#impl-ConstExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#method.new" class="fn">new</a>( expr: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, across_partitions: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>

Create a new constant expression from a physical expression, specifying whether the constant expression is the same across partitions.

Note that you can also use `ConstExpr::from` to create a constant expression from just a physical expression, with the *safe* assumption of heterogenous values across partitions unless the expression is a literal.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#method.format_list" class="fn">format_list</a>(input: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>\]) -\> impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a>

Returns a [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display")able list of `ConstExpr`.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#impl-Clone-for-ConstExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#impl-Debug-for-ConstExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#impl-Display-for-ConstExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#impl-From%3CArc%3Cdyn+PhysicalExpr%3E%3E-for-ConstExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(expr: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#impl-PartialEq-for-ConstExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html" class="struct" title="struct datafusion::physical_expr::ConstExpr">ConstExpr</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ConstExpr.html#blanket-implementations" class="anchor">§</a>
