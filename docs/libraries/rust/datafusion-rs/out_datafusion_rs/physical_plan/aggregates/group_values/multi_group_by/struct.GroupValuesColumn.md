# Struct GroupValuesColumn Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/aggregates/group_values/multi_group_by/mod.rs.html#169" class="src">Source</a>

``` rust
pub struct GroupValuesColumn<const STREAMING: bool> { /* private fields */ }
```

Expand description

A [`GroupValues`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html "trait datafusion::physical_plan::aggregates::group_values::GroupValues") that stores multiple columns of group values, and supports vectorized operators for them

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#impl-GroupValuesColumn%3CSTREAMING%3E" class="anchor">§</a>

### impl\<const STREAMING: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html" class="struct" title="struct datafusion::physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn">GroupValuesColumn</a>\<STREAMING\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#method.try_new" class="fn">try_new</a>( schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html" class="struct" title="struct datafusion::physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn">GroupValuesColumn</a>\<STREAMING\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a new instance of GroupValuesColumn if supported for the specified schema

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#impl-GroupValues-for-GroupValuesColumn%3CSTREAMING%3E" class="anchor">§</a>

### impl\<const STREAMING: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html" class="trait" title="trait datafusion::physical_plan::aggregates::group_values::GroupValues">GroupValues</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html" class="struct" title="struct datafusion::physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn">GroupValuesColumn</a>\<STREAMING\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#method.intern" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html#tymethod.intern" class="fn">intern</a>( &mut self, cols: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], groups: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Calculates the group id for each input row of `cols`, assigning new group ids as necessary. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html#tymethod.intern)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#method.size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of bytes of memory used by this [`GroupValues`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html "trait datafusion::physical_plan::aggregates::group_values::GroupValues")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this [`GroupValues`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html "trait datafusion::physical_plan::aggregates::group_values::GroupValues") is empty

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The number of values (distinct group values) stored in this [`GroupValues`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html "trait datafusion::physical_plan::aggregates::group_values::GroupValues")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#method.emit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html#tymethod.emit" class="fn">emit</a>( &mut self, emit_to: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Emits the group values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#method.clear_shrink" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/trait.GroupValues.html#tymethod.clear_shrink" class="fn">clear_shrink</a>(&mut self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>)

Clear the contents and shrink the capacity to the size of the batch (free up memory usage)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/group_values/multi_group_by/struct.GroupValuesColumn.html#blanket-implementations" class="anchor">§</a>
