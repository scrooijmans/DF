# Enum Partitioning Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/partitioning.rs.html#114" class="src">Source</a>

``` rust
pub enum Partitioning {
    RoundRobinBatch(usize),
    Hash(Vec<Arc<dyn PhysicalExpr>>, usize),
    UnknownPartitioning(usize),
}
```

Expand description

Output partitioning supported by [`ExecutionPlan`](https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html)s.

Calling [`ExecutionPlan::execute`](https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute) produce one or more independent streams of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es in parallel, referred to as partitions. The streams are Rust `async` [`Stream`](https://docs.rs/futures/latest/futures/stream/trait.Stream.html)s (a special kind of future). The number of output partitions varies based on the input and the operation performed.

For example, an `ExecutionPlan` that has output partitioning of 3 will produce 3 distinct output streams as the result of calling `ExecutionPlan::execute(0)`, `ExecutionPlan::execute(1)`, and `ExecutionPlan::execute(2)`, as shown below:

``` text
                                                  ...         ...        ...
              ...                                  ▲           ▲           ▲
                                                   │           │           │
               ▲                                   │           │           │
               │                                   │           │           │
               │                               ┌───┴────┐  ┌───┴────┐  ┌───┴────┐
    ┌────────────────────┐                     │ Stream │  │ Stream │  │ Stream │
    │   ExecutionPlan    │                     │  (0)   │  │  (1)   │  │  (2)   │
    └────────────────────┘                     └────────┘  └────────┘  └────────┘
               ▲                                   ▲           ▲           ▲
               │                                   │           │           │
    ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─                          │           │           │
            Input        │                         │           │           │
    └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─                          │           │           │
               ▲                               ┌ ─ ─ ─ ─   ┌ ─ ─ ─ ─   ┌ ─ ─ ─ ─
               │                                 Input  │    Input  │    Input  │
               │                               │ Stream    │ Stream    │ Stream
                                                  (0)   │     (1)   │     (2)   │
              ...                              └ ─ ▲ ─ ─   └ ─ ▲ ─ ─   └ ─ ▲ ─ ─
                                                   │           │           │
                                                   │           │           │
                                                   │           │           │

ExecutionPlan with 1 input                      3 (async) streams, one for each
that has 3 partitions, which itself             output partition
has 3 output partitions
```

It is common (but not required) that an `ExecutionPlan` has the same number of input partitions as output partitions. However, some plans have different numbers such as the `RepartitionExec` that redistributes batches from some number of inputs to some number of outputs

``` text
              ...                                     ...         ...        ...

                                                       ▲           ▲           ▲
               ▲                                       │           │           │
               │                                       │           │           │
      ┌────────┴───────────┐                           │           │           │
      │  RepartitionExec   │                      ┌────┴───┐  ┌────┴───┐  ┌────┴───┐
      └────────────────────┘                      │ Stream │  │ Stream │  │ Stream │
               ▲                                  │  (0)   │  │  (1)   │  │  (2)   │
               │                                  └────────┘  └────────┘  └────────┘
               │                                       ▲           ▲           ▲
               ...                                     │           │           │
                                                       └──────────┐│┌──────────┘
                                                                  │││
                                                                  │││
RepartitionExec with 1 input
partition and 3 output partitions                 3 (async) streams, that internally
                                                   pull from the same input stream
                                                                 ...
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#additional-examples" class="doc-anchor">§</a>Additional Examples

A simple `FileScanExec` might produce one output stream (partition) for each file (note the actual DataFusion file scanners can read individual files in parallel, potentially producing multiple partitions per file)

Plans such as `SortPreservingMerge` produce a single output stream (1 output partition) by combining some number of input streams (input partitions)

Plans such as `FilterExec` produce the same number of output streams (partitions) as input streams (partitions).

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#variant.RoundRobinBatch" class="anchor">§</a>

### RoundRobinBatch(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Allocate batches using a round-robin algorithm and the specified number of partitions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#variant.Hash" class="anchor">§</a>

### Hash(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Allocate rows based on a hash of one of more expressions and the specified number of partitions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#variant.UnknownPartitioning" class="anchor">§</a>

### UnknownPartitioning(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Unknown partitioning scheme with a known number of partitions

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#impl-Partitioning" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#method.partition_count" class="fn">partition_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of partitions in this partitioning scheme

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#method.satisfy" class="fn">satisfy</a>( &self, required: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html" class="enum" title="enum datafusion::physical_expr::Distribution">Distribution</a>, eq_properties: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true when the guarantees made by this [`Partitioning`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html "enum datafusion::physical_expr::Partitioning") are sufficient to satisfy the partitioning scheme mandated by the `required` [`Distribution`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html "enum datafusion::physical_expr::Distribution").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#method.project" class="fn">project</a>( &self, mapping: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/equivalence/struct.ProjectionMapping.html" class="struct" title="struct datafusion::physical_expr::equivalence::ProjectionMapping">ProjectionMapping</a>, input_eq_properties: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

Calculate the output partitioning after applying the given projection.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#impl-Clone-for-Partitioning" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#impl-Debug-for-Partitioning" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#impl-Display-for-Partitioning" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#impl-PartialEq-for-Partitioning" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html#blanket-implementations" class="anchor">§</a>
