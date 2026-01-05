# Enum Boundedness Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#794" class="src">Source</a>

``` rust
pub enum Boundedness {
    Bounded,
    Unbounded {
        requires_infinite_memory: bool,
    },
}
```

Expand description

Represents whether a stream of data **generated** by an operator is bounded (finite) or unbounded (infinite).

This is used to determine whether an execution plan will eventually complete processing all its data (bounded) or could potentially run forever (unbounded).

For unbounded streams, it also tracks whether the operator requires finite memory to process the stream or if memory usage could grow unbounded.

Boundedness of the output stream is based on the the boundedness of the input stream and the nature of the operator. For example, limit or topk with fetch operator can convert an unbounded stream to a bounded stream.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#variant.Bounded" class="anchor">§</a>

### Bounded

The data stream is bounded (finite) and will eventually complete

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#variant.Unbounded" class="anchor">§</a>

### Unbounded

The data stream is unbounded (infinite) and could run forever

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#variant.Unbounded.field.requires_infinite_memory" class="anchor field">§</a>`requires_infinite_memory: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether this operator requires infinite memory to process the unbounded stream. If false, the operator can process an infinite stream with bounded memory. If true, memory usage may grow unbounded while processing the stream.

For example, `Median` requires infinite memory to compute the median of an unbounded stream. `Min/Max` requires infinite memory if the stream is unordered, but can be computed with bounded memory if the stream is ordered.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#impl-Boundedness" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#method.is_unbounded" class="fn">is_unbounded</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#impl-Clone-for-Boundedness" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#impl-Debug-for-Boundedness" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#impl-PartialEq-for-Boundedness" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#impl-Copy-for-Boundedness" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#impl-Eq-for-Boundedness" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#impl-StructuralPartialEq-for-Boundedness" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html#blanket-implementations" class="anchor">§</a>
