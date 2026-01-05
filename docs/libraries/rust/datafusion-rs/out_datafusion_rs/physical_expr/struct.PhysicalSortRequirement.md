# Struct PhysicalSortRequirement Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/sort_expr.rs.html#243" class="src">Source</a>

``` rust
pub struct PhysicalSortRequirement {
    pub expr: Arc<dyn PhysicalExpr>,
    pub options: Option<SortOptions>,
}
```

Expand description

Represents sort requirement associated with a plan

If the requirement includes [`SortOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html "struct datafusion::common::arrow::compute::SortOptions") then both the expression *and* the sort options must match.

If the requirement does not include [`SortOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html "struct datafusion::common::arrow::compute::SortOptions")) then only the expressions must match.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#examples" class="doc-anchor">§</a>Examples

With sort options (`A`, `DESC NULLS FIRST`):

- `ORDER BY A DESC NULLS FIRST` matches
- `ORDER BY A ASC NULLS FIRST` does not match (`ASC` vs `DESC`)
- `ORDER BY B DESC NULLS FIRST` does not match (different expr)

Without sort options (`A`, None):

- `ORDER BY A DESC NULLS FIRST` matches
- `ORDER BY A ASC NULLS FIRST` matches (`ASC` and `NULL` options ignored)
- `ORDER BY B DESC NULLS FIRST` does not match (different expr)

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#structfield.expr" class="anchor field">§</a>`expr: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr"><code>PhysicalExpr</code></a>`>`

Physical expression representing the column to sort

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#structfield.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html" class="struct" title="struct datafusion::common::arrow::compute::SortOptions"><code>SortOptions</code></a>`>`

Option to specify how the given column should be sorted. If unspecified, there are no constraints on sort options.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#impl-PhysicalSortRequirement" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#method.new" class="fn">new</a>( expr: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html" class="struct" title="struct datafusion::common::arrow::compute::SortOptions">SortOptions</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>

Creates a new requirement.

If `options` is `Some(..)`, creates an `exact` requirement, which must match both `options` and `expr`.

If `options` is `None`, Creates a new `expr_only` requirement, which must match only `expr`.

See [`PhysicalSortRequirement`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html "struct datafusion::physical_expr::PhysicalSortRequirement") for examples.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#method.compatible" class="fn">compatible</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this requirement is equal or more specific than `other`.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#impl-Clone-for-PhysicalSortRequirement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#impl-Debug-for-PhysicalSortRequirement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#impl-Display-for-PhysicalSortRequirement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#impl-From%3CPhysicalSortExpr%3E-for-PhysicalSortRequirement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#impl-From%3CPhysicalSortRequirement%3E-for-PhysicalSortExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>

The default sort options `ASC, NULLS LAST` when the requirement does not specify sort options. This default is consistent with PostgreSQL.

Reference: <https://www.postgresql.org/docs/current/queries-order.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#impl-PartialEq-for-PhysicalSortRequirement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortRequirement">PhysicalSortRequirement</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortRequirement.html#blanket-implementations" class="anchor">§</a>
