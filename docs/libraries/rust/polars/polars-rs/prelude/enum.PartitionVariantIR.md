# Enum PartitionVariantIR Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/options/sink.rs.html#525" class="src">Source</a>

``` rust
pub enum PartitionVariantIR {
    MaxSize(u32),
    Parted {
        key_exprs: Vec<ExprIR>,
        include_key: bool,
    },
    ByKey {
        key_exprs: Vec<ExprIR>,
        include_key: bool,
    },
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#variant.MaxSize" class="anchor">§</a>

### MaxSize(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#variant.Parted" class="anchor">§</a>

### Parted

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#variant.Parted.field.key_exprs" class="anchor field">§</a>`key_exprs: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/expr_ir/struct.ExprIR.html" class="struct" title="struct polars_plan::plans::expr_ir::ExprIR"><code>ExprIR</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#variant.Parted.field.include_key" class="anchor field">§</a>`include_key: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#variant.ByKey" class="anchor">§</a>

### ByKey

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#variant.ByKey.field.key_exprs" class="anchor field">§</a>`key_exprs: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/expr_ir/struct.ExprIR.html" class="struct" title="struct polars_plan::plans::expr_ir::ExprIR"><code>ExprIR</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#variant.ByKey.field.include_key" class="anchor field">§</a>`include_key: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#impl-Clone-for-PartitionVariantIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html" class="enum" title="enum polars::prelude::PartitionVariantIR">PartitionVariantIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html" class="enum" title="enum polars::prelude::PartitionVariantIR">PartitionVariantIR</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#impl-Debug-for-PartitionVariantIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html" class="enum" title="enum polars::prelude::PartitionVariantIR">PartitionVariantIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#impl-PartialEq-for-PartitionVariantIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html" class="enum" title="enum polars::prelude::PartitionVariantIR">PartitionVariantIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html" class="enum" title="enum polars::prelude::PartitionVariantIR">PartitionVariantIR</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#impl-Eq-for-PartitionVariantIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html" class="enum" title="enum polars::prelude::PartitionVariantIR">PartitionVariantIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#impl-StructuralPartialEq-for-PartitionVariantIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html" class="enum" title="enum polars::prelude::PartitionVariantIR">PartitionVariantIR</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html#blanket-implementations" class="anchor">§</a>
