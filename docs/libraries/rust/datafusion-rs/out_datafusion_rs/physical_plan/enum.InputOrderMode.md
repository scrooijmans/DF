# Enum InputOrderMode Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/ordering.rs.html#43" class="src">Source</a>

``` rust
pub enum InputOrderMode {
    Linear,
    PartiallySorted(Vec<usize>),
    Sorted,
}
```

Expand description

Specifies how the input to an aggregation or window operator is ordered relative to their `GROUP BY` or `PARTITION BY` expressions.

For example, if the existing ordering is `[a ASC, b ASC, c ASC]`

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#window-functions" class="doc-anchor">§</a>Window Functions

- A `PARTITION BY b` clause can use `Linear` mode.
- A `PARTITION BY a, c` or a `PARTITION BY c, a` can use `PartiallySorted([0])` or `PartiallySorted([1])` modes, respectively. (The vector stores the index of `a` in the respective PARTITION BY expression.)
- A `PARTITION BY a, b` or a `PARTITION BY b, a` can use `Sorted` mode.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#aggregations" class="doc-anchor">§</a>Aggregations

- A `GROUP BY b` clause can use `Linear` mode, as the only one permutation `[b]` cannot satisfy the existing ordering.
- A `GROUP BY a, c` or a `GROUP BY c, a` can use `PartiallySorted([0])` or `PartiallySorted([1])` modes, respectively, as the permutation `[a]` satisfies the existing ordering. (The vector stores the index of `a` in the respective PARTITION BY expression.)
- A `GROUP BY a, b` or a `GROUP BY b, a` can use `Sorted` mode, as the full permutation `[a, b]` satisfies the existing ordering.

Note these are the same examples as above, but with `GROUP BY` instead of `PARTITION BY` to make the examples easier to read.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#variant.Linear" class="anchor">§</a>

### Linear

There is no partial permutation of the expressions satisfying the existing ordering.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#variant.PartiallySorted" class="anchor">§</a>

### PartiallySorted(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

There is a partial permutation of the expressions satisfying the existing ordering. Indices describing the longest partial permutation are stored in the vector.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#variant.Sorted" class="anchor">§</a>

### Sorted

There is a (full) permutation of the expressions satisfying the existing ordering.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#impl-Clone-for-InputOrderMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html" class="enum" title="enum datafusion::physical_plan::InputOrderMode">InputOrderMode</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html" class="enum" title="enum datafusion::physical_plan::InputOrderMode">InputOrderMode</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#impl-Debug-for-InputOrderMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html" class="enum" title="enum datafusion::physical_plan::InputOrderMode">InputOrderMode</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#impl-PartialEq-for-InputOrderMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html" class="enum" title="enum datafusion::physical_plan::InputOrderMode">InputOrderMode</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html" class="enum" title="enum datafusion::physical_plan::InputOrderMode">InputOrderMode</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#impl-StructuralPartialEq-for-InputOrderMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html" class="enum" title="enum datafusion::physical_plan::InputOrderMode">InputOrderMode</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.InputOrderMode.html#blanket-implementations" class="anchor">§</a>
