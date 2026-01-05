# Struct SortMergeJoinExec Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/joins/sort_merge_join/exec.rs.html#105" class="src">Source</a>

``` rust
pub struct SortMergeJoinExec {
    pub left: Arc<dyn ExecutionPlan>,
    pub right: Arc<dyn ExecutionPlan>,
    pub on: Vec<(Arc<dyn PhysicalExpr>, Arc<dyn PhysicalExpr>)>,
    pub filter: Option<JoinFilter>,
    pub join_type: JoinType,
    pub sort_options: Vec<SortOptions>,
    pub null_equality: NullEquality,
    /* private fields */
}
```

Expand description

Join execution plan that executes equi-join predicates on multiple partitions using Sort-Merge join algorithm and applies an optional filter post join. Can be used to join arbitrarily large inputs where one or both of the inputs don’t fit in the available memory.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#join-expressions" class="doc-anchor">§</a>Join Expressions

Equi-join predicate (e.g. `<col1> = <col2>`) expressions are represented by [`Self::on`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.on "method datafusion::physical_plan::joins::SortMergeJoinExec::on").

Non-equality predicates, which can not be pushed down to join inputs (e.g. `<col1> != <col2>`) are known as “filter expressions” and are evaluated after the equijoin predicates. They are represented by [`Self::filter`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.filter "method datafusion::physical_plan::joins::SortMergeJoinExec::filter"). These are optional expressions.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#sorting" class="doc-anchor">§</a>Sorting

Assumes that both the left and right input to the join are pre-sorted. It is not the responsibility of this execution plan to sort the inputs.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#streamed-vs-buffered" class="doc-anchor">§</a>“Streamed” vs “Buffered”

The number of record batches of streamed input currently present in the memory will depend on the output batch size of the execution plan. There is no spilling support for streamed input. The comparisons are performed from values of join keys in streamed input with the values of join keys in buffered input. One row in streamed record batch could be matched with multiple rows in buffered input batches. The streamed input is managed through the states in `StreamedState` and streamed input batches are represented by `StreamedBatch`.

Buffered input is buffered for all record batches having the same value of join key. If the memory limit increases beyond the specified value and spilling is enabled, buffered batches could be spilled to disk. If spilling is disabled, the execution will fail under the same conditions. Multiple record batches of buffered could currently reside in memory/disk during the execution. The number of buffered batches residing in memory/disk depends on the number of rows of buffered input having the same value of join key as that of streamed input rows currently present in memory. Due to pre-sorted inputs, the algorithm understands when it is not needed anymore, and releases the buffered batches from memory/disk. The buffered input is managed through the states in `BufferedState` and buffered input batches are represented by `BufferedBatch`.

Depending on the type of join, left or right input may be selected as streamed or buffered respectively. For example, in a left-outer join, the left execution plan will be selected as streamed input while in a right-outer join, the right execution plan will be selected as the streamed input.

Reference for the algorithm: <https://en.wikipedia.org/wiki/Sort-merge_join>.

Helpful short video demonstration: <https://www.youtube.com/watch?v=jiWCPJtDE2c>.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#structfield.left" class="anchor field">§</a>`left: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan"><code>ExecutionPlan</code></a>`>`

Left sorted joining execution plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#structfield.right" class="anchor field">§</a>`right: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan"><code>ExecutionPlan</code></a>`>`

Right sorting joining execution plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#structfield.on" class="anchor field">§</a>`on: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr"><code>PhysicalExpr</code></a>`>, `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr"><code>PhysicalExpr</code></a>`>)>`

Set of common columns used to join on

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#structfield.filter" class="anchor field">§</a>`filter: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/struct.JoinFilter.html" class="struct" title="struct datafusion::physical_plan::joins::utils::JoinFilter"><code>JoinFilter</code></a>`>`

Filters which are applied while finding matching rows

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#structfield.join_type" class="anchor field">§</a>`join_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType"><code>JoinType</code></a>

How the join is performed

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#structfield.sort_options" class="anchor field">§</a>`sort_options: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html" class="struct" title="struct datafusion::common::arrow::compute::SortOptions"><code>SortOptions</code></a>`>`

Sort options of join columns used in sorting left and right execution plans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#structfield.null_equality" class="anchor field">§</a>`null_equality: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.NullEquality.html" class="enum" title="enum datafusion::common::NullEquality"><code>NullEquality</code></a>

