# Trait PruningStatistics Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/pruning.rs.html#63" class="src">Source</a>

``` rust
pub trait PruningStatistics {
    // Required methods
    fn min_values(&self, column: &Column) -> Option<Arc<dyn Array>>;
    fn max_values(&self, column: &Column) -> Option<Arc<dyn Array>>;
    fn num_containers(&self) -> usize;
    fn null_counts(&self, column: &Column) -> Option<Arc<dyn Array>>;
    fn row_counts(&self, column: &Column) -> Option<Arc<dyn Array>>;
    fn contained(
        &self,
        column: &Column,
        values: &HashSet<ScalarValue>,
    ) -> Option<BooleanArray>;
}
```

Expand description

A source of runtime statistical information to [`PruningPredicate`](https://docs.rs/datafusion/latest/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html)s.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#supported-information" class="doc-anchor">§</a>Supported Information

1.  Minimum and maximum values for columns

2.  Null counts and row counts for columns

3.  Whether the values in a column are contained in a set of literals

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#vectorized-interface" class="doc-anchor">§</a>Vectorized Interface

Information for containers / files are returned as Arrow [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef"), so the evaluation happens once on a single `RecordBatch`, which amortizes the overhead of evaluating the predicate. This is important when pruning 1000s of containers which often happens in analytic systems that have 1000s of potential files to consider.

For example, for the following three files with a single column `a`:

``` text
file1: column a: min=5, max=10
file2: column a: No stats
file2: column a: min=20, max=30
```

PruningStatistics would return:

``` text
min_values("a") -> Some([5, Null, 20])
max_values("a") -> Some([10, Null, 30])
min_values("X") -> None
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#tymethod.min_values" class="fn">min_values</a>(&self, column: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>

Return the minimum values for the named column, if known.

If the minimum value for a particular container is not known, the returned array should have `null` in that row. If the minimum value is not known for any row, return `None`.

Note: the returned array must contain [`Self::num_containers`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.num_containers "method datafusion_common::pruning::PruningStatistics::num_containers::num_containers") rows

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#tymethod.max_values" class="fn">max_values</a>(&self, column: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>

Return the maximum values for the named column, if known.

See [`Self::min_values`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.min_values "method datafusion_common::pruning::PruningStatistics::min_values::min_values") for when to return `None` and null values.

Note: the returned array must contain [`Self::num_containers`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.num_containers "method datafusion_common::pruning::PruningStatistics::num_containers::num_containers") rows

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#tymethod.num_containers" class="fn">num_containers</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of containers (e.g. Row Groups) being pruned with these statistics.

This value corresponds to the size of the [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef") returned by [`Self::min_values`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.min_values "method datafusion_common::pruning::PruningStatistics::min_values::min_values"), [`Self::max_values`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.max_values "method datafusion_common::pruning::PruningStatistics::max_values::max_values"), [`Self::null_counts`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.null_counts "method datafusion_common::pruning::PruningStatistics::null_counts::null_counts"), and [`Self::row_counts`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.row_counts "method datafusion_common::pruning::PruningStatistics::row_counts::row_counts").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#tymethod.null_counts" class="fn">null_counts</a>(&self, column: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>

Return the number of null values for the named column as an [`UInt64Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.UInt64Array.html "type datafusion::common::arrow::array::UInt64Array")

See [`Self::min_values`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.min_values "method datafusion_common::pruning::PruningStatistics::min_values::min_values") for when to return `None` and null values.

Note: the returned array must contain [`Self::num_containers`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.num_containers "method datafusion_common::pruning::PruningStatistics::num_containers::num_containers") rows

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#tymethod.row_counts" class="fn">row_counts</a>(&self, column: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>

Return the number of rows for the named column in each container as an [`UInt64Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.UInt64Array.html "type datafusion::common::arrow::array::UInt64Array").

See [`Self::min_values`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.min_values "method datafusion_common::pruning::PruningStatistics::min_values::min_values") for when to return `None` and null values.

Note: the returned array must contain [`Self::num_containers`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.num_containers "method datafusion_common::pruning::PruningStatistics::num_containers::num_containers") rows

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#tymethod.contained" class="fn">contained</a>( &self, column: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>, values: &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>

Returns [`BooleanArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html "struct datafusion::common::arrow::array::BooleanArray") where each row represents information known about specific literal `values` in a column.

For example, Parquet Bloom Filters implement this API to communicate that `values` are known not to be present in a Row Group.

The returned array has one row for each container, with the following meanings:

- `true` if the values in `column` ONLY contain values from `values`
- `false` if the values in `column` are NOT ANY of `values`
- `null` if the neither of the above holds or is unknown.

If these statistics can not determine column membership for any container, return `None` (the default).

Note: the returned array must contain [`Self::num_containers`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.num_containers "method datafusion_common::pruning::PruningStatistics::num_containers::num_containers") rows

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#impl-PruningStatistics-for-CompositePruningStatistics" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html" class="trait" title="trait datafusion::common::pruning::PruningStatistics">PruningStatistics</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/struct.CompositePruningStatistics.html" class="struct" title="struct datafusion::common::pruning::CompositePruningStatistics">CompositePruningStatistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#impl-PruningStatistics-for-PartitionPruningStatistics" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html" class="trait" title="trait datafusion::common::pruning::PruningStatistics">PruningStatistics</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/struct.PartitionPruningStatistics.html" class="struct" title="struct datafusion::common::pruning::PartitionPruningStatistics">PartitionPruningStatistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html#impl-PruningStatistics-for-PrunableStatistics" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html" class="trait" title="trait datafusion::common::pruning::PruningStatistics">PruningStatistics</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/struct.PrunableStatistics.html" class="struct" title="struct datafusion::common::pruning::PrunableStatistics">PrunableStatistics</a>
