# Enum EmitTo Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/groups_accumulator.rs.html#25" class="src">Source</a>

``` rust
pub enum EmitTo {
    All,
    First(usize),
}
```

Expand description

Describes how many rows should be emitted during grouping.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#variant.All" class="anchor">§</a>

### All

Emit all groups

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#variant.First" class="anchor">§</a>

### First(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Emit only the first `n` groups and shift all existing group indexes down by `n`.

For example, if `n=10`, group_index `0, 1, ... 9` are emitted and group indexes `10, 11, 12, ...` become `0, 1, 2, ...`.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#impl-EmitTo" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#method.take_needed" class="fn">take_needed</a>\<T\>(&self, v: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>

Removes the number of rows from `v` required to emit the right number of rows, returning a `Vec` with elements taken, and the remaining values in `v`.

This avoids copying if Self::All

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#impl-Clone-for-EmitTo" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#impl-Debug-for-EmitTo" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#impl-PartialEq-for-EmitTo" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#impl-Copy-for-EmitTo" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#impl-Eq-for-EmitTo" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#impl-StructuralPartialEq-for-EmitTo" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#blanket-implementations" class="anchor">§</a>
