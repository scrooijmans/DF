# Enum Distribution Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/partitioning.rs.html#242" class="src">Source</a>

``` rust
pub enum Distribution {
    UnspecifiedDistribution,
    SinglePartition,
    HashPartitioned(Vec<Arc<dyn PhysicalExpr>>),
}
```

Expand description

How data is distributed amongst partitions. See [`Partitioning`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html "enum datafusion::physical_expr::Partitioning") for more details.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#variant.UnspecifiedDistribution" class="anchor">§</a>

### UnspecifiedDistribution

Unspecified distribution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#variant.SinglePartition" class="anchor">§</a>

### SinglePartition

A single partition is required

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#variant.HashPartitioned" class="anchor">§</a>

### HashPartitioned(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>)

Requires children to be distributed in such a way that the same values of the keys end up in the same partition

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#impl-Distribution" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html" class="enum" title="enum datafusion::physical_expr::Distribution">Distribution</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#method.create_partitioning" class="fn">create_partitioning</a>(self, partition_count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

Creates a `Partitioning` that satisfies this `Distribution`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#impl-Clone-for-Distribution" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html" class="enum" title="enum datafusion::physical_expr::Distribution">Distribution</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html" class="enum" title="enum datafusion::physical_expr::Distribution">Distribution</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#impl-Debug-for-Distribution" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html" class="enum" title="enum datafusion::physical_expr::Distribution">Distribution</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#impl-Display-for-Distribution" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html" class="enum" title="enum datafusion::physical_expr::Distribution">Distribution</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#blanket-implementations" class="anchor">§</a>