Defines the null equality for the join.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#impl-SortMergeJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SortMergeJoinExec">SortMergeJoinExec</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.try_new" class="fn">try_new</a>( left: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, right: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, on: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>)\>, filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/struct.JoinFilter.html" class="struct" title="struct datafusion::physical_plan::joins::utils::JoinFilter">JoinFilter</a>\>, join_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>, sort_options: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html" class="struct" title="struct datafusion::common::arrow::compute::SortOptions">SortOptions</a>\>, null_equality: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.NullEquality.html" class="enum" title="enum datafusion::common::NullEquality">NullEquality</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SortMergeJoinExec">SortMergeJoinExec</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Tries to create a new [SortMergeJoinExec](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html "struct datafusion::physical_plan::joins::SortMergeJoinExec"). The inputs are sorted using `sort_options` are applied to the columns in the `on`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#error" class="doc-anchor">§</a>Error

This function errors when it is not possible to join the left and right sides on keys `on`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.probe_side" class="fn">probe_side</a>(join_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>

Get probe side (e.g streaming side) information for this sort merge join. In current implementation, probe side is determined according to join type.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.on" class="fn">on</a>(&self) -\> &\[(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>)\]

Set of common columns used to join on

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.right" class="fn">right</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>

Ref to right execution plan

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.join_type" class="fn">join_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>

Join type

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.left" class="fn">left</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>

Ref to left execution plan

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.filter" class="fn">filter</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/struct.JoinFilter.html" class="struct" title="struct datafusion::physical_plan::joins::utils::JoinFilter">JoinFilter</a>\>

Ref to join filter

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.sort_options" class="fn">sort_options</a>(&self) -\> &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html" class="struct" title="struct datafusion::common::arrow::compute::SortOptions">SortOptions</a>\]

Ref to sort options

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.null_equality" class="fn">null_equality</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.NullEquality.html" class="enum" title="enum datafusion::common::NullEquality">NullEquality</a>

Null equality

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.swap_inputs" class="fn">swap_inputs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#notes" class="doc-anchor">§</a>Notes:

