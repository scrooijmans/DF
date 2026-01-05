# Struct Statistics Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/stats.rs.html#270" class="src">Source</a>

``` rust
pub struct Statistics {
    pub num_rows: Precision<usize>,
    pub total_byte_size: Precision<usize>,
    pub column_statistics: Vec<ColumnStatistics>,
}
```

Expand description

Statistics for a relation Fields are optional and can be inexact because the sources sometimes provide approximate estimates for performance reasons and the transformations output are not always predictable.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#structfield.num_rows" class="anchor field">§</a>`num_rows: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision"><code>Precision</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

The number of table rows.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#structfield.total_byte_size" class="anchor field">§</a>`total_byte_size: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision"><code>Precision</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Total bytes of the table rows.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#structfield.column_statistics" class="anchor field">§</a>`column_statistics: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics"><code>ColumnStatistics</code></a>`>`

Statistics on a column level.

It must contains a [`ColumnStatistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html "struct datafusion::common::ColumnStatistics") for each field in the schema of the table to which the [`Statistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html "struct datafusion::common::Statistics") refer.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#impl-Statistics" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.new_unknown" class="fn">new_unknown</a>(schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

Returns a [`Statistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html "struct datafusion::common::Statistics") instance for the given schema by assigning unknown statistics to each column in the schema.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.unknown_column" class="fn">unknown_column</a>(schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>\>

Returns an unbounded `ColumnStatistics` for each field in the schema.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.with_num_rows" class="fn">with_num_rows</a>(self, num_rows: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision">Precision</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

Set the number of rows

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.with_total_byte_size" class="fn">with_total_byte_size</a>( self, total_byte_size: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision">Precision</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

Set the total size, in bytes

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.add_column_statistics" class="fn">add_column_statistics</a>(self, column_stats: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

Add a column to the column statistics

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.to_inexact" class="fn">to_inexact</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

If the exactness of a [`Statistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html "struct datafusion::common::Statistics") instance is lost, this function relaxes the exactness of all information by converting them [`Precision::Inexact`](https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html#variant.Inexact "variant datafusion::common::stats::Precision::Inexact").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.project" class="fn">project</a>(self, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

Project the statistics to the given column indices.

For example, if we had statistics for columns `{"a", "b", "c"}`, projecting to `vec![2, 1]` would return statistics for columns `{"c", "b"}`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.with_fetch" class="fn">with_fetch</a>( self, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, fetch: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, skip: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, n_partitions: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Calculates the statistics after applying `fetch` and `skip` operations.

Here, `self` denotes per-partition statistics. Use the `n_partitions` parameter to compute global statistics in a multi-partition setting.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.try_merge_iter" class="fn">try_merge_iter</a>\<'a, I\>( items: I, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>,

Summarize zero or more statistics into a single `Statistics` instance.

The method assumes that all statistics are for the same schema. If not, maybe you can call `SchemaMapper::map_column_statistics` to make them consistent.

Returns an error if the statistics do not match the specified schemas.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.try_merge" class="fn">try_merge</a>( self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Merge this Statistics value with another Statistics value.

Returns an error if the statistics do not match (different schemas).

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#example" class="doc-anchor">§</a>Example

``` rust
let stats1 = Statistics::default()
  .with_num_rows(Precision::Exact(1))
  .with_total_byte_size(Precision::Exact(2))
  .add_column_statistics(ColumnStatistics::new_unknown()
     .with_null_count(Precision::Exact(3))
     .with_min_value(Precision::Exact(ScalarValue::from(4)))
     .with_max_value(Precision::Exact(ScalarValue::from(5)))
  );

let stats2 = Statistics::default()
  .with_num_rows(Precision::Exact(10))
  .with_total_byte_size(Precision::Inexact(20))
  .add_column_statistics(ColumnStatistics::new_unknown()
      // absent null count
     .with_min_value(Precision::Exact(ScalarValue::from(40)))
     .with_max_value(Precision::Exact(ScalarValue::from(50)))
  );

let merged_stats = stats1.try_merge(&stats2).unwrap();
let expected_stats = Statistics::default()
  .with_num_rows(Precision::Exact(11))
  .with_total_byte_size(Precision::Inexact(22)) // inexact in stats2 --> inexact
  .add_column_statistics(
    ColumnStatistics::new_unknown()
      .with_null_count(Precision::Absent) // missing from stats2 --> absent
      .with_min_value(Precision::Exact(ScalarValue::from(4)))
      .with_max_value(Precision::Exact(ScalarValue::from(50)))
  );

assert_eq!(merged_stats, expected_stats)
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#impl-Clone-for-Statistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#impl-Debug-for-Statistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#impl-Default-for-Statistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

Returns a new [`Statistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html "struct datafusion::common::Statistics") instance with all fields set to unknown and no columns.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#impl-Display-for-Statistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#impl-PartialEq-for-Statistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#impl-Eq-for-Statistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#impl-StructuralPartialEq-for-Statistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Statistics.html#blanket-implementations" class="anchor">§</a>
