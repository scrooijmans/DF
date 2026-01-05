# Enum EmissionType Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#839" class="src">Source</a>

``` rust
pub enum EmissionType {
    Incremental,
    Final,
    Both,
}
```

Expand description

Represents how an operator emits its output records.

This is used to determine whether an operator emits records incrementally as they arrive, only emits a final result at the end, or can do both. Note that it generates the output – record batch with `batch_size` rows but it may still buffer data internally until it has enough data to emit a record batch or the source is exhausted.

For example, in the following plan:

``` text
  SortExec [EmissionType::Final]
    |_ on: [col1 ASC]
    FilterExec [EmissionType::Incremental]
      |_ pred: col2 > 100
      DataSourceExec [EmissionType::Incremental]
        |_ file: "data.csv"
```

- DataSourceExec emits records incrementally as it reads from the file
- FilterExec processes and emits filtered records incrementally as they arrive
- SortExec must wait for all input records before it can emit the sorted result, since it needs to see all values to determine their final order

Left joins can emit both incrementally and finally:

- Incrementally emit matches as they are found
- Finally emit non-matches after all input is processed

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#variant.Incremental" class="anchor">§</a>

### Incremental

Records are emitted incrementally as they arrive and are processed

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#variant.Final" class="anchor">§</a>

### Final

Records are only emitted once all input has been processed

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#variant.Both" class="anchor">§</a>

### Both

Records can be emitted both incrementally and as a final result

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#impl-Clone-for-EmissionType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#impl-Debug-for-EmissionType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#impl-PartialEq-for-EmissionType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#impl-Copy-for-EmissionType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#impl-Eq-for-EmissionType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#impl-StructuralPartialEq-for-EmissionType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html#blanket-implementations" class="anchor">§</a>