This function should be called BEFORE inserting any repartitioning operators on the join’s children. Check [`super::super::HashJoinExec::swap_inputs`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html#method.swap_inputs "method datafusion::physical_plan::joins::HashJoinExec::swap_inputs") for more details.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#impl-Clone-for-SortMergeJoinExec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SortMergeJoinExec">SortMergeJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SortMergeJoinExec">SortMergeJoinExec</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#impl-Debug-for-SortMergeJoinExec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SortMergeJoinExec">SortMergeJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#impl-DisplayAs-for-SortMergeJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SortMergeJoinExec">SortMergeJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.fmt_as" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html#tymethod.fmt_as" class="fn">fmt_as</a>( &self, t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Format according to `DisplayFormatType`, used when verbose representation looks different from the default one [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html#tymethod.fmt_as)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#impl-ExecutionPlan-for-SortMergeJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SortMergeJoinExec">SortMergeJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.try_swapping_with_projection" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.try_swapping_with_projection" class="fn">try_swapping_with_projection</a>( &self, projection: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html" class="struct" title="struct datafusion::physical_plan::projection::ProjectionExec">ProjectionExec</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Tries to swap the projection with its input [`SortMergeJoinExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html "struct datafusion::physical_plan::joins::SortMergeJoinExec"). If it can be done, it returns the new swapped version having the [`SortMergeJoinExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html "struct datafusion::physical_plan::joins::SortMergeJoinExec") as the top plan. Otherwise, it returns None.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.name" class="fn">name</a>(&self) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Short name for the ExecutionPlan, such as ‘DataSourceExec’. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.name)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the execution plan as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.properties" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties" class="fn">properties</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Return properties of the output of the `ExecutionPlan`, such as output ordering(s), partitioning information etc. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.required_input_distribution" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_distribution" class="fn">required_input_distribution</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html" class="enum" title="enum datafusion::physical_expr::Distribution">Distribution</a>\>

Specifies the data distribution requirements for all the children for this `ExecutionPlan`, By default it’s \[[Distribution::UnspecifiedDistribution](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#variant.UnspecifiedDistribution "variant datafusion::physical_expr::Distribution::UnspecifiedDistribution")\] for each child,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.required_input_ordering" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_ordering" class="fn">required_input_ordering</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.OrderingRequirements.html" class="enum" title="enum datafusion::physical_expr::OrderingRequirements">OrderingRequirements</a>\>\>

Specifies the ordering required for all of the children of this `ExecutionPlan`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_ordering)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.maintains_input_order" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.maintains_input_order" class="fn">maintains_input_order</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Returns `false` if this `ExecutionPlan`’s implementation may reorder rows within or between partitions. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.maintains_input_order)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.children" class="fn">children</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Get a list of children `ExecutionPlan`s that act as inputs to this plan. The returned list will be empty for leaf nodes such as scans, will contain a single value for unary nodes, or two values for binary nodes (such as joins).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.with_new_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.with_new_children" class="fn">with_new_children</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SortMergeJoinExec">SortMergeJoinExec</a>\>, children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a new `ExecutionPlan` where all existing children were replaced by the `children`, in order

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.execute" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute" class="fn">execute</a>( &self, partition: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, context: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Begin execution of `partition`, returning a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.metrics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.metrics" class="fn">metrics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::MetricsSet">MetricsSet</a>\>

Return a snapshot of the set of [`Metric`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html "struct datafusion::physical_plan::Metric")s for this [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan"). If no `Metric`s are available, return None. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.metrics)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.statistics" class="fn">statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 48.0.0: Use `partition_statistics` method instead

Returns statistics for this `ExecutionPlan` node. If statistics are not available, should return [`Statistics::new_unknown`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html#method.new_unknown "associated function datafusion::common::Statistics::new_unknown") (the default), not an error. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.statistics)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.partition_statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.partition_statistics" class="fn">partition_statistics</a>( &self, partition: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns statistics for a specific partition of this `ExecutionPlan` node. If statistics are not available, should return [`Statistics::new_unknown`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html#method.new_unknown "associated function datafusion::common::Statistics::new_unknown") (the default), not an error. If `partition` is `None`, it returns statistics for the entire plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.static_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.static_name" class="fn">static_name</a>() -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Short name for the ExecutionPlan, such as ‘DataSourceExec’. Like [`name`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.name "method datafusion::physical_plan::ExecutionPlan::name") but can be called without an instance.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get the schema for this execution plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.check_invariants" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.check_invariants" class="fn">check_invariants</a>(&self, check: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.InvariantLevel.html" class="enum" title="enum datafusion::physical_plan::execution_plan::InvariantLevel">InvariantLevel</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns an error if this individual node does not conform to its invariants. These invariants are typically only checked in debug mode. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.check_invariants)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.benefits_from_input_partitioning" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.benefits_from_input_partitioning" class="fn">benefits_from_input_partitioning</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Specifies whether the `ExecutionPlan` benefits from increased parallelization at its input for each child. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.benefits_from_input_partitioning)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.reset_state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.reset_state" class="fn">reset_state</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Reset any internal state within this [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.reset_state)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.repartitioned" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.repartitioned" class="fn">repartitioned</a>( &self, \_target_partitions: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported, attempt to increase the partitioning of this `ExecutionPlan` to produce `target_partitions` partitions. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.repartitioned)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.supports_limit_pushdown" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.supports_limit_pushdown" class="fn">supports_limit_pushdown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if a limit can be safely pushed down through this `ExecutionPlan` node. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.supports_limit_pushdown)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.with_fetch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_fetch" class="fn">with_fetch</a>(&self, \_limit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Returns a fetching variant of this `ExecutionPlan` node, if it supports fetch limits. Returns `None` otherwise.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.fetch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.fetch" class="fn">fetch</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Gets the fetch count for the operator, `None` means there is no fetch.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.cardinality_effect" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.cardinality_effect" class="fn">cardinality_effect</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html" class="enum" title="enum datafusion::physical_plan::execution_plan::CardinalityEffect">CardinalityEffect</a>

Gets the effect on cardinality, if known

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.gather_filters_for_pushdown" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown" class="fn">gather_filters_for_pushdown</a>( &self, \_phase: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>, parent_filters: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterDescription.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterDescription">FilterDescription</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Collect filters that this node can push down to its children. Filters that are being pushed down from parents are passed in, and the node may generate additional filters to push down. For example, given the plan FilterExec -\> HashJoinExec -\> DataSourceExec, what will happen is that we recurse down the plan calling `ExecutionPlan::gather_filters_for_pushdown`: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.handle_child_pushdown_result" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result" class="fn">handle_child_pushdown_result</a>( &self, \_phase: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>, child_pushdown_result: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildPushdownResult.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::ChildPushdownResult">ChildPushdownResult</a>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Handle the result of a child pushdown. This method is called as we recurse back up the plan tree after pushing filters down to child nodes via [`ExecutionPlan::gather_filters_for_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown "method datafusion::physical_plan::ExecutionPlan::gather_filters_for_pushdown"). It allows the current node to process the results of filter pushdown from its children, deciding whether to absorb filters, modify the plan, or pass filters back up to its parent. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#method.with_new_state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_new_state" class="fn">with_new_state</a>( &self, \_state: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Injects arbitrary run-time state into this execution plan, returning a new plan instance that incorporates that state *if* it is relevant to the concrete node implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_new_state)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html#blanket-implementations" class="anchor">§</a>
